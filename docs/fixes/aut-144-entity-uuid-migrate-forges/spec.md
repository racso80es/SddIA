---
feature_name: aut-144-entity-uuid-migrate-forges
created: "2026-11-28"
process: bug-fix
branch_name: fix/aut-144-entity-uuid-migrate-forges
persist_ref: docs/fixes/aut-144-entity-uuid-migrate-forges
tracker_ref: AUT-144
historia_ref: AUT-133
precedent_ref: AUT-137
execution_id: "d0b4e016-3133-4fdd-ba51-38cd393156cc"
document_id: PBI-c4982d86
---

# Especificación — AUT-144

## Objetivo

Extender a las forjas **process / event / agent / skill / suite** la misma capacidad atómica de `entity_uuid_migrate` ya operable en **action** (AUT-137): migrar UUID del frontmatter, sincronizar la celda UUID del `index.md` de la familia y transferir cobertura EDA (`transfer_entity_coverage`), bajo rollback que no deje drift parcial artefacto↔índice↔matriz.

Queda **fuera de alcance**: reimplementar migrate de action; anemia de contenido (AUT-133); peaje/oráculo (AUT-132); forjas ajenas; strip ad-hoc de índices/`eda-coverage`; renombrado de archivos.

## Diagnóstico estabilizado

| Hecho | Evidencia |
|-------|-----------|
| Canal seed ya propaga | `entity_manager::with_forge_seed_optionals` copia `entity_uuid_migrate` a inputs de **cualquier** clase |
| Solo action consume | `run_action_forge` (update) llama `migrate_entity_uuid_record`; process/event/agent/skill/suite **no** |
| Helper índice acoplado a actions | `sync_action_index_entity_uuid` exige `cols.len()==8` y reconstruye fila con layout Name\|UUID\|… (6 celdas) |
| Residual gate | ~20 paths UUID-only fuera de action (7 process + 7 event + 3 agent + 3 skill) ancla `b8a9d377` |
| Testigo drift suite | `fail-fast-lab`: en main artefacto/índice/coverage alineados en `c3d4e5f6-…`; post-lote puede divergir (artefacto v4 vs índice legacy + ambas claves en `eda-coverage.json`) |

## Diseño

### D1 — Contrato semántico (paridad action)

- Input opcional `entity_uuid_migrate`: UUID v4 destino (validación `is_uuid_v4`).
- Flujo único reutilizado: `migrate_entity_uuid_record` (FM → sync índice → `transfer_entity_coverage`).
- Idempotencia: old==new → `{ skipped: true }` sin mutación.
- Salida de forja: incluir `entity_uuid_migration` / `entity_uuid_new` como action cuando hubo migrate.
- Seed: **sin cambios** en `with_forge_seed_optionals` (ya reenvía la clave).

### D2 — Generalizar sync de celda UUID (bloqueo técnico)

`sync_action_index_entity_uuid` **no** sirve tal cual para agents/skills/suites/events (otra clave de match y distinto `cols.len()`).

**Decisión:** introducir helper parametrizado en `forges/common.rs` y adaptar `migrate_entity_uuid_record` para recibirlo (o un layout):

```text
sync_index_entity_uuid(
  index_path,
  match_col,      // índice 1-based de celda tras split '|'
  match_value,    // "name" | "`name.md`"
  uuid_col,       // siempre 2ª celda de datos en las familias objetivo
  new_uuid,
) -> Result<(), String>
```

Reglas del helper:

1. Iterar filas `|…|`; ignorar separadores/cabeceras.
2. Match exacto `cols[match_col].trim() == match_value` (aceptar también variante con/sin backticks si hace falta para resiliencia).
3. Sustituir **solo** la celda UUID (`cols[uuid_col]`) por `` `{new_uuid}` ``; **preservar** el resto de celdas sin `format!` fijo ni `len==8`.
4. Error si no hay fila para la entidad.

Thin wrappers / layouts por familia (solo preparan match):

| Familia | `index_path` | Match | Notas |
|---------|--------------|-------|-------|
| process | `{dest_root}/index.md` (`artifact_process_index`) | Name = `name` (col Name) | Layout Name\|UUID\|… análogo a actions |
| event | `{events_root}/index.md` (**familia** domain/telemetry/orchestration; **no** root) | `` `{name}.md` `` (Archivo fuente) o col `name` | Root solo census; no lleva UUID de clase |
| agent | `SddIA/agents/index.md` | `` `{name}.md` `` / `name` | |
| skill | `SddIA/skills/index.md` | `` `{name}.md` `` / `name` | Update hoy no toca índice → debe hacerlo en migrate |
| suite | `SddIA/suites/index.md` | `` `{name}.md` `` / `name` | Idem |
| action | `SddIA/actions/index.md` | Name = `name` | Reescribir `sync_action_index_entity_uuid` como delegado del helper genérico (sin regresión AUT-137) |

### D3 — Call sites en `factory.rs` (update)

En cada `run_{process,event,agent,skill,suite}_forge`, rama **update**, espejo de action:

1. Leer `optional_str(inputs, "entity_uuid_migrate")`.
2. Si presente, ejecutar migrate **dentro** del mismo perímetro atómico que el update mutante (ver D4), **después** de patches YAML/body del artefacto y **antes** de validaciones anti-anemia que dependan del UUID final.
3. Pasar el `index_path` correcto de la familia (tabla D2).
4. Propagar meta `entity_uuid_migration` al JSON de salida; `handoff_uuid` / sello emitir con UUID nuevo si hubo migrate.

**Process:** hoy varias ramas update no pasan por rollback unificado; si hay migrate, **obligar** camino con rollback + sync índice (aunque el resto del update sea version-only).

**Skill:** update actual no sync índice ni rollback; migrate exige envolver mutación+sync en D4.

**Event:** índice de **familia** (`events_root.join("index.md")`), nunca `SddIA/events/index.md`.

**Action:** no tocar semántica salvo adaptar al helper genérico.

### D4 — Atomicidad y rollback (AC-4)

AC-4 exige restaurar **artefacto e índice** ante fallo intermedio (paridad funcional action + cierre explícito del fix).

**Decisión:**

1. Snapshot previo de **artefacto** y del **`index.md` de familia** tocado por migrate.
2. Ejecutar mutaciones (patches + `migrate_entity_uuid_record`) en un closure; ante `Err`, restaurar ambos paths byte-a-byte.
3. `transfer_entity_coverage` corre dentro del mismo closure: si falla post-FM/índice, el restore de artefacto+índice deja la matriz como única superficie a reconciliar; preferible orden **FM → índice → coverage** (igual action) y, si coverage falla, revertir FM+índice para no dejar triple drift. Si la matriz ya se escribió y luego falla otra validación, revertir también coverage cuando sea viable (snapshot de clave old/new o re-transfer inverso old←new); documentar en implementation si el motor solo garantiza artefacto+índice y coverage best-effort — **objetivo de cierre:** test de rollback sin UUID nuevo en índice ni cobertura huérfana del new.

Patrón de referencia ya presente: agent update con restore manual de índice ante fallo de `upsert_cognitive_role_index`. Preferible factorizar `with_paths_rollback(&[artifact, index], …)` en `common.rs` si reduce duplicación; no es obligatorio si cada forja snapshottea explícitamente.

`with_file_rollback` actual (solo un path) **no basta** solo para AC-4 de este ítem.

### D5 — Remediación demostrable (AC-5)

Tras el canal operable:

1. Migrar vía `entity-manager` update + `entity_uuid_migrate` los **20** paths residuales UUID-only (inventario ancla post-gate / `b8a9d377`).
2. Alinear `fail-fast-lab`: UUID v4 único en artefacto, `SddIA/suites/index.md` y una sola clave en `SddIA/core/eda-coverage.json` (sin huérfano legacy).
3. Tests de regresión `execute-process` en verde (incl. AUT-137 action + nuevos).

La remediación del lote es **entrega del fix** (no solo capacidad latente), sin bypass manual de genoma.

## Superficies de código

| Área | Path |
|------|------|
| Call sites forja | `SddIA/engine/execute-process/src/forges/factory.rs` |
| Migrate + sync + rollback | `SddIA/engine/execute-process/src/forges/common.rs` |
| Coverage | `SddIA/engine/execute-process/src/engine/eda_coverage.rs` (`transfer_entity_coverage` — reutilizar) |
| Seed | `SddIA/engine/execute-process/src/engine/entity_manager.rs` (sin cambio de contrato) |
| Tests | `common.rs` (ampliar fixture multi-familia); factory/integration update+migrate+rollback por familia |
| Artefactos remediación | 20 paths residuales + `SddIA/suites/fail-fast-lab.md` + índices/coverage |

## Criterios de aceptación → verificación

| AC | Verificación |
|----|--------------|
| AC-1 | Update + `entity_uuid_migrate` en process, event, agent, skill, suite cambia UUID FM; éxito o rollback total |
| AC-2 | Fila correspondiente en `index.md` de familia muestra UUID nuevo; fallo de sync aborta/revierte |
| AC-3 | Post-éxito: `eda-coverage.json` sin clave old del testigo; cobertura bajo new |
| AC-4 | Inyectar error tras FM o tras índice → artefacto e índice restaurados; sin drift parcial observable |
| AC-5 | 20 residuales migrados por canal; `fail-fast-lab` alineado; suite tests execute-process verde |

## Tests mínimos (diseño)

1. **Unit (common):** `sync_index_entity_uuid` sobre fixtures process (Name), skills/agents/events/suites (Archivo fuente); `migrate_entity_uuid_record` multi-layout.
2. **Unit/forja:** update+migrate por cada una de las 5 forjas (TempDir mínimo).
3. **Rollback:** fallo forzado post-migrate parcial → FM e índice = pre-estado; coverage sin clave new huérfana.
4. **Regresión action:** test AUT-137 `migrate_entity_uuid_syncs_index_and_coverage` sigue pasando tras refactor del helper.
5. **Remediación:** evidencia en execution/validacion de los 20 + fail-fast-lab (no inventar éxito sin corrida).

## Riesgos y no-regresión

- **Layouts distintos:** no reutilizar rebuild hardcodeado actions.
- **Event root vs familia:** migrar UUID en índice de familia correcto.
- **Process multi-root:** usar el mismo `dest_root`/`index.md` que ya resuelve la forja.
- **Orden sync post-migrate:** syncs de versión/rol/census **no** deben pisar el UUID nuevo (paridad `sync_action_index_row` que preserva UUID).
- **Sin blueprint de proceso:** no se emite `plan.md` de proceso; esta spec es suficiente para Tekton.

## Blueprint

No aplica (no hay proceso nuevo ni cambio de contract de fase). Cascada bug-fix continúa con `implementation.md` / `execution.md` / `validacion.md` en fases posteriores.

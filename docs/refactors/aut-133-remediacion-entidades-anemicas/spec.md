---
feature_name: aut-133-remediacion-entidades-anemicas
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-133-remediacion-entidades-anemicas
persist_ref: docs/refactors/aut-133-remediacion-entidades-anemicas
tracker_ref: AUT-133
historia_ref: AUT-125
pbi_ref: PBI-81e91c67
execution_id: "5ea79c56-a610-4223-b2b3-7498a510d239"
base: clarify.md
scope: remediacion-genoma-anti-anemia
---
# Spec — AUT-133

Liquidar la deuda documental residual del genoma anémico (residuo AUT-125 / PR #480 REJECT `3899742f-04f5-480b-a1f6-866d93d1d5a3`): inventario pre-remediación trazable, remediación con contenido real vía `entity-manager`, política explícita de UUID no v4/inválido y promoción del CI de `--if-touched` a gate global solo con `violation_count: 0` sin plantilla.

Canal PBI: Linear `AUT-133` / `PBI-81e91c67` (`uuid` `22aef184-e2f3-4861-977c-ac32d9eddc2e`, forge_key `a6caf15c…4454`). Cuerpo issue no local; alcance = `clarify.md` + esta spec + inventario semilla AUT-125.

## Oráculos (O-1…O-5 → AC)

| ID | Objetivo | Oráculo mecánico |
| --- | --- | --- |
| AC-1 | Inventario pre-remediación versionado (KA-DOC-1) | Bajo `persist_ref`: `pre-remediation-inventory.json` con `kind: pre-remediation-inventory`, `violation_count > 0`, `entities[]` (ruta + familia + códigos), `genome_tip_sha` del tip de `main` auditado, `source_command: sddia-qa gate-entity-no-anemia --json` (global, sin `--if-touched`). Prohibido publicar solo `"violations":[]` / post-remediación vacío como inventario. Semilla canónica: `docs/refactors/gate-anti-anemia-entidades-dominio/audit-inventory.json` (`violation_count: 322`, 142 entidades, tip `fd7ef31b…`). |
| AC-2 | Remediación contenido real vía `entity-manager` (KA-DOC-2) | Cada path del inventario remediado: `./sddia-run.sh --process entity-manager` con `lifecycle_operation: update` + `semantic_seed` derivado de handler/cápsula/contrato vigente (sin texto genérico compartido). Post-ola: `sddia-qa gate-entity-no-anemia --json` sobre el subconjunto tocado → 0 violaciones de cuerpo/secciones en esos paths; detector de duplicados del gate **no** marca plantilla repetida entre entidades remediadas. Prohibido `remediate_bulk.py` y equivalentes bajo `docs/`. |
| AC-3 | UUID no v4 / inválido: migración explícita o excepción (KA-UUID-1) | Registro auditable `uuid-migration-register.md` bajo `persist_ref`: una fila por path con `uuid_old`, `disposition` ∈ {`migrate`,`exception`}, `uuid_new` (si migrate), evidencia de sync de `index.md` de familia + upsert/purga en `SddIA/core/eda-coverage.json`. Gate post-cierre: 0 `UUID_NOT_V4` / `UUID_INVALID` salvo filas `exception` con justificación y owner. Prohibida rotación silenciosa del campo `uuid`. |
| AC-4 | Promoción CI a gate global (KA-GATE-GLOBAL-1) | Cuando AC-1…AC-3 verificables y `sddia-qa gate-entity-no-anemia --json` → `violation_count: 0` en tip de rama: el step en `.github/workflows/sddia-index-qa.yml` (job `wasi-runtime-smoke`) deja de usar `--if-touched --base origin/main` y pasa a `gate-entity-no-anemia --json` (scan global). No declarar victoria por `--if-touched` verde con inventario anémico en `main`. |
| AC-5 | Un solo PBI de deuda (D5) + traza evolution | Inventario + remediación + UUID + promoción CI en esta `refactorization`. Evolution con `relacionado: AUT-133` (alta hija o extensión del padre `a8f3c2e1-…` / cadena AUT-125) + fila `Evolution_log.md` cuando Tekton selle. |

**Ola 0 (KZ-PRM-01):** capturar `oracle-red.log` = salida JSON del gate **global** sobre tip de `main` (esperado: `violation_count` ≥ línea base heredada; semilla 322) **antes** de mutar genoma. Worktree limpio vía `skill:git-manager`.

## Contrato de remediación

### Inventario (D1)

1. Al abrir ejecución Tekton: re-ejecutar `sddia-qa gate-entity-no-anemia --json` (global) sobre tip de `main` acordado; materializar el JSON completo en `persist_ref/pre-remediation-inventory.json` con metadatos (`genome_tip_sha`, `captured_at`, `remediation_ref: AUT-133`).
2. Si el tip coincide con la semilla AUT-125 (`322` / mismos códigos agregados), basta congelar copia + tip SHA; si diverge, el nuevo JSON **reemplaza** la línea base operativa (no se inventa victoria).
3. El inventario se congela: no se sobrescribe con post-remediación. El cierre publica aparte `post-remediation-gate.json` con `violation_count: 0`.

### Contenido real (D2)

| Regla | Comportamiento canónico |
| --- | --- |
| Canal | Solo `process:entity-manager` → `*-creator` / forja nativa (`lifecycle_operation: update`) + sello `emit-domain-mutation` (DA-2). |
| Semilla | `semantic_seed` con cuerpo (`orchestration_logic`, `execution_logic`, fases, payload, etc.) **específico** de la implementación real (handler Rust, cápsula WASI, contrato de familia). |
| Secciones | Cumplir `FAMILY_RULES` del peaje: `purpose_kw` **y** `second_kw` en headings `##` distintas, sin stubs (`TODO`/`TBD`/`FIXME`/línea `...` fuera de code fences). |
| Anti-plantilla | Prohibido párrafo/plantilla idéntica compartida entre ≥2 entidades (p. ej. «Ejecución conforme al contrato…»). El detector de duplicados del gate es oráculo de rechazo. |
| Preservación | No vaciar `inputs`/`outputs`/`context`/contratos; no downgrade SemVer injustificado. Depende de integridad de forjas (**AUT-131**). |
| Fuera | Scripts ad-hoc masivos bajo `docs/`; edición directa del genoma sin EM. |

Orden de olas recomendado (por peso del inventario semilla `by_family`): **process → action → tool → skill → daemon → norm → event → suite → codex → agent**. Dentro de cada ola, remediar path a path (o lotes pequeños ≤10) con re-scan parcial tras cada lote.

Testigo de patrón válido (ya remediado en AUT-125): `SddIA/actions/switch-llm-provider.md` vía `entity-manager`.

### UUID (D3)

Códigos en inventario semilla: `UUID_NOT_V4` (19) + `UUID_INVALID` (6).

| Disposición | Procedimiento |
| --- | --- |
| `migrate` | 1) Registrar en `uuid-migration-register.md` (`uuid_old` → `uuid_new` v4). 2) Mutar entidad **y** fila del `index.md` de familia **y** `SddIA/core/eda-coverage.json` (purga huérfano old / cobertura new) en la misma unidad de trabajo gobernada (EM + sync de índice de forja; sin dejar asimetría). 3) Verificar simetría índice ↔ frontmatter. |
| `exception` | Fila formal: path, `uuid_old`, motivo, owner, fecha, `tracker_ref`; el gate global de cierre puede excluir solo esas filas documentadas — **no** se usa excepción para eludir contenido anémico. |

Prohibido: cambiar `uuid` en el `.md` sin tocar índice/`eda-coverage` (reincidencia REJECT #480).

### Promoción CI (D4)

| Estado | Comando en `sddia-index-qa.yml` |
| --- | --- |
| Mientras `violation_count > 0` en tip | Mantener `gate-entity-no-anemia --if-touched --base origin/main --json` (ratchet delta AUT-125). |
| Tras AC-1…AC-3 + global `0` | Sustituir por `gate-entity-no-anemia --json` (global) en el mismo step del job `wasi-runtime-smoke`. |

La promoción es el **último** cambio productivo del PR (o commit de cierre explícito tras evidencia `post-remediation-gate.json`).

## Superficies

| Path | Rol |
| --- | --- |
| `docs/refactors/aut-133-remediacion-entidades-anemicas/pre-remediation-inventory.json` | Inventario congelado KA-DOC-1 |
| `docs/refactors/aut-133-remediacion-entidades-anemicas/uuid-migration-register.md` | Norma migración/excepción UUID |
| `docs/refactors/aut-133-remediacion-entidades-anemicas/post-remediation-gate.json` | Evidencia cierre global `violation_count: 0` |
| Entidades `.md` listadas en inventario (bajo `SddIA/…`) | Cuerpos remediados vía EM |
| `SddIA/*/index.md` (familias tocadas) | Simetría UUID / catálogo |
| `SddIA/core/eda-coverage.json` | Cobertura EDA post-migración |
| `.github/workflows/sddia-index-qa.yml` | Promoción peaje global |
| `SddIA/process/entity-manager.md` + creators | Canal DA-2 de mutación |
| `SddIA/tools/sddia-qa` / `entity_no_anemia` | Oráculo (motor = AUT-132; no reabrir aquí) |
| `SddIA/evolution/*` + `Evolution_log.md` | Traza AUT-133 |

## Dependencias / coordinación

| Hermano | Relación |
| --- | --- |
| AUT-125 | Padre / ratchet `--if-touched`; inventario semilla 322 |
| AUT-132 | Oráculo/peaje fiable — **preferible cerrado o verificado** antes de remediación masiva |
| AUT-131 | Forjas sin defaults destructivos — **requerido** para updates seguros vía EM |

## Fuera de alcance

- Rediseño de contratos de familia / `FAMILY_RULES`.
- Corrección del motor del peaje → **AUT-132**.
- Bugs de forjas (preservación, rollback, suite) → **AUT-131**.
- Reescritura de entidades ya conformes (ausentes del inventario).
- Reintroducción de scripts ad-hoc de remediación masiva bajo `docs/`.
- Aceptación/merge de AUT-125 (#480).

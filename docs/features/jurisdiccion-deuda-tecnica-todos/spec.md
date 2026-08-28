---
feature_name: jurisdiccion-deuda-tecnica-todos
created: "2026-08-28"
process: feature
base: main
scope: jurisdiccion-deuda-tecnica-todos
version_spec: "1.0.0"
---

# Especificación — jurisdicción docs/todos

## Hito 1 — Norma táctica `todos-jurisdiction`

Forja: `./sddia-run.sh --process entity-manager` con `entity_class: norm`, `lifecycle_operation: create`, `entity_name: todos-jurisdiction`.

| Semilla | Valor |
|---------|-------|
| `tactical_norm_name` | `todos-jurisdiction` |
| `tactical_norm_version` | `1.0.0` |
| `tactical_norm_author` | `dedalo` |
| `norm_scope` | `agnostic` |
| `norm_category` | `workflow` |
| `tactical_norm_dependencies` | `[4c448c82-de41-460f-b24f-82a84fa5ed69]` (`features-documentation-pattern`) |
| `tactical_norm_friction` | Seis buckets sin jurisdicción; deuda no-fractura invisible al TQM |

**Directriz Core (contenido a destilar en creator):**

| Bucket relativo a `directories.documentation/todos/` | Despachable | Archivable | Ciclo |
|------------------------------------------------------|-------------|------------|--------|
| `pending/` (`paths.todos.pending`) | Sí | Sí → `done/` | Cola operativa |
| `done/` (`paths.todos.done`) | No (solo ancla de ruta en estímulo) | Destino | Cierre documental |
| `kitchen/` | No | No | Incubación humana; promoción explícita a `pending/` |
| `historias/` | No | No | Narrativa/códice; no PBI |
| `tmp/` | No | No | **Deprecado**; no depositar. Distinto de `.tmp/` |
| `DeudaTecnica/` | No | No | **Retirado** como destino; no depositar |

**Restricciones duras:**

- Prohibido un tercer estado de Done.
- Prohibido anclar TQM/archivado a buckets inertes.
- Portador no-fractura: `type: deuda` + `tech_debt_ids[]` (`DT-*`) en `pending/`.
- `friction_ids[]` usa `F-*`.
- Mutación de la norma solo vía `entity-manager`.
- Consumir `paths.todos.*`; no literales de host.
- No reimplementar `fracture_pbi` / `materialize-fracture-pbi` / `enrich-fracture-pbi-kaizen`.

## Hito 2 — Migración física (CA3/CA4)

| Origen | Destino | Frontmatter |
|--------|---------|-------------|
| Paciente 0 deploy | `pending/` mismo filename | `dispatch: false`; conservar `type: deuda`, `process_candidate`; actualizar `companion_*` paths |
| Paciente 0 teardown | `pending/` | Ídem; `companion_deploy_ref` → nueva ruta pending |
| Escaneo lineal resolutor | `pending/` | Ya CA4-conforme; `dispatch: false` |
| `DeudaTecnica/` vacío | borrar directorio si vacío | — |
| `docs/todos/tmp/*` consolidado | descarte (delete) o README deprecación | no promover |
| kitchen / historias | no mover en este PBI | norma los declara inertes |

## Hito 3 — Verificación física (CA5)

Tests en `task_queue_manager.rs` (existentes + casos nuevos):

1. `extract_pbi_path` sobre estímulo con path `pending/` de cada migrado → `Some(ruta)`.
2. Estímulo con `docs/todos/DeudaTecnica/…` o `kitchen/…` → `None` (o no extrae ese ancla).
3. Predicado archivado: path `pending/` aceptado; path `DeudaTecnica/` → skip `PBI fuera de pending/`.

Comando evidencia:

```text
cd SddIA && cargo test -p execute-process extract_pbi_path -- --nocapture
```

Pegar stdout en `validacion.md`. No sustituir por narrativa.

## Hito 4 — Evolución

Registro `{uuid}.md` bajo `SddIA/evolution/` vía `sddia-qa evolution-rehash` / cápsula register. `relacionado` incluye `4be8aeee-896a-4d2f-b2d3-3ee0d05fbd80`.

## Fuera de alcance

- Ampliar `extract_pbi_path` a más prefijos.
- Índice acelerador del resolutor (`PBI-DT-FRACTURE-RESOLVER-SCAN-LINEAL`).
- Forja de `paciente0-deploy` / `paciente0-undeploy`.
- Promoción masiva de kitchen a pending.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Norma forjada e indexada; jurisdicción por bucket |
| CA2 | Done = pending→done + validacion APTO + `pbi_archived` |
| CA3 | Laudo D1 ejecutado (incluye descarte `Optimizacion_BioIA`) |
| CA4 | Portador + enum en la norma; habitante scan-lineal en pending |
| CA5 | Salida CLI de tests, no afirmación |
| CA6 | Diff sin `fracture_pbi.rs` / handlers de fan-out |

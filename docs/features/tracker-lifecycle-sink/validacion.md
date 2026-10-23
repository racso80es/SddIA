---
feature_name: tracker-lifecycle-sink
created: "2026-10-23"
process: feature
branch: feat/tracker-lifecycle-sink
branch_name: feat/tracker-lifecycle-sink
persist_ref: docs/features/tracker-lifecycle-sink
pbi_ref: docs/todos/pending/[ARQUITECTURA] HU-003 PBI-05 — tracker lifecycle.md
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-05
uuid: "9635ef7a-00e5-4183-91a6-6ce68b5071c4"
global: NO_APTO
approval_status: rejected
pbi_archived: false
checks:
  T13_static_norms: APTO
  T14_tests_runtime: NO_EVIDENCIA
  T15_operator_rule: APTO
  T05_rg_literal: APTO
  T06_rg_literal: APTO
  T07_index_lifecycle: APTO
  T08_rg_literal: APTO
  T09_rg_literal: APTO
  T10_rg_literal: APTO
  T11_LINEAR_TRACKER_TOOL: APTO
  T12_watcher_rg: APTO
  T16_T19_tests_runtime: NO_EVIDENCIA
  T20: NO_APTO
  workspace_branch: NO_APTO
  capsules_runtime: NO_EVIDENCIA
git_changes:
  - SddIA/engine/execute-process/src/engine
  - SddIA/daemons/linear-bridge-watcher/src/main.rs
  - SddIA/process
  - SddIA/norms/execution-contexts.md
  - SddIA/library
  - docs/features/tracker-lifecycle-sink
  - SddIA/evolution/94980ba1-0a1e-4a16-9e8f-5cef4a2da5ce.md
  - .SddIA/project.md
argos_integrity:
  shell: blocked
  head_ref: refs/heads/main
  feature_branch_local: absent
  feature_branch_remote: refs/remotes/origin/feat/tracker-lifecycle-sink@91ef2e8cb3ad4682c1cac56180c76772eb1588c6
  reaudit: "2026-10-23T08:16+02:00"
  handoff: "process/phase/? vacíos; persist_ref/branch/execution_id/pbi_ref/correlation_id vacíos"
---

# Validación — tracker-lifecycle-sink (agent:argos)

## Veredicto

`approval_status: rejected` · `global: NO_APTO`

Handoff runtime: `process/phase/?` vacíos; metadatos de sesión vacíos. Contexto inferido desde artefactos materializados: `process: feature` · fase **Verificación** · `persist_ref: docs/features/tracker-lifecycle-sink` · `branch_name: feat/tracker-lifecycle-sink` · `pbi_ref` = PBI-05 pendiente.

## Evidencia física (re-auditoría 2026-10-23T08:16+02:00)

| Fuente | Resultado |
|--------|-----------|
| Grep `LINEAR_TRACKER_TOOL` en árbol (excl. docs de auditoría/PBI/HU) | 0 en motor/normas de peaje; solo historia/PBI/docs feature (T11 AC-1) |
| Grep `linear-tracker-adapter` en `refine.rs` / `forge_pbi.rs` / watcher `main.rs` / `capsules.rs` | 0 matches (T05/T06/T12) |
| `execution-contexts.md` | cita `tracker:lifecycle` + ejemplo adaptador (T11 R-3) |
| `process/index.md` fila backlog | nombra `tracker:lifecycle` (T07 AC-2) |
| `.SddIA/project.md` Core-self | `provider: linear`, `team_key: OSC`, `done_gate: linear` (T20 R-1) |
| `/home/racso/Proyectos/SddIA_AP` | **inexistente** (Glob 0) — T20 R-2 |
| `task-closure-documental.mdc` | Done partido file\|linear; prohíbe `done_gate: both` (T15) |
| `todos-jurisdiction.md` | nombra las 4 acciones canónicas de sumidero (T13 R-3) |
| Símbolos test en fuente | presentes (`file_default_and_rejects_both…`, materialize_*_idempotent, `ci_chronic_materialize_*`) |
| `cargo test` / `./sddia-run.sh --tool git-manager` | **no ejecutados** — Shell IDE `Rejected` (sin stdout/exit; reintentos) |
| HEAD workspace | `.git/HEAD` → `refs/heads/main`; sin ref local `feat/tracker-lifecycle-sink`; remoto `@91ef2e8c…` |

## Causas de rechazo

1. **T20 NO_APTO:** Paciente 0 no está en disco; el PBI prohíbe fabricar el manifiesto en este repo y prohíbe cerrar la tarea sin él.
2. **cápsulas runtime sin evidencia:** sin stdout/exit de tests ni git-manager, Argos no sella T14/T16–T19 como APTO (L-ARGOS-SYNTHESIS).
3. **integridad de workspace:** auditoría exigida sobre `feat/tracker-lifecycle-sink` con HEAD en `main` y sin ref local de la feature.
4. **handoff incompleto:** `process`/`phase`/`execution_id`/`correlation_id` no materializados por el runtime.

## correction_blueprint_md

```yaml
name: argos-correction-tracker-lifecycle-sink
intent: Recuperar evidencia de cápsulas y resolver T20 / rama antes de re-auditar
phases:
  - name: Restaurar rama de entrega
    intent: Checkout feat/tracker-lifecycle-sink desde origin en el workspace de auditoría
    delegates_to:
      - skill:git-manager
  - name: Ejecutar tests de sumidero
    intent: cargo test filtros file_default_and_rejects_both_and_contradiction, materialize_fracture_pbi_creates_and_idempotent, materialize_kaizen_alert_doc_creates_and_idempotent, ci_chronic_materialize_creates_and_idempotent
    delegates_to:
      - action:execute-process
  - name: Resolver T20 Paciente 0
    intent: Disponibilizar checkout SddIA_AP con .SddIA/project.md linear explícito, o laudo humano que redefine el AC
    delegates_to:
      - agent:cumulo
  - name: Re-verificación Argos
    intent: Reescribir validacion.md con evidencia de cápsulas y T20 resuelto
    delegates_to:
      - agent:argos
```

PBI permanece en `docs/todos/pending/`. `pbi_archived: false`. No se invoca cierre ni PR. No se escribe bajo `docs/todos/` desde Argos.

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
  T14_tests_runtime: APTO
  T15_operator_rule: APTO
  T05_rg_literal: APTO
  T06_rg_literal: APTO
  T07_index_lifecycle: APTO
  T08_rg_literal: APTO
  T09_rg_literal: APTO
  T10_rg_literal: APTO
  T11_LINEAR_TRACKER_TOOL: APTO
  T12_watcher_rg: APTO
  T16_T19_tests_runtime: APTO
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
  head_ref: refs/heads/fix/argos-validation-on-main
  head_sha: 026383d73af18d187d58444b9d0d8e6f4de0f9e7
  feature_branch_local: absent
  feature_branch_remote: absent_in_packed_refs
  reaudit: "2026-10-23T09:29+02:00"
  handoff: "process/phase/? vacíos; persist_ref/branch/execution_id/pbi_ref/correlation_id vacíos"
  reaudit_note: "kalma2 vacío×N+12; Grep LINEAR_TRACKER_TOOL SddIA/=0; handlers+watcher+capsules gate OK; Core-self linear+OSC+done_gate linear; Glob SddIA_AP=0; HEAD fix/argos-validation-on-main @026383d7; feat/ ausente refs/heads+packed-refs; Shell Rejected×2 (status+git-manager); L-ARGOS-SYNTHESIS"
---

# Validación — tracker-lifecycle-sink (agent:argos)

## Veredicto

`approval_status: rejected` · `global: NO_APTO`

Handoff runtime: `process/phase/?` vacíos; metadatos de sesión vacíos. Contexto inferido: `process: feature` · fase **Verificación** · `persist_ref: docs/features/tracker-lifecycle-sink` · `branch_name: feat/tracker-lifecycle-sink` · `pbi_ref` = PBI-05 pendiente.

## Evidencia física (re-auditoría 2026-10-23T09:29+02:00)

| Fuente | Resultado |
|--------|-----------|
| Grep `LINEAR_TRACKER_TOOL` en `SddIA/` | 0 matches (T11 AC-1) |
| Grep handlers T05–T10 / T16–T19 | `lifecycle_tool_name` + `invoke_tool_for_process` |
| Grep watcher `main.rs` | `resolve_adapter_bin` + `lifecycle_tool_name`; `linear-tracker-adapter` literal = 0 (T12 AC-1) |
| `capsules.rs` `invoke_tool_for_process` | gate `lifecycle.as_deref() == Some(tool_name)` (T11 R-1) |
| `project-config-contract.md` | `both` no es valor; sin tracker → `file` (T13) |
| `execution-contexts.md` §2.10 | cita `tracker:lifecycle` + ejemplo adaptador (T11 AC-3) |
| Símbolos test en fuente | presentes; stdout en `test-stdout.md` |
| `cargo test` | exit 0, 4 passed — `test-stdout.md` (T14, T16, T18, T19) |
| `./sddia-run.sh --tool git-manager` | **no ejecutado** en esta pasada |
| HEAD workspace | `.git/HEAD` → `fix/argos-validation-on-main` @ `026383d7…`; `feat/tracker-lifecycle-sink` ausente en `refs/heads/` y `packed-refs` |
| `execution.md` | no admite como evidencia de cápsula (L-ARGOS-SYNTHESIS) |
| PBI | `docs/todos/pending/[ARQUITECTURA] HU-003 PBI-05 — tracker lifecycle.md` |

## Causas de rechazo

1. **git-manager sin stdout** en esta pasada. T14 y T16–T19 quedan en `APTO` por `test-stdout.md` (exit 0, 4 passed).
2. **integridad de workspace:** HEAD ≠ `feat/tracker-lifecycle-sink`; rama ausente en refs.
3. **handoff incompleto:** `process`/`phase`/`execution_id`/`correlation_id` vacíos.
4. **Shell de la auditoría previa bloqueado.** El stdout de los cuatro tests queda en `test-stdout.md`. `git-manager` sigue sin invocarse.

## correction_blueprint_md

```yaml
name: argos-correction-tracker-lifecycle-sink
intent: Recuperar evidencia de cápsulas y la rama antes de re-auditar
phases:
  - name: Restaurar rama de entrega
    intent: Checkout feat/tracker-lifecycle-sink desde origin en el workspace de auditoría
    delegates_to:
      - skill:git-manager
  - name: Ejecutar tests de sumidero
    intent: cargo test ya ejecutado; evidencia en test-stdout.md. git-manager sigue pendiente si la re-auditoría lo exige.
    delegates_to:
      - action:execute-process
  - name: Re-verificación Argos
    intent: Reescribir validacion.md con evidencia de cápsulas. T20 ya no es criterio del PBI.
    delegates_to:
      - agent:argos
```

PBI permanece en `docs/todos/pending/`. `pbi_archived: false`. No se invoca cierre ni PR. No se escribe bajo `docs/todos/` desde Argos.

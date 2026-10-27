---
feature_name: gate-evolution-evol-stem-not-v4
created: "2026-10-27"
updated: "2026-10-27T13:35:00Z"
process: bug-fix
phase: Verificación
agent: argos
agents: argos
branch: fix/gate-evolution-evol-stem-not-v4
branch_name: fix/gate-evolution-evol-stem-not-v4
persist_ref: docs/fixes/gate-evolution-evol-stem-not-v4
tracker_ref: AUT-9
document_id: PBI-e90c2357
pbi_ref: AUT-9
execution_id: "5fb64f21-06f9-4e20-af62-a81d6de226da"
global: NO_APTO
pbi_archived: false
approval_status: rechazado
verdict: rechazado
resolution: FAIL_INTEGRITY_EVIDENCE
git_manager_invoked: false
git_manager_error: "Shell Rejected sobre ./sddia-run.sh --tool git-manager; sin stdout físico esta sesión"
formal_execute_process: false
formal_evidence_detail: "sin TECH_FORMAL_EXECUTE_PROCESS: ausente _agent_handoff.md Runtime evidence; cargo test no ejecutable (Shell Rejected)"
handoff_machine_file: absent
evidence_bridge_notes: "source=none — prompt kalma2 con process/phase/persist_ref vacíos (?); inferido bug-fix/Verificación desde persist_ref documental; sin bloque ### Runtime evidence (machine)"
shell_session: "Rejected — git-manager, cargo test y subagente Shell igualmente Rejected"
---

# Validación — gate-evolution-evol-stem-not-v4 (Argos)

**NO_APTO** — fallo de integridad de evidencia (no rechazo de código por hallazgos de cápsula).

## Checks

| Check | Resultado | Nota |
|-------|-----------|------|
| `HANDOFF_MACHINE_FILE` | NO_APTO | falta `persist_ref/_agent_handoff.md` |
| `TECH_FORMAL_EXECUTE_PROCESS` | NO_APTO | source=none; sin stdout de `cargo test` esta sesión |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | NO_APTO | Shell Rejected; sin copia bridge |
| `GIT_EVIDENCE_SESSION_SHELL` | NO_APTO | Shell Rejected |
| `RBAC_AUTHORING_KM_POLICY` | APTO | Argos no escribió `docs/todos/**`; grep AUT-9/PBI-e90c2357 sin hits bajo `docs/todos/` |
| `DOC_SPEC` | APTO | `spec.md` presente |
| `DOC_IMPLEMENTATION` | APTO | `implementation.md` presente |
| `DOC_EXECUTION` | APTO | `execution.md` presente (narrativa previa; no sustituye cápsula) |
| `DOC_CONTRACT_ROW` | APTO (estático FS) | fila `EVOL_ID_NOT_UUID_V4` en `evolution_contract.md` §8 |
| `DOC_AC_TESTS_PRESENT` | APTO (estático FS) | `non_v4_stem_*` + `REASON_ID_NOT_UUID_V4` en `lib.rs` — **no** equivale a tests OK |
| `AC_RUNTIME_TESTS` | NO_APTO | `cargo test -p sddia-evolution-register --lib` no materializado |
| `branch` | NO_APTO | sin evidencia git-manager; snapshot conversación: rama `fix/gate-evolution-evol-stem-not-v4` |
| `git_changes` | NO_APTO | sin listado físico esta sesión |

`pbi_archived: false` — provider Linear; no archivar fichero en `docs/todos/done/`.

## Narrativa previa (no acusada esta sesión)

`execution.md` afirma 26/26 tests, alta QA y PR #401. Sin Evidence Bridge ni re-ejecución de cápsulas, Argos **no** revalida ese APTO.

## correction_blueprint

```yaml
name: rehacer-verificacion-argos
intent: Materializar evidencia determinista y re-sellar validacion.md
delegates_to:
  - tool:git-manager
  - action:execute-process
phases:
  - name: evidencia-git
    intent: status/diff vía skill git-manager (JSON stdin; prune en fetch)
    delegates_to:
      - tool:git-manager
  - name: evidencia-tech
    intent: cargo test -p sddia-evolution-register --lib (+ bridge Runtime evidence)
    delegates_to:
      - action:execute-process
  - name: re-sellado
    intent: Argos reescribe validacion.md copiando TECH_FORMAL / GIT_EVIDENCE del handoff
    delegates_to:
      - agent:argos
```

Requisito operativo: Shell/IDE que permita `./sddia-run.sh` o handoff nativo con `TECH_FORMAL_EXECUTE_PROCESS` / `GIT_EVIDENCE_VIA_GIT_MANAGER`.

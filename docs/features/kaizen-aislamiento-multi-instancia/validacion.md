---
feature_name: kaizen-aislamiento-multi-instancia
created: "2026-08-26"
updated: "2026-08-26T08:19:00Z"
process: feature
branch: feat/kaizen-aislamiento-multi-instancia
global: APTO
pbi_archived: true
checks:
  AC-SYS-02-TEMPLATE: APTO
  AC-SYS-02-USER-UNIT: APTO
  AC-DEP-10: APTO
  AC-CEN-PKILL: APTO
  AC-ENTITY-MANAGER: APTO
  AC-TWO-ROOT-LAB: APTO
  AC-INBOX-SHA: APTO
  AC-TWO-ROOT-AP-TREE: NO_APTO
  R-07: N_A
git_changes:
  - SddIA/templates/systemd/sddia-daemon@.service.template
  - SddIA/templates/systemd/sddia-email-watcher@.service.template
  - SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs
  - SddIA/scripts/common/sddia_shell_lib.sh
  - SddIA/scripts/daemons/_run_daemon.sh
  - start-sddia.sh
  - SddIA/process/instance-creator.md
  - SddIA/norms/sddia-distribution-protocol.md
  - docs/todos/done/[KAIZEN] aislamiento multi-instancia centinelas.md
---

# Validación — kaizen-aislamiento-multi-instancia

Motor + ensayo **dos raíces** (forja + `.SddIA/sandbox/iso-b`). `SddIA_AP` ausente: no bloquea el AC «o dos raíces».

## Evidencia 2026-08-26T08:19

| Check | Hecho |
|-------|--------|
| User `%f` | `@SddIA` → forja; `@SddIA_AP` → path AP |
| iso-b `ExecStart` render | `%f/SddIA/scripts/daemons/event-watcher.sh` |
| event-watcher cwd | `…/SddIA` PID 57131 vs `…/sandbox/iso-b` PID 56793 |
| restart forja | iso-b **sigue vivo**; 0 `pkill -x` |
| GET `/api/email-inbox` | forja `:8765` SHA `62ce3b2b…` (2995 B) ≠ iso `:18765` SHA `2b4af084…` (`items:[]`) |
| R-07 | Host=forja lab; `sddia-email-watcher@…SddIA` active. Paciente 0 no restaurado. |

PBI en `docs/todos/done/`.

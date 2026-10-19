---

feature_name: kalma2-reincidence-stale-runtime-and-failed-projection
created: "2026-10-15"
process: bug-fix
branch_name: fix/kalma2-reincidence-stale-runtime-and-failed-projection
items:
  - phase-end-failed
  - dirty-branch-rollback
  - orchestrator-mtime
  - sse-chunk-flush
  - witness-seal
execution_id: "c213e83f-599a-434e-b9d3-0eae5c8aae07"
---
# Implementación — reincidencia PR #347

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/interfaces/kalma2-bridge/src/main.rs` | `phase_status: failed` entra en la proyección y pisa `awaiting_agents`. El detalle sale de `verdict` o `error`. El replay SSE se rellena hasta `n * 8192 + 1` para que el encoder de `tiny_http` emita el bloque. El orquestador usa el ELF más nuevo. |
| `SddIA/engine/execute-process/src/engine/workspace_init.rs` | Tras un `dirty-worktree`, checkout de la rama previa y borrado de la rama nueva si no existía. |
| `SddIA/engine/execute-process/src/engine/invoke_orchestrator.rs` | Misma regla de mtime que F-DEP-07. |
| `SddIA/sddia-daemon-runtime/src/lib.rs` | `resolve_execute_process_elf`. |
| `SddIA/sddia-daemon-runtime/src/github_bridge.rs` | Delega en esa función. |
| `SddIA/daemons/telegram-watcher/src/main.rs` | Idem. |
| `SddIA/daemons/event-watcher/src/main.rs` | Idem. |
| `SddIA/daemons/event-sweeper/src/main.rs` | Idem. |
| `SddIA/tools/sddia-qa/src/resolve.rs` | Delega en `resolve_orchestrator_bin`. |
| `SddIA/engine/execute-process/src/engine/capsule_paths.rs` | `kalma2-bridge` se localiza en `SddIA/interfaces/kalma2-bridge`. |
| `SddIA/engine/execute-process/src/engine/capsule_seal.rs` | Testigo `.sha256` sin genoma cuando `write_genome` es false. |
| `start-sddia.sh` | Sella testigos release de `execute-process` y `kalma2-bridge`, y debug de `execute-process`. |

## Fuera de este diff

Los dos archivos de `docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli/` siguen sucios y no forman parte del fix.

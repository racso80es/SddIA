---
feature_name: kalma2-process-dispatch
created: "2026-07-20"
process: feature
items_applied:
  - T1 handler TQM
  - T2 extract_pbi_ref
  - T4 smokes
---

# Execution — kalma2-process-dispatch

## Registro

| Paso | Resultado |
|------|-----------|
| `cargo test -p execute-process --lib task_queue_manager` | 3/3 OK |
| `cargo test -p execute-process --lib pbi` (incluye `kalma2_extracts_pbi_ref_with_spaces`) | OK |
| `cargo build -p execute-process` | OK → `SddIA/target/debug/execute-process` |
| Smoke TQM paquete a7725b42-like + `SDDIA_LAB_SKIP_GIT=1` | `success` · `handler=task-queue-manager-kalma2` · hijo `bug-fix` · `pbi_ref` extraído |
| Smoke `kalma2-interact` path con espacios | evento dominio con `pbi_ref` poblado |
| Smoke TQM legado `tasks_path` | `success` (compat) |

## Notas

- Artefactos efímeros de smoke (`docs/fixes/e6cbecb9032c`, lab-tqm) eliminados.
- Rama de trabajo: `feat/kalma2-process-dispatch` (sin checkout del hijo gracias a `SDDIA_LAB_SKIP_GIT`).

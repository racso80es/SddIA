---

feature_name: kalma2-reincidence-stale-runtime-and-failed-projection
created: "2026-10-19"
process: bug-fix
branch_name: fix/kalma2-reincidence-stale-runtime-and-failed-projection
items_applied:
  - phase-end-failed
  - dirty-branch-rollback
  - orchestrator-mtime
  - sse-chunk-flush
  - witness-seal
  - rebuild-and-recycle
execution_id: "c213e83f-599a-434e-b9d3-0eae5c8aae07"
---
# Ejecución — reincidencia PR #347

## Tests

```text
run_dirty_outside_scope_aborts_without_system_fracture ... ok
prefers_release_when_debug_is_older ... ok
prefers_debug_when_strictly_newer ... ok
resolve_execute_process_elf_skips_older_debug ... ok
project_status_failed_phase_overrides_awaiting_agents_pec ... ok
progress_stream_replay_arrives_on_first_read ... ok
resolve_orchestrator_prefers_newer_release_elf ... ok
```

## Runtime

| Binario | mtime | Testigo |
|---------|-------|---------|
| `SddIA/target/release/execute-process` | 2026-10-19 16:38 CEST | `elf_sha256: sha256:4c2ad4f5fd644fc57915258ac5158385294f100b9cd92c0fa6aa0c5ea6164ae8` |
| `SddIA/target/debug/execute-process` | 2026-10-19 16:25 CEST | sellado en el mismo ciclo |
| `SddIA/target/release/kalma2-bridge` | 2026-10-19 16:27 CEST | `elf_sha256: sha256:efa242bcc3a871ad189364c19d582f22780fab8af008ec6a8e157c7d7d9228a0` |

`strings` en el release de `execute-process` contiene `workspace_init_failed`, `delete_new_feature_before_recreate` y el símbolo `short_verdict`. En `kalma2-bridge` release (no stripped) las comparaciones `phase_status` / `blocked` / `awaiting_agents` están en el binario; los nombres de función privados quedan inline.

## Reciclo

`sddia-kalma2-bridge@home-racso-Proyectos-SddIA.service`: PID 26035 (16:26) → PID 38851 (16:39). `exe` = `SddIA/target/release/kalma2-bridge`. `ActiveState=active`.

## Fuera del diff

Siguen sin stage los dos archivos de `docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli/`.

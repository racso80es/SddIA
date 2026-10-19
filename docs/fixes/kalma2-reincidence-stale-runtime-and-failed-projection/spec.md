---
feature_name: kalma2-reincidence-stale-runtime-and-failed-projection
created: "2026-10-15"
process: bug-fix
base: main
scope: R-CA1..R-CA6
branch_name: fix/kalma2-reincidence-stale-runtime-and-failed-projection
execution_id: "381178a8-8a56-4a06-a8ce-d47be87242f2"
---

# Spec — reincidencia PR #347

## Diagnóstico

| ID | Causa |
|----|--------|
| R-CA1 | `phase_end_from_trace` descarta `phase_status: failed`. El PEC temprano `awaiting_agents` queda como frase de `GET /api/status`. |
| R-CA2 | Tras `checkout_work_branch`, un `dirty-worktree` devuelve `Err` sin volver a `prior_branch` ni borrar la rama si `!branch_existed`. |
| R-CA3 | Los resolutores Rust eligen `target/debug/execute-process` antes que release. `start-sddia.sh` no escribe el testigo `.sha256` de `execute-process` ni de `kalma2-bridge`. El shell (`_sddia_resolve_orchestrator`) ya aplica F-DEP-07. |
| R-CA4 | `tiny_http` 0.12 codifica el cuerpo SSE con `chunked_transfer::Encoder` de 8192 bytes y sin `flush_after_write`. `Request::respond` hace flush del socket solo al terminar `raw_print`. Un `Read` de stream no devuelve 0, así que los frames menores que el bloque no salen. El hilo del pool queda en `ClientConnection::next` esperando la siguiente petición keep-alive; el pool crece y no explica los 0 bytes. |
| R-CA5 | Recompilar release y debug de `execute-process`, release de `kalma2-bridge`, sellar testigos y reciclar el puente vivo. |
| R-CA6 | Tests del override `failed`, del replay SSE y del rollback de rama nueva. |

## Contrato

1. `phase_status == failed` entra en `latest_phase_end`. `phase_end_projection` devuelve `status: failed`. El detalle sale de `metadata.verdict`, si no de `metadata.error`, si no de `error` en la hoja. Pisa el PEC `awaiting_agents`.
2. Si `dirty_paths_outside_scope` falla después del checkout, checkout de `prior_branch` y, si la rama no existía, `delete_branch` local con `force`. El `Err("dirty-worktree: …")` se conserva. El gate no se relaja.
3. Con debug y release presentes, debug solo si su mtime es estrictamente mayor. Empate o debug más viejo → release. Aplica a `kalma2-bridge`, `invoke_orchestrator`, `sddia-qa` y watchers (`telegram-watcher`, `event-watcher`, `event-sweeper`, `github_bridge`). `start-sddia.sh` sella testigos con `write_genome: false` para `execute-process` y `kalma2-bridge`. `kalma2-bridge` se localiza en `SddIA/interfaces/kalma2-bridge`.
4. Cada lote SSE (replay, evento nuevo o ping) se rellena hasta `n * 8192 + 1` bytes para que el encoder emita el bloque. El replay de `.events/progress/{cid}/` cabe en la primera lectura, en menos de 1 s.

## Fuera de alcance

No se rediseña la WUI. No se relaja `SDDIA_LAB_ALLOW_DIRTY` como política de producto. No se implementa `PBI-LINEAR-B-07-CORE-SELF-GATE`.

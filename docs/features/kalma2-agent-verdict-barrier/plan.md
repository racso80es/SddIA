---
feature_name: kalma2-agent-verdict-barrier
created: "2026-08-31"
process: feature
phases: 6
branch_name: feat/kalma2-agent-verdict-barrier
persist_ref: docs/features/kalma2-agent-verdict-barrier
pbi_ref: docs/todos/pending/[KAIZEN] Kalma2 agent-runtime — veredicto blocked, DNS y halt-after-phase (a9fe100f).md
document_id: PBI-KAIZEN-KALMA2-AGENT-VERDICT-BARRIER
execution_id: "c56f0a70-c2e9-468f-8c98-9c0d044bbd4c"
---

# Plan — kalma2-agent-verdict-barrier

Orden: A → B → C (un PR). B asume que `blocked`/`awaiting_agents` sobreviven al normalizador.

## Fase 1 — Ola A prótesis

1. `parse_agent_verdict(transcript) -> Optional[str]`: última coincidencia `(?i)veredicto\s*:\s*(ok|blocked)`.
2. `run_agent_phase` CLI ok: si verdict `blocked` → status blocked / success true; si `ok` o ausente → executed.
3. `is_transient_network_error`: tokens `enotfound`, `getaddrinfo`, `eai_again`, `could not resolve host`; `timeout` en traza → False.
4. Orden fail path: network → awaiting (sin REQUIRE_CLI); soft+REQUIRE_CLI → failed; soft → awaiting; else failed.
5. Tests Python: blocked+CLI0; ENOTFOUND ± REQUIRE_CLI; timeout sigue no-soft.

## Fase 2 — Ola A Rust

1. `agent_runtime.rs` match: añadir `"blocked"`.
2. Test `configured_cli_can_block` (paridad `configured_cli_can_await`).

## Fase 3 — Ola B halt + PEC

1. `executor.rs`: `stop_after_design()` lee `SDDIA_TQM_STOP_AFTER`. `is_dedalo_design_phase`. Tras Dedalo `executed` armar barrera con reason `stop_after`. Helper skip con reason param (default `prior_agent_phase_not_executed`).
2. `thermodynamic.rs` `derive_cycle_phase`: si algún report `reason == "stop_after"` → `awaiting_agents` (prioridad ≥ simulated; awaiting status sigue primero).
3. Tests: secuencia Dedalo executed + stop_after salta Tekton; derive no `completed`.

## Fase 4 — Ola B TQM L2

1. `child_env_for_kalma2` devuelve también `l2_skip: bool`.
2. Propagar `SDDIA_TQM_STOP_AFTER` desde input `stop_after` o env del padre al hijo.
3. Envelope `data.delivery_close = "skipped_l2"` si `l2_skip`.
4. Test: correlation sin FULL_CYCLE → campo presente; con FULL_CYCLE → ausente; task_text con «PR» no muta el flag.

## Fase 5 — Ola C

1. `workspace_init.rs` `distill_mission`: strip FM; H1 + document_id + primer párrafo ≤400. Test: dump PBI no aparece entero en Misión; `pbi_ref` en FM.
2. `build_prompt`: un canal. Test: no hay sección `## Cuerpo PBI` si hay `pbi_ref`.
3. `executor.rs` post-bucle: write `phase_reports.json`. Test unitario write en temp dir.

## Fase 6 — Cierre documental

`implementation.md` / `execution.md` / `validacion.md`. PBI → `docs/todos/done/`. Evolution registro UUID v4 + `sddia-qa evolution-rehash`. DCC. Tests: `python3 -m unittest SddIA/scripts/tools/test_kalma2_runtime_timeout.py`; `cargo test -p execute-process` filtros `agent_runtime`, `thermodynamic`, `executor`, `task_queue_manager`, `workspace_init`.

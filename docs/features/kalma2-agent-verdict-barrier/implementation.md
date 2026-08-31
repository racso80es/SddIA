---
feature_name: kalma2-agent-verdict-barrier
created: "2026-08-31"
process: feature
items:
  - parser-veredicto-blocked
  - normalizador-blocked
  - dns-node-awaiting
  - stop-after-design
  - tqm-skipped-l2
  - objectives-destilado
  - prompt-un-canal
  - phase-reports-json
branch_name: feat/kalma2-agent-verdict-barrier
persist_ref: docs/features/kalma2-agent-verdict-barrier
document_id: PBI-KAIZEN-KALMA2-AGENT-VERDICT-BARRIER
agents: tekton
execution_id: "c56f0a70-c2e9-468f-8c98-9c0d044bbd4c"
---

# Implementation — kalma2-agent-verdict-barrier

## Touchpoints

| Path | Cambio |
|------|--------|
| `kalma2-agent-runtime-cursor.py` | `parse_agent_verdict`; `is_transient_network_error`; CLI 0+blocked → `status=blocked` success true; DNS awaiting ignora REQUIRE_CLI; `build_prompt` un canal |
| `test_kalma2_runtime_timeout.py` | Fixtures CA-A1/A2/prompt |
| `agent_runtime.rs` | Allowlist `blocked`; test `configured_cli_can_block` |
| `executor.rs` | `SDDIA_TQM_STOP_AFTER=design` tras Dedalo executed → skip reason `stop_after`; `phase_reports.json` |
| `thermodynamic.rs` | reason `stop_after` → `cycle_phase=awaiting_agents` |
| `task_queue_manager.rs` | Propaga `SDDIA_TQM_STOP_AFTER`; `data.delivery_close=skipped_l2` |
| `workspace_init.rs` | `distill_mission` |

## No tocado

`delivery_close.rs`. YAML process. Genoma DA-2. Keepalive email-watcher.

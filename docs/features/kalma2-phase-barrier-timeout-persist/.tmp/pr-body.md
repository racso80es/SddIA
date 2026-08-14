## Summary
- Barrera de fase: Verificación/cierre no corren si Ejecución ∈ {failed, awaiting_agents}.
- Timeout de cursor-agent es terminal (no soft); override `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION`.
- Conserva rama `refactor/` e inyecta `persist_ref` TQM→hijo→handoff.

## Test plan
- [x] cargo test -p execute-process --lib (workspace_init, TQM, executor, agent_runtime)
- [x] python3 SddIA/scripts/tools/test_kalma2_runtime_timeout.py
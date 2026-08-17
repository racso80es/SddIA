## Summary
- Fan-out real de `Process_Execution_Completed`: testigo durable S2 (`persist-pec-correlation-proof`) + Telegram.
- `GET /api/status` proyecta completed/failed/initialized/awaiting_agents tras purge del padre (lee `.SddIA/proofs/pec-correlation/{cid}.json`).
- `event-bus-audit` detecta EMPTY_SUBSCRIBERS, FAMILY_MISMATCH, ORPHAN_REGISTRY_KEY y PURGE_BLACKHOLE.

## Test plan
- [x] `cargo test -p execute-process persist_pec`
- [x] `cargo test -p execute-process telegram_message_for_pec`
- [x] `cargo test -p kalma2-bridge proof`
- [x] `cargo test -p event-bus-audit circuit_coverage`
- [ ] Smoke WUI Kalma2: Forjar Proceso → status ≠ timeout 120s ciego
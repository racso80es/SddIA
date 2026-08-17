---
feature_name: kaizen-pec-subscribers-circuit-audit
created: "2026-08-17"
process: feature
phases: "T0-docs,T1-action-forge,T2-native-proof,T3-telegram-pec,T4-registry,T5-bridge,T6-audit,T7-tests"
branch_name: feat/kaizen-pec-subscribers-circuit-audit
persist_ref: docs/features/kaizen-pec-subscribers-circuit-audit
document_id: PBI-KAIZEN-PEC-SUBSCRIBERS-CIRCUIT-AUDIT
---

# Blueprint — kaizen-pec-subscribers-circuit-audit

## Estrategia

Cerrar el agujero PEC `[]` + purge **sin** retener el JSON táctico. Fan-out real (S2 + S1) y proyección durable en instancia. Audit de cobertura en la cápsula ya indexada.

Laudos: **L-O1-XOR = S2** en `spec.md`.

## Fases

### T0 — Dedalo (este documento)

- [x] spec.md / plan.md
- Entregable: XOR cerrado; touchpoints listados

### T1 — Forja acción

- [x] `./sddia-run.sh --process entity-manager` CREATE `persist-pec-correlation-proof`
- [x] Enriquecer contrato (inputs/outputs) sobre el stub de forge
- Gate: `{name}.md` + fila en `actions/index.md` + sello `Domain_Entity_Created`

### T2 — Handler nativo

- [x] `persist_pec_correlation_proof.rs` + registro en `actions::try_run_native`
- [x] Rama en `dispatch_subscriber` (evento completo, no solo payload)
- [x] `skipped-no-correlation` en `OK_STATUSES`
- Gate: unit escribe `{proofs}/pec-correlation/{cid}.json`

### T3 — Telegram PEC

- [x] `build_telegram_message_from_event` arm `Process_Execution_Completed`
- Mensaje: `process_name` + `correlation_id` + `status` / `cycle_phase`

### T4 — Registro Cúmulo

- [x] `event-orchestration-subscriptions.json`: S2 luego S1
- Gate: clave PEC no vacía

### T5 — Bridge status

- [x] `find_pec_proof` + fallback en `build_status_body`
- Gate: 200 post-purge con testigo; 404 si no hay rastro

### T6 — event-bus-audit O2

- [x] Cruce catálogo↔registros; cuatro códigos; umbral Kaizen L-O2-THRESH
- Gate: unit de hallazgos; `cargo test -p event-bus-audit`

### T7 — Fricción

- [x] `cargo test -p execute-process persist_pec`
- [x] `cargo test -p kalma2-bridge` (status/proof)
- [x] implementation.md / execution.md / evolution

## Touchpoints

| Path | Rol |
|------|-----|
| `SddIA/actions/persist-pec-correlation-proof.md` | Genoma (entity-manager) |
| `SddIA/engine/execute-process/src/engine/persist_pec_correlation_proof.rs` | Nativo S2 |
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | Dispatch + Telegram PEC |
| `SddIA/engine/execute-process/src/engine/route_fractal_core.rs` | OK_STATUSES |
| `SddIA/core/event-orchestration-subscriptions.json` | Fan-out |
| `SddIA/interfaces/kalma2-bridge/src/main.rs` | Lectura testigo |
| `SddIA/tools/event-bus-audit/src/main.rs` | O2 |
| `.gitignore` | `.SddIA/proofs/` |

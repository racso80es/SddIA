---
feature_name: kalma2-pasarela-asincrona-eda
created: "2026-07-22"
process: feature
document_id: PBI-044-KALMA2-PASARELA-ASINCRONA-EDA
items:
  - T1-accept-execute-spawn-202
  - T2-correlation-id-plumb-genoma
  - T3-ui-accepted-poll
  - T4-docs-execution
---

# Implementation — kalma2-pasarela-asincrona-eda

## Touchpoints

| # | Path | Cambio |
|---|------|--------|
| 1 | `SddIA/interfaces/kalma2-bridge/src/main.rs` | `accept_execute`: UUID v4 + `Command::spawn` + reaper; HTTP **202** + envelope `accepted`; `handle_execute` / `interact(mode=execute)` sin `run_orchestrator_inputs` |
| 2 | `SddIA/interfaces/kalma2-bridge/Cargo.toml` | dep `uuid` (v4) |
| 3 | `SddIA/engine/execute-process/src/engine/handlers/kalma2.rs` | `build_kalma2_process_event` honra `correlation_id` UUID; `plumb_correlation_id` top-level → nested; tests L4 |
| 4 | `interfaces/kalma2/app.js` | Rama `202`/`status==accepted` → `pollStatus(correlation_id)`; fallback legado `emitted` |

## Notas de forja

- Bridge **no** escribe `.events/**`; chispa = hijo `kalma2-interact` (L1/Q1).
- `correlation_id ≡ event_id` preasignado por bridge e inyectado en `--inputs` (L3/Q3).
- Chat / SSE intactos (L6). H3 Telegram defer (L7).
- Eventos / subscriptions / allowlist: **sin mutación**.
- Fix U1: test `bridge_execute_path_has_no_eda_write_helpers` audita solo prod (split `#[cfg(test)]`) — evitaba autoincriminación por `include_str`.

## AC cubiertos en código

| AC | Materialización |
|----|-----------------|
| AC-R1 | 202 + `accepted` + cid sin join del ciclo |
| AC-R2 | spawn genómico; audit estático sin `write_fractal_event` |
| AC-R3 | despacho/nervio intacto (sin tocar subscriptions) |
| AC-R4 | poll UI con cid del acuse vía `GET /api/status` existente |
| AC-R5 | chat no mutado como Done |
| AC-R6 | defer H3 documentado |

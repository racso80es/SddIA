---
feature_name: man-50-cierre-man-43
created: "2026-11-20"
updated: "2026-11-20"
process: feature
branch_name: feat/man-50-cierre-man-43
persist_ref: docs/features/man-50-cierre-man-43
pbi_ref: MAN-50
parent_ref: MAN-43
related_ref: MAN-49
document_id: PBI-91b7ec84
phase: ejecucion
status: implemented
norm_ref: features-documentation-pattern@1.3.1
execution_id: "6793b598-8a33-43d8-87db-3c3012d5712d"
agent: tekton
---
# Implementación — man-50-cierre-man-43

Cierre residual **B-01..B-03** post-merge MAN-49/MAN-43 (auditoría `docs/audits/man-49-man-43-post-merge-audit-20261120.md`).

## 1. R-01 (B-01 — DLQ solo puerto)
- `eda_bus_topology.rs`: eliminado default `bus_defaults_from_root(".events")` → `bus_defaults_empty()`; añadidos `publish_dead_letter_via_port` / `nack_subscriber_dead_letter_via_port` (`record_ecst`+`move_to_dlq` / `record_delivery_nack`+`move_to_dlq`).
- `sddia-daemon-runtime/src/eda_bus.rs`: mismo default vacío (sin literal `".events"`).
- Migrados a puerto: `write_di_dead_letter`, `write_resolve_dead_letter`, `write_output_dead_letter`, `write_cerbero_di_dead_letter`, `write_cerbero_envelope_dead_letter`, `write_dead_letter_fallback` + rama `no_subscriber` en `route_domain_core`.
- `write_json_atomic` se conserva para usos no-DLQ (p. ej. radamanto/daemons).
- `eda_event_token_hygiene`: veto de literales `".events"` / `".events/"` (y variantes); `assert_repo_root_bus_dirs_absent` incluye `".events"`. Allowlist acotada para `fractal_bus` / `instance_creator`.

## 2. R-03a (B-03a — allowlist vacía)
- `eda-unit-backend.toml` → `integration_src = []`.
- `gate_eda_unit_no_real_backend.rs` → `INTEGRATION_SRC: &[]`.
- Scrub de tokens prohibidos en unitarios de `route_domain_core`, `system_vitality`, `daemon_heartbeat` (`.events/` → `lab-bus/`; `events.redb` → `eda-store.redb` en cfg(test)).

## 3. R-02 (B-02 — binarios + raíz)
- Rebuild **debug** de `sddia-workspace-server`, `event-watcher`, `execute-process` (OK).
- Rebuild **release** en curso / parcial (ver `execution.md`).
- Purga de `.events/` en raíz del piloto (ausente post-purga); assert raíz cubre `".events"`.

## 4. R-03b (B-03b — docs/genoma)
- `.antigravity/rules/[ARQUITECTURA] constitution_core.md`: eliminado token `/.events/*`.
- `SddIA/events/events-contract.md` §4 alineado a `EventManagerPort` (sin `SDDIA_EVENT_STORE`).
- Códex process ×4: tokens `SDDIA_EVENT_STORE` sustituidos; sello `entity-manager` invocado (update; ver nota L-GENOME en execution).
- `docs/audits/man-43-o6-remediation-20261113.md`: dictamen alineado APTO_TOTAL / sin tokens residuales.

## 5. Fuera de este ciclo
- Auto-sello `APTO` (solo Argos).
- Semillas Kaizen bajo `docs/todos/`.

---

feature_name: man-38-redb-port-lock
process: bug-fix
persist_ref: docs/fixes/man-38-redb-port-lock
tracker_ref: MAN-38
execution_id: "be63c70f-633c-452e-aaf1-8ffa645fd54b"
---
# Ownership — EventManagerPort productivo (MAN-38)

## Quién abre Redb

Solo el host (`sddia-event-manager-host`) vía `resolve_productive_port` → factory `default` → `RedbEventAdapter` sobre `.SddIA/runtime/events.redb`.

**API productiva canónica:** `with_productive_port(repo, op_label, f)` — abre, ejecuta, dropea.

## Lifetime productivo

1. Una operación del trait, o un **lote atómico acotado** (p. ej. auditoría `audit_scan`, `pec_for_correlation`, `materialize_to_bus`).
2. Al salir del scope se libera el lock del fichero.
3. Prohibido retener `Arc<dyn EventManagerPort>` entre ticks de daemon salvo lab/tests de vida breve.

## Lifetime de test

`eda_port::install_port(mock)` en execute-process; `resolve_test_port()` en host. `ParallelLabGuard` / `clear_port` higienizan el lab.

## Retry

`SDDIA_EVENT_MANAGER_OPEN_BUDGET_MS` (default 2000 ms) es escudo transitorio; no sustituye scoped ownership.

## Telemetría

`[event-manager-host] open_contention process=… operation=… wait_ms=… budget_ms=… outcome=ok|locked|error`

## Barrido T4 (callers migrados)

| Locus | Patrón |
|-------|--------|
| `sddia-daemon-runtime` (`record_ecst`, `eda_sweep`, `github_bridge`) | `with_productive_port` |
| `kalma2-bridge` / `eda_productive.rs` | scoped por función pública |
| `event-bus-audit`, `repo-retention-purger`, `iota_ci_smoke` | scoped |
| `lancedb_thought_repo` publish + tests | scoped |
| `execute-process` `actions` tests | `with_event_port` |

## Residual documentado

| Símbolo | Uso |
|---------|-----|
| `resolve_productive_port` | Solo **dentro** de `sddia-event-manager-host` y `with_productive_port` |
| `eda_port::port_arc` | Anti-patrón productivo; tests / apertura efímera; preferir `with_event_port` |

---
feature_name: kaizen-eda-domain-iota-routing
created: "2026-06-12"
process: bug-fix
branch_name: fix/kaizen-eda-domain-iota-routing
persist_ref: docs/fixes/kaizen-eda-domain-iota-routing
pbi_ref: docs/todos/pending/Kaizen_eda-domain-iota-routing.md
---

# Especificación — Kaizen EDA domain + IOTA Testnet real

## Problema

1. `Manual_Task_Requested` y `Kaizen_Idea_Captured` tenían `[]` en `event-domain-subscriptions.json`.
2. `SDDIA_LAB_SIMULATE_IOTA=1` en sesiones manuales anulaba anclaje DLT real pese a bóveda `=0`.
3. `route_domain_event_core` invocaba `npx tsx` sin dependencia declarada; `node_modules` ausente.
4. `bus-operator.sh` inexistente; operadores invocaban solo `.py`.

## Cambio

| Área | Artefacto |
|------|-----------|
| Suscripciones | `SddIA/core/event-domain-subscriptions.json` — IOTA en `Manual_Task_Requested`, `Kaizen_Idea_Captured`, `TelegramMessage_Received` |
| Launcher skill | `scripts/skills/bus-operator.sh` |
| Bóveda / env | `env_loader.py` — precedencia vault en flags IOTA |
| Invocador IOTA | `route_domain_event_core.py`, `dlt_bus_materializer.py`, `audit-entity-eda-coverage.py` → `ts-node` |
| Toolchain | `install-deps.sh`; `event-watcher.sh` — PATH Node `.tools/` |
| Watcher docs | `event-watcher.py` / `.bat` / `.sh` — mensajes Testnet real |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| KZ-CA1 | `resolve_subscribers` devuelve IOTA para `Manual_Task_Requested` |
| KZ-CA2 | `route-domain` + IOTA real: `transaction_digest` sin prefijo `lab-sim-` |
| KZ-CA3 | `event-watcher --once` enruta 15 eventos `domain/` sin fallo IOTA |
| KZ-CA4 | Documentación fix + PBI archivado + `validacion.md` APTO |

## Fuera de alcance

- Purga física de `./.events/domain/` (D3 + `purge_after=false`).
- Re-cablear `bus-operator` en `route-domain` (deuda PBI-005).

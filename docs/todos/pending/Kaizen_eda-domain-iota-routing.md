---
document_id: PBI-KAIZEN-EDA-DOMAIN-IOTA-ROUTING
title: "[Kaizen] EDA domain — suscriptores IOTA, launcher bus-operator e invocación Testnet real"
format: markdown
version: "1.0.0"
created: "2026-06-12"
status: pending
priority: alta
process: bug-fix
branch_name: fix/kaizen-eda-domain-iota-routing
feature_ref: docs/fixes/kaizen-eda-domain-iota-routing
---

# [Kaizen] EDA domain — suscriptores IOTA, launcher bus-operator e invocación Testnet real

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-KAIZEN-EDA-DOMAIN-IOTA-ROUTING` |
| **Estatus** | Pendiente |
| **Fix** | [`docs/fixes/kaizen-eda-domain-iota-routing/`](../../fixes/kaizen-eda-domain-iota-routing/) |

## 1. Incidente

Eventos en `./.events/domain/` sin gestión efectiva: `Manual_Task_Requested` con suscriptores vacíos, IOTA simulado o fallido (`tsx`/`npm` ausente), y confusión operativa con `bus-operator` (cápsula no enruta el bus fractal).

## 2. Alcance

- Suscriptores `cumulo → iota-immutable-publisher` en `event-domain-subscriptions.json`.
- Launcher `scripts/skills/bus-operator.sh` (D8).
- Bóveda + `env_loader`: IOTA real (`SDDIA_LAB_SIMULATE_IOTA=0`).
- Invocador `ts-node` + `install-deps.sh`; watcher con Node `.tools/`.

## 3. Done

Un PR mergeado + `validacion.md` APTO + PBI en `docs/todos/done/`.

---
document_id: PBI-FIX-IOTA-DLT-7c7bba8c
title: "[FIX] iota-immutable-publisher — DLT opaco PullRequest_Presented"
format: markdown
version: "1.0.0"
created: "2026-07-20"
status: done
priority: alta
process: bug-fix
branch_name: fix/iota-dlt-pr-presented-persistence-7c7bba8c
fix_ref: docs/fixes/iota-dlt-pr-presented-persistence-7c7bba8c
incident_ref: "dead-letter/.events — 7c7bba8c-4286-4302-bb31-f8928b81b132.cumulo.iota-immutable-publisher"
related:
  - .events/dead-letter/subscribers/7c7bba8c-4286-4302-bb31-f8928b81b132.cumulo.iota-immutable-publisher.json
  - .events/dead-letter/7c7bba8c-4286-4302-bb31-f8928b81b132.json
  - SddIA/tools/iota-immutable-publisher.md
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
---

# [FIX] iota-immutable-publisher — DLT opaco PullRequest_Presented

## Incidente

| Campo | Valor |
|-------|--------|
| Evento | `PullRequest_Presented` |
| UUID | `7c7bba8c-4286-4302-bb31-f8928b81b132` |
| Suscriptor | `cumulo.iota-immutable-publisher` |
| Emisor | `delivery-close-cycle` |
| Rama payload | `fix/event-sweeper-heartbeat-fracture-f4befd66c513` |
| Traza DLT | `iota publish failed` |

## Evidencia correlacionada (mismo event_uuid)

| Suscriptor | error_trace |
|------------|-------------|
| `cumulo.iota-immutable-publisher` | `iota publish failed` (opaco) |
| `argos.pull-request-review` | `INPUT_VALIDATION` — falta `pr_url` |
| `argos.send-telegram-notification` | `Network is unreachable` |

Payload ECST del dominio: solo `{ "branch", "status": "presented" }` — sin `pr_url` ni metadatos de anclaje.

## Hipótesis de causa raíz (pre-Dedalo)

1. **Pérdida de traza:** `sddia-io::emit_error` escribe en `error`; `invoke_iota_publisher` (route-domain) lee solo `feedback`/`message` → fallback literal `iota publish failed`.
2. **Persistencia DLT fallida:** con `SDDIA_LAB_SIMULATE_IOTA=0`, ausencia de mock/relay usable en el watcher → publish real no materializa digest.
3. **ECST incompleto:** `delivery-close-cycle` emite `PullRequest_Presented` sin `pr_url` (rompe aduana Argos; degrada contexto del anclaje).

## Mandato

Corregir causa raíz de fallo de anclaje IOTA y de opacidad DLT; validar re-ruta / no-regresión en `PullRequest_Presented`. **Prohibido bypass raw** hasta cierre documentado.

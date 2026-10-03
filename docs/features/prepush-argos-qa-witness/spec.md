---
feature_name: prepush-argos-qa-witness
created: "2026-10-02"
process: bug-fix
---

# Especificación — dictamen OSC-12

## Witness (R-1)

Evento: `Local_QA_Requested` · `event_id` `a55f1d12-003b-40d3-ae21-6e69d575c257` · rama `feat/tracker-operations-context`.

Dead-letter suscriptor `argos.pull-request-review` (`.events/dead-letter/subscribers/a55f1d12-….argos.pull-request-review.json`):

| Campo | Valor |
|-------|--------|
| `dispatch_mode` | `sync` |
| `delegation.exit_code` | `1` |
| `delegation.target` | `pull-request-review` |
| `error_trace` | `fase "Triaje documental" failed` |

El binario `execute-process` (`route-domain-event`) propagó **exit ≠ 0**. `pre_push_gate.sh` solo llama a `delivery-close-cycle` si `invoke_process route-domain-event` retorna 0 (`blocking: true` en el JSON activa `SyncRouteGuard` en `handlers/route_domain.rs`, equivalente a `SDDIA_LAB_ROUTE_SYNC=1` del contrato).

## Dictamen (R-2)

**No hubo exit 0 indebido** en este witness: el fallo fue causal (PPR `status_code` 1 por fase `Triaje documental` / posterior veredicto Argos en handoff con la misma `correlation_id`). El hook **no** debe empujar DCC tras ese route; el síntoma «DCC con Argos en dead-letter» exige otro camino (p. ej. `SDDIA_SKIP_HOOKS`, push sin hook, o push anidado con guarda `SDDIA_HOOK_DELIVERY_CLOSE`).

Refuerzo motor: fase agente `blocked` en PPR escribe `argos_verdict: block` en state (`residual_runner` / `executor`) para alinear agregación terminal (`phase_terminal`).

## Fuera de alcance

Reabrir veredicto F2 de `tracker-operations-context`; rellenar carpetas F2 ajenas a este PBI.

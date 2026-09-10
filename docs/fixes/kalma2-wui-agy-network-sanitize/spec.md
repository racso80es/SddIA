---
feature_name: kalma2-wui-agy-network-sanitize
created: "2026-09-10"
process: bug-fix
base: main
scope: kalma2-bridge-epidermal-agy-network
version_spec: "1.0.0"
branch_name: fix/kalma2-wui-agy-network-sanitize
persist_ref: docs/fixes/kalma2-wui-agy-network-sanitize
pbi_ref: docs/todos/done/[FIX] Kalma2 WUI — red agy sanitizada en epidermis (sin retry).md
document_id: PBI-FIX-KALMA2-AGY-NETWORK-SANITIZE
execution_id: "2dab4bf4-408b-4dc6-919a-680c92588cf7"
incident_ref: "Auditoría empírica WUI 2026-09-10 — agy exit=1 network issue connecting"
---

# Especificación — sanitización red agy en epidermis `kalma2-bridge`

## Diagnóstico

`agy` SUCCESS de sesión (`conversation_id`) + `status: ERROR` de red. Skill: `agy exit=1; stderr=…; stdout=…`. `sanitize_bridge_message` no clasifica. Recorte 240. «Please try again» = copy del CLI, no retry (DA-5).

Ajuste de host (IPv6 WiFi `disabled`) es instancia; no entra en este PR.

## Corrección

### H1 — Clasificador

Antes del recorte 240, tras timeout y antes de auth:

| Señal | Canónico |
|-------|----------|
| `network issue connecting` (ci) | `AIUA_AGY_NETWORK_MSG` (82 chars) |
| `dial tcp` (ci) | el mismo |

Distinto de 503, auth y `agy-timeout`. HTTP puente sigue 500.

### H2 — Tests herméticos

Fixture empírico recortado; fixture `dial tcp` de elegibilidad; no-regresión auth.

### H3 — Intactos

Skill `antigravity-cli-executor`, `app.js`. Cero sleep/retry.

## CA

AGY-NET-CA1…CA6 y CA-CI según PBI.

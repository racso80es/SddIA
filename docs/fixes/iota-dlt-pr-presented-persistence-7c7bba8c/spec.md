---
feature_name: iota-dlt-pr-presented-persistence-7c7bba8c
created: "2026-07-20"
process: bug-fix
branch_name: fix/iota-dlt-pr-presented-persistence-7c7bba8c
persist_ref: docs/fixes/iota-dlt-pr-presented-persistence-7c7bba8c
pbi_ref: "docs/todos/pending/[FIX] iota-immutable-publisher — DLT opaco PullRequest_Presented (7c7bba8c).md"
---

# Especificación — IOTA DLT opaco en PullRequest_Presented

## Problema confirmado

Evento `7c7bba8c-4286-4302-bb31-f8928b81b132` (`PullRequest_Presented`) → DLT suscriptor `cumulo.iota-immutable-publisher` con `error_trace: "iota publish failed"`.

Causa raíz dual:

| # | Defecto | Evidencia |
|---|---------|-----------|
| 1 | `invoke_iota_publisher` lee solo `feedback`/`message`; `sddia-io::emit_error` escribe en `error` | Capsule stdout `{"success":false,"error":"iota-publish-unavailable:..."}` → DLT pierde clasificación |
| 2 | Sello ECST emite sin `pr_url` cuando estado/forge no lo materializa | Payload dominio solo `{branch,status}`; Argos `pull-request-review` DLT `missing:["pr_url"]` |

Instancia: `SDDIA_LAB_SIMULATE_IOTA=0` + wallet presente + **sin** `IOTA_PUBLISH_RELAY_URL`/`SDDIA_LAB_MOCK_IOTA_URL` → fallo real `iota-publish-unavailable` (oculto por #1).

## Objetivo técnico

1. Propagar el mensaje clasificado de la cápsula al testigo DLT (`error` ∨ `feedback` ∨ `message`).
2. Impedir emisión de `PullRequest_Presented` sin `pr_url` (gate en sello delivery-close).
3. Validar ruta lab con `SDDIA_LAB_SIMULATE_IOTA=1` → digest `lab-sim-*` y sin DLT opaco.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Fallo cápsula IOTA con `error` poblado → `error_trace` DLT contiene el texto clasificado (no el literal genérico `iota publish failed` salvo que sea el mensaje real) |
| CA2 | `emit-pr-presented` / sello delivery exige `pr_url`; ausencia → fallo de fase, no evento incompleto |
| CA3 | Lab simulate: publish OK con digest `lab-sim-*` |
| CA4 | `cargo test -p execute-process` filtros iota/route_domain relevantes OK |
| CA5 | Cascada documental `implementation.md` + `execution.md` + `validacion.md` APTO |

## Fuera de alcance

- Purga física de DLT históricos.
- Implementar SDK IOTA nativo (sigue relay/mock/simulate).
- Remediación Telegram `Network is unreachable` del mismo UUID.
- Mutación de bóveda `.SddIA/.dev/.env` (ops: configurar relay si SIMULATE=0).

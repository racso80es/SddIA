---
document_id: PBI-FIX-FRACTURE-b3a715381787
uuid: "b3a71538-1787-4000-8000-000000000001"
title: "[FIX] route-domain-event — fractura sistémica"
format: markdown
version: "1.1.0"
created: "2026-08-28"
updated: "2026-08-29"
status: "abierto"
priority: alta
process: bug-fix
fracture_hash: b3a715381787
fracture_process: route-domain-event
friction_id: F-DLT-RELAY-SIN-SUPERVISOR
incident_ref: "System_Fracture_Detected — b3a715381787"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/tools/iota-immutable-publisher.md
  - SddIA/daemons/iota-publish-relay.md
  - .SddIA/services/iota-publish-relay/server.mjs
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (6a49e0ad310e).md
---

# [FIX] route-domain-event — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `route-domain-event` |
| Emisor | `execute-process` |
| Acción intentada | `merkle-batch-preseal` |
| Cápsula física | `iota-immutable-publisher` → `publish_via_relay` |
| Endpoint | `POST http://127.0.0.1:8787/v1/publish` |
| Clasificación | `F-DLT-RELAY-SIN-SUPERVISOR` |

## Traza de error

```
merkle-batch-preseal failed: iota-relay-unreachable: http://127.0.0.1:8787/v1/publish: status code 500
```

**Lectura literal:** el relay **está vivo y responde** (HTTP 500, no `connection refused`). El fallo ocurre **dentro** del publish del relay, no en su ausencia. El prefijo `iota-relay-unreachable` de `ureq` es engañoso: envuelve toda respuesta no-2xx, incluida 500 con relay activo (`iota-immutable-publisher/src/main.rs` → `publish_via_relay`, `.map_err(|e| "iota-relay-unreachable: {e}")`).

## Delimitación (vs fractura histórica `6a49e0ad310e`)

| Aspecto | `6a49e0ad310e` (cerrada) | `b3a715381787` (esta) |
|---------|--------------------------|------------------------|
| Traza | `Campo obligatorio ausente o inválido: payload` | `status code 500` en `/v1/publish` |
| Causa física | Relay caído → publisher sin payload | Relay vivo → publish falla en ejecución |
| Remediación previa | Centinela `iota-publish-relay` + L-REQUIRED `/health` (Kaizen DLT #208) | **No cubierta**: `/health` OK no garantiza publish OK |

La supervisión del centinela cubre disponibilidad (`/health`), **no** la salud del publish real (wallet, package Move, red Testnet). Por eso la clasificación `F-DLT-RELAY-SIN-SUPERVISOR` es **imprecisa** para este caso.

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Hipótesis de causa raíz (a verificar)

Ordenadas por probabilidad. El 500 se origina en `server.mjs` (`sendJson(res, 500, { success:false, error })`):

1. **`config-missing: IOTA_WALLET_SECRET`** — bóveda `.SddIA/.dev/.env` sin wallet Testnet válida.
2. **`IOTA_ANCHOR_PACKAGE_ID` ausente/erróneo** — publica package efímero o falla la transacción Move.
3. **Fallo SDK IOTA / red Testnet** — timeout, faucet/gas agotado, endpoint IOTA no alcanzable desde el hijo Node.
4. **Mismatch de contrato publish** — payload Merkle (array de strings) rechazado por el `server.mjs` actual.

### Verificación (diagnóstico, sin bypass de entrega)

- Aislar el 500 real: revisar log del hijo Node del daemon `iota-publish-relay` (mensaje `error`/`feedback` del cuerpo de respuesta).
- Confirmar `/health` OK vs publish KO para probar que es fallo de publish, no de disponibilidad.
- Auditar bóveda `.SddIA/.dev/.env`: `IOTA_WALLET_SECRET`, `IOTA_ANCHOR_PACKAGE_ID`, `SDDIA_LAB_SIMULATE_IOTA`.
- Inspeccionar cola `.SddIA/dlt/reanchor-queue/` (SSOT `cumulo.paths.json` → `eda_instance.dlt_reanchor`): el lote se drenará solo si el siguiente batch encuentra `/health` OK **y** el publish deja de fallar.

## Propuesta evolutiva

Doble vía; el Vértice Biológico prioriza:

- **Fix de causa raíz (infra DLT):** remediar la config/entorno que produce el 500 en publish (candidatos 1–3).
- **Kaizen de observabilidad (recomendado):** desambiguar `iota-relay-unreachable`. Distinguir `relay-down` (transporte / connection refused) de `relay-publish-error` (HTTP no-2xx con cuerpo de error), propagando el `error`/`feedback` del relay a la traza y a `classify_batch_anchor_friction`. Nueva fricción `F-DLT-PUBLISH-ERROR` para no mimetizar con `SIN-SUPERVISOR`.

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async)*

### Diagnóstico de causa raíz

- Fallo de **publish** en relay vivo (HTTP 500), no ausencia de supervisor. La telemetría de fricción colapsa dos modos de fallo distintos bajo un mismo `friction_id`, ocultando la causa física.

### Veredicto evolutivo

**Corrección de infraestructura + Kaizen de clasificación** (`root_cause_fix` + `observability`).

### Propuestas

- **Infra:** restaurar precondiciones de publish IOTA (wallet/package/red) en la bóveda de instancia.
- **Observabilidad:** propagar cuerpo de error del relay y separar `relay-down` de `relay-publish-error` en la clasificación de fricción.

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.

## Criterio de cierre

- [ ] Causa raíz del HTTP 500 identificada y documentada (candidato confirmado en la tabla de hipótesis)
- [ ] Publish real reproducible en verde: `/v1/publish` responde 2xx con `transaction_digest`
- [ ] Cola `.SddIA/dlt/reanchor-queue/` drenada (sin lotes huérfanos)
- [ ] (Si aplica Kaizen) `F-DLT-PUBLISH-ERROR` desambiguada de `F-DLT-RELAY-SIN-SUPERVISOR`
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este TODO movido a `docs/todos/done/` (sin copia stale en `pending/`)

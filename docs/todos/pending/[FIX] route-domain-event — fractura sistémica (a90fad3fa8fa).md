---
document_id: PBI-FIX-FRACTURE-a90fad3fa8fa
uuid: "832fb2e6-ebde-4ec7-9077-696b16f88b92"
title: "[FIX] route-domain-event — fractura sistémica"
format: markdown
version: "1.1.0"
created: "2026-08-30"
updated: "2026-09-01"
status: "abierto"
priority: alta
process: bug-fix
fracture_hash: a90fad3fa8fa
fracture_process: route-domain-event
friction_id: F-DLT-PUBLISH-ERROR
incident_ref: "System_Fracture_Detected — a90fad3fa8fa"
refined: true
suggested_branch: fix/iota-dlt-publish-fetch-failed
persist_ref_suggested: docs/fixes/iota-dlt-publish-fetch-failed
source_audit: "2026-09-01T07:33Z host: unit sddia-iota-publish-relay@home-racso-Proyectos-SddIA active since 2026-08-30 19:58:10 CEST; ELF debug mtime 19:57:40; hijo node pid=67195 :8787; GET /health 200; SHA-256[:12](traza)=a90fad3fa8fa; PBI commit 5e40ed9 20:07:06 CEST; bóveda wallet+package presentes (valores no volcados), SDDIA_LAB_SIMULATE_IOTA=0, SDDIA_LAB_MOCK_IOTA_URL vacío; cola eda_instance.dlt_reanchor vacía; heartbeat iota-publish-relay healthy/alive; server.mjs catch solo err.message."
review_notes: "v1.0.0 stub Cúmulo válido + Mayeuta prompt_adjustment (alucinación catch-all failed). v1.1.0 refinamiento Tekton: causa = fetch SDK→fullnode Testnet con relay vivo; taxonomía b3a715381787 ya operó; ELF 701c77ebeab8-R1 ya desplegado. No es fallo de prompt ni de supervisor."
friction_ids:
  - F-DLT-PUBLISH-ERROR
architectural_constraints:
  - A-FRACTURE-HASH-INMUTABLE
  - A-NO-REABRIR-TAXONOMIA-B3A715
  - A-NO-REABRIR-ELF-R1
  - A-HEALTH-NO-ES-PUBLISH
  - A-NO-SIMULATE-COMO-CIERRE
  - A-NO-VOLCAR-BOVEDA
execution_file_lock:
  - .SddIA/services/iota-publish-relay/server.mjs
gates_this_wave:
  - DLT-FETCH-CA1
  - DLT-FETCH-CA2
  - DLT-FETCH-CA3
  - DLT-FETCH-CA4
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/tools/iota-immutable-publisher.md
  - SddIA/daemons/iota-publish-relay.md
  - .SddIA/services/iota-publish-relay/server.mjs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (b3a715381787).md
  - docs/fixes/route-domain-event-fracture-b3a715381787/validacion.md
  - docs/todos/done/[REGRESIÓN] route-domain-event — fractura sistémica (701c77ebeab8)-R1.md
  - docs/fixes/iota-publish-relay-elf-fosil-r1/validacion.md
related_pbis:
  - id: PBI-FIX-FRACTURE-b3a715381787
    rol: "Hermano taxonomía. Prefijo iota-relay-publish-error + F-DLT-PUBLISH-ERROR entregados. CA5 instancia (causa física del 500) diferida — este sello la cumple en parte: cuerpo = fetch failed."
  - id: PBI-FIX-FRACTURE-701c77ebeab8-R1
    rol: "Hermano deploy. RELAY-R1-CA3 (sello DLT post-relay sano) quedó PENDIENTE_INSTANCIA. Este hash es ese primer lote: health 200, publish 500."
  - id: PBI-FIX-FRACTURE-6a49e0ad310e
    rol: "Ancestro. Relay caído → payload inválido. Distinta traza, distinto friction_id. No reabrir."
---

# [FIX] route-domain-event — fractura sistémica

> **Refinamiento v1.1.0.** El stub Cúmulo (sello + traza) es válido. La síntesis Mayeuta (`prompt_adjustment`) es **inválida**. Afirmaciones descartadas en §7.

## 1. Identidad del sello (no tocar)

| Campo | Valor | Notas |
|-------|--------|--------|
| `fracture_hash` | `a90fad3fa8fa` | SHA-256[:12] de la traza literal. Verificado 2026-09-01. Inmutable. |
| `fracture_process` | `route-domain-event` | Pre-sellado Merkle del lote. El hijo HTTP es `server.mjs`, no el ELF supervisor. |
| `friction_id` | `F-DLT-PUBLISH-ERROR` | `classify_batch_anchor_friction`: `iota-relay-publish-error` **antes** de `unreachable`. |
| Emisor | `execute-process` | Correcto. |
| Acción intentada | `merkle-batch-preseal` | Correcto. |
| Cápsula | `iota-immutable-publisher` → `publish_via_relay` | Prefijo de traza emitido ahí. |
| Endpoint | `POST http://127.0.0.1:8787/v1/publish` | Relay instancia. |

### Traza (inmutable)

```
merkle-batch-preseal failed: iota-relay-publish-error: status=500 fetch failed
```

Prohibido alterar `fracture_hash` / `fracture_process` / traza.

## 2. Hechos verificados (2026-09-01)

Zona: CEST = UTC+2. Auditoría Tekton ~07:33Z / 09:33 CEST.

| Hecho | Evidencia |
|-------|----------|
| Materialización PBI | Commit `5e40ed9` `2026-08-30 20:07:06 +0200`. Stub Cúmulo + sección Mayeuta en el mismo write. |
| Linaje temporal | Merge R1 ELF `d98f900` 20:03:33. Unit restart **19:58:10**. Este sello **~9 min después** del relay sano. |
| Centinela | `sddia-iota-publish-relay@home-racso-Proyectos-SddIA` **active running** desde 19:58:10. PID 67163. Sin murder-loop post-restart. |
| ELF vs fuente | `SddIA/target/debug/iota-publish-relay` mtime **19:57:40** ≥ `main.rs` 18:27:15. No es el fósil del 28 ago. |
| Hijo HTTP | `node server.mjs` pid=67195. `ss` LISTEN `127.0.0.1:8787`. |
| `/health` | HTTP 200 `{ok:true,service:iota-publish-relay}` (GET diagnóstico; no es bypass de entrega). |
| Espejo | `heartbeat-audit.json` `iota-publish-relay` `classification=healthy`, `status=alive`, `missed_cycles=0`. |
| Bóveda | `IOTA_WALLET_SECRET` presente. `IOTA_ANCHOR_PACKAGE_ID` presente. `SDDIA_LAB_SIMULATE_IOTA=0`. `SDDIA_LAB_MOCK_IOTA_URL` vacío. `IOTA_PUBLISH_RELAY_URL=http://127.0.0.1:8787/v1/publish`. Valores no volcados. |
| Taxonomía | Prefijo `iota-relay-publish-error:` = CA-1 de `b3a715381787` **operando**. No es `iota-relay-unreachable`. |
| Cuerpo 500 | `fetch failed` = `err.message` del `catch` en `server.mjs` (`sendJson(res, 500, {error, feedback})`). No hay `console.error` del catch; `relay.log` solo tiene `listening`. |
| Cola re-anclaje | `.SddIA/dlt/reanchor-queue/` **vacía** en auditoría. No hay lote huérfano observable hoy. |
| Payload Merkle | Publisher convierte array → raíz hex **string** y esa string viaja al relay. Un mismatch array/string sería HTTP **400** `Campo obligatorio ausente: payload`, no este 500. |

## 3. Qué hizo mal Mayeuta

`analyze_fracture_kaizen`: token `failed` en `merkle-batch-preseal failed` **y** en `fetch failed` → catch-all → `prompt_adjustment` («bloqueo sin Kintsugi»).

No hay cubo para `iota-relay-publish-error` / `F-DLT-PUBLISH-ERROR`. El clasificador **no describe este incidente**.

`System_Fracture_Detected` **ya** se emitió (Cúmulo materializó este PBI). No hay entrega Tekton a castrar. El párrafo Mayeuta v1.0.0 queda como síntoma del clasificador, no como mandato de diseño.

Cubo Mayeuta para este prefijo = **deuda ortogonal** (motor `enrich_fracture_pbi_kaizen.rs`). Fuera de `execution_file_lock` de esta ola.

## 4. Discriminación de hipótesis

Candidatos heredados del PBI refinado `b3a715381787` (f8273a5). La taxonomía ahora expone el cuerpo; se puede laudar.

| # | Hipótesis | ¿Sostiene este sello? | Lectura |
|---|-----------|------------------------|---------|
| 1 | `config-missing: IOTA_WALLET_SECRET` | **Refutado** | Bóveda presente. Publisher aborta *antes* del relay con ese literal. El cuerpo no es `config-missing`. |
| 2 | `IOTA_ANCHOR_PACKAGE_ID` ausente/erróneo → fallo Move | **Improbable** | Package presente. Abort Move / efectos de transacción no se reportan como `fetch failed`. Si el SDK ni siquiera alcanza el fullnode, no llega a Move. |
| 3 | Fallo SDK IOTA / red Testnet (`IotaClient` → `getFullnodeUrl("testnet")`) | **Sostiene** | `fetch failed` es el `TypeError.message` de undici/Node cuando `fetch()` al fullnode falla. Causa real (`ENOTFOUND`, timeout, TLS, refused) vive en `err.cause` y **no** viaja en el 500. |
| 4 | Mismatch contrato: array Merkle rechazado por `server.mjs` | **Refutado** | Publisher manda string (raíz hex). Rechazo de tipo = HTTP 400, no 500 `fetch failed`. |
| — | Relay caído / supervisor impaciente / ELF fósil | **Refutado** | Health 200, hijo LISTEN, ELF ≥ fuente, journal sin kill-en-el-mismo-segundo. Eso es `701c77ebeab8`. |
| — | Taxonomía `unreachable` vs `publish-error` | **Refutado como causa** | El prefijo correcto **ya** está en la traza. `b3a715381787` cerrado en código. |
| — | Operador evadió Kintsugi / `prompt_adjustment` | **Refutado** | Fractura emitida. PBI materializado. No hay ciclo de entrega en vuelo. |
| — | `SDDIA_LAB_SIMULATE_IOTA=1` como cierre | **Prohibido** | Simular oculta el sello; no ancla. |

**Laudo:** candidato **3**. Relay vivo; publish muere en el transporte HTTP del SDK hacia el fullnode Testnet. Diagnóstico de instancia **lossy**: el 500 no incluye `err.cause`.

## 5. Causa estructural

Dos capas. El sello es el síntoma de la primera; la segunda impide laudo fino sin reproducir.

### 5.1 Transporte SDK → fullnode (causa del sello)

`publishImmutableData` en `server.mjs` construye `IotaClient({ url: getFullnodeUrl("testnet") })` y llama `signAndExecuteTransaction` / `waitForTransaction` (o `tx.publish` si el package no estuviera cacheado). Cualquier fallo de `fetch` nativo se captura en el `catch` y se serializa como HTTP 500 `{error: err.message}`.

`/health` no toca IOTA. Health 200 **no** implica publish OK (`A-HEALTH-NO-ES-PUBLISH`).

### 5.2 Diagnóstico truncado (impedimento de laudo fino)

`catch (err)` usa solo `err.message`. En Node 22 / undici, `TypeError: fetch failed` deja el código de sistema en `err.cause`. Sin ese campo en el JSON, la traza no distingue DNS, timeout, TLS o fullnode caído.

`relay.log` no registra el catch. Journal del unit no ve stdout del 500. La única evidencia del sello es la traza de fractura.

## 6. Alcance del fix (si laudo 3)

Ciclo `bug-fix`. Genoma `SddIA/tools/` **no** se toca (taxonomía ya entregada). DA-2 intacto.

### Dentro

- Propagar en el 500 de `server.mjs` `err.cause` (código + syscall + mensaje) además de `err.message`. Loguear el catch a stderr/`relay.log`.
- Verificar alcanzabilidad del fullnode Testnet **desde el hijo Node** (no desde un `curl` de entrega). Restaurar red/gas/endpoint si el cause lo nombra. Sin volcar bóveda.
- Un lote Merkle post-fix con `transaction_digest` / `merkle_anchored` (cierra RELAY-R1-CA3 del hermano).
- Documentar en `execution.md` el cause concreto (no solo `fetch failed`).

### Fuera

- Reabrir `PBI-FIX-FRACTURE-b3a715381787` o reescribir prefijos `iota-relay-publish-error`.
- Reabrir Ola 0/1 ni el resolutor ELF de `701c77ebeab8-R1`.
- Touchpoint Kintsugi / `prompt_adjustment`.
- `SDDIA_LAB_SIMULATE_IOTA=1` como Done.
- Cubo Mayeuta `iota-relay-publish-error` (Kaizen motor aparte).
- Mutar `iota-immutable-publisher` salvo que el 500 deje de mapear a `F-DLT-PUBLISH-ERROR` (no es el caso).
- POST raw a `/v1/publish` o `gh`/`git` de entrega.

## 7. Afirmaciones descartadas del stub v1.0.0

| Afirmación del stub | Verdad |
|---------------------|--------|
| `prompt_adjustment` / «no continuar entrega» | No hay entrega. Hay 500 de publish con relay vivo. |
| «Bloqueo operativo sin escalado Kintsugi» | `System_Fracture_Detected` emitido; Cúmulo abrió este PBI; Mayeuta lo ensució. |
| Causa = operador | Causa = fetch SDK→Testnet + cuerpo lossy. |
| Proceso «roto» `route-domain-event` | El proceso clasificó y fracturó **bien**. Falló el anclaje on-chain. |

## 8. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| DLT-FETCH-CA1 | HTTP 500 de publish incluye `error`/`feedback` con `message` **y** `cause` (o equivalente serializado). `relay.log` o stderr del hijo deja traza del catch. | Reproducción controlada o test del handler; no POST de entrega. |
| DLT-FETCH-CA2 | Cause del sello `a90fad3fa8fa` (o de una reproducción isomorfa) documentado en `execution.md`: DNS / timeout / TLS / fullnode / gas. Sin secretos. | `execution.md` con literal de cause, no paráfrasis. |
| DLT-FETCH-CA3 | Un evento DLT-suscrito post-fix obtiene `transaction_digest` / `merkle_anchored`. | Lote `route-domain-event` real; `SIMULATE=0`. |
| DLT-FETCH-CA4 | `/health` 200 **y** publish 2xx no se colapsan en el mismo check. Fractura nueva con `fetch failed` sin cause = regresión de CA1. | `validacion.md`. |

## 9. Criterio de cierre

- [ ] DLT-FETCH-CA1…CA4
- [ ] Argos APTO en `validacion.md` del fix (`pbi_archived: true`)
- [ ] Este TODO movido a `docs/todos/done/` en la **misma** rama del PR

Prohibido declarar Done con `SIMULATE=1` o con taxonomía «ya estaba» y sin CA3.

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta v1.0.0 — **inválida**; veredicto Tekton v1.1.0)*

### Diagnóstico de causa raíz (corregido)

- HTTP 500 en relay **vivo**: cuerpo `fetch failed` = fallo de `fetch` del SDK IOTA hacia el fullnode Testnet. `/health` OK no cubre este modo.
- Mayeuta mapeó el token `failed` a `prompt_adjustment`. Falso.

### Veredicto evolutivo

**Corrección de instancia DLT + diagnóstico lossy** (`process_fix` sobre `server.mjs` + operación Testnet). No `prompt_adjustment`.

### Propuestas

- **Instancia:** serializar `err.cause` en el 500; loguear el catch.
- **Operación:** restaurar alcanzabilidad Testnet según cause; un sello Merkle real.
- **No hacer:** reabrir taxonomía ni ELF R1; no simular IOTA; no ajustar prompt operador.

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.

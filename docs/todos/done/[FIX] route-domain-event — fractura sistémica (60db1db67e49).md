---
document_id: PBI-FIX-FRACTURE-60db1db67e49
uuid: "60db1db6-7e49-4000-8000-000000000001"
title: "[FIX] route-domain-event — fractura sistémica"
format: markdown
version: "1.2.3"
created: "2026-09-09"
updated: "2026-09-09"
status: "cerrado"
priority: alta
process: bug-fix
fracture_hash: 60db1db67e49
fracture_process: route-domain-event
friction_id: F-DLT-PUBLISH-ERROR
incident_ref: "System_Fracture_Detected — 60db1db67e49"
refined: true
suggested_branch: fix/route-domain-event-gas-version-60db1db67e49
persist_ref_suggested: docs/fixes/route-domain-event-gas-version-60db1db67e49
source_audit: "2026-09-09T09:42 CEST Filtro A v1.2.0 sobre v1.1.0. Hash SHA-256[:12](traza SIN newline)=60db1db67e49 re-verificado. Evento 22822d62-eca0-456f-87c5-7ef47294868a (05:21:19Z). Detonante PullRequest_Presented 0214542a-9938-44ad-a41a-4f61128f0d4e PR #274 (05:21:14Z). RPC Testnet: gas coin 0x93e4c1ee… tipo 0x2::coin::Coin<0x2::iota::IOTA> owner 0x31c1aa9e…. Cadena de TXs: 47ruid4… 05:21:17.180Z (114→115, digest gas 5fX2xzFe… = input de la TX rechazada); racer 8Z9h67wr… 05:21:19.182Z (115→116); ANu3L1yn… 05:21:20.982Z (116→117) = anclaje posterior con éxito, NO el racer. ELF execute-process mtime 2026-09-08 14:45 CEST < cubo DLT 41717 (commit 15:54 / merge 16:04) → Mayeuta runtime aún catch-all failed. Fuente actual: is_dlt_publish_error_trace ya evita prompt_adjustment pero diagnostica transporte. Cola reanchor vacía; evento 0214542a merkle_anchored. Dead-letter suscriptor 05:27:01Z es eco batch-anchor-failed, no segunda prueba HTTP."
review_notes: "v1.0.0 stub Cúmulo válido + Mayeuta prompt_adjustment (ELF fósil). v1.1.0 identificó colisión de versión de gas y sobre-escalado, pero atribuyó el racer a ANu3L1yn (116→117) y trató el cubo DLT como ausente. v1.2.0 Filtro A: racer = 8Z9h67wr (115→116); ANu3L1yn = éxito ~2 s después; cubo DLT 41717 existe en fuente y es demasiado genérico (transporte); kaizen Mayeuta sistémico = retirar failed del catch-all + subtipar DLT. v1.2.1: F2 nunca cableado. v1.2.2: laudo L-ENRICH-KINTSUGI-DETERMINISTA reafirmado. v1.2.3: diferido Mayeuta LLM = PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION (docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md). Veredicto process_fix."
friction_ids:
  - F-DLT-PUBLISH-ERROR
  - F-MAYEUTA-CATCHALL-FAILED
  - F-MAYEUTA-DLT-GENERIC
architectural_constraints:
  - A-FRACTURE-HASH-INMUTABLE
  - A-NO-REABRIR-TAXONOMIA-B3A715
  - A-NO-REABRIR-CAUSE-PROPAGATION-A90FAD
  - A-NO-REABRIR-PREDICADO-RED-41717
  - A-HEALTH-NO-ES-PUBLISH
  - A-NO-SIMULATE-COMO-CIERRE
  - A-NO-VOLCAR-BOVEDA
  - A-NO-BYPASS-RAW
  - A-NO-RETRY-BACKOFF-DA5
  - A-SERIALIZE-TX-RELAY
  - A-NO-ISSUES-INPUTS-COMO-TRANSIENTE-GENERICO
  - A-CATCHALL-FAILED-NO-PROMPT
  - A-ENRICH-KINTSUGI-DETERMINISTA
  - A-NO-LLM-EN-ENRICH
  - A-GENOMA-ENRICH-VIA-ENTITY-MANAGER
laudos:
  - id: L-ENRICH-KINTSUGI-DETERMINISTA
    fuente: PBI-FIX-MAYEUTA-HB-KAIZEN-CLASSIFIER
    dictado: "2026-08-30"
    reafirmado: "2026-09-09"
    texto: "Enrich Kintsugi (action:enrich-fracture-pbi-kaizen / analyze_fracture_kaizen) es handler nativo + matcher léxico. Misma traza → misma sección Kaizen, sin bóveda SDDIA_LLM_*, sin llm:interact. LLM como camino del enrich viola paridad determinista."
deferred_pbis:
  - id: PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION
    status: abierto
    process: feature
    path: "docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md"
    rol: "Único sitio para valorar skill:mayeuta-llm sobre Kintsugi (triaje asíncrono de fracturas inéditas). No despachar en esta rama. No reabre L-ENRICH-KINTSUGI-DETERMINISTA salvo laudo biológico nuevo en ese ciclo."
gates_this_wave:
  - DLT-GAS-CA1
  - DLT-GAS-CA2
  - DLT-GAS-CA3
  - DLT-GAS-CA4
  - MAYEUTA-CA5
  - MAYEUTA-CA6
  - MAYEUTA-CA7
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/tools/iota-immutable-publisher.md
  - SddIA/daemons/iota-publish-relay.md
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - SddIA/agents/mayeuta.md
  - .SddIA/services/iota-publish-relay/server.mjs
  - .SddIA/services/iota-publish-relay/relay-error.mjs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (41717b4bb229).md
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (b3a715381787).md
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (a90fad3fa8fa).md
  - docs/todos/done/[FIX] Mayeuta — clasificador Kaizen ciego a latido de centinelas.md
  - docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md
related_pbis:
  - id: PBI-FIX-FRACTURE-41717b4bb229
    rol: "Ancestro de transporte DLT y cubo Kaizen genérico. Predicado dlt_transient_network_trace (ENETUNREACH/ETIMEDOUT) intacto — no reabrir. Cubo is_dlt_publish_error_trace ya clasifica todo iota-relay-publish-error como process_fix + 'causa de transporte'. Este sello demuestra que ese diagnóstico es falso para colisión de inputs/gas. DLT-NET-CA3 ('catch-all failed intacto para no-DLT') queda superado por F-MAYEUTA-CATCHALL-FAILED."
  - id: PBI-FIX-FRACTURE-b3a715381787
    rol: "Ancestro de taxonomía DLT. Prefijo iota-relay-publish-error y F-DLT-PUBLISH-ERROR. No reabrir."
  - id: PBI-FIX-FRACTURE-a90fad3fa8fa
    rol: "Ancestro de propagación de causas (PR #245). El relay serializa el error IOTA; por eso esta traza expone Object ID/Version y no 'fetch failed'."
  - id: PBI-FIX-MAYEUTA-HB-KAIZEN-CLASSIFIER
    rol: "Fuente del laudo L-ENRICH-KINTSUGI-DETERMINISTA (2026-08-30). F2 (mayeuta-llm en enrich) no se absorbe aquí."
  - id: PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION
    rol: "FEATURE abierto: docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md. Canal LLM asíncrono ≠ Enrich Kintsugi determinista. No es gate de 60db1db67e49."
---

# [FIX] route-domain-event — fractura sistémica

> **Refinamiento v1.2.3 (Filtro A).** Stub Cúmulo inmutable. Causa física = colisión de versión de gas coin (racer `8Z9h67wr…`, no `ANu3L1yn…`). Kaizen de esta ola = matcher léxico (catch-all `failed` + subtipo DLT). **Laudo `L-ENRICH-KINTSUGI-DETERMINISTA` reafirmado.** Uso de `mayeuta-llm` en Kintsugi = `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION` (`docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md`). No esta rama.

---

## 1. Identidad del sello (inmutable)

| Campo | Valor | Notas |
|-------|-------|-------|
| `fracture_hash` | `60db1db67e49` | `SHA-256[:12]` de la traza **sin** newline. Re-verificado 2026-09-09. Inmutable. |
| `fracture_process` | `route-domain-event` | Pre-sellado Merkle de lote (`merkle-batch-preseal`). |
| `friction_id` | `F-DLT-PUBLISH-ERROR` | Taxonomía `b3a715381787`: HTTP 500 del **relay** con proceso vivo. |
| Emisor | `execute-process` | Orquestador Core. |
| Acción intentada | `merkle-batch-preseal` | Pre-anclaje DLT de eventos de dominio. |
| Cápsula | `iota-immutable-publisher` → `publish_via_relay` | Prefijo de traza emitido en la cápsula. |
| Endpoint | `POST http://127.0.0.1:8787/v1/publish` | Relay HTTP de instancia. El HTTP 500 es del relay, no del fullnode. |

### Traza de error literal (inmutable)

```
merkle-batch-preseal failed: iota-relay-publish-error: status=500 Transaction execution failed due to issues with transaction inputs, please review the errors and try again:
- Object ID 0x93e4c1ee3a81aa2815b2f23485885a37f6c97eb20d7e58458d8dcc4dce6ff880 Version 850727115 Digest 5fX2xzFeULF5XhHLu3iLNotiKiR5bu8CVwJC9j5X6GBa is not available for consumption, current version: 850727116
```

> **Mandato:** Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado. Prohibido alterar `fracture_hash`, `fracture_process` o la traza de error literal.

---

## 2. Hechos verificados (Filtro A v1.2.0, 2026-09-09)

Zona: CEST = UTC+2. Cadena IOTA re-consultada en Testnet RPC (`api.testnet.iota.cafe`).

| Hecho | Evidencia |
|-------|-----------|
| **Integridad del hash** | `printf '%s' '<traza_literal>' \| sha256sum` → `60db1db67e49c7332d6b7e7b2ee351f4c4b78e9c6c5a09fedc691ac6c7ac024d`. Prefijo 12 = sello. |
| **Naturaleza del objeto** | `iota_getObject`: `0x93e4c1ee…` es `0x2::coin::Coin<0x2::iota::IOTA>`, owner `0x31c1aa9e…`. Es la moneda de gas del relay. |
| **Evento de fractura** | `22822d62-eca0-456f-87c5-7ef47294868a` (`System_Fracture_Detected`, `2026-09-09T05:21:19Z`). Payload `process_name: route-domain-event`, `friction_id: F-DLT-PUBLISH-ERROR`. Cúmulo materializó PBI `05:21:23.023Z`; Mayeuta enrich `05:21:23.071Z` exit 0. |
| **Detonante de bus** | `PullRequest_Presented` `0214542a-9938-44ad-a41a-4f61128f0d4e` (`05:21:14Z`, PR #274, rama `docs/aiua-audit-live-20260909`). El timestamp es emisión del evento, no del anclaje. |
| **Cadena on-chain (misma gas coin)** | `47ruid4tith29Wr8oULJkeGWDMeW1YaheRH22dqGe76s` `05:21:17.180Z` (114→**115**, digest gas `5fX2xzFe…` = input de la TX rechazada). **Racer:** `8Z9h67wrxsdGMKsBND6rcmPx2LK7r4rMRQsken4XnQHq` `05:21:19.182Z` (115→**116**, creó objeto ancla `0x2c48fe83…`). **Posterior:** `ANu3L1ynKQ54k1mM8isYVpdkXTMKNj3wfSBnknoCpFfv` `05:21:20.982Z` (116→**117**, creó `0x82610a0b…`) = anclaje con éxito del evento 0214542a, **no** el racer. Tres publishes OK en ~4 s + una TX rechazada que aún apuntaba a 115. |
| **Dead-letter suscriptor** | `.events/dead-letter/subscribers/0214542a-….cumulo.iota-immutable-publisher.json` a las `05:27:01Z` con prefijo `batch-anchor-failed:`. Es eco de `last_batch_anchor_error` del lote (~6 min después), **no** prueba independiente de un segundo `POST /v1/publish` en el instante de la fractura. |
| **Concurrencia en relay (código)** | `server.mjs`: `http.createServer` async sin cola/mutex. Cada `POST /v1/publish` llama `publishImmutableData` → `signAndExecuteTransaction` sobre la misma wallet. Permite la ráfaga observada. No prueba por sí solo *cuál* par de requests chocó. |
| **Sobre-escalado Core** | `dlt_transient_network_trace` solo tokens de socket (`enetunreach`, `etimedout`, `enotfound`, `network is unreachable`, `connection timed out`). Esta causa no match → `emit_dlt_batch_fracture` escribe pending. `stamp_batch_anchor_error` + `enqueue_dlt_reanchor` ocurren **siempre** (dual path, precedente 41717). |
| **Cola y anclaje posterior** | `.SddIA/dlt/reanchor-queue/` vacía ahora. Evento 0214542a tiene `merkle_anchored: true` + digest `ANu3L1yn…`. El lote **sí** se ancló ~2 s después. Kintsugi fue ruido: la cola/reintento absorbió. |
| **ELF Mayeuta** | `SddIA/target/debug/execute-process` mtime `2026-09-08 14:45:44 CEST`. Cubo `is_dlt_publish_error_trace` entra en fuente a las `15:54` / merge `1b39497` `16:04`. Enrich de este sello (09-09 05:21Z) corrió **sin** ese cubo → catch-all `failed` → `prompt_adjustment`. |
| **Fuente actual del clasificador** | Con ELF fresco, `iota-relay-publish-error` ya sale `process_fix` y **no** `prompt_adjustment`. El texto del cubo sigue siendo «Causa de transporte (`err.cause`) hacia fullnode Testnet». Incorrecto para esta traza. |

Copia residual del mismo `event_id` 22822d62 en `.events/pending/a33e704e2440.json` (hash de contenido). Fuera de alcance (idempotencia de fan-out; PBI kaizen distinto).

---

## 3. Qué hizo mal Mayeuta

Dos capas distintas. v1.1.0 las colapsó en una sola.

### 3.1 Runtime que escribió v1.0.0 (ELF fósil)

`analyze_fracture_kaizen` del binario anterior a 41717: `has_any(&["timeout", "block", "abort", "failed", "colaps"])`. La traza contiene `merkle-batch-preseal failed` y `Transaction execution failed` → cubo operador:

- Causa: «Bloqueo operativo sin escalado Kintsugi previo al intento de recuperación manual.» Falso: no hubo operador ni recuperación manual; el Core ya había emitido `System_Fracture_Detected`.
- Veredicto: `prompt_adjustment`. Falso: pre-sellado DLT autónomo.

### 3.2 Fuente vigente (post-41717) — seguiría fallando el diagnóstico

`is_dlt_publish_error_trace` (token `iota-relay-publish-error`) inhibe el catch-all. Veredicto `process_fix` (correcto). Diagnóstico hardcoded: transporte hacia Testnet (incorrecto). DLT-NET-CA3 de 41717 solo exigía «no prompt_adjustment», no discriminación de subtipo.

### 3.3 Defecto sistémico (por qué «la mayoría de fixes» nacen mal)

El catch-all usa `failed`, verbo de casi toda traza de cápsula (`{acción} failed:`). Cada ola añade un cubo (hook, DNS, heartbeat, orphan, DLT, shell, PAT…). El siguiente sello con `failed` y sin cubo específico vuelve a `prompt_adjustment`. Parcheo aditivo infinito. **Kaizen de esta ola:** dejar de fabricar operador a partir de `failed`, **dentro del matcher nativo**.

**Laudo `L-ENRICH-KINTSUGI-DETERMINISTA` (reafirmado 2026-09-09).** Dictado en `PBI-FIX-MAYEUTA-HB-KAIZEN-CLASSIFIER` (2026-08-30): «LLM como camino principal viola paridad determinista del handler nativo». Este PBI **no lo reabre**. Enrich Kintsugi = `analyze_fracture_kaizen` puro: misma `error_trace` → misma sección, cero `llm:interact`, cero bóveda `SDDIA_LLM_*`. El uso de `skill:mayeuta-llm` sobre el canal Kintsugi **no es residual vago**: queda nominado como `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION` (`docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md`). Hasta un laudo biológico en ese ciclo, Tekton no invoca LLM desde enrich.

---

## 4. Discriminación de hipótesis

| # | Hipótesis | Evaluación | Veredicto |
|---|-----------|------------|-----------|
| **H1** | Bóveda ausente / secret corrupto | Un `config-missing: IOTA_WALLET_SECRET` aborta antes del execute. La traza es rechazo de inputs con Object ID de gas. | **Refutada** |
| **H2** | Fullnode caído / error de red | No hay `ENETUNREACH`/`ETIMEDOUT`. El relay contestó HTTP 500 con payload de consenso Move (inputs). El fullnode ejecutó la validación. | **Refutada** |
| **H3** | Bytecode Move / paquete inválido | El validador rechazó **consumo de objeto de gas**, no la ejecución de `publish_immutable`. Tres TX hermanas del mismo segundo crearon objetos ancla con éxito. | **Refutada** |
| **H4** | Operador IA / prompt | Emisor nativo `execute-process`. Sin LLM en el pre-sellado. | **Refutada (alucinación v1.0.0)** |
| **H5** | Colisión de versión de gas coin | TX rechazada fijó gas v115 digest `5fX2xzFe…` (creada por `47ruid4…` a las 05:21:17.180Z). A las 05:21:19.182Z `8Z9h67wr…` consumió esa versión. Current version reportada: 116. | **CONFIRMADA** |
| **H6** | Ausencia de serialización en `server.mjs` | Código sin mutex. Tres `publish_immutable` OK en 4 s sobre la misma coin. Causa estructural que **habilita** H5. El par HTTP exacto no está en logs del relay. | **CONFIRMADA como defecto; ráfaga on-chain como evidencia de clase** |
| **H7** | Sobre-escalado en `route_domain_core.rs` | Predicado transitorio ciego a `is not available for consumption`. Fractura emitida; el mismo evento quedó `merkle_anchored` 2 s después. | **CONFIRMADA** |
| **H8** | `ensureAnchorPackageId` sin cache (dos TX en un request) | Plausible si `IOTA_ANCHOR_PACKAGE_ID` vacío. No verificado (bóveda no se vuelca). No sustituye H6: las tres TX on-chain son publishes de datos (objetos ancla created). | **No necesaria; no bloquea el fix** |
| **H9** | `ANu3L1yn…` fue el racer (tesis v1.1.0) | RPC: `previousVersion` 116→117 a las 05:21:20.982Z. La TX rechazada vio current **116**, no 117. El racer es `8Z9h67wr…`. | **Refutada** |

---

## 5. Causa estructural

### 5.1 Modelo de objetos IOTA (gas coin owned)

Cada objeto es `(ObjectID, version, digest)`. El SDK fija la versión al construir la TX. Si otra TX confirma antes, el validador responde `is not available for consumption, current version: V+1`. No es corrupción; es exclusión optimista del protocolo. Tres confirms seguidas sobre `0x93e4c1ee…` lo demuestran.

### 5.2 Relay de instancia sin serialización (`server.mjs`)

`POST /v1/publish` ejecuta `publishImmutableData` en paralelo. Una sola wallet / una sola gas coin. Fan-out EDA (batch Merkle + suscriptor `cumulo.iota-immutable-publisher` + otros eventos del mismo tick) produce ráfagas. **Palanca primaria.**

Mutex + `waitForTransaction` de la TX precedente **antes** de construir la siguiente es necesario, no suficiente: un RPC rancio puede devolver v115 tras un confirm. Por eso existe 5.3.

### 5.3 Sobre-escalado Kintsugi (`route_domain_core.rs`)

Precedente 41717: dual path stamp+cola **y** fractura; se suprime la fractura solo si la causa es red transitoria. Colisión de versión es tan transitoria como un `ETIMEDOUT` **si** el siguiente intento re-selecciona gas. Homólogo: ampliar predicado con firma **estrecha**, no con el wrapper `issues with transaction inputs` (cubre inputs permanentes: tipo erróneo, objeto inexistente, gas insuficiente).

Opaco (`config-missing`, Move bytecode, 500 sin firma de consumo) **sigue** emitiendo fractura. No reabrir el predicado de red de 41717.

### 5.4 Clasificador Kaizen (`enrich_fracture_pbi_kaizen.rs`)

1. Catch-all `failed` → `prompt_adjustment` (este sello en runtime; familia route-domain-event: a90fad, 41717, 701c77).
2. Cubo DLT genérico → transporte para todo `iota-relay-publish-error` (fuente vigente).
3. `verdict_priority` prefiere `prompt_adjustment` sobre `process_fix` si ambos disparan. Cualquier cubo nuevo debe **inhibir** el catch-all (ya lo hace `!dlt_publish`; no basta si el catch-all sigue existiendo para el resto).
4. El clasificador **sigue** siendo función pura (laudo `L-ENRICH-KINTSUGI-DETERMINISTA`). Corregir cubos ≠ introducir inferencia.

---

## 6. Alcance (`bug-fix`)

Motor: `server.mjs` (instancia), `route_domain_core.rs`, `enrich_fracture_pbi_kaizen.rs`. Diff documental de `SddIA/actions/enrich-fracture-pbi-kaizen.md` **solo** vía `entity-manager` (DA-2). No genoma `tools/` / `process/` / `agents/` salvo ese bump de acción.

### Dentro

1. **Serialización en relay (`server.mjs`):** cola/mutex async: un `publishImmutableData` a la vez; la TX previa debe haber completado `waitForTransaction` antes de construir la siguiente. Palanca primaria contra H5/H6.
2. **Predicado transitorio DLT (`route_domain_core.rs`):** extender (o extraer `dlt_transient_error_trace`) con firma conjunta case-insensitive:
   - `is not available for consumption`
   - `current version:`
   Match → stamp + `enqueue_dlt_reanchor`; **no** `emit_dlt_batch_fracture`. Prohibido usar solo `issues with transaction inputs`. Tokens de red de 41717 intactos. `config-missing` y 500 opaco siguen emitiendo.
3. **Kaizen Mayeuta (sistémico, no un cubo más):**
   - **F-MAYEUTA-CATCHALL-FAILED:** retirar `failed` del catch-all. Trazas sin cubo específico → fallback ya existente (`process_fix` + «requiere laudo humano»), **nunca** `prompt_adjustment` por verbo de fallo. `prompt_adjustment` queda en el enum para cubos explícitos de operador; no lo fabrica el catch-all.
   - **F-MAYEUTA-DLT-GENERIC:** subtipar el cubo DLT. `ENETUNREACH`/`ETIMEDOUT`/`ENOTFOUND` → transporte (no-reg 41717). Firma de consumo/versión → colisión de gas/inputs. Resto de `iota-relay-publish-error` → `process_fix` **sin** afirmar transporte.
   - Test de esta traza literal (CA3) + test de no-reg ENETUNREACH (CA6) + test de `… failed:` genérico sin cubo (CA5).
   - Bump de `enrich-fracture-pbi-kaizen.md` vía `entity-manager`.
4. **E2E instancia (`SIMULATE=0`):** cápsula `iota-immutable-publisher` con relay vivo → `transaction_digest` real. No `curl`. No lab-simulate.

### Fuera

- Reabrir taxonomía `b3a715381787`, cause-propagation `a90fad3fa8fa`, predicado de **red** `41717`.
- Invocar `skill:mayeuta-llm` / `llm:interact` desde enrich. Residual **F2** del PBI heartbeat: **nunca estuvo cableado**. Queda **diferido** a `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION` (`docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md`). No es gate de esta rama. Ver laudo + recuadro bajo esta lista.
- Mutar genoma `actions/` a mano (DA-2).
- Alterar firmas de otros procesos en `route_domain_core.rs`.
- Retry síncrono / `sleep` / backoff en Core (DA-5).
- `SDDIA_LAB_SIMULATE_IOTA=1` como Done.
- Volcar `IOTA_WALLET_SECRET`.
- Bypass raw.
- Tratar el pending residual `a33e704e2440.json` como alcance de este fix.
- Inventar veredicto `infrastructure_resilience`.

**Laudo `L-ENRICH-KINTSUGI-DETERMINISTA` (reafirmado).** Enrich Kintsugi (`action:enrich-fracture-pbi-kaizen` → `analyze_fracture_kaizen`) es handler nativo + matcher léxico. Misma traza → misma sección Kaizen. Sin bóveda `SDDIA_LLM_*`. Sin `llm:interact`. Los tests CA3/CA5/CA6 congelan literales; un LLM los haría no reproducibles. Fuente: PBI heartbeat 2026-08-30 («LLM como camino principal viola paridad determinista del handler nativo»). Este hash **reafirma** el laudo; no lo sustituye.

`mayeuta-llm` permanece el provider de `llm:interact` para Kalma2, triaje correo y `SYNTHESIZE`/`CLASSIFY_INTENT`. Ese canal **no** es el enrich. Meterlo en `60db1db67e49` convertiría el `bug-fix` en `feature` (DI en el suscriptor de fractura, prompt+schema, timeout/fail-open, texto distinto por host, Mayeuta diseñando arquitectura por `SYNTHESIZE`). **No diagnostica este sello mejor** que subtipar el cubo DLT.

**Diferido — `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION`.** Único vehículo para *valorar* LLM sobre Kintsugi. Path: `docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md`. No es criterio de cierre de este fix. Ese ciclo `feature` exige laudo biológico **nuevo**; hasta entonces el enrich sigue determinista. Esta ola **sí** corta `prompt_adjustment` por `failed` y el falso «transporte» DLT. **No** hará que un error inédito nazca con causa raíz correcta: sin cubo → fallback «requiere laudo humano».

---

## 7. Afirmaciones descartadas e inexactitudes

### Stub v1.0.0 (Mayeuta / ELF fósil)

| Afirmación | Corrección |
|------------|------------|
| `prompt_adjustment` / «Ajustar instrucción operador IA» | Alucinación del catch-all `failed`. Veredicto: `process_fix`. |
| «Bloqueo operativo sin escalado Kintsugi previo a recuperación manual» | El Core ya emitió fractura y encoló reanchor. Cero intervención manual. |

### Refinamiento v1.1.0 (Filtro A previo)

| Afirmación v1.1.0 | Corrección v1.2.0 |
|-------------------|-------------------|
| «`ANu3L1yn…` tomó gas 116 y mutó a 117; esa TX provocó la colisión» | RPC: 116→117 a las 05:21:20.982Z. La rechazada vio current **116**. Racer = `8Z9h67wr…` (115→116, 05:21:19.182Z). `ANu3L1yn…` es el anclaje **posterior** con éxito (coincidente con `merkle_anchored` del 0214542a). |
| «Mayeuta emitió `prompt_adjustment` porque el cubo DLT no existe» | El cubo **existe en fuente** desde 41717 (15:54 CEST 08-09). El runtime del enrich era ELF de 14:45. Causa del v1.0.0 = binario rancio, no ausencia de cubo en el árbol. |
| «Fullnode respondió HTTP 500» | HTTP 500 es del relay local (`formatPublishFailure`). El fullnode rechazó inputs de la TX. |
| «`issues with transaction inputs` es transitorio» | Es wrapper genérico. Firma transitoria = `is not available for consumption` + `current version:`. |
| «Dead-letter del suscriptor prueba dos POST concurrentes» | Prefijo `batch-anchor-failed:` a las 05:27:01Z = eco del stamp del lote, 6 min tarde. La ráfaga se prueba on-chain (tres confirms), no con ese JSON. |
| Cubo Kaizen = solo «excluir catch-all para esta traza» | Insuficiente. Fuente vigente ya excluye catch-all DLT y **miente** con transporte. Hace falta subtipo + retirada sistémica de `failed`. |
| «Optimistic Locking Conflict» como nombre de error IOTA | Analogía. El literal del protocolo es `is not available for consumption`. |
| «Resolver Mayeuta con `mayeuta-llm` en esta ola» | **Fuera de jurisdicción.** Laudo `L-ENRICH-KINTSUGI-DETERMINISTA`. Vehículo = `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION` (`docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md`). |

---

## 8. Criterios de aceptación (Gates)

| ID | Criterio | Verificación |
|----|----------|--------------|
| **DLT-GAS-CA1** | `server.mjs` serializa `publishImmutableData`. Dos `POST /v1/publish` solapados no construyen TX con la misma versión de gas coin. | Test de cola/mutex en el relay (mock del cliente IOTA). `relay-error.test.mjs` actual **no** cubre esto (solo `formatPublishFailure`). |
| **DLT-GAS-CA2** | Traza con `is not available for consumption` **y** `current version:` **no** materializa `System_Fracture_Detected`. Stamp + `dlt_reanchor` sí. Wrapper `issues with transaction inputs` **sin** esa firma **sí** emite. `config-missing` sigue emitiendo. Predicado de red 41717 intacto. | Tests `route_domain_core` (`dlt_transient` / `emit_dlt_batch_fracture`). |
| **DLT-GAS-CA3** | Traza literal de este sello en `analyze_fracture_kaizen` → `process_fix`; sección menciona colisión de gas/inputs; **sin** «transporte» / `err.cause` / «Ajustar instrucción operador» / `prompt_adjustment`. | `cargo test --manifest-path SddIA/Cargo.toml -p execute-process --lib -- analyze_fracture_kaizen`. |
| **DLT-GAS-CA4** | Publish `SIMULATE=0` vía cápsula con relay vivo → `transaction_digest` en Testnet. | `execution.md` + `validacion.md`. Cápsula, no curl. |
| **MAYEUTA-CA5** | Catch-all: `failed` **no** produce `prompt_adjustment`. Traza `{acción} failed:` sin cubo de dominio → fallback laudo humano / `process_fix`, no operador. | Test nuevo en `enrich_fracture_pbi_kaizen.rs`. |
| **MAYEUTA-CA6** | No-reg 41717: traza `iota-relay-publish-error` + `cause: ENETUNREACH` → `process_fix`, menciona transporte/red, sin operador. | Test existente `analyze_fracture_kaizen_dlt_publish_error_not_prompt` (ajustar aserciones de subtipo si el texto cambia). |
| **MAYEUTA-CA7** | Enrich Kintsugi determinista: `enrich_fracture_pbi_kaizen.rs` no invoca `mayeuta-llm` ni `llm:interact`. CA3/CA5/CA6 son función pura (misma traza → mismos literales). | Grep del módulo + suite `analyze_fracture_kaizen`. |

---

## 9. Criterio de cierre

- [x] `DLT-GAS-CA1` (mutex relay).
- [x] `DLT-GAS-CA2` (predicado estrecho; opaco intacto).
- [x] `DLT-GAS-CA3` + `MAYEUTA-CA5` + `MAYEUTA-CA6` + `MAYEUTA-CA7` verdes en `execute-process`.
- [x] `DLT-GAS-CA4` en runtime real (`SIMULATE=0`) — digest `5rkFWghseVYgDh5DTQsECyeRS9T1d99hkkBXa7ELoEja`.
- [x] `enrich-fracture-pbi-kaizen.md` bump via `entity-manager` (no Write directo).
- [ ] Argos `global: APTO` + CA-CI `run_id` verde post-PR (`validacion.md` `pbi_archived: true`).
- [x] Este PBI en `docs/todos/done/` en la rama del PR.

---

## 10. Conclusión analítica y propuesta evolutiva

*(Síntesis corregida — Kintsugi async, v1.2.2)*

### Diagnóstico de causa raíz

- **Colisión de versión de gas coin** en IOTA Testnet: la TX de pre-sellado fijó `0x93e4c1ee…` v115 digest `5fX2xzFe…`; el racer `8Z9h67wr…` confirmó 115→116 en el mismo segundo (`05:21:19.182Z`).
- **Relay sin serialización** (`server.mjs`): habilita ráfagas sobre una sola moneda de gas (tres confirms 05:21:17.180 / 19.182 / 20.982).
- **Sobre-escalado Kintsugi:** `dlt_transient_network_trace` no cubre la firma de consumo; se emitió fractura aunque el evento quedó anclado 2 s después (`ANu3L1yn…`).
- **Mayeuta:** runtime ELF fósil → `prompt_adjustment`; fuente vigente → `process_fix` con diagnóstico de transporte falso. Catch-all `failed` es la fábrica de análisis incorrectos al abrir fixes.

### Veredicto evolutivo

**`process_fix`.** Prohibido `prompt_adjustment`. Prohibido `infrastructure_resilience`.

### Propuestas

1. **Relay:** mutex/cola en `server.mjs` alrededor de `publishImmutableData` + `waitForTransaction`.
2. **Aduana Core:** predicado transitorio estrecho (`is not available for consumption` ∧ `current version:`) → cola `dlt_reanchor` sin `System_Fracture_Detected`.
3. **Mayeuta (esta ola, matcher nativo):** retirar `failed` del catch-all; subtipar cubo DLT (red vs gas/inputs vs opaco); fallback = laudo humano, no operador. Laudo `L-ENRICH-KINTSUGI-DETERMINISTA` intacto.
4. **Mayeuta (futuro):** `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION` — `docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md`. No es propuesta de ejecución de `60db1db67e49`.

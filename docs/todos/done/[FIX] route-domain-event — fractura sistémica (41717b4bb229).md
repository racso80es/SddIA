---
document_id: PBI-FIX-FRACTURE-41717b4bb229
uuid: "41717b4b-b229-4000-8000-000000000001"
title: "[FIX] route-domain-event — fractura sistémica"
format: markdown
version: "1.2.0"
created: "2026-09-08"
updated: "2026-09-08"
status: "cerrado"
priority: alta
process: bug-fix
fracture_hash: 41717b4bb229
fracture_process: route-domain-event
friction_id: F-DLT-PUBLISH-ERROR
incident_ref: "System_Fracture_Detected — 41717b4bb229"
refined: true
suggested_branch: fix/route-domain-event-enetunreach-41717b4bb229
persist_ref_suggested: docs/fixes/route-domain-event-enetunreach-41717b4bb229
source_audit: "2026-09-08T13:33Z Filtro A sobre v1.1.0: hash SHA-256[:12](traza SIN newline)=41717b4bb229 verificado; PR cause-propagation = #245 no #259; PBI versionado d44d7b3 12:30:13Z, instancia 863d1511 failed_at 12:50:01Z (isomorfa posterior); cola dlt_reanchor vacía a las 15:24 CEST; 863d1511 sin merkle_anchored; veredicto Kaizen válido solo process_fix (infrastructure_resilience no existe)."
review_notes: "v1.0.0 stub Cúmulo válido + Mayeuta prompt_adjustment (alucinación catch-all 'failed'). v1.1.0 refinamiento con causa ENETUNREACH correcta y tres alucinaciones (PR #259, detonante único 863d1511, veredicto infrastructure_resilience). v1.2.0 Filtro A: causa física intacta; homólogo = predicado F4c (d0cfd5b66ff1 + tokens gh 49ce2db7152d), no 'DCC ya clasificó este incidente'."
friction_ids:
  - F-DLT-PUBLISH-ERROR
architectural_constraints:
  - A-FRACTURE-HASH-INMUTABLE
  - A-NO-REABRIR-TAXONOMIA-B3A715
  - A-NO-REABRIR-CAUSE-PROPAGATION-A90FAD
  - A-HEALTH-NO-ES-PUBLISH
  - A-NO-SIMULATE-COMO-CIERRE
  - A-NO-VOLCAR-BOVEDA
  - A-NO-BYPASS-RAW
  - A-NO-RETRY-BACKOFF-DA5
gates_this_wave:
  - DLT-NET-CA1
  - DLT-NET-CA2
  - DLT-NET-CA3
  - DLT-NET-CA4
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/tools/iota-immutable-publisher.md
  - SddIA/daemons/iota-publish-relay.md
  - .SddIA/services/iota-publish-relay/relay-error.mjs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (a90fad3fa8fa).md
  - docs/fixes/iota-dlt-publish-fetch-failed/spec.md
  - docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (49ce2db7152d).md
  - docs/fixes/dcc-gh-api-connect-49ce2db7152d/spec.md
  - docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (d0cfd5b66ff1).md
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (b3a715381787).md
related_pbis:
  - id: PBI-FIX-FRACTURE-a90fad3fa8fa
    rol: "Ancestro de diagnóstico. Traza lossy ('status=500 fetch failed'). Fix: serializar err.cause (PR #245, persist_ref iota-dlt-publish-fetch-failed). Cubo Mayeuta DLT quedó deuda ortogonal (fuera de execution_file_lock). Este sello demuestra que cause viaja: '| cause: ENETUNREACH'."
  - id: PBI-FIX-FRACTURE-d0cfd5b66ff1
    rol: "Patrón homólogo F4c: DNS/red transitoria ('network is unreachable', timeout) → blocked F-DCC-DNS-UNRESOLVED sin System_Fracture_Detected. DLT carece de este predicado."
  - id: PBI-FIX-FRACTURE-49ce2db7152d
    rol: "Hermano de host, no el mismo ciclo DCC. Fractura DCC al abrir PR de feat/nucleo-aiua-tormentosa-motor (token gh ausente en F4c). Spec DCC declara 41717 fuera de alcance. PR #272 = fix F4c de ese hermano; DCC de #272 SÍ abrió el PR (PullRequest_Presented 863d1511)."
  - id: PBI-FIX-FRACTURE-b3a715381787
    rol: "Ancestro de taxonomía. Prefijo iota-relay-publish-error + F-DLT-PUBLISH-ERROR. No reabrir."
  - id: PBI-FIX-FRACTURE-701c77ebeab8-R1
    rol: "Ancestro de runtime ELF/supervisor. No reabrir."
---

# [FIX] route-domain-event — fractura sistémica

> **Refinamiento v1.2.0 (Filtro A).** Stub Cúmulo (sello + traza) válido e inmutable. Síntesis Mayeuta v1.0.0 (`prompt_adjustment`) inválida. Causa física = `ENETUNREACH` hacia fullnode Testnet con relay local vivo. Afirmaciones v1.0.0 y errores v1.1.0 en §7.

---

## 1. Identidad del sello (inmutable)

| Campo | Valor | Notas |
|-------|-------|-------|
| `fracture_hash` | `41717b4bb229` | `SHA-256[:12]` de la traza **sin** newline. Verificado 2026-09-08. Inmutable. |
| `fracture_process` | `route-domain-event` | Pre-sellado Merkle (`merkle-batch-preseal`). |
| `friction_id` | `F-DLT-PUBLISH-ERROR` | Taxonomía `b3a715381787`: HTTP 500 con relay vivo. |
| Emisor | `execute-process` | Correcto. |
| Acción intentada | `merkle-batch-preseal` | Correcto. |
| Cápsula | `iota-immutable-publisher` → `publish_via_relay` | Prefijo de traza emitido ahí. |
| Endpoint | `POST http://127.0.0.1:8787/v1/publish` | Relay de instancia. |

### Traza de error literal (inmutable)

```
merkle-batch-preseal failed: iota-relay-publish-error: status=500 fetch failed | cause: ENETUNREACH
```

> **Mandato:** Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado. Prohibido alterar `fracture_hash`, `fracture_process` o la traza.

---

## 2. Hechos verificados (Filtro A, 2026-09-08)

Zona: CEST = UTC+2.

| Hecho | Evidencia |
|-------|-----------|
| **Hash** | `printf '%s' '<traza>' \| sha256sum` → `41717b4bb229`. Con newline el digest cambia. |
| **Taxonomía** | Prefijo `iota-relay-publish-error` = CA de `b3a715381787` operando. No es `iota-relay-unreachable`. |
| **Cause-propagation** | `relay-error.mjs` `causeSuffix` serializa `code`/`syscall`/`message`. Traza `\| cause: ENETUNREACH` = `err.cause.code`. Entregado en **PR #245** (`a90fad3fa8fa`), no #259. |
| **Relay ≠ publish** | `/health` no toca IOTA (`A-HEALTH-NO-ES-PUBLISH`). Health 200 no implica publish 2xx. |
| **Bóveda** | `IOTA_WALLET_SECRET` e `IOTA_ANCHOR_PACKAGE_ID` presentes en `.SddIA/.dev/.env`. `SDDIA_LAB_SIMULATE_IOTA=0`. `SDDIA_LAB_MOCK_IOTA_URL` vacío. Valores no volcados. |
| **Cola re-anclaje (15:24 CEST)** | `.SddIA/dlt/reanchor-queue/` **vacía**. El lote no espera drenaje automático ahora. |
| **Instancia 863d1511** | `PullRequest_Presented` PR #272 / `fix/dcc-gh-api-connect-49ce2db7152d`, `timestamp` `2026-09-08T12:49:58Z`, subscriber DLQ `failed_at` `12:50:01.636Z`. `delivery_state.last_batch_anchor_error` coincide con la traza. **Sin** `merkle_anchored` / `transaction_digest`. Padre en `processed/` y `dead-letter/`. |
| **863d1511 no es el sello original** | PBI versionado en `d44d7b3` (`2026-09-08 14:30:13 +0200` = `12:30:13Z`), **19 min antes** de 863d1511. El hash identifica la **traza**, no un UUID. Hubo al menos un fallo isomorfo anterior; 863d1511 es reincidiva al presentar PR #272. |
| **49ce2db7152d** | Traza DCC: `error connecting to api.github.com` al resolver PR de `feat/nucleo-aiua-tormentosa-motor`. Versionado en el **mismo** commit `d44d7b3`. Hermano de host (corte de red). **No** es el DCC que abrió PR #272: ese DCC **tuvo éxito** (evento Presented existe). |
| **F4c DCC** | `d0cfd5b66ff1` ya clasifica `network is unreachable` / timeout. `49ce2db7152d` (PR #272) solo añadió tokens del CLI `gh`. DLT no tiene predicado equivalente: `emit_dlt_batch_fracture` corre tras **cualquier** error de cápsula. |
| **Mayeuta** | `analyze_fracture_kaizen`: catch-all `has_any(["timeout","block","abort","failed","colaps"])` → `prompt_adjustment`. La traza contiene `merkle-batch-preseal failed` y `fetch failed`. Veredictos existentes: `new_norm` \| `refactor_tool` \| `prompt_adjustment` \| `process_fix`. **No existe** `infrastructure_resilience`. |
| **Dual path** | `stamp_batch_anchor_error` **encola** `dlt_reanchor` **y** `emit_dlt_batch_fracture` escribe `System_Fracture_Detected`. No es ausencia de cola; es sobre-escalado. |
| **Deuda a90fad** | Cubo Mayeuta DLT explícitamente fuera de aquella ola. |

Auditoría de PIDs systemd / checkpoint Testnet en v1.1.0 = **snapshot efímero**; no es identidad del sello. Re-verificar en `execution.md` si se usa como evidencia de cierre.

---

## 3. Qué hizo mal Mayeuta

`analyze_fracture_kaizen` (catch-all `failed`, hoy ~L282–298 en `enrich_fracture_pbi_kaizen.rs`):

1. Token `failed` en la traza → «Bloqueo operativo sin escalado Kintsugi» + `prompt_adjustment`.
2. Falso: `route_domain_core.rs` ya emitió `System_Fracture_Detected`; Cúmulo materializó este PBI; no había entrega Tekton que castrar.
3. No hay cubo para `iota-relay-publish-error` / `F-DLT-PUBLISH-ERROR`. Toda traza `merkle-batch-preseal failed: …` cae en el catch-all.

Citar número de línea en tests/spec es frágil; el contrato es el token `failed` en el blob concatenado.

---

## 4. Discriminación de hipótesis

| # | Hipótesis | Evaluación | Veredicto |
|---|-----------|------------|-----------|
| **H1** | Bóveda ausente | **Refutada** | Presente. Faltante → `config-missing: IOTA_WALLET_SECRET` antes de red. |
| **H2** | Fallo Move / package | **Refutada** | Transporte HTTP falló antes del RPC. |
| **H3** | Relay caído / ELF fósil | **Refutada como causa del sello** | Prefijo `publish-error`, no `unreachable`. Health ≠ publish. |
| **H4** | Operador / `prompt_adjustment` | **Refutada (alucinación)** | Emisión nativa Core. |
| **H5** | `ENETUNREACH` host → fullnode Testnet | **CONFIRMADA** | `err.cause.code` en el 500. POSIX `ENETUNREACH` (errno 101 en Linux). |
| **H6** | Cola `dlt_reanchor` ausente | **Refutada** | `enqueue_dlt_reanchor` existe; se llama en el mismo fallo. Hoy la cola está vacía (no hay drenaje pendiente observable). |

---

## 5. Causa estructural

### 5.1 Transporte host → Testnet (síntoma del sello)

`fetch failed \| cause: ENETUNREACH`: sin ruta hacia `api.testnet.iota.cafe` en ese instante. Ambiental. El Core no «arregla» la red del host.

### 5.2 Sobre-escalado Kintsugi (defecto de código)

Homólogo DCC: `dcc_transient_network_trace` + `dcc_net_block_suppresses_fracture` → `blocked` + `F-DCC-DNS-UNRESOLVED`, **sin** `System_Fracture_Detected`. Tokens F4c incluyen `network is unreachable`.

DLT: `preseal_merkle_batch` / `try_drain_dlt_reanchor_queue` ante error de cápsula: stamp + enqueue **y** `emit_dlt_batch_fracture`. Un corte efímero materializa PBI Kintsugi aunque la cola pueda reintentar en el siguiente tick con relay sano.

**No** es «en lugar de» usar la cola. Es dual path. El fix homólogo: **suprimir** la fractura cuando `err.cause` es red transitoria; **conservar** stamp + cola. Prohibido retry/backoff/sleep (DA-5). Prohibido `fail_soft` que finja anclaje.

Publish 500 **opaco** (sin cause de red; Move; config-missing) **sigue** emitiendo fractura.

### 5.3 Brecha Mayeuta

Sin cubo DLT. Catch-all `failed` → `prompt_adjustment`. Si se añade `process_fix` **sin** excluir el catch-all, `verdict_priority` prefiere `prompt_adjustment`. La guarda del catch-all debe negar el cubo DLT.

---

## 6. Alcance (`bug-fix`)

Motor: `route_domain_core.rs` + `enrich_fracture_pbi_kaizen.rs`. **No** genoma `SddIA/tools/` / `SddIA/process/`. **No** reabrir `server.mjs` / `relay-error.mjs`.

### Dentro

1. **Predicado de red transitoria DLT** (homólogo F4c): tokens case-insensitive `enetunreach`, `etimedout`, `enotfound`, `network is unreachable`, `connection timed out` sobre la causa de publish. Si match: stamp + `enqueue_dlt_reanchor`; **no** `emit_dlt_batch_fracture`. Opaco (p. ej. `config-missing`, 500 sin cause de red) **sí** emite.
2. **Cubo Kaizen DLT:** `iota-relay-publish-error` / `F-DLT-PUBLISH-ERROR` → `process_fix` (red transitoria / publish con relay vivo). Catch-all `failed` no aplica. Tests `analyze_fracture_kaizen`.
3. **E2E instancia (`SIMULATE=0`):** cápsula `iota-immutable-publisher` (no `curl` POST). Preferible re-encolar/drenar `863d1511` si sigue sin `merkle_anchored`; si el evento ya no es resoluble, lote isomorfo con `transaction_digest`. Documentar UUID real en `execution.md`.
4. **Documentar** dual path y `A-HEALTH-NO-ES-PUBLISH` en spec/plan/validacion.

### Fuera

- Reabrir taxonomía `b3a715381787` o cause-propagation `a90fad3fa8fa` (PR #245).
- Reabrir código DCC `49ce2db7152d` / `d0cfd5b66ff1`.
- Inventar veredicto `infrastructure_resilience`.
- `prompt_adjustment` / touchpoint operador.
- `SDDIA_LAB_SIMULATE_IOTA=1` como Done.
- Retry, `sleep`, polling (DA-5).
- Bypass raw. Volcar bóveda.

---

## 7. Afirmaciones descartadas

### Stub v1.0.0 (Mayeuta)

| Afirmación | Realidad |
|------------|----------|
| `prompt_adjustment` / «no continuar entrega» | No hay entrega. Hay 500 de publish + cause de red. |
| «Bloqueo sin escalado Kintsugi» | Fractura emitida; PBI materializado. |
| Causa = operador | Causa = `ENETUNREACH` + sobre-escalado Kintsugi. |

### Refinamiento v1.1.0 (Filtro A)

| Afirmación v1.1.0 | Corrección |
|-------------------|------------|
| «relay-error.mjs (PR #259)» | **PR #245.** |
| «Evento detonante raíz = 863d1511 / PR #272» | Instancia **isomorfa posterior**. PBI existía en `d44d7b3` (12:30:13Z). |
| «DCC colapsó exactamente a las 14:50 CEST a la vez que 863d1511» | 14:50 CEST = fallo IOTA al presentar PR #272 (DCC de ese PR **ok**). 49ce2db7152d es DCC **previo** sobre `feat/nucleo-aiua-tormentosa-motor`. |
| «DCC ya clasificó red transitoria en PR #272; DLT requiere homólogo» | PR #272 clasifica el **token gh**, no este sello DLT. El homólogo real es F4c (`d0cfd5b66ff1`) + extender predicado DLT. |
| Veredicto `infrastructure_resilience` | **No existe** en `verdict_priority`. Solo `process_fix`. |
| «Fractura en lugar de usar la cola» | Usa **ambas**. Defecto = emitir Kintsugi además de encolar. |
| CA3 «solo laudo documental» | Insuficiente. El homólogo F4c es **código**. |
| CA1 = rescate obligatorio de un UUID | Higiene de instancia. Gate de código = predicado + tests. E2E = un digest real `SIMULATE=0`. |

---

## 8. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| **DLT-NET-CA1** | Traza PBI (`iota-relay-publish-error` + `cause: ENETUNREACH`) **no** materializa `System_Fracture_Detected`. Stamp + cola `dlt_reanchor` sí. | Tests `route_domain_core` (pending vacío; queue file presente). |
| **DLT-NET-CA2** | 500/`iota-relay-publish-error` **sin** token de red (p. ej. `config-missing`) **sí** emite. | Test de regresión sobre `emit_dlt_batch_fracture_publish_error_friction`. |
| **DLT-NET-CA3** | Cubo `analyze_fracture_kaizen` para esta traza → `process_fix`; sección **sin** `prompt_adjustment`. Catch-all `failed` intacto para trazas no-DLT. | `cargo test -p execute-process --lib -- analyze_fracture_kaizen`. |
| **DLT-NET-CA4** | Un publish `SIMULATE=0` con `transaction_digest`. `/health` 200 y publish 2xx no se colapsan. | `execution.md` + `validacion.md`. Cápsula, no curl. |

CA-CI: `global: APTO` solo con `run_id` verde (`features-documentation-pattern` v1.2.1).

---

## 9. Criterio de cierre

- [x] DLT-NET-CA1…CA3 (CA4 PENDIENTE_INSTANCIA: cápsula reprodujo ENETUNREACH; F1 fuera de código).
- [ ] Argos APTO en `validacion.md` (`pbi_archived: true`) — CA-CI pendiente de run verde.
- [x] Este PBI en `docs/todos/done/` en la misma rama del PR.

Prohibido Done con `SIMULATE=1` o con «la cola ya existía» y sin CA1.

---

## 10. Conclusión (v1.2.0)

### Diagnóstico

Sello `41717b4bb229` = HTTP 500 de publish con relay vivo y `err.cause=ENETUNREACH` (host → Testnet). Mayeuta mapeó `failed` a operador. El Core ya clasifica y encola; **no debería** abrir Kintsugi por un corte de transporte transitorio.

### Veredicto

**`process_fix`.** Prohibido `prompt_adjustment`. Prohibido `infrastructure_resilience`.

### Propuestas

1. Predicado transitorio DLT (paridad F4c) en `emit` de fractura batch.
2. Cubo Mayeuta DLT con exclusión del catch-all.
3. E2E `SIMULATE=0` vía cápsula.

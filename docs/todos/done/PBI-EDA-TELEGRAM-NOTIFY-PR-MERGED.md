---
document_id: PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED
uuid: "5880d6fc-99f3-4ecf-8c9e-a4885d45f117"
title: "[ARQUITECTURA] Notificación Telegram reactiva post-merge — síntesis IA + metadatos estáticos"
format: markdown
version: "1.2.0"
created: "2026-09-05"
updated: "2026-09-05T19:35:00+02:00"
status: cerrado
refinement_status: implemented
ola: 2
priority: media
process: feature
executor_vehicle: feature
type: arquitectura
dispatch: false
suggested_branch: feat/eda-telegram-notify-pr-merged
persist_ref_suggested: docs/features/eda-telegram-notify-pr-merged
pr_url: https://github.com/racso80es/SddIA/pull/262
fix_ref: docs/features/eda-telegram-notify-pr-merged
related:
  - SddIA/library/codexes/codex-software-engineering/process/accept-pr.md
  - SddIA/events/domain/pull-request-merged.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/core/event-subscriptions.json
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/tools/send-telegram-notification.md
  - SddIA/tools/gemini-http-infer.md
  - SddIA/norms/events-contract.md
  - SddIA/norms/external-ai-constraints.md
  - SddIA/norms/skill-io-git-manager-frozen.md
---

# PBI — Telegram post-merge (ola 2: estático + síntesis)

## 0. Filtro A — ola 2 (propuesta cruda vs runtime)

La propuesta de ola 2 **no se copia literal**. Contraste empírico (rama `feat/eda-telegram-notify-pr-merged`, Clase `pull-request-merged` v1.0.0, `route_domain_core.rs`, `gemini-http-infer`, `skill-io-git-manager-frozen` v1.1.0):

| ID | Afirmación cruda | Verdad | Corrección en este PBI |
|----|------------------|--------|------------------------|
| FA-O2-1 | La acción extrae «commits/diffs **del payload**» | Payload REQUIRED = `source_branch`, `target_branch`, `merge_commit_hash`, `author`, `security_clearance`. OPTIONAL = `pr_url`, `repository_name`. **No hay** commits, mensajes ni diffs. | Contexto LLM = campos ECST + `correlation_id` de **envelope**. Diff completo = fuera. |
| FA-O2-2 | `git-manager` puede alimentar el LLM con el diff | Enum congelado: `status`…`diff_name_only`. `get_last_commit` = `rev-parse` → `commitHash` (sin subject). `diff_name_only` = nombres de fichero. No hay `show`/`log`/`diff` de contenido. | Ola 2 MVP: prompt ECST-only. `diff_name_only` opcional fail-soft (**no** MVP). |
| FA-O2-3 | `route_domain_core` construye el estático **y** el JSON apunta a una action | Fan-out `tool: send-telegram-notification` es el único camino que llama `build_telegram_message_from_event`. Un suscriptor `action:` **no** entra ahí. Tool+action a la vez = **doble Telegram**. | Un solo suscriptor Argos: `action: notify-humanized-pr-merged`. El handler reutiliza el compositor (misma crate). |
| FA-O2-4 | La action recibe el evento completo | `dispatch_subscriber` pasa a `try_run_native` **solo** `payload` (`Value::Object(payload_obj.clone())`). `correlation_id` vive en envelope. | Patrón `persist-pec-correlation-proof`: `run_from_event(repo, event)` **antes** del despacho genérico. |
| FA-O2-5 | El `.md` de la action «orquesta» | Acciones catalogadas EDA tienen handler nativo en `actions.rs`. Sin él, el runtime cae a `invoke_execute_action` (sin lógica Gemini). | Contrato `.md` **más** handler nativo. Forja vía `entity-manager` (uuid inmutable post-create). |
| FA-O2-6 | Prefijo `[EXECUTE AS RAW KERNEL…]` = protocolo creator | `external-ai-constraints` § Prefijo creator: `DO NOT BYPASS EDA BUS. USE SddIA CLI.` No es I/O de `gemini-http-infer`. | El texto anti-verbosidad es **contenido de `request.prompt`**, no el prefijo Tekton. No afirmar equivalencia protocolaria. |
| FA-O2-7 | Ejemplo de síntesis = verdad del merge | El LLM no «sabe» este PBI. El ejemplo es **plantilla de formato**. | Fixture de layout; la síntesis no es string hardcodeada. |
| FA-O2-8 | Renumerar DD-1…DD-4 pisa ola 1 | Ola 1: DD-1 Argos, compositor estático, paridad JSON, hash Clase, IOTA intacto — **sigue vigente**. | IDs **O2-DD-***. Ola 1 no se borra. |
| FA-O2-9 | PBI en `done/` + `implemented` | PR #262 **OPEN**. `accept-pr` **no** ejecutado (Kintsugi DCC anidado `HEAD`). Done = PR mergeado. | Este archivo vuelve a `pending/`. `status: abierto`. |
| FA-O2-10 | Fallo LLM = colapso EDA | Fan-out independiente por suscriptor. IOTA no depende de Telegram. | Fail-soft Gemini: testigo action `success`. Telegram fallido sí falla el suscriptor (paridad ola 1). |

**No-alucinación residual ola 1 (sigue):** `correlation_id` ≠ payload; `event-watcher` no compone el mensaje; SSOT = `event-domain-subscriptions.json`; `pr_url` OPTIONAL y `accept-pr` no lo inyecta.

---

## 1. Estado empírico (post-ola 1)

| Hecho | Valor |
|-------|--------|
| Rama / PR | `feat/eda-telegram-notify-pr-merged` · [#262](https://github.com/racso80es/SddIA/pull/262) OPEN |
| persist_ref | `docs/features/eda-telegram-notify-pr-merged` · `execution_id` ola 1 `fccb9d32-8996-4594-8293-71c27926a017` |
| Compositor estático | `build_telegram_message_from_event` rama `PullRequest_Merged` (tests `telegram_message_for_pr_merged`) |
| Suscriptor actual | `argos` + `tool: send-telegram-notification` |
| IOTA | `cumulo` + `iota-immutable-publisher` — no tocar |
| Clase uuid | `cfb8ce66-784e-4826-8a0a-a20c671e3a60` inmutable; `hash_signature` ola 1 `sha256:6cd7add8…` |
| `gemini-http-infer` | Tool existente uuid `7a8da3ad-4916-4ee3-8407-aa1ecdc7ecba`. I/O: `request.prompt` (+ `model` o `SDDIA_GEMINI_MODEL`). Timeout default 30s (`SDDIA_GEMINI_HTTP_TIMEOUT_SECS`). Lab: `SDDIA_LAB_MOCK_OUTBOUND` / `SDDIA_LAB_MOCK_GEMINI_URL`. |
| Cerbero (lab) | `entity-manager`, `feature`, `delivery-close-cycle` ∈ `revoked`. Forja de genoma = mismo patrón lab ola 1 (relay IDE), no init `feature` desde `main`. |
| Kintsugi | Fractura DCC anidado `PBI-FIX-FRACTURE-1e62e8b851f8`. Ola 2 **no** ejecuta `accept-pr` ni `gh pr merge`. |

Ola 2 **continúa esta rama / este PR**. Prohibido `feature` init desde `main` (perdería ola 1).

---

## 2. Objetivo y mapa de datos

El Vértice Biológico recibe **un** mensaje Telegram post-`PullRequest_Merged` = **bloque estático** (ola 1, determinista) **+** **síntesis de valor** (LLM, ≤2 líneas, fail-soft).

Flujo:

```text
PullRequest_Merged
  ├─ iota-immutable-publisher          (sin cambio)
  └─ argos / action: notify-humanized-pr-merged
        ├─ estático ← build_telegram_message_from_event(event)
        ├─ síntesis ← gemini-http-infer (prompt ECST; fail-soft)
        └─ send-telegram-notification(estático [+ síntesis])
```

`send-telegram-notification` se invoca **desde el handler**, no como segundo suscriptor EDA.

### Ejemplo canónico (formato)

```text
✅ PR Fusionado — feat/accept-pr-telegram-notify
━━━━━━━━━━━━━━━━━━━━━━━━
📦 Commit: a1b2c3d (main)
👤 Integrador: integration-operator
🔐 Auditor: Argos · pr-acceptance-protocol
🔗 Correlación: 7f3a9c2e…

🧠 Síntesis de Valor: Se ha implementado el suscriptor EDA para Telegram,
desacoplando las notificaciones post-merge del flujo síncrono.
```

Reglas del ejemplo: las líneas 1–6 son el compositor ola 1 (si `pr_url` ausente, no hay línea URL). El bloque `🧠 Síntesis de Valor:` **solo** se añade si el LLM devolvió texto no vacío. El párrafo de síntesis es **ilustrativo de tono y longitud**, no un oracle de test ni un string del genoma.

### Mapa de datos → prompt LLM (MVP)

| Fuente | Campos | Uso |
|--------|--------|-----|
| payload ECST | `source_branch`, `target_branch`, `merge_commit_hash`, `author`, `security_clearance.{auditor,policy_applied}`, `pr_url?`, `repository_name?` | Hechos. Prohibido inventar files/intent no presentes. |
| envelope | `correlation_id`, `event_type` | Hechos. `correlation_id` **no** está en payload. |
| fuera MVP | diffs, subjects, lista de commits | No existen en ECST ni en I/O congelado de `git-manager` como contenido. |

---

## 3. Decisiones de diseño

### Ola 1 (intactas)

- Titular Telegram = **argos** (simetría Presented).
- Estático = `build_telegram_message_from_event`; cero acoplamiento en `accept_pr.rs`.
- SSOT JSON domain + paridad legado. Intent sin «anomalías».
- Sin `timestamp`. Sin `traceability_*`. `target_branch` desde payload (fallback `main`).
- Mutar Clase § Suscripciones vía `entity-manager` + `markdown_body_replacements` (uuid inmutable).

### Ola 2

**O2-DD-1 · Suscriptor = nueva action.** En `event-domain-subscriptions.json` (y paridad `event-subscriptions.json`) el segundo ítem de `PullRequest_Merged` deja de ser `tool: send-telegram-notification` y pasa a `action: notify-humanized-pr-merged` con `agent: argos`. IOTA no se mueve. Prohibido dejar tool+action (doble envío).

**O2-DD-2 · Orquestación semántica.** `notify-humanized-pr-merged` (handler nativo) consume `tool: gemini-http-infer` (`invoke_tool_capsule_json` / equivalente) con `request.prompt` estricto y contexto ECST. **No** extrae commits/diffs del payload. No se forja una tool nueva: `gemini-http-infer` ya existe.

**O2-DD-3 · Prompt anti-conjetura (Filtro A).** El prompt inyectado **incluye** (como texto, no como prefijo creator):

```text
[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. PENALIZE CONJECTURE. MAX 2 LINES]
```

Más: devolver **solo** valor de negocio; prohibido re-narrar metadatos ya visibles (hash, auditor, rama); prohibido afirmar archivos o intenciones no listados en el contexto; temperatura baja (`temperature` ≤ 0.2 si se pasa). Truncar respuesta a 2 líneas / ~400 chars antes de ensamblar.

**O2-DD-4 · Ensamblaje final.** Tras (o en ausencia de) LLM: concatenar estático + (opcional) `\n\n🧠 Síntesis de Valor: {texto}` → `send-telegram-notification`. El estático lo construye la **misma función** ola 1, invocada desde el handler con el **evento completo**.

**O2-DD-5 · Despacho envelope.** En `dispatch_subscriber`, rama especial `action == "notify-humanized-pr-merged"` → `run_from_event(repo, event)` (espejo `persist-pec-correlation-proof`). No depender del camino payload-only.

**O2-DD-6 · Fail-soft LLM.** Si `gemini-http-infer` exit ≠ 0, timeout (`http-post-failed` / ureq), HTTP 4xx/5xx, candidato vacío, `GEMINI_API_KEY` ausente (fuera de lab-mock), o texto solo whitespace: **omitir síntesis**, enviar estático, `success: true` en la action. No dead-letter por Gemini. Fallo de Telegram = fallo del suscriptor (ola 1).

**O2-DD-7 · Forja del contrato.** `SddIA/actions/notify-humanized-pr-merged.md` + fila `actions/index.md` vía `entity-manager` `entity_class: action` `create`. Prohibido `Write` sobre `SddIA/actions/`. Prohibido `update` genérico posterior que regenere uuid. Context propuesto: `ecosystem-evolution` (orquestación EDA; no DLT). Capabilities: delegación `gemini-http-infer` + `send-telegram-notification`.

---

## 4. Alcance

### Dentro

- Contrato `SddIA/actions/notify-humanized-pr-merged.md` (prompt base en el cuerpo; I/O YAML).
- Handler nativo + `run_from_event` + reutilización del compositor estático.
- JSON SSOT + paridad: destino action.
- Clase `pull-request-merged` § Suscripciones: tool → action (EM, uuid inmutable).
- Tests: orquestación Gemini (lab-mock); fail-soft (infer-failed / timeout simulado); estático sin síntesis; un solo envío; IOTA intacto.
- Docs persist_ref ola 2 (clarify/objectives/spec/plan). Evolution al mutar genoma.

### Fuera

- Entrenamiento / fine-tune del LLM.
- Gestión de cuotas, retry-backoff o circuit-breaker de la API (un intento; luego fail-soft).
- Versionar payload ECST 1.1.0 con diffs/commits.
- Ampliar `git-manager` (`show`/`log`/diff de contenido) — norma congelada.
- `diff_name_only` en el prompt (fase posterior, fail-soft).
- Backfill `pr_url` en `accept-pr`.
- Mutar `send-telegram-notification` o `gemini-http-infer`.
- Segundo suscriptor tool en paralelo.
- `accept-pr` / merge PR #262 / reparación de `PBI-FIX-FRACTURE-1e62e8b851f8` (Kintsugi: laudo humano).
- Init `feature` desde `main`.

---

## 5. Artefactos

| Artefacto | Mutación ola 2 | DA-2 |
|-----------|----------------|------|
| `SddIA/core/event-domain-subscriptions.json` | `PullRequest_Merged` Argos: `tool` → `action: notify-humanized-pr-merged` | no (core) |
| `SddIA/core/event-subscriptions.json` | paridad | no |
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | despacho `run_from_event`; compositor sigue en crate | no |
| `SddIA/engine/execute-process/src/engine/actions.rs` (+ módulo handler) | nativo `notify-humanized-pr-merged` | no |
| `SddIA/actions/notify-humanized-pr-merged.md` | **create** EM | sí |
| `SddIA/actions/index.md` | fila (EM / sync-entity-index) | sí |
| `SddIA/events/domain/pull-request-merged.md` | tabla suscriptor Telegram → action | sí (EM replacements) |
| `SddIA/core/eda-coverage.json` | hash Clase (sello EM) | sello |
| `SddIA/evolution/` | alta ola 2 | sello |
| `docs/features/eda-telegram-notify-pr-merged/` | spec/plan ola 2 | no |
| este PBI | v1.2.0 en `pending/` | no |

---

## 6. Criterios de aceptación

| ID | Criterio |
|----|----------|
| TG-MERGED-CA1 | `event-domain-subscriptions.json` + paridad: `PullRequest_Merged` tiene `agent: argos` + `action: notify-humanized-pr-merged` y **no** `tool: send-telegram-notification` en esa clave. IOTA intacto. |
| TG-MERGED-CA2 | La action orquesta `gemini-http-infer` con `request.prompt` que incluye el contexto ECST del PR (`source_branch`, `merge_commit_hash`, `author`, clearance) y el bloque anti-conjetura O2-DD-3. Lab-mock cuenta como invocación. |
| TG-MERGED-CA3 | Bloque estático = reglas ola 1 (hash 7, target payload, sin `traceability_*`, correlación envelope si `len>=8`). |
| TG-MERGED-CA4 | Si el LLM falla, excede timeout o HTTP error: fail-soft → mensaje estático **sin** síntesis; testigo action `success`; el hilo EDA **no** colapsa por Gemini. |
| TG-MERGED-CA5 | `iota-immutable-publisher` en `PullRequest_Merged` sin cambio de intent. |
| TG-MERGED-CA6 | Clase `pull-request-merged` § Suscripciones lista la action (no la tool Telegram); uuid inmutable; `hash_signature` recalculado por EM. |
| TG-MERGED-CA7 | Despacho usa evento completo (`correlation_id` en estático). Un merge → **un** `send-telegram-notification`. |

---

## 7. Riesgos

- **Cerbero revoked** (`entity-manager` / `feature`): forja `.md` bloqueada sin relay lab. No crear el contrato a mano.
- **Latencia Gemini (30s)** en el fan-out del centinela: acotar timeout de ola 2 (`SDDIA_GEMINI_HTTP_TIMEOUT_SECS` ≤ 15 recomendado en el handler si se pinna) para no bloquear IOTA (fan-out ya es por suscriptor; IOTA va primero en el JSON).
- **Alucinación residual:** el fail-soft no filtra invención si Gemini «tiene éxito» con texto conjetural. Mitigación = prompt + truncado 2 líneas; no un segundo LLM juez (fuera).
- **Lab-mock** (`lab-mock:{model}:{prompt80}`) no es síntesis de negocio: en tests de ensamblaje, mock controlado o aserción de **presencia** del bloque, no del copy ilustrativo del §2.

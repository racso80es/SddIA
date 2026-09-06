---
document_id: PBI-EMAIL-TRIAGE-HEURISTIC
uuid: "2d939386-db39-44f0-804f-1d5ab6ed78c2"
title: "[OPERATIVO] Bucle de Triaje Heurístico y Asimilación de Contexto (Cold-Start)"
format: markdown
version: "1.4.0"
created: "2026-09-02"
updated: "2026-09-06"
status: "propuesta"
refinement_status: refined
priority: alta
type: operativo
process: feature
dispatch: false
suggested_branch: feat/email-triage-heuristic-cold-start
persist_ref_suggested: docs/features/email-triage-heuristic-cold-start
depends_on:
  - PBI-TELEGRAM-INLINE-KEYBOARD
spawned:
  - PBI-TELEGRAM-INLINE-KEYBOARD
  - PBI-PREF-STORE-LANCEDB-MIGRATION
  - PBI-EMAIL-NOISE-HEURISTIC-DIGEST
  - PBI-EMAIL-DIGEST-PREFERENCE-REPLY
related:
  - SddIA/library/norms/email-triage-matrix.md
  - SddIA/library/codexes/codex-kalma2-assistant.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-triage-gateway.md
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/events/domain/email-received.md
  - SddIA/events/domain/email-triaged.md
  - SddIA/events/domain/user-preference-change-requested.md
  - SddIA/events/domain/user-preference-changed.md
  - SddIA/process/user-preference-ingest.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/skills/user-preference-store.md
  - SddIA/user-preference-core/
  - docs/features/memoria-preferencias-usuario/spec.md
  - SddIA/library/norms/capability-contracts/memory.pref_query.schema.json
  - SddIA/tools/send-telegram-notification.md
  - SddIA/norms/capsule-json-io.md
  - SddIA/actions/emit-user-preference-change-requested.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-quick-action-ingest.md
  - SddIA/process/radamanto-batch.md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
laudos:
  - D1-A
  - D2-CUARENTENA
  - D3-NO-EXPANDIR-ACTIONABLE
refinement_notes: >-
  Filtro A v1.4.0 (2026-09-06). Laudos: D1=A; D2=cuarentena (muro cold-start +
  P-EXEMPT-C solo explicit_user); D3=ok. Digest/réplica spawned. Auditoría =
  proofs email-triaged, no telemetry ni radamanto-batch. v1.3.0: §0.
---

### [OPERATIVO] Bucle de Triaje Heurístico y Asimilación de Contexto (Cold-Start)

#### 0. Correcciones Filtro A (v1.2.0 → v1.3.0)

Hechos del genoma vigentes; lo que v1.2.0 afirmaba y queda anulado:

| Afirmación v1.2.0 | Hecho SSOT | Corrección |
|-------------------|------------|------------|
| `depends_on` incluye LanceDB | Store MVP = JSON durable bajo `paths.userPreferencesStore` (`.SddIA/vector_store/user_preferences/`). LanceDB es deuda explícita de `memoria-preferencias-usuario` spec §4 | LanceDB sale de `depends_on`; queda en `spawned` |
| El gateway tiene contrato cápsula `request.{from,subject,snippet}` | Proceso `email-triage-gateway` v1.0.0: input `event_file_path`; handler nativo `email_triage.rs`. `capsule-json-io` rige skills/tools, no este proceso | §4 reescrito |
| `notification_payload` + `reply_markup` salen del gateway | Fan-out: `Email_Triaged` → `send-telegram-notification` (dispatcher `route_domain_core.build_telegram_message_from_event`). Poke actual: `Correo accionable\nfrom=…\nsubject=…\nuid=…` solo si `verdict=actionable` | Botonera = dispatcher + tool + watcher (Slice 2), no output del proceso |
| Vía B: solo logística/finanzas/seguridad/citas → `actionable` | Matriz §3: `actionable` exige `title`+`datetime` extraíbles; handler LLM solo eleva reunión/cita con fecha en asunto. `C-NOREPLY` marca `noise` a `no-reply@` / `noreply@` / `notifications@` **antes** de Clasificación | Cold-start = matriz vigente. Esas categorías no son reglas nuevas en este PBI |
| Consulta `from` + `subject` en claro | `QuerySpec`: `subject_key?`, `predicate?`, `scope`, `max_results`. Spec: `subject_key` = hash, no PII en ECST | Hash canónico del remitente; el asunto del correo no es clave de query |
| Fail-open `{}` | `query_context_block`: `{schema_version: "1.0.0", preferences: []}` (L-FAIL-POLICY: vacío ≠ permitir todo) | Bloque versionado, no objeto vacío |
| `[Ajustar regla]` → `telegram-fallback-responder` | Ese proceso es triaje inverso de texto libre (`TelegramMessage_Received`), no FSM de excepción de preferencia | Fuera de alcance; Slice 2 no lo reutiliza |
| `decision_path: preference` sin bump | Evento `Email_Triaged` y matriz §5: `deterministic` \| `llm` solamente. Outputs del proceso: lo mismo | Bump menor de evento + matriz + proceso (ver §4) |
| Opción B = cápsula que envuelve el proceso | Skills no orquestan procesos | Opción B descartada como inversión ontológica |

#### 1. Naturaleza del Activo

Capa de contexto histórico (preferencias del portador) sobre el circuito de triaje ya productivo:

```
Email_Received (email-watcher)
  → email-triage-gateway  (Triaje-C → Clasificacion llm:interact → Asiento-Agenda agenda:persist → Emision)
  → Email_Triaged
  → send-telegram-notification  (solo verdict=actionable; ruido silenciado)
```

**Valor:** Cold-start = matriz actual (cero regresión). Con hábitos `status: active` en el store JSON, el gateway resuelve `mute`/`priority` sin LLM cuando el patrón es inequívoco, y conjuga preferencias parciales en el prompt de Clasificación cuando no lo es.

**Fuera de alcance de este PBI:**

- Migración LanceDB / KNN (`PBI-PREF-STORE-LANCEDB-MIGRATION`).
- Refactor del modelo de tres vías (`noise` \| `passive` \| `actionable`).
- Mutación IMAP (matriz §5; `email-quick-action-ingest` tampoco STORE).
- Rediseño amplio del códice (Opción C).
- FSM conversacional «ajustar regla» vía `telegram-fallback-responder`.
- Digest diario de ruido y réplica humana → preferencias (`PBI-EMAIL-NOISE-HEURISTIC-DIGEST`, `PBI-EMAIL-DIGEST-PREFERENCE-REPLY`).
- Override de Triaje-C **sin** hábito `explicit_user` (el muro cold-start no se relaja por inferencia ni en el primer correo).

**Deuda spawned (no bloquea Slice 1):** store vectorial; botonera Telegram (bloquea Slice 2); digest de cuarentena; réplica del digest.

#### 2. Circuito vigente (no reimaginar)

| Pieza | Estado | Rol |
|-------|--------|-----|
| Evento `Email_Received` | `snippet` REQUIRED; `body` FORBIDDEN | Estímulo. El gateway lee el payload del ECST, no un envelope de cápsula |
| Evento `Email_Triaged` | `snippet` y `body` FORBIDDEN; `from`/`subject` OPTIONAL v1.1.0 | Veredicto + poke |
| Proceso `email-triage-gateway` | Empacado en `codex-kalma2-assistant`; handler nativo | Aduana. G5: si Triaje-C concluye, Clasificacion = `skipped` |
| Norma `email-triage-matrix` 1.0.1 | Códice: prevalece sobre heurística del ejecutor o del LLM | Ley de veredicto |
| Capacidad `memory:pref-query` | Provider `skill:user-preference-store`; crate `user-preference-core` | Lectura síncrona opt-in (L-NO-QUERY-EVENT) |
| Proceso `user-preference-ingest` | Suscriptor de `User_Preference_Change_Requested` | Persistencia vía `memory:pref-write` |
| Consumidor piloto de query | `telegram-fallback-responder` (fail-open) | **No** es el consumidor de correo; este PBI añade opt-in en el gateway |

Predicados MVP cerrados (`memoria-preferencias-usuario` spec §3): `priority` (`value.level`: `max`\|`high`\|`normal`\|`low`), `mute`, `attention_window`. Alta de predicado = mutación de spec, no de este PBI.

#### 3. Lógica propuesta (dos slices)

##### Slice 1 — Triaje-P + conjugación (no depende de botonera)

Ante `Email_Received`, el handler amplía la coreografía. **Orden canónico (D2: muro en cold-start; exención solo con hábito `explicit_user`):**

1. **Consulta P (barata, siempre, fail-open).** `memory:pref-query`. `QuerySpec` sobre `subject_key` = hash canónico del remitente normalizado (lowercase; local-part+domain del addr; función de hash en spec de la feature; **no** dirección en claro en ECST). `include_proposed: false`. Bloque `{schema_version: "1.0.0", preferences: []}` si vacío/caído. Cero LLM.
2. **Exención C (solo `explicit_user` + `predicate: priority` + `value.level` ∈ {`max`,`high`}, `status: active`).** Si hay match: **no** se aplican `C-LIST` / `C-NOREPLY` / `C-SUBJECT-NOISE` a este mensaje. `matched_rule: P-EXEMPT-C`. No cierra `actionable` por sí solo (matriz §3: hace falta `title`+`datetime`). Sigue a Clasificacion / elevación estructural. Sin esta exención, el muro del paso 3 es absoluto — cold-start idéntico a hoy.
3. **Triaje-C (muro):** si no hubo exención. `C-LIST`, `C-NOREPLY`, `C-SUBJECT-NOISE` → `noise`, `decision_path: deterministic`, Clasificacion skipped, **cero** poke, **cero** LLM. G5 intacto. Constancia: `Email_Triaged` + proof (ver §6 D2).
4. **Mute P (si C no concluyó):** `predicate: mute` activo → `noise`, `decision_path: preference`, `matched_rule: P-MUTE-SENDER`, sin LLM.
5. **Clasificacion (existente, prompt ampliado):** si nadie cerró. Prompt actual + bloque `user_preference_context`. Prohibido inyectar `sensitivity: personal` en logs/telemetría CLI. Cold-start (bloque vacío, sin exención) = prompt y C actuales ⇒ CA1.
6. **Asiento-Agenda / Emision:** sin cambio de semántica. `decision_path`: `preference` si cerró mute; `deterministic` si cerró C; `llm` si Clasificacion cerró. `P-EXEMPT-C` no es cierre: solo impide el muro C.

Cold-start (store vacío o query fail-open): comportamiento **idéntico** al handler actual, incluida la elevación post-LLM de reunión/cita con fecha en asunto y la trampa de verbosidad comercial.

##### Slice 2 — Asimilación (bloqueado por `PBI-TELEGRAM-INLINE-KEYBOARD`)

El poke de `Email_Triaged` accionable ya existe. Este slice lo extiende:

- Texto: una línea de motivo (`Heurística Base` / `Preferencia P-MUTE-SENDER` / `priority:max`) **sin** cuerpos ni snippet (FORBIDDEN en el evento).
- Botones inline (`InlineKeyboardMarkup`) vía extensión de `send-telegram-notification` (`reply_markup`). El dispatcher —no el gateway— construye el markup al fan-out.
- `telegram-watcher` intercepta `callback_query` y emite `User_Preference_Change_Requested` (acción `emit-user-preference-change-requested`). `callback_data` ≤ 64 bytes (límite Telegram).
- Mapeo de botones (autoridad `explicit_user`; `inferred` no pasa a `active` sin confirmación — spec §3):
  - `[Útil]` → `operation: activate`, `predicate: priority`, `priority_level: high`, `subject_key` = hash del remitente del poke, `channel: telegram`.
  - `[Ignorar similares]` → `operation: activate`, `predicate: mute`, mismo `subject_key`.
- `[Ajustar regla]` **excluido** de este PBI: exige FSM de turno y no es `telegram-fallback-responder`.

#### 4. Contratos que hay que versionar (no hay cápsula nueva de triaje)

| Artefacto | Cambio |
|-----------|--------|
| `email-triage-gateway` proceso | Fase **Triaje-P** (`memory:pref-query` ≥1.0.0): query → exención C opcional → (C) → mute → Clasificacion. Output `decision_path`: `deterministic` \| `llm` \| `preference` |
| Handler `email_triage.rs` | Implementar el orden §3. G5: C o mute que cierran skippean Clasificacion. Query P no cuenta como inferencia. |
| `email-triage-matrix` | Bump menor: `P-EXEMPT-C` / `P-MUTE-SENDER`; C no concluye si hay exención `explicit_user`+`priority` max\|high; `decision_path` incluye `preference`. **Sin este bump, el handler viola el códice** |
| `Email_Triaged` | Bump menor: `decision_path` enum + `preference`. No añadir `snippet`/`reply_markup` al payload |
| `send-telegram-notification` + `telegram-watcher` | Slice 2; viven en `PBI-TELEGRAM-INLINE-KEYBOARD` |
| Skill `user-preference-store` | Sin cambio de contrato. Envelope query ya es `capsule-json-io` v2.0; `op: QUERY` / `QUERY_CONTEXT` + `spec` |

Input del proceso sigue siendo `event_file_path`. No se finge un `request` de cápsula sobre el gateway.

#### 5. Criterios de Aceptación

**Slice 1**

- [ ] **CA1 (Cold-Start):** Store vacío o fail-open. Mismos veredictos que el handler actual sobre la matriz 1.0.1 (Triaje-C, skip LLM, elevación estructural de cita, trampa comercial). Cero invocaciones extra de `mayeuta-llm` vs baseline.
- [ ] **CA2 (Triaje-P mute):** Preferencia `status: active`, `predicate: mute`, `subject_key` = hash del remitente. Si Triaje-C no concluyó: `noise`, `decision_path: preference`, sin `llm:interact`.
- [ ] **CA3 (Conjugación):** Preferencias parciales que no eximen ni mutan (p.ej. `attention_window` o `priority:normal`) viajan al prompt de Clasificacion como bloque `user_preference_context` v1.0.0. No viaja `body`. No se eleva `actionable` sin `title`+`datetime`. `priority` max\|high no es «parcial»: es `P-EXEMPT-C` (CA11).
- [ ] **CA5 (Tres vías):** Outputs y `Email_Triaged.verdict` ∈ {`noise`,`passive`,`actionable`}.
- [ ] **CA6 (Decision path):** `deterministic` \| `llm` \| `preference` coherente con quien **cerró**. Matriz y evento bumpados.
- [ ] **CA7 (Ceguera IMAP):** Cero `STORE`/expunge/delete en todos los caminos, incluido mute.
- [ ] **CA9 (PII query):** `QuerySpec.subject_key` nunca es la dirección en claro. ECST de preferencias sigue FORBIDDEN de `body`/`snippet`/utterance completa.
- [ ] **CA10 (Fail-open):** Store caído → bloque `{schema_version, preferences:[]}` → degradación a CA1, no a «permitir todo».
- [ ] **CA11 (Muro C / exención):** Sin preferencia activa (cold-start): `noreply@` → `C-NOREPLY` / `noise` / sin poke / sin LLM. Con preferencia `explicit_user`+`priority` max\|high activa para ese `subject_key`: C no cierra; el mensaje sigue a Clasificacion. `inferred` o `proposed` **no** eximen. `priority` no inventa `datetime`.

**Slice 2** (gate: `PBI-TELEGRAM-INLINE-KEYBOARD` implementado)

- [ ] **CA4 (Retroalimentación):** Callback de botón emite `User_Preference_Change_Requested` válido; `user-preference-ingest` persiste revisión `explicit_user`.
- [ ] **CA8 (Botonera):** `reply_markup` en el poke accionable; `callback_query` respondido con `answerCallbackQuery`. Mensajes de texto del watcher sin regresión.

#### 6. Laudos (v1.4.0)

| ID | Laudo | Efecto en este PBI |
|----|-------|-------------------|
| **D1** | **A** — extender `email-triage-gateway` + handler + bump menor de matriz/evento. C aplazada. B descartada | Slice 1 |
| **D2** | **Cuarentena asíncrona.** Cold-start: C implacable. Exención `P-EXEMPT-C` solo tras hábito `explicit_user`. Digest+réplica spawned | CA11; spawned digest/réplica |
| **D3** | **No** ampliar `actionable` sin `title`+`datetime` | CA1, CA3 |

##### D2 — Arquitectura (Filtro A sobre la síntesis)

**Muro (este PBI, Slice 1).** Cold-start: `C-NOREPLY` / `C-LIST` / `C-SUBJECT-NOISE` siguen implacables. Sin notificación, sin LLM, sin interrupción. El dispatcher ya silencia todo `verdict ≠ actionable`.

**Pista de auditoría (ya existe; no es telemetría).** `Email_Triaged` es familia **domain**, no `telemetry`. El bus fractal `./.events/domain/` es volátil (sweeper). La constancia durable es `eda_instance.proofs` → `.SddIA/proofs/email-triaged/{event_id}.json` (handler `persist_email_triaged_proof`; incluye `noise`). Kalma2 WUI (`list_actionable_email_items`) **filtra** `verdict=actionable`: el ruido ya está en disco y nadie lo lee. No hay entidad «Discovery» en el genoma; no se inventa un status.

**Digest (spawned, no este PBI).** Proceso **nuevo** empacado en `codex-kalma2-assistant` (misma jurisdicción que el gateway). **No** es `radamanto-batch`: ese proceso consume `Raw_Execution_Finished`, prohíbe LLM y no interpreta correo. **No** es un job de Argos-juez: Argos solo figura hoy como agente del **tool** `send-telegram-notification` en el fan-out de `Email_Triaged`. Agente titular del proceso nuevo: `cumulo` (precedente del gateway). Eferencia: el mismo tool. Fuente: proofs, ventana `[since, until)` inyectada por estímulo de **instancia** (no hay `OnCalendar`/cron en el Core; el Core permanece agnóstico). Agregación **determinista** (conteo por `from` + `matched_rule` + asunto muestra). Cero LLM. Cero mutación IMAP: el ejemplo «hoy purgué N remitentes» es léxico falso; el veredicto es `noise`, el buzón no se toca. Alcance: `verdict=noise` ∧ `decision_path=deterministic` ∧ `matched_rule` ∈ {`C-LIST`,`C-NOREPLY`,`C-SUBJECT-NOISE`}. Excluir `P-MUTE-SENDER` (hábito ya asimilado). Newsletter cae en C-LIST / C-SUBJECT-NOISE, no solo C-NOREPLY. Track: `PBI-EMAIL-NOISE-HEURISTIC-DIGEST`.

**Réplica (spawned, no este PBI).** Obtener indicación humana del digest y persistir `User_Preference_Change_Requested` → ingest. Track: `PBI-EMAIL-DIGEST-PREFERENCE-REPLY`. No reutilizar `telegram-fallback-responder` ni `email-quick-action-ingest` (acciones `archive|draft|delegate`).

**Consecuencia para `priority:max` sobre `noreply@`:** en cold-start (sin hábito) sigue `noise`. Tras réplica explícita del digest (`explicit_user`, no inferida), `P-EXEMPT-C` impide que C cierre ese `subject_key`; el siguiente correo pasa a Clasificacion. Sin esa réplica el muro no se mueve. `inferred` no exime.

#### 7. Restricciones duras

- Prohibido mutar IMAP.
- Prohibido veredicto `actionable` por verbosidad, tono o urgencia (matriz §4).
- Prohibido ejecutar Clasificacion si Triaje-C concluyó **o** mute P cerró. La query P y `P-EXEMPT-C` no son Clasificacion.
- Prohibido `inferred` → `active` sin confirmación humana.
- Prohibido anclaje DLT de `value` de preferencia (L-NO-DLT-VALUE).
- Mutación de genoma (`email-triage-matrix`, proceso, evento, skill/tool) vía `entity-manager`, no a mano.
- Slice 2 no arranca con botonera ausente: el poke textual actual permanece.
- Prohibido que `inferred` / `proposed` eximan Triaje-C. Solo `explicit_user`+`priority` max\|high (`P-EXEMPT-C`).
- Prohibido enganchar el digest a `radamanto-batch` o al bus `telemetry`.

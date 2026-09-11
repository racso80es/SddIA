---
document_id: PBI-EMAIL-NOISE-HEURISTIC-DIGEST
uuid: "bd5eab3f-0693-408a-ab04-de8b43a36c54"
title: "[OPERATIVO] Digest heurístico de ruido de correo (Cuarentena asíncrona)"
format: markdown
version: "1.2.0"
created: "2026-09-06"
updated: "2026-09-11"
status: "propuesta"
refinement_status: refined
priority: media
type: operativo
process: feature
dispatch: false
suggested_branch: feat/email-noise-heuristic-digest
persist_ref_suggested: docs/features/email-noise-heuristic-digest
depends_on: []
blocks_on:
  - PBI-EMAIL-DIGEST-PREFERENCE-REPLY
spawned_by: PBI-EMAIL-TRIAGE-HEURISTIC
related:
  - docs/todos/done/[OPERATIVO] Bucle de Triaje Heurístico y Asimilación de Contexto (Cold-Start).md
  - docs/todos/pending/[OPERATIVO] Réplica del digest de ruido → preferencias.md
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/events/domain/email-triaged.md
  - SddIA/library/norms/email-triage-matrix.md
  - SddIA/core/cumulo.paths.json
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/tools/send-telegram-notification.md
  - SddIA/tools/send-telegram-notification/src/main.rs
  - SddIA/library/codexes/codex-kalma2-assistant.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-triage-gateway.md
  - SddIA/process/process-creator.md
  - SddIA/user-preference-core/src/lib.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
refinement_notes: >-
  Filtro A v1.2.0 (2026-09-11). Residual sobre v1.1.0: (1) normalize_email_addr
  no descodifica RFC 2047 — el decode ya ocurre al persistir el proof;
  (2) «agente titular cumulo» es alucinación heredada de D2: el digest no es
  suscriptor de dominio; (3) C-* abierto ≠ conjunto cerrado de la matriz;
  (4) default MarkdownV2 es del contrato .md, no de la cápsula Rust vigente;
  (5) process-creator default [0] = software-engineering; kalma2 exige
  process_domain_root explícito; (6) cursor until-only recontaría ventanas
  solapadas — clamp since; (7) events_scanned = matches del filtro, no
  ficheros leídos; (8) fallo Telegram no avanza cursor; (9) proofs_root vía
  eda_instance.proofs, no path cableado.
---

### [OPERATIVO] Digest heurístico de ruido de correo (Cuarentena asíncrona)

#### 1. Origen y Justificación

Laudo D2 de `PBI-EMAIL-TRIAGE-HEURISTIC` (archivado en `docs/todos/done/`, cerrado en PR #266): el muro Triaje-C permanece activo e implacable en cold-start; el crecimiento de contexto de preferencias no se hace correo a correo de forma intrusiva ni reactiva.

Este PBI materializa el **informe batch consolidado** (cuarentena asíncrona) a partir de los proofs persistidos en disco. Permite al portador biológico inspeccionar qué remitentes han sido silenciados por las reglas deterministas de cold-start (`C-LIST`, `C-NOREPLY`, `C-SUBJECT-NOISE`).

Este PBI no interpreta intenciones ni altera preferencias. Su salida alimenta el lazo de asimilación humana que será cerrado por `PBI-EMAIL-DIGEST-PREFERENCE-REPLY` (declarado en `blocks_on`).

---

#### 2. Fe de Erratas — Filtro A

##### 2.1 v1.0.0 → v1.1.0 (conservado)

| Elemento previo (v1.0.0) | Inexactitud | Realidad SSOT | Corrección v1.1.0 |
|---|---|---|---|
| Estado y ruta del padre | `docs/todos/pending/...` y «paralelo a Slice 1» | Padre cerrado en `docs/todos/done/` (PR #266) | Puntero a `done/` |
| Campo `timestamp` | Dentro de `payload` | Envelope raíz `proof["timestamp"]` (`persist_email_triaged_proof`) | Filtro temporal sobre clave raíz |
| Agregación por `from` crudo | Sin normalizar | Display names distintos, misma addr | `normalize_email_addr`; fallback `_unknown` |
| Ranking | Sin desempate | `HashMap` no ordena | `(count DESC, normalized_from ASC)`; regla alfabética |
| `parse_mode` | No especificado | Contrato tool: default `MarkdownV2` | Pasar `parse_mode: null` |
| Idempotencia | Ruta vaga | `daemons_instance.state` | `{state}/email-noise-digest.json` |
| `blocks_on` omitido | `depends_on: []` solo | Hijo declara `depends_on` este PBI | `blocks_on: [PBI-EMAIL-DIGEST-PREFERENCE-REPLY]` |

##### 2.2 v1.1.0 → v1.2.0 (esta pasada)

| Elemento v1.1.0 | Inexactitud / Incoherencia | Realidad SSOT en el Repositorio | Corrección v1.2.0 |
|---|---|---|---|
| `normalize_email_addr` «descodifica RFC 2047» | La función **no** decodifica RFC 2047. | `user-preference-core`: trim, extrae `<addr>`, ASCII lowercase. El gateway **sí** aplica `decode_rfc2047` a `from`/`subject` **antes** de escribir el proof (`email_triage.rs` ~L596–L605). | Digest llama `normalize_email_addr(payload.from)` sobre texto ya decodificado. No reimplementar RFC 2047. No copiar L-HASH del padre como si viviera en esa función. |
| Agente titular `cumulo` | Copiado de D2 «precedente del gateway». | `Email_Received` → `{agent: cumulo, process: email-triage-gateway}`. El digest **no** es fan-out de `Email_Triaged` (eso es `{agent: argos, tool: send-telegram-notification}`, poke por evento `actionable`). El proceso gateway **no** declara `delegates_to: agent:cumulo`. | Cero alta en `event-domain-subscriptions.json`. Cero agente titular. Estímulo = CLI/timer de **instancia** → `execute-process --process email-noise-digest`. Handler nativo, mismo patrón que el gateway. |
| `matched_rule ∈ C-*` (circuito y Slice 1) vs conjunto cerrado (CA-1 / matriz) | Prefijo abierto ≠ SSOT. | Matriz vigente: exactamente `C-LIST`, `C-NOREPLY`, `C-SUBJECT-NOISE`. | Filtro = conjunto cerrado. Nueva regla C = bump de este PBI, no glob `C-*`. |
| Default `parse_mode: MarkdownV2` implica HTTP 400 si se omite | Contrato `.md` vs cápsula. | Cápsula Rust: `parse_mode` JSON `null` / ausente / string `"null"` → `Option::None` → no envía el campo → Telegram texto plano. El default MarkdownV2 está **solo** en el contrato documental. | Mandato: JSON `"parse_mode": null`. No depender del desalineamiento. Si la cápsula se alinea al `.md`, omitir el campo rompería el digest. |
| `process_domain_roots[0]` implícito | Empaque kalma2 sin flag de root. | `process-creator`: jurisdiction `domain` + multi-root → default **`[0]`** = `codex-software-engineering/process`. Kalma2 es `[1]`. | EM create **exige** `process_jurisdiction: domain` y `process_domain_root: SddIA/library/codexes/codex-kalma2-assistant/process`. `process_contract_version: 1.4.0` (EM default interno `1.3.0`). |
| Cursor solo por `until` | Re-ejecutar `until` mayor recontarían `[since, last_until)`. | No hay cursor de `since`. Timer diario no solapa; CLI sí puede. | Si existe estado y `since < last_until < until`: `effective_since = last_until`. Si `until <= last_until`: skip. |
| `events_scanned` | §9 «proofs evaluados en la ventana» vs CA-4 `= 0` si filtro vacío. | Ambigüedad de métrica. | `events_scanned` = proofs que **pasan el filtro completo** (entran a agregación). Vacío o skip → `0`. |
| Path de proofs cableado `.SddIA/proofs/email-triaged/` | Circuito y Slice 1 hardcodean. | `proofs_root()` lee `eda_instance.proofs` (default `.SddIA/proofs`) y concatena `email-triaged/`. | Reutilizar la misma resolución. |
| Cursor tras poke fallido | No dicho. | Reintento exigiría no avanzar frontera. | Cápsula Telegram `success != true` → error de proceso; **no** escribir estado. |
| `process_membership` de ambos procesos kalma2 | «precedente gateway y quick-action». | Códice `process_membership` lista **solo** `email-triage-gateway`. `email-quick-action-ingest` está en `process/index.md`, no en membership. | Alta membership + índice del **root kalma2** vía EM. No «corregir» el hueco de quick-action en este PBI. |
| Hash a mano | «Recálculo de hash_signature» | DA-2: `library_codexes` y process de dominio = `entity-manager`. | Prohibido `Write`/`StrReplace` sobre esos árboles. Hash lo emite el creator. |

---

#### 3. Filtro A — Qué NO es este activo

| Tentación | Hecho SSOT en el Repositorio |
|-----------|------------------------------|
| «Telemetría del Core» | `Email_Triaged` es `event_family: domain`. Telemetría = `Raw_Execution_Finished` / `Daemon_Heartbeat` en `./.events/telemetry/`. Prohibido cruzar buses. |
| `radamanto-batch` / agente Radamanto | Consume `Raw_Execution_Finished`; `llm_profile: none`; no interpreta correo. |
| Job de Argos-juez / fan-out `Email_Triaged` | Argos+tool = poke **por evento** `actionable`. Este proceso es batch CLI, no suscriptor. |
| Escanear `./.events/domain/` | Bus fractal volátil (sweeper). Fuente durable: `{eda_instance.proofs}/email-triaged/`. |
| «Estado Discovery» | No existe en el genoma. |
| «Hoy purgué N remitentes» | Prohibido IMAP `STORE`. Veredicto `noise`. Léxico: clasificados / silenciados. |
| Solo `C-NOREPLY` | Matriz: también `C-LIST` y `C-SUBJECT-NOISE`. |
| LLM para «destacan» | Agregación aritmética. Cero `mayeuta-llm` / `llm:interact`. |
| Cron en el Core | No hay `OnCalendar` en el repositorio Core. `since`/`until` los inyecta la instancia. |
| Nueva Clase ECST / suscripción de dominio | No hay estímulo de dominio. Prohibido inventar `Email_Noise_Digest_Requested`. |
| Daemon nuevo | El cursor vive en `daemons_instance.state` (estado de instancia, `.gitignore`). El proceso no es un centinela. |
| Cápsula nueva de triaje | Handler nativo en `execute-process`, como el gateway. |

---

#### 4. Circuito Propuesto

```
Estímulo de instancia (timer / CLI; fuera del genoma Core)
  → execute-process --process email-noise-digest
       inputs: { since, until }   # RFC3339; ventana semiabierta [since, until)
  → Validar since < until (si no: error, cero I/O)
  → Leer cursor {daemons_instance.state}/email-noise-digest.json
       si existe y until <= last_until ⇒ skip (success, skipped=true, cero poke, cero scan)
       si existe y since < last_until < until ⇒ effective_since = last_until
       si no ⇒ effective_since = since
  → Resolver proofs_root (eda_instance.proofs) / email-triaged/
  → Filtrar (todas las condiciones, conjunto cerrado de reglas)
  → Normalizar remitente (normalize_email_addr) y agregar
  → Si 0 matches ⇒ persistir cursor, success (notified=false, cero poke)
  → Si > 0 ⇒ mensaje plano ≤ 4000 chars; invoke_capsule_json(send-telegram-notification)
       parse_mode: null
       si cápsula falla ⇒ error; cursor intacto
  → Persistir cursor (last_until = until) y envelope
```

- **Empaque:** `SddIA/library/codexes/codex-kalma2-assistant/process/email-noise-digest.md`. Alta en `process_membership` del códice y en `process/index.md` de **ese** root.
- **Forja:** `entity-manager` `create` clase `process` con `process_jurisdiction: domain`, `process_domain_root: SddIA/library/codexes/codex-kalma2-assistant/process`, `process_contract_version: 1.4.0`. Después `update` del códice (membership + cuerpo). Prohibido escritura directa DA-2.
- **Sin agente titular. Sin suscripción de dominio.**
- **Tool eferente:** `send-telegram-notification` vía `invoke_capsule_json` (mismo call que `route_domain_core` para este tool).
- **Handler nativo:** `handlers::email_noise_digest::run`. Registro en `engine/mod.rs` + `handlers/mod.rs`. Core; no es genoma DA-2.
- **Dependencia crate:** `user-preference-core` ya está en `execute-process/Cargo.toml`.

---

#### 5. Filtro de Inclusión

Para cada `*.json` en `{proofs_root}/email-triaged/`, **todas** las condiciones:

1. **Envelope raíz:**
   - `kind == "email-triaged-proof"`
   - `event_type == "Email_Triaged"`
   - `timestamp` parseable RFC3339 y `effective_since <= ts < until`

2. **`proof.payload`:**
   - `verdict == "noise"`
   - `decision_path == "deterministic"`
   - `matched_rule` ∈ {`"C-LIST"`, `"C-NOREPLY"`, `"C-SUBJECT-NOISE"`}

JSON ilegible, sin `timestamp` válido, o que falle cualquier cláusula → ignorado (no aborta el lote).

**Exclusiones explícitas:** `P-MUTE-SENDER`, `decision_path == "preference"`, `verdict` ∈ {`actionable`, `passive`}, `decision_path == "llm"`.

---

#### 6. Agregación Determinista y Normalización

1. **Clave:** `normalize_email_addr(payload.from)`. Si `from` ausente o normalizado vacío: `"_unknown"`.
2. **Por remitente:**
   - `count`: matches de ese remitente.
   - `matched_rule`: moda; empate → lexicográfico (`C-LIST` < `C-NOREPLY` < `C-SUBJECT-NOISE`).
   - `subject`: `payload.subject` del proof con `timestamp` máximo; vacío/ausente → `"(sin asunto)"`. Prohibido `snippet`/`body` (FORBIDDEN en `Email_Triaged`). Empate de timestamp → `event_id` ASC (determinismo).
3. **Orden final:** `count` DESC, `normalized_from` ASC.

---

#### 7. Formateo y Límite de Telegram

Texto plano. `parse_mode: null` en el JSON de la cápsula.

```text
Ruido Triaje-C {since_date}
eventos={total_events} remitentes={unique_senders}
- {sender} ({count}, {matched_rule}) {latest_subject}
¿Inyectar priority:max? Respuesta en ciclo aparte.
```

- `{since_date}`: fecha UTC `YYYY-MM-DD` de `since` (input original, no el clamp).
- Techo de construcción: 4000 caracteres (API Telegram 4096).
- Overflow: cabecera + pie + línea `+ {M} remitentes omitidos` fijos; caben las primeras K líneas del ranking; `M = unique_senders - K`.
- `subject` > 80 caracteres → truncar y sufijo `…` (U+2026).
- La pregunta del pie **no** implementa réplica (PBI hijo).

---

#### 8. Idempotencia y Estado de Instancia

- **Ruta:** `{daemons_instance.state}/email-noise-digest.json` (Cúmulo; default `.SddIA/daemons/state/`; `.gitignore`).
- **Esquema:**
  ```json
  {
    "last_until": "2026-09-06T23:59:59Z",
    "last_run": "2026-09-07T00:00:05Z",
    "last_events_scanned": 12,
    "last_senders_count": 4,
    "last_notified": true
  }
  ```
- **Skip:** archivo existe y `until <= last_until` → no scan, no poke:
  ```json
  {
    "success": true,
    "skipped": true,
    "reason": "already-processed",
    "events_scanned": 0,
    "senders": 0,
    "notified": false
  }
  ```
- **Clamp:** `since < last_until < until` → escanear `[last_until, until)`.
- **Persistir cursor** solo si skip no aplica y el lote termina en éxito (vacío o poke OK). `last_until = until` (el input, no `effective_since`).
- **Fallo de cápsula / I/O de estado:** error; cursor previo intacto.

---

#### 9. Contratos y Empaque

1. **Proceso `email-noise-digest.md`** (root kalma2, `process-contract v1.4.0`, `workspace_template` obligatorio):
   - Inputs: `since`, `until` (RFC3339).
   - Outputs: `events_scanned` (matches), `senders`, `notified`, `skipped` (opcional).
   - Fases documentales (handler nativo las ejecuta; no `agent:`):
     - `Agregacion-Cuarentena`
     - `Notificacion-Digest` (`delegates_to: tool:send-telegram-notification`)
   - Context: `ecosystem-evolution` (precedente `email-quick-action-ingest`).

2. **Códice:** EM `update` de `codex-kalma2-assistant` — `process_membership` += `email-noise-digest`; cuerpo menciona el digest. Hash del creator.

3. **Orquestador (Core, no EM):**
   - `handlers/mod.rs`: `pub mod email_noise_digest;`
   - `engine/mod.rs`: `canonical == "email-noise-digest" => handlers::email_noise_digest::run(...)`

---

#### 10. Slices de Implementación

##### Slice 1 — Agregación y escaneo
- Módulo `email_noise_digest.rs`: `proofs_root` + subdir `email-triaged`.
- Filtro §5. Normalización. Orden §6. Clamp §8.
- Tests con fixtures temporales (desempate count, regla, timestamp, RFC2047 ya decodificado en fixture, `_unknown`).

##### Slice 2 — Idempotencia, truncado, Telegram
- Estado §8. Skip. Vacío ⇒ cero poke + cursor.
- Formateador ≤ 4000. `parse_mode: null`.
- Fallo de cápsula ⇒ no cursor.
- Tests: re-ejecución, solape, vacío, truncado, parse_mode.

##### Slice 3 — Contrato, códice, dispatch
- EM create process (flags de jurisdicción/root/contrato §4).
- EM update códice membership.
- Dispatch `mod.rs`. Suite `cargo test -p execute-process --lib -- email_noise_digest`.

---

#### 11. Criterios de Aceptación

- [ ] **CA-1 (Filtro):** Solo proofs con `timestamp` raíz en `[effective_since, until)`, `kind`/`event_type` canónicos, `payload.verdict=noise`, `decision_path=deterministic`, `matched_rule` ∈ {`C-LIST`,`C-NOREPLY`,`C-SUBJECT-NOISE`}. `preference` / `P-MUTE-SENDER` / `actionable|passive` / `llm` ignorados.
- [ ] **CA-2 (Normalización):** `Shop <noreply@shop.tld>` y `noreply@shop.tld` colapsan a `noreply@shop.tld`. Empate de `count` → `normalized_from` ASC.
- [ ] **CA-3 (Regla y asunto):** Moda de regla (desempate lexicográfico); asunto del `timestamp` más reciente (`event_id` ASC si empate); `"(sin asunto)"` si vacío.
- [ ] **CA-4 (Silencio):** 0 matches → `success: true`, `events_scanned: 0`, `senders: 0`, `notified: false`, cursor actualizado, cero invocación al tool.
- [ ] **CA-5 (Telegram plano):** Matches > 0 → un mensaje; cápsula con `"parse_mode": null`.
- [ ] **CA-6 (Truncado):** Overflow → top K + `+ M remitentes omitidos` + pie; longitud ≤ 4096.
- [ ] **CA-7 (Cursor):** `until <= last_until` → skip. `since < last_until < until` → no recontar `[since, last_until)`. Fallo de poke → cursor intacto.
- [ ] **CA-8 (Empaque):** Proceso en root kalma2 (no `SddIA/process/` ni software-engineering). Membership + índice de ese root. Despachable `execute-process --process email-noise-digest`. Cero fila nueva en `event-domain-subscriptions.json`.
- [ ] **CA-9 (Tests):** `cargo test -p execute-process --lib -- email_noise_digest` cubre filtro, agregación, truncado, skip, clamp y fallo de poke.
- [ ] **CA-10 (Forja):** Genoma process/códice solo vía `entity-manager`. `process_jurisdiction: domain`, `process_domain_root` kalma2, `process_contract_version: 1.4.0`. Handler y tests = Core.

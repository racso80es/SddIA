---
document_id: PBI-EMAIL-DIGEST-PREFERENCE-REPLY
uuid: "bba79183-b76b-4e05-aa02-5adf1d09b500"
title: "[OPERATIVO] Réplica del digest de ruido → preferencias"
format: markdown
version: "1.2.0"
created: "2026-09-06"
updated: "2026-09-12"
status: "propuesta"
refinement_status: refined
priority: media
type: operativo
process: feature
dispatch: false
suggested_branch: feat/email-digest-preference-reply
persist_ref_suggested: docs/features/email-digest-preference-reply
depends_on:
  - PBI-EMAIL-NOISE-HEURISTIC-DIGEST
  - PBI-TELEGRAM-INLINE-KEYBOARD
spawned_by: PBI-EMAIL-TRIAGE-HEURISTIC
related:
  - docs/todos/done/[OPERATIVO] Digest heurístico de ruido de correo (Cuarentena asíncrona).md
  - docs/todos/done/[OPERATIVO] Soporte de Botonera Inline en Telegram (InlineKeyboardMarkup + callback_query).md
  - docs/todos/done/[OPERATIVO] Bucle de Triaje Heurístico y Asimilación de Contexto (Cold-Start).md
  - docs/todos/pending/[NÚCLEO] Intercepción Ontológica y Delegación de Hábitos desde Kalma2.md
  - docs/features/telegram-inline-keyboard/spec.md
  - docs/features/email-noise-heuristic-digest/spec.md
  - docs/features/memoria-preferencias-usuario/spec.md
  - SddIA/events/domain/telegram-callback-received.md
  - SddIA/events/domain/user-preference-change-requested.md
  - SddIA/events/domain/user-preference-changed.md
  - SddIA/process/user-preference-ingest.md
  - SddIA/actions/emit-user-preference-change-requested.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/library/codexes/codex-kalma2-assistant.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-noise-digest.md
  - SddIA/engine/execute-process/src/engine/handlers/email_noise_digest.rs
  - SddIA/engine/execute-process/src/engine/handlers/telegram_gateway.rs
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/engine/execute-process/src/engine/handlers/user_preference.rs
  - SddIA/engine/execute-process/src/engine/user_preference_change_requested.rs
  - SddIA/user-preference-core/src/lib.rs
refinement_notes: >-
  Filtro A v1.2.0 (2026-09-12). Residual sobre v1.1.0: (1) token = 8 hex chars
  del subject_key, no «8 bytes»; (2) botonera por remitente listado, no un par
  global; (3) User_Preference_Change_Requested no declara authority/status —
  ingest los destila de operation=activate; (4) mute exige value.muted=true
  (default ingest sería level:high); (5) ign no va en teclado v1; (6) cursor
  vacío/skip debe preservar tokens; (7) P-EXEMPT-C también acepta high, este
  PBI solo emite max; (8) NÚCLEO-INTERCEPCION no bloquea (NL ≠ botonera).
---

### [OPERATIVO] Réplica del digest de ruido → preferencias

#### 1. Origen y Justificación

El Laudo D2 de `PBI-EMAIL-TRIAGE-HEURISTIC` (cerrado en PR #266, archivado en `docs/todos/done/`) estableció que en cold-start el muro determinista Triaje-C es implacable y que las preferencias humanas no se capturan interrumpiendo al usuario correo a correo, sino de forma asíncrona mediante un informe batch consolidado (`PBI-EMAIL-NOISE-HEURISTIC-DIGEST`, cerrado).

El digest notifica periódicamente al Vértice Biológico a través de Telegram con el resumen de remitentes silenciados por las reglas deterministas (`C-LIST`, `C-NOREPLY`, `C-SUBJECT-NOISE`). Hoy el pie dice «¿Inyectar priority:max? Respuesta en ciclo aparte.» y `email_noise_digest.rs` invoca `send-telegram-notification` **sin** `reply_markup`. `TelegramCallback_Received` existe (PR botonera) con suscripción vacía `[]`.

**Este PBI cierra el lazo de realimentación:**
1. El digest emite botonera inline **por remitente listado** (`InlineKeyboardMarkup`).
2. El watcher ya intercepta `callback_query`, hace ACK de offset, `answerCallbackQuery` y llama a `telegram-gateway` (este PBI **no** muta `telegram-watcher`).
3. `telegram-gateway` sella `TelegramCallback_Received` (payload REQUIRED: `callback_data`, `chat_id`, `source`; OPTIONAL: `message_id`).
4. `route-domain-event` despacha al proceso nuevo `email-digest-preference-reply`.
5. El proceso resuelve el token → `subject_key` SHA-256 y, si la acción persiste, invoca `emit-user-preference-change-requested`.
6. `user-preference-ingest` destila `operation: activate` → `status: active` ∧ `authority: explicit_user` y persiste.
7. El siguiente `Email_Received` del mismo remitente: `email-triage-gateway` evalúa `P-EXEMPT-C` (ya en `email_triage.rs`, PR #266) y omite el muro C.

`NUCLEO-INTERCEPCION-HABITOS-KALMA2` (pending) es otro canal: lenguaje natural → hábito. No bloquea ni se implementa aquí. Prohibido mezclar utterance con `callback_data`.

---

#### 2. Fe de Erratas — Filtro A

##### 2.1 v1.0.0 → v1.1.0 (conservado)

| Elemento v1.0.0 (Borrador) | Inexactitud / Alucinación | Realidad SSOT | Corrección v1.1.0 |
|---|---|---|---|
| Punteros a dependencias en `pending/` | Rutas a botonera/digest pendientes | Ambas en `docs/todos/done/` | `depends_on` + enlaces a `done/` |
| Estado de `P-EXEMPT-C` | «Solo si Slice 1 del padre está implementado» | Activo en `email_triage.rs` (PR #266) | Condicional hipotético eliminado |
| Canal dual «botonera o comando de texto» | Texto libre como fallback de digest | XOR: texto → Kaizen/Manual_Task + `TelegramMessage_Received`; `callback_data` → `TelegramCallback_Received` | Canal canónico = botonera |
| Hash de remitente en `callback_data` | SHA-256 hex 64 chars + prefijo > 64 bytes | `callback_data_ok`: no vacío y `len() ≤ 64` (bytes UTF-8) | Token corto mapeado en estado |
| Salto directo a `User_Preference_Change_Requested` | Omite consumidor de callback | `"TelegramCallback_Received": []`; ingest solo consume `User_Preference_Change_Requested` | Proceso intermedio de dominio |
| Conflación mute / ignore | «mute si ignorar» | `P-MUTE-SENDER` persiste; ignore no escribe | Semánticas separadas |
| Digest «no implementa el informe» | Alcance mal recortado | Digest emite texto plano, sin `reply_markup` ni tokens | Este PBI muta `email_noise_digest.rs` (Core) |

##### 2.2 v1.1.0 → v1.2.0 (esta pasada)

| Elemento v1.1.0 | Inexactitud / Incoherencia | Realidad SSOT | Corrección v1.2.0 |
|---|---|---|---|
| Token = «primeros 8 **bytes** de `subject_key`» | `subject_key` es hex SHA-256 (**64 chars ASCII** = 32 bytes). 8 bytes ≠ 8 hex. | `canonical_subject_key_from_addr` = `hex(SHA-256(UTF-8(normalize_email_addr(from))))` | Token = **8 hex chars** (prefijo). Colisión en el lote → extender a 12 hex. Nunca el hash completo en `callback_data`. |
| Circuito: un par global `[⭐][🔇]` vs Slice 1 «por fila/remitente» | Incoherencia interna. Un token global no desambigua N remitentes. | Digest lista N líneas `sender_line`; truncado a K. Telegram ≤ 100 botones. | Teclado = **2 botones por línea listada** (K remitentes del mensaje, no los omitidos). `2×K ≤ 100`. |
| Payload emit con `authority` y `status` | Campos **no** están en REQUIRED/OPTIONAL de `user-preference-change-requested.md` | `preference_from_event_payload`: `activate` → `Active` + `ExplicitUser`. Ignora `payload.authority`. | Emitir `operation: activate`. No inventar campos ECST. CA-7 verifica el **asiento persistido**, no un campo fantasma. |
| `mute` solo `predicate: mute` | Sin `value`, ingest default `{level: payload.priority_level \|\| "high"}` | `p_mute_sender` exige `predicate=="mute"` ∧ `value.muted==true` | `predicate: mute` **y** `value: {muted: true}` |
| Teclado con acción `ign` en el parser, ausente en botones | Circuito parsea `ign`; UI no la emite | `operation: ignore` en ingest = skip persistencia. No-tap ya es cero escritura. | Teclado v1 = solo `max` \| `mute`. Parser: `ign` → skip **sin emitir** evento. Acción desconocida → fail-closed. |
| `save_state` reescribe el JSON entero | Hoy **no** hay clave `tokens`; ventana vacía/skip machacarían el mapa | `save_state` body fijo: `last_until`, `last_run`, `last_events_scanned`, `last_senders_count`, `last_notified` | Skip: no tocar tokens. Vacío: preservar tokens previos. Notify OK: sustituir mapa por los K listados (+ merge de tokens viejos no colisionados, opcional; mínimo = reemplazo del lote notificado). |
| `P-EXEMPT-C` = solo `level=max` | Matriz y `p_exempt_c`: `max` **o** `high` | Este PBI solo ofrece botón max | Emitir `priority_level: "max"`. No afirmar que `high` quede fuera de la matriz. |
| `default_notify(repo, message)` | Firma actual no transporta markup | `invoke_capsule_json(..., {message, parse_mode: null})` | Extender invocación con `reply_markup`. Tests mock de firma. |
| Pie «ciclo aparte» | Queda mentiroso tras este PBI | `FOOTER` constante en handler | Reemplazar por CTA de botones. Líneas numeradas `N.` para correlacionar con `⭐ N` / `🔇 N`. |
| `sender_key` del digest = email normalizado | Agregación usa `normalize_email_addr`, no el hash | Tokens y ECST usan `canonical_subject_key_from_addr(row.key)` | Mapa token → `{subject_key, rule, created_at}`. Cero addr en estado. |
| Watcher como trabajo de este PBI | Circuito redibuja ACK + `answerCallbackQuery` | Ya implementado en `telegram-watcher` | Fuera de alcance. Cero diff en daemon. |
| `index.md` «y membership» como si EM indexara códice y process en un paso | EM create process actualiza índice del **root**; membership del códice = **update** aparte | Precedente digest: dos sellos EM | Create process + update códice. No «arreglar» el hueco de membership de `email-quick-action-ingest`. |

---

#### 3. Filtro A — Qué NO es este activo

| Tentación / Anti-patrón | Hecho SSOT |
|---|---|
| Usar `TelegramMessage_Received` / texto libre | XOR del gateway. Contaminar el canal dispara fallback LLM. |
| Addr o asunto en `callback_data` / eventos de preferencia | FORBIDDEN `body`/`snippet` en callback y en `User_Preference_Change_Requested`. Digest **sí** muestra addr en el texto (legado); CA-2 no lo revoca. |
| SHA-256 completo en `callback_data` | 64 hex + prefijo viola API y `callback_data_ok`. |
| Suscribir `user-preference-ingest` a `TelegramCallback_Received` | Ingest acoplado a su ECST. No conoce Telegram. |
| Nested `execute-process` desde el handler de réplica | Invocar `user_preference_change_requested::run` (misma semántica que la acción). |
| IMAP `STORE` / borrar correo | Preferencia local futura. |
| Reabrir el lote ya `noise` | Aplica al **siguiente** `Email_Received`. |
| Autoridad `inferred`/`proposed` | `activate` destila `explicit_user`. Matriz: `inferred` no exime C. |
| Mutar `telegram-watcher` | Ya XOR + ACK + answerCallbackQuery. |
| Implementar `NUCLEO-INTERCEPCION-HABITOS-KALMA2` | NL → hábito. Otro PBI. |
| Escribir a mano process/códice bajo `library_codexes` | DA-2: `entity-manager`. Handler Rust = Core, no genoma. |
| Alta en `SddIA/process/` o software-engineering | Root kalma2, como el digest. |

---

#### 4. Circuito de Integración

```
[Timer / CLI de instancia]
   │
   ▼
1. execute-process --process email-noise-digest
   ├─ Agrega remitentes C-* (sin cambio de filtro)
   ├─ Numera las K líneas que caben en el mensaje
   ├─ token = 8 hex de canonical_subject_key_from_addr(row.key); colisión → 12 hex
   ├─ Persiste tokens en {daemons_instance.state}/email-noise-digest.json
   └─ send-telegram-notification:
        message: texto numerado + CTA
        parse_mode: null
        reply_markup.inline_keyboard:
           fila i: [⭐ i] dpref:max:<token> | [🔇 i] dpref:mute:<token>
   │
   ▼
[Vértice pulsa botón]
   │
   ▼
2. telegram-watcher (sin cambio)
   ├─ ACK offset → answerCallbackQuery → telegram-gateway {callback_data, chat_id, message_id}
   │
   ▼
3. telegram-gateway
   └─ write_fractal_event TelegramCallback_Received en eda_fractal.domain
   │
   ▼
4. route-domain-event
   └─ TelegramCallback_Received → email-digest-preference-reply {event_file_path}
      (allowlist route_domain_core.rs junto a email-triage-gateway / email-quick-action-ingest / user-preference-ingest)
   │
   ▼
5. email-digest-preference-reply (nuevo, root kalma2)
   ├─ Gate: callback_data no prefija "dpref:" → skipped (success, preference_emitted=false)
   ├─ Parse: dpref:<action>:<token>  action ∈ {max, mute, ign}
   ├─ Token ausente/expirado → skipped-expired (success o failed documentado; cero emit)
   ├─ ign → skipped, recorded=false, cero emit
   ├─ max → emit activate, channel=telegram, subject_kind=person,
   │        subject_key=<sha256>, predicate=priority, priority_level=max,
   │        scope_type=channel, scope_id=email, source_event_id=<event_id>
   └─ mute → idem con predicate=mute, value={muted: true}  (sin priority_level)
   │
   ▼
6. user-preference-ingest (existente)
   └─ activate → persist Active + ExplicitUser; sella User_Preference_Changed
   │
   ▼
7. siguiente Email_Received → email-triage-gateway
   └─ P-EXEMPT-C si priority max|high explicit_user active → skip muro C → Clasificacion
```

---

#### 5. Protocolo `callback_data` y correlación

##### 5.1 Formato (bytes UTF-8 ≤ 64; objetivo ≤ 22)

`dpref:<action>:<token>`

| Campo | Valor | Bytes típicos |
|---|---|---|
| ns | `dpref` | 5 |
| action | `max` \| `mute` \| `ign` | 3–4 |
| token | 8 hex (12 si colisión en el lote) | 8 o 12 |

Ejemplo: `dpref:max:7f2e1a4b` = 18 bytes. `callback_data_ok` usa `str.len()` (bytes). ASCII only.

##### 5.2 Estado

Path: `{daemons_instance.state}/email-noise-digest.json` vía `state_dir(repo)`.

```json
{
  "last_until": "2026-09-12T00:00:00Z",
  "last_run": "2026-09-12T00:05:00Z",
  "last_events_scanned": 15,
  "last_senders_count": 3,
  "last_notified": true,
  "tokens": {
    "7f2e1a4b": {
      "subject_key": "7f2e1a4b89c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9",
      "rule": "C-NOREPLY",
      "created_at": "2026-09-12T00:05:00Z"
    }
  }
}
```

`tokens` **no** guarda addr ni subject. Skip / ventana vacía: **no borrar** `tokens`. Notify OK: escribir el mapa del lote notificado.

##### 5.3 Texto del digest (evolución del formateador)

```text
Ruido Triaje-C {YYYY-MM-DD}
eventos={n} remitentes={u}
1. {addr} ({count}, {rule}) {subject≤80}
2. …
⭐ N = priorizar (max) · 🔇 N = silenciar
```

Botones solo para las K líneas visibles. Omitidos del truncado → sin botón (no hay token inútil).

---

#### 6. Contrato del proceso `email-digest-preference-reply`

1. **Ubicación:** `SddIA/library/codexes/codex-kalma2-assistant/process/email-digest-preference-reply.md`.
2. **Forja DA-2:** `entity-manager` create process: `process_jurisdiction: domain`, `process_domain_root: SddIA/library/codexes/codex-kalma2-assistant/process`, `process_contract_version: 1.4.0`. Luego EM update `codex-kalma2-assistant` (`process_membership` += nombre; cuerpo). Prohibido default root `[0]`.
3. **Inputs:** `event_file_path` (EDA). Opcional lab: `callback_data`, `chat_id`, `message_id`.
4. **Outputs:** `success`, `skipped` (bool), `reason` (string opcional), `preference_emitted`, `operation`, `target_event_id`.
5. **Fases:**
   - `Gate-Callback` — ns `dpref:` o skipped.
   - `Correlacion-Token` — mapa estado; fail-closed si falta.
   - `Emision-Preferencia` — `user_preference_change_requested::run` o skip (`ign`).
   - `Acuse-Tactico` — opcional fail-soft `send-telegram-notification`; **no** es CA.

---

#### 7. Modificaciones a componentes existentes

1. **`email_noise_digest.rs` (Core):** tokens, markup, pie, numeración, persistencia de `tokens`, pasar `reply_markup` a la cápsula. `email-noise-digest.md` se actualiza vía EM **update** (fase Notificacion-Digest menciona markup).
2. **`event-domain-subscriptions.json` (core, no DA-2):**
   ```json
   "TelegramCallback_Received": [
     {
       "agent": "cumulo",
       "process": "email-digest-preference-reply",
       "intent": "Réplica digest ruido → User_Preference_Change_Requested."
     }
   ]
   ```
3. **`route_domain_core.rs`:** incluir `email-digest-preference-reply` en la rama `event_file_path`.
4. **`engine/mod.rs` + `handlers/mod.rs`:** dispatch nativo `email_digest_preference_reply::run`.

Cero cambio: `telegram-watcher`, clases ECST existentes, matriz (P-EXEMPT-C ya cubre max\|high).

---

#### 8. Slices

##### Slice 1 — Botonera y tokens en `email-noise-digest`

Tokens 8/12 hex. Markup por K líneas. Estado `tokens`. Tests: `callback_data.len() ≤ 64`, cero addr en markup/tokens, skip no borra tokens.

##### Slice 2 — Handler `email-digest-preference-reply`

Módulo nativo + dispatch + allowlist EDA. Gate ns. Token miss → skipped-expired. max/mute emiten; ign no. Forja EM process + códice. Suscripción JSON.

##### Slice 3 — Cierre de lazo `P-EXEMPT-C`

Test integración (tmp repo): digest con `noreply@example.com` → callback `dpref:max:<token>` → ingest → `Email_Received` mismo from → Triaje-C `skipped` reason `P-EXEMPT-C` (no cierra `C-NOREPLY`).

---

#### 9. Criterios de Aceptación

- [ ] **CA-1 (Callback):** Todo `callback_data` del digest ≤ 64 bytes UTF-8, formato `dpref:<max|mute>:<token-hex>`, ASCII. Objetivo ≤ 22 bytes.
- [ ] **CA-2 (Cero PII en tránsito de réplica):** Ni botones, ni `TelegramCallback_Received`, ni `User_Preference_Change_Requested` contienen addr/asunto/cuerpo. El texto del digest puede seguir mostrando addr (legado).
- [ ] **CA-3 (Correlación):** Notify OK escribe `tokens` token → `subject_key` (64 hex). Skip/vacío no borran el mapa previo.
- [ ] **CA-4 (EDA):** `TelegramCallback_Received` → `email-digest-preference-reply` con `event_file_path`.
- [ ] **CA-5 (Coexistencia):** `callback_data` sin prefijo `dpref:` → `skipped`, `preference_emitted: false`, cero emit, exit 0.
- [ ] **CA-6 (Fail-closed):** Token desconocido → `skipped-expired` (o `failed` documentado), cero emit, almacén de preferencias intacto.
- [ ] **CA-7 (Activate max):** `dpref:max:*` emite `operation=activate`, `predicate=priority`, `priority_level=max`, `channel=telegram`. Ingest persiste `authority=explicit_user`, `status=active`, `value.level=max`. Nunca `inferred`/`proposed`.
- [ ] **CA-8 (Mute vs ignore):** `mute` emite `predicate=mute` + `value.muted=true`. `ign` (si llega) y no-tap: cero escritura de preferencias y cero evento de cambio.
- [ ] **CA-9 (P-EXEMPT-C):** Tras asiento max del remitente `noreply@`, un `Email_Received` posterior no cierra por `C-NOREPLY`; fase Triaje-C `skipped` / `P-EXEMPT-C`.
- [ ] **CA-10 (DA-2):** Process + update códice solo vía `entity-manager` (jurisdicción domain / root kalma2 / contrato 1.4.0). Handler y tests = Core.
- [ ] **CA-11 (Tests):** `cargo test -p execute-process --lib -- email_noise_digest` y `-- email_digest_preference_reply` verdes. CA-CI GitHub = `PENDIENTE-CI` hasta `run_id` verde; `global: APTO` solo entonces.

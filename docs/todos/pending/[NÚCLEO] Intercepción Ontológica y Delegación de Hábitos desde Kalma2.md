---
document_id: PBI-NUCLEO-INTERCEPCION-HABITOS-KALMA2
uuid: "8f3d1b22-6b9c-4e89-a512-9c3e4f7a1102"
title: "[NÚCLEO] Intercepción Ontológica y Delegación de Hábitos desde Kalma2"
format: markdown
version: "1.2.0"
created: "2026-09-07"
updated: "2026-09-12"
status: "propuesta"
refinement_status: refined
priority: "alta"
type: nucleo
process: feature
dispatch: false
suggested_branch: feat/intercepcion-habitos-kalma2
persist_ref_suggested: docs/features/intercepcion-habitos-kalma2
spawned_by: PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA
depends_on:
  - PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
  - PBI-NUCLEO-AIUA-ANATOMIA-MOTORA-EDA
  - PBI-PREF-STORE-LANCEDB-MIGRATION
  - PBI-EMAIL-DIGEST-PREFERENCE-REPLY
blocks_on: []
related:
  - SddIA/conscience/aiua_core.md
  - SddIA/process/aiua-stimulus-processing.md
  - SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs
  - SddIA/engine/execute-process/src/engine/aiua_intent.rs
  - SddIA/engine/execute-process/src/engine/user_preference_change_requested.rs
  - SddIA/actions/dispatch-aiua-intent.md
  - SddIA/actions/emit-user-preference-change-requested.md
  - SddIA/events/domain/user-preference-change-requested.md
  - SddIA/events/domain/user-preference-changed.md
  - SddIA/process/user-preference-ingest.md
  - SddIA/engine/execute-process/src/engine/handlers/user_preference.rs
  - SddIA/user-preference-core/src/lib.rs
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/daemons/email-watcher.md
  - docs/features/memoria-preferencias-usuario/spec.md
  - docs/todos/done/[NÚCLEO] Puente Perceptivo: Interacción Biológica con Tormentosa desde Kalma2 WUI.md
  - docs/todos/done/[NÚCLEO] Anatomía Motora de Aiúa — Inyección de Capacidades (Function Calling) y Orquestación EDA.md
  - docs/todos/done/[OPERATIVO] Réplica del digest de ruido → preferencias.md
  - docs/todos/done/[ARQUITECTURA] Migración del Store de Preferencias de Usuario a LanceDB.md
architectural_constraints:
  - L-ONTOLOGY-SPLIT
  - L-FAIL-POLICY
  - L-TOMBSTONE
  - L-CUMULO-PATH
  - L-NO-DLT-VALUE
  - inmutabilidad-imap-read-only
  - filtro-materializacion-sin-terminal
  - events-contract v1.1.0
  - process-contract v1.4.0
refinement_notes: >-
  Filtro A v1.2.0 (2026-09-12) sobre v1.1.0. Hallazgos verificados contra genoma:
  (A) PR #285 no es anatomía motora (ese PR es kalma2-wui-agy-network-sanitize);
  anatomía motora = PR #286 / docs/features/aiua-motor-anatomy-eda;
  (B) Filtro de Materialización (aiua_core.md §4) = Intención ≠ Ejecución; no es la
  matriz borrar→mute (eso es Filtro A + matriz defensiva del despachador);
  (C) user-preference-ingest no invoca Mayeuta; destilación nativa determinista;
  (D) canonical_subject_key_from_hint no existe; preference_from_event_payload usa
  subject_hint como subject_key en claro y hace fallback ilegal de subject_kind al hint;
  (E) emisores autorizados del ECST = kalma2-bridge y emit-user-preference-change-requested;
  CA-4 v1.1 exigía emitter_agent aiua-stimulus-processing (fuera de contrato);
  (F) P-MUTE-SENDER consulta solo canonical_subject_key_from_addr(from); un hábito
  subject_kind=topic / hint=computrabajo no silencia el correo; CA-8 v1.1 era falso;
  (G) authority/status no son campos del ECST; activate → Active + ExplicitUser en ingest;
  (H) dual-run JSON+LanceDB ya existe en put_revision_durable; no reimplementar;
  (I) C-* concluye antes que P-MUTE-SENDER (list_headers_beat_mute); CA-8 debe usar From
  que no dispare C-NOREPLY/C-LIST/C-SUBJECT-NOISE.
---

### [NÚCLEO] Intercepción Ontológica y Delegación de Hábitos desde Kalma2

#### 1. Origen y Visión Ontológica

El ecosistema SddIA tiene canal aferente Kalma2 WUI → `POST /api/aiua/interact` → `aiua-stimulus-processing` (PR #276) y anatomía motora por tendones `aiua-intent` (PR #286, `docs/features/aiua-motor-anatomy-eda`). El subsistema de preferencias (`user-preference-ingest`, `user-preference-core`, dual-run JSON SSOT + réplica LanceDB vía `put_revision_durable`) ya opera para emisores estructurados (`kalma2-bridge`, réplica digest).

Vacío real: el latido Aiúa no tiene tendón motor que transforme una directiva en lenguaje natural (*"borra los correos de computrabajo"*, *"silencia las alertas de X"*) en `User_Preference_Change_Requested` sin mutación IMAP y sin RPC síncrono.

**Canal canónico (v1.2.0):**

1. **Entrada:** Kalma2 → `aiua-stimulus-processing`. Aiúa contempla (Filtro de Materialización: no ejecuta). Si hay voluntad de hábito, emite un bloque `aiua-intent` con `name: delegar_habito`.
2. **Aduana defensiva (despachador, no LLM):** `dispatch-aiua-intent` aplica la matriz no-destructiva. Verbos de borrado (`borra`, `elimina`, `limpia`, `delete`, `expunge`) se fuerzan a `predicate_hint: mute` + `value: {"muted": true}`. `raw_utterance` no entra al ECST (FORBIDDEN: utterance completa).
3. **Despacho EDA:** el tendón **reutiliza** el handler nativo `emit-user-preference-change-requested` (`user_preference_change_requested::run`). Emisor autorizado intacto. `payload.channel: kalma2`. `operation: activate` (o `revoke` si la utterance lo pide). No se inventa un emisor `aiua-stimulus-processing` para esta clase.
4. **Ingesta:** `event-watcher` → `route-domain` → `user-preference-ingest` (handler nativo; **sin Mayeuta**). Destila hints, hashea `subject_hint` a `subject_key`, persiste vía `put_revision_durable`, sella `User_Preference_Changed`.
5. **Efecto periférico (tiempo posterior, DA-5):** `email-triage-gateway` ante `Email_Received` consulta JSON SSOT. Si alguna clave candidata del From/asunto coincide con un mute activo → `verdict: noise`, `matched_rule: P-MUTE-SENDER`. IMAP inalterado.

---

#### 2. Fe de Erratas

##### 2.1 v1.0.0 → v1.1.0 (retenido)

| # | Alucinación v1.0.0 | Corrección |
|---|---|---|
| 1 | Tokens scraper `[span_N]` | Purgados |
| 2 | Primera persona | Tercera persona técnica |
| 3 | Evento «Chispa Simbiótica de Nivel 2» | `User_Preference_Change_Requested` |
| 4 | Cúmulo escribe LanceDB | Cúmulo = SSOT de rutas; persistencia = `UserPreferenceStore` |
| 5 | Borrado IMAP autónomo | IMAP read-only; «borrar» → mute |
| 6 | «Vectores en LanceDB» | Entidad `UserPreference` + dual-run |
| 7 | RPC Tormentosa → Mayeuta | Tendón `aiua-intent` desacoplado |
| 8 | «Ceguera Espacial» = impulsos ciegos | Resolución de rutas vía Cúmulo |
| 9 | Táctica del Refugio como aduana de hábitos | Política eferente Telegram; no gate de persistencia |

##### 2.2 v1.1.0 → v1.2.0 (esta pasada)

| # | Inexactitud v1.1.0 | Evidencia SSOT | Corrección v1.2.0 |
|---|---|---|---|
| A | PR #285 = anatomía motora | `docs/fixes/kalma2-wui-agy-network-sanitize/validacion.md` = PR #285; anatomía motora = PR #286 | Citar #276 (puente) y #286 (tendones) |
| B | Filtro de Materialización traduce «borrar»→mute | `aiua_core.md` §4: Intención ≠ Ejecución | Matriz borrar→mute = Filtro A + despachador; FdM = Aiúa no tiene extremidades |
| C | Ingest «normaliza hints con Mayeuta» | `user-preference-ingest.md`: fases Gate/Destilar/Persistir/Sellar, handler nativo | Cero agente Mayeuta en este circuito |
| D | Función existente `canonical_subject_key_from_hint` | Solo existe `canonical_subject_key_from_addr`. `preference_from_event_payload` copia `subject_hint` a `subject_key` y usa el hint como `subject_kind` si falta el kind | **Nueva** función de hash; arreglar destilación |
| E | `emitter_agent: aiua-stimulus-processing` en CA-4 | Clase: emisores `kalma2-bridge` y `emit-user-preference-change-requested`. SDLC sí usa `aiua-stimulus-processing` para `Aiua_Process_Requested`, otra clase | Reusar emit canónico |
| F | `subject_kind: topic` + hint `computrabajo` activa P-MUTE-SENDER | `email_triage.rs` consulta `canonical_subject_key_from_addr(from)` | Default email-mute: `subject_kind: person`. Triaje expande candidatos (addr, dominio, etiquetas, tokens de display/asunto ≥3) y hashea cada uno |
| G | ECST lleva `authority`/`status` | FORBIDDEN/OPTIONAL de la clase no los declara; ingest deriva de `operation` | Emitir `operation: activate`; no inventar campos |
| H | Slice 2 «garantizar dual-run» como trabajo nuevo | `put_revision_durable` ya replica a LanceDB si la tabla existe | Reusar; CA-6 = regresión del camino nuevo |
| I | CA-8 con From `noreply@…` | Triaje-C (`C-NOREPLY`) gana a P-MUTE-SENDER | Fixture sin reglas C concluyentes |

---

#### 3. Filtro A — Qué NO es este activo

| Anti-patrón | Restricción SSOT |
|---|---|
| Mutación IMAP (`STORE \Deleted`, `EXPUNGE`, `APPEND`) | Prohibido. `email-watcher` = `EXAMINE` + `BODY.PEEK`. |
| Terminal / tools `agy` / Write desde Aiúa | Prohibido (`aiua_core.md` §3–§4, §6). Salida = `aiua-intent`. |
| Eventos ad-hoc («chispas») | Prohibido. Solo clases ECST catalogadas. |
| `subject_key` en claro (addr o hint) | Prohibido en el asiento persistido. Hash SHA-256. |
| Nuevo emisor fuera de la clase | Prohibido. Reusar `emit-user-preference-change-requested`. |
| `raw_utterance` / `body` / `snippet` en el ECST | FORBIDDEN. Opcional: `utterance_ref` = hash corto. |
| RPC síncrono Aiúa → ingest → triaje | Prohibido (DA-5). El latido acaba al sellar el ECST. |
| Reimplementar store LanceDB | Prohibido. `put_revision_durable` es el chokepoint. |
| Confundir FdM con matriz mute | FdM no clasifica correo; el despachador sí. |

---

#### 4. Circuito End-to-End

```
1. Kalma2 POST /api/aiua/interact { prompt }
2. aiua-stimulus-processing
   retrieve-active-context → invoke-aiua-core → antigravity-cli-executor
   ```aiua-intent
   {"name":"delegar_habito","args":{
     "subject_hint":"computrabajo","subject_kind":"person",
     "predicate_hint":"mute","operation":"activate",
     "scope_type":"channel","scope_id":"email",
     "raw_utterance":"borra los correos de computrabajo"
   }}
   ```
   dispatch-aiua-intent:
     - is_motor_tendon("delegar_habito") = true
     - matriz defensiva → predicate mute + value {muted:true}
     - llama user_preference_change_requested::run
     - HTTP 200; no espera ingest (DA-5)
3. user-preference-ingest (async, bus)
     Gate → Destilar (hash hint) → Persistir (put_revision_durable) → Sellar
4. Tiempo posterior: Email_Received
     email-triage-gateway:
       claves = hash(addr) ∪ hash(tokens From/asunto)
       mute activo → noise / P-MUTE-SENDER
       IMAP intacto
```

Lab: overlay `SDDIA_LAB_MOCK_AIUA_INTENT` (ya existe). No exigir combustión LLM real en CI.

---

#### 5. Tendón `delegar_habito`

##### 5.1 Genoma `SddIA/conscience/aiua_core.md` §6

| Tendón | Motor | Destino |
|---|---|---|
| `delegar_habito` | Sí | `User_Preference_Change_Requested` vía acción `emit-user-preference-change-requested` |

##### 5.2 Args

```json
{
  "name": "delegar_habito",
  "args": {
    "subject_hint": "computrabajo",
    "subject_kind": "person",
    "predicate_hint": "mute",
    "priority_level": null,
    "operation": "activate",
    "scope_type": "channel",
    "scope_id": "email",
    "raw_utterance": "borra los correos de computrabajo"
  }
}
```

Defaults si omitidos: `subject_kind=person`, `predicate_hint=mute`, `operation=activate`, `scope_type=channel`, `scope_id=email`. `subject_hint` obligatorio.

##### 5.3 Matriz defensiva (despachador; invariante aunque el LLM alucine `delete`)

| Expresión / hint | `predicate_hint` | `value` | `operation` |
|---|---|---|---|
| borra / elimina / limpia / delete / expunge / destroy | `mute` | `{"muted": true}` | `activate` |
| ignora / silencia / no me avises | `mute` | `{"muted": true}` | `activate` |
| prioriza / urgente / máxima prioridad | `priority` | `{"level": "max"}` | `activate` |
| importante / relevante | `priority` | `{"level": "high"}` | `activate` |
| reactiva / vuelve a avisarme | `mute` | `{"muted": false}` | `revoke` |

---

#### 6. Adaptaciones de runtime (no genoma DA-2 salvo acciones)

##### 6.1 `aiua_intent.rs`

- `const HABIT_TENDON: &str = "delegar_habito"` en `is_motor_tendon`.
- Rama en `run` distinta de SDLC (`goal`/`target_component` no aplican).
- Sanitizar: no copiar `raw_utterance` al payload ECST; `utterance_ref` opcional = SHA-256 hex del utterance.
- Invocar `user_preference_change_requested::run` con `operation`, `channel: "kalma2"`, payload conforme a la clase.
- Tras sellar, `validate_ecst_event` sobre la instancia (o construir el evento validable antes de escribir).
- Tests unitarios: extracción, matriz borra→mute, rechazo de `raw_utterance` en payload, `is_motor_tendon`.

##### 6.2 `user-preference-core` — destilación

Nueva función (no existía):

`canonical_subject_key_from_hint(hint)` = `hex(SHA-256(UTF-8(normalize_hint(hint))))`

`normalize_hint`: trim, ASCII/Unicode lowercase, colapsar whitespace interno a un espacio.

`preference_from_event_payload`:

- `subject_kind` **solo** de `payload.subject_kind`; default `"person"`. Prohibido fallback a `subject_hint`.
- `subject_key`: si `payload.subject_key` es SHA-256 hex (64 `[0-9a-f]`), usarlo; si no, hashear `subject_key` o `subject_hint` con `canonical_subject_key_from_hint`. Error si ambos vacíos.
- Si `predicate == mute` y `value` carece de `muted`, default `{"muted": true}` (hoy el default `{level: high}` rompería P-MUTE-SENDER).

##### 6.3 `email_triage.rs` — candidatos de clave

Además de `canonical_subject_key_from_addr(from)`:

Candidatos de texto (From display, local-part, labels de dominio salvo TLD de 2–3 letras, tokens alfanuméricos del asunto con longitud ≥ 3), cada uno hasheado con `canonical_subject_key_from_hint`. Unión de `query` por esas claves. `p_mute_sender` no cambia de semántica (mute activo + `value.muted=true`).

Orden L-ORDER intacto: Triaje-P → P-EXEMPT-C → C → P-MUTE-SENDER → Clasificacion. C concluyente sigue ganando a mute.

##### 6.4 `dispatch-aiua-intent.md` (DA-2 → `entity-manager` update)

Documentar el tendón y la delegación a `emit-user-preference-change-requested`. El artefacto actual está truncado en el índice; la forja restaura cuerpo completo + hábito.

`aiua_core.md` (conscience, fuera de DA-2) se edita tras topología `docs/features/intercepcion-habitos-kalma2`.

No mutar la clase del evento salvo que el emit reutilizado deje de validar; el invocador nuevo no es emisor ECST.

---

#### 7. Touchpoints

| Path | Vía | Cambio |
|---|---|---|
| `SddIA/conscience/aiua_core.md` | edición directa post-feature | Fila tendón §6 |
| `SddIA/actions/dispatch-aiua-intent.md` | `entity-manager` update | Documentar `delegar_habito` |
| `SddIA/engine/execute-process/src/engine/aiua_intent.rs` | código | Tendón + despacho + tests |
| `SddIA/user-preference-core/src/lib.rs` | código | Hash hint + destilación + tests |
| `SddIA/engine/execute-process/src/engine/handlers/email_triage.rs` | código | Candidatos de clave + test CA-8 |
| `SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs` | tests | Overlay `delegar_habito`; no join a ingest |
| `SddIA/evolution/` | `sddia-qa evolution-register` | Hito |

Fuera de alcance: Kalma2 WUI, `email-watcher` IMAP, reescritura de LanceDB, nuevos eventos, Mayeuta.

---

#### 8. Slices

##### Slice 1 — Genoma + parser

- `aiua_core.md` §6 + `is_motor_tendon`.
- Matriz defensiva y builder de payload.
- Tests `aiua_intent.rs`.

##### Slice 2 — Emit + destilación

- Rama `delegar_habito` → `user_preference_change_requested::run`.
- Hash de hint + defaults mute.
- Test: evento de hábito recorre Gate/Destilar/Persistir/Sellar; `subject_key` es hash, no plaintext.

##### Slice 3 — Triaje reactivo

- Candidatos de clave en `email_triage.rs`.
- Test: preferencia hasheada desde hint `computrabajo` + `Email_Received` From `Jobs <alertas@computrabajo.com>` (sin C-*) → `noise` / `P-MUTE-SENDER`.
- Test overlay estímulo: despacho no invoca ingest (DA-5).
- Cero comandos IMAP en el diff.

---

#### 9. Criterios de Aceptación

- [ ] **CA-1:** `aiua_core.md` §6 lista `delegar_habito` como motor hacia `User_Preference_Change_Requested` vía `emit-user-preference-change-requested`.
- [ ] **CA-2:** `resolve_intent` / `extract_aiua_intent` extraen `name: delegar_habito`. `is_motor_tendon` es true.
- [ ] **CA-3:** Args con `raw_utterance` que contiene borra/elimina/limpia/delete producen `predicate: mute` y `value.muted=true`. Cero rama IMAP/destructiva en el despachador.
- [ ] **CA-4:** El ECST sellado es `User_Preference_Change_Requested` válido (`validate_ecst_event`), `payload.channel=kalma2`, `emitter_agent=emit-user-preference-change-requested`. Payload sin `raw_utterance`/`body`/`snippet`.
- [ ] **CA-5:** Ingest resuelve `subject_hint` a `subject_key` SHA-256 hex de 64 chars; el asiento no guarda el hint en claro como `subject_key`.
- [ ] **CA-6:** `operation: activate` persiste `status: active`, `authority: explicit_user` vía `put_revision_durable` (JSON SSOT; réplica LanceDB si la tabla existe). No campos `authority`/`status` en el ECST.
- [ ] **CA-7:** Diff y tests de `email-watcher` / triage: cero `EXPUNGE` / `STORE \Deleted`. Invariante IMAP read-only.
- [ ] **CA-8:** Hábito mute con hint `computrabajo` + `Email_Received` From humano/marca `alertas@computrabajo.com` (sin C concluyente) → `verdict=noise`, `matched_rule=P-MUTE-SENDER`.
- [ ] **CA-9:** `aiua-stimulus-processing` retorna tras el emit; no llama `user-preference-ingest` ni triage.
- [ ] **CA-10:** `cargo test -p execute-process -p user-preference-core` verde (kalma2-bridge solo si el diff lo toca; no está en el alcance).

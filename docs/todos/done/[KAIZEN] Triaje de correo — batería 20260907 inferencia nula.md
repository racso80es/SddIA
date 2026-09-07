---
document_id: PBI-KAIZEN-EMAIL-TRIAGE-BATCH-20260907
uuid: "b4b41c88-d90b-47fc-bf18-94efd37f2298"
title: "[KAIZEN] Triaje de correo — batería 2026-09-07: inferencia nula y ceguera semántica"
format: markdown
version: "1.2.0"
created: "2026-09-07"
updated: "2026-09-07"
status: done
refinement_status: implemented
persist_ref: docs/features/kaizen-email-triage-infer-20260907
pr_url: "https://github.com/racso80es/SddIA/pull/268"
priority: alta
type: kaizen
process: feature
dispatch: false
suggested_branch: feat/kaizen-email-triage-infer-20260907
persist_ref_suggested: docs/features/kaizen-email-triage-infer-20260907
audit_instance: /home/racso/Proyectos/SddIA
audit_at: "2026-09-07T14:20:16Z/2026-09-07T14:26:22Z"
systemd_unit: "sddia-email-watcher@home-racso-Proyectos-SddIA.service"
wui_port: 8765
derived_from:
  - PBI-EMAIL-TRIAGE-HEURISTIC
  - PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
friction_ids:
  - F-TRIAGE-02
  - F-TRIAGE-03
  - F-TRIAGE-04
  - F-TRIAGE-05
  - F-TRIAGE-06
tech_debt_ids:
  - DT-TRIAGE-LLM-QUALITY
depends_on: []
blocks_on: []
spawned_by: null
related:
  - SddIA/library/norms/email-triage-matrix.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-triage-gateway.md
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/engine/execute-process/src/engine/telemetry_receipt.rs
  - SddIA/engine/execute-process/src/engine/capsules.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/skills/mayeuta-llm.md
  - SddIA/skills/mayeuta-llm/src/main.rs
  - SddIA/daemons/email-watcher.md
  - SddIA/events/domain/email-received.md
  - SddIA/events/domain/email-triaged.md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - docs/todos/done/[OPERATIVO] Bucle de Triaje Heurístico y Asimilación de Contexto (Cold-Start).md
  - docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md
  - docs/todos/pending/[OPERATIVO] Digest heurístico de ruido de correo (Cuarentena asíncrona).md
  - docs/todos/pending/[OPERATIVO] Réplica del digest de ruido → preferencias.md
  - docs/todos/kitchen/PBI-MULTI-LLM-ROUTER.md
  - docs/features/kaizen-email-triage-infer-20260907/baterias-correo-kalma2.md
refinement_notes: >-
  Filtro A v1.2.0 (2026-09-07). Corrige alucinaciones v1.1.0: UUIDs de proofs
  5816/5817/5818 y G5 sintético no existían en disco; 5816 no es «Aigües de
  Barcelona» (no verificado en .eml); F-TRIAGE-06 no es agenda-manager (asiento
  inline persist_agenda en email_triage.rs). A1 no es incondicional: 5819 no
  marca classification-degraded por L-GUARD. Dos defectos apilados: (1) mismatch
  tokens raíz vs data.telemetry_receipt; (2) classify_llm no lee exit_code/success
  de CapsuleInvokeResult (invoke_capsule_json devuelve Ok también si la cápsula
  falla). SDDIA_LLM_INFER_COMMAND no lo lee mayeuta-llm (solo CHAT/CLI).
---

# [KAIZEN] Triaje de correo — batería 2026-09-07: inferencia nula y ceguera semántica

## 0. Propósito y Mandato

Capturar y erradicar las **fricciones empíricas** registradas en el lote de seis correos de prueba (2026-09-07, 16:20–16:26 CEST) contra el circuito productivo de la forja, asegurando que el triaje cumpla de forma medible con la norma `email-triage-matrix` v1.1.0.

**Done de este PBI (cuando se ejecute y despache):**
1. **Peaje cognitivo medible:** Inferencia real con `thermodynamic_cost.tokens_in + tokens_out > 0` en correos ambiguos (sin regla determinista C-\* ni L-GUARD de asunto) en presencia de un proveedor LLM activo, **o** emisión explícita de fallo/degradación justificada si el CLI de inferencia colapsa (eliminando la deglución silenciosa de errores en `classify_llm`).
2. **Corrección del contrato de telemetría:** El handler nativo `email_triage.rs` extrae los tokens de telemetría conforme al contrato de `skill:mayeuta-llm` (`data.telemetry_receipt`), eliminando la ceguera de `tokens_in`/`tokens_out` en la raíz del envelope.
3. **Triaje de ruido estanco:** Los fixtures de boletines cierran como `noise` por la vía que corresponda: `C-LIST` determinista si portan cabeceras RFC estándar, o semántica LLM si C-\* no concluye.
4. **Respeto a laudo D3:** Actos con fecha/hora no se pierden en fail-open `passive` cuando el LLM está activo y extrae `datetime` del snippet; correos sin fecha verificable permanecen en `passive` sin invención de marcas temporales.

**Límites estrictos:**
- **No es** ola de redeploy de Paciente 0 (no altera scripts de despliegue ni variables de ignición).
- **No reabre** incidencias de infraestructura F-DEP-\* ni fracturas F-SYS-\*.
- **No muta** el buzón IMAP (prohibido `STORE`, `EXPUNGE` o marcado como leído según norma §5).

---

## 0bis. Hallazgos y Correcciones Filtro A (v1.0.0 → v1.2.0)

Contraste contra SSOT (`email_triage.rs`, `mayeuta-llm/src/main.rs`, `telemetry_receipt.rs`, `capsules.rs`, proofs en disco 2026-09-07):

| # | Afirmación | Veredicto SSOT | Hecho |
|---|------------|----------------|-------|
| **A1** | «tokens siempre 0 ⇒ degradado incondicional aunque el LLM infiera» | **Parcial.** Bug real de consumidor; no incondicional | `classify_llm` lee `body.get("tokens_in")` / `tokens_out` en la **raíz**. `mayeuta-llm` `emit()` pone `data.telemetry_receipt.{prompt_tokens,completion_tokens}`. `extract_from_capsule_body` **sí** mira `data.telemetry_receipt`; el handler no lo usa. `classification-degraded` solo si `SDDIA_LLM_REQUIRE_INFER` ∧ peaje 0 ∧ **sin** L-GUARD. UID 5819: tokens 0 **sin** el flag (`subject_elevation`). |
| **A2** | «duration_ms=0 = CLI muerto; deglución» | **Dos defectos apilados** | `invoke_capsule_json` retorna `Ok(CapsuleInvokeResult)` también con `exit_code != 0` / `success: false` (solo `Err` si no resuelve/spawn). `classify_llm` toma `Ok(r) => r.body` y **no** mira `r.exit_code`. Texto vacío → `passive`. `duration_ms=0` en 5814–5818 ⇒ subproceso <1 ms (CLI ausente/fail-fast o cápsula `emit(false)`). `resolve_cli_raw` ignora `SDDIA_LLM_INFER_COMMAND` (solo `SDDIA_LLM_CHAT_COMMAND` ≻ `SDDIA_LLM_CLI_COMMAND`). Corregir envelope **no** basta si el CLI no corre. |
| **A8** | UUIDs proofs 5816/5817/5818 en v1.1.0 | **Alucinación** | En disco: `678370e9-c1dc-4015-992a-7fd002fbe07e`, `b9221595-7835-443b-bdf8-dfa5490c898b`, `66da9fab-d3c2-4211-af0b-33ae5785cdc2`. Los sufijos v1.1.0 no existen. |
| **A9** | Proof G5 `6e552199-65b1-…` | **Alucinación de instancia** | Canónico histórico: `6e552199-416d-4043-91c9-fb1bae9e2057` (PBI Kaizen Paciente 0, instancia `SddIA_AP`). **Ausente** en proofs de esta forja. |
| **A10** | «5816 Aigües de Barcelona» | **No verificado** | Fixture: domicilio PT.FABRIQUETA 16. No afirmar operador. |
| **A11** | F-TRIAGE-06 = `agenda-manager` | **Inexacto** | Skill `agenda-manager` es pivote DI `agenda:persist`. El asiento lo escribe `persist_agenda` **inline** en `email_triage.rs` (`.SddIA/agenda/{id}.json`). Cero invocación de la cápsula en el lote. |
| **A3** | «F-TRIAGE-02 está resuelta con `classification-degraded`» | **Falso éxito** | El flag documenta que la inferencia fue nula, pero no resuelve la causa. El lote 2026-09-07 disparó degradación en 5 de 6 mensajes. La deuda `DT-TRIAGE-LLM-QUALITY` sigue abierta y requiere corrección en la extracción de telemetría y diagnóstico del CLI. |
| **A4** | «Ampliar L-GUARD a facturas / documentación / empleo» | **Infracción de Laudo D3** | Laudo D3 de `PBI-EMAIL-TRIAGE-HEURISTIC`: `actionable` = acto concreto + `title` + `datetime` extraíbles. Queda terminantemente prohibido introducir listas de palabras clave comerciales (`factura`, `pago`, `plazo`, `computrabajo`) en el código de L-GUARD. |
| **A5** | «Parchear C-SUBJECT-NOISE con marcas (Codely, Computrabajo)» | **Entropía semántica** | El muro determinista C-\* opera sobre estándares RFC (`List-*`, `noreply@`) o patrones estables de desuscripción. Las marcas son volátiles; el ruido de boletín debe canalizarse por `C-LIST` (cabeceras reales) o por clasificación semántica del LLM. |
| **A6** | «F-TRIAGE-03 (WUI oculta passive) bloquea el gate» | **Desacople UX / Dominio** | La visualización en el bridge Kalma2 (`GET /api/email-inbox`) es una decisión de presentación de interfaz. No bloquea el pipeline de triaje ni la verificación del peaje cognitivo. |
| **A7** | «F-TRIAGE-06 (agenda pasada y dedup) en este gate» | **Fuera de alcance** | La fecha pasada (`28/08/2026`) en el UID 5819 es un defecto de calidad en el parser de agenda o en la deduplicación de estímulos, no de la aduana de triaje de correo. Se registra para tratamiento en slice posterior. |

---

## 1. Circuito Observado (Instancia Forja)

Topología operativa observada el 2026-09-07 16:20–16:26 CEST:

```text
IMAP EXAMINE + BODY.PEEK  (email-watcher, poll 60s, last_uid=5819)
  → Email_Received  (eda_fractal.domain; .eml persistido en .SddIA/inbox/{uid}.eml)
  → email-triage-gateway
       Triaje-P (fail-open) → Triaje-C (muro C-*) → Mute P
         → Clasificacion (mayeuta-llm SYNTHESIZE)
         → Elevación estructural (L-GUARD sobre asunto)
         → Asiento-Agenda (agenda:persist si actionable)
         → Email_Triaged (eda_fractal.domain + proof durable)
  → route_domain_core:
       - Poke Telegram: ÚNICAMENTE si verdict == actionable
       - GET /api/email-inbox: ÚNICAMENTE lista top 20 actionable
```

### 1.1 Estado de los Componentes en la Batería

| Pieza | Estado 2026-09-07 | Observaciones |
|-------|-------------------|---------------|
| `sddia-email-watcher@home-racso-Proyectos-SddIA` | `active (running)` | Watermark alcanzado: `last_uid=5819` `updated_at=2026-09-07T14:26:21Z`. |
| Ingestión física | APTO | 6 archivos `.eml` en `.SddIA/inbox/581{4..9}.eml`. Cero `IMAP STORE` (mensajes permanecen intactos en buzón). |
| Proofs durables | APTO | Generados en `.SddIA/proofs/email-triaged/{event_id}.json`. |
| Bus de eventos | APTO | `Email_Received` seguido de `Email_Triaged` en ≤2 segundos por cada mensaje. |
| Interfaz WUI (`:8765`) | Parcial | Solo muestra UID 5819 (diseño actual filtra `passive` y `noise`). |
| Bóveda de inferencia | Presente | `SDDIA_LLM_CLI_COMMAND`, `SDDIA_LLM_INFER_COMMAND`, `SDDIA_LLM_REQUIRE_INFER=1` inyectados en runtime. |

---

## 2. Lote Empírico (Seis Fixtures, Mismo Remitente)

Remitente táctico: `OSCAR PEREZ <racso80es@gmail.com>` → destinatario `oscar.perez.garcia.bcn@gmail.com`.
Al ser reenvíos desde buzón personal, **carecen de cabeceras RFC** (`List-Id`, `List-Unsubscribe`, `Precedence: bulk`). El muro determinista `C-LIST` no concluyó en ninguno.

| UID | Asunto | Envío CEST | Proof ID | Veredicto | Path | Extra Flags | Latencia / Coste |
|-----|--------|------------|----------|-----------|------|-------------|-------------------|
| 5814 | Factura 08 agosto 2026 | 16:20:00 | `d5305406-6021-40e6-a646-4f6bb88a334a` | passive | llm | `classification-degraded` | duration: 0 ms, tok: 0/0 |
| 5815 | DOCUMENTACIÓN 2T 2026 - OSCAR A. PEREZ | 16:23:15 | `45c1f26a-a954-4836-99de-a8172e05c8c9` | passive | llm | `classification-degraded` | duration: 0 ms, tok: 0/0 |
| 5816 | Ya tienes disponible tu factura de agua | 16:23:49 | `678370e9-c1dc-4015-992a-7fd002fbe07e` | passive | llm | `classification-degraded` | duration: 0 ms, tok: 0/0 |
| 5817 | Nuevos puestos que se ajustan a tu experiencia | 16:24:22 | `b9221595-7835-443b-bdf8-dfa5490c898b` | passive | llm | `classification-degraded` | duration: 0 ms, tok: 0/0 |
| 5818 | ¡Ya se puede crear un Event Bus con Kafka! + Fable 5.1… | 16:24:53 | `66da9fab-d3c2-4211-af0b-33ae5785cdc2` | passive | llm | `classification-degraded` | duration: 0 ms, tok: 0/0 |
| 5819 | Reunión con Racso el 28/08/2026 a las 18:44 | 16:25:25 | `e215ebf7-bb18-4330-aab2-61e8e50be071` | **actionable** | llm | `subject_elevation`, agenda `2b49036a-…` | duration: 12 ms, tok: 0/0 |

### 2.1 Semántica Real del Contenido vs Resultado Observado

- **UID 5814:** Factura mensual + aviso de cobro pendiente. Sin fecha concreta extraíble → `passive` es ontológicamente correcto, pero la causa fue peaje 0 (degradado).
- **UID 5815:** Hilo fiscal gestoría BSC Assessors: entrega de documentación 2T antes del día 15; cargo 63,00 € previsto 20/07/2026. Al no tener formato de reunión en asunto, L-GUARD no operó; al fallar el LLM, cayó a `passive` degradado sin intentar extracción del cuerpo.
- **UID 5816:** Factura de agua (PT.FABRIQUETA 16; 39,86 €; fecha 04-09-2026 en el cuerpo). Informativo → `passive` ontológicamente correcto; el lote lo obtuvo por degradación, no por juicio LLM.
- **UID 5817:** Boletín de empleo Computrabajo (vacantes de programador Python). Intención de diseño: `noise`. Al no portar `List-*` y fallar el LLM, cayó a `passive`.
- **UID 5818:** Newsletter técnica Codely (Kafka, Fable). Intención de diseño: `noise`. Idem anterior: clasificado como `passive` degradado.
- **UID 5819:** Chat informal («Ahora llega»). El acto y la fecha completa vivían en el **asunto**: `Reunión con Racso el 28/08/2026 a las 18:44`. Elevado correctamente por L-GUARD a `actionable` con asiento en agenda y poke en Telegram, pese a peaje LLM = 0.

---

## 3. Fricciones Empíricas y Diagnóstico Causal SSOT

### 3.1 F-TRIAGE-02 — Inferencia Nula y Desconexión de Telemetría (`DT-TRIAGE-LLM-QUALITY`)

| Dimensión | Detalle Técnico |
|-----------|-----------------|
| **Síntoma** | 6 de 6 correos rinden `tokens_in: 0` y `tokens_out: 0`. 5 de 6 marcan `classification-degraded: true`. Duración de ejecución de 0 ms en los no elevados. |
| **Causa Primaria (Consumidor)** | `classify_llm` inspecciona `body.get("tokens_in")`/`tokens_out` en raíz. La cápsula emite `data.telemetry_receipt.{prompt_tokens,completion_tokens}`. Con LLM **sano** el peaje seguiría leyéndose 0. |
| **Causa Secundaria (Subproceso)** | `duration_ms: 0` en 5814–5818: la cápsula cerró en <1 ms. `Ok(result)` no se filtra por `exit_code`/`success`. `SDDIA_LLM_INFER_COMMAND` no alimenta `mayeuta-llm`. |
| **Solución de Diseño** | 1) `telemetry_receipt::extract_from_capsule_body(&body, "skill:mayeuta-llm")` → `tokens_in`/`tokens_out`;<br>2) Si `exit_code != 0` o `success != true`, extras `classification_error` + `classification-degraded` + L-GUARD; no parsear texto vacío como juicio. |

### 3.2 F-TRIAGE-03 — Eferencia Exclusiva de `actionable` (Observabilidad Humana)

- **Comportamiento observado:** La interfaz WUI (`/api/email-inbox`) y las notificaciones de Telegram solo muestran veredictos `actionable`. De los 6 correos del lote, 5 resultaron invisibles para el operador humano.
- **Veredicto SSOT:** Cumple estrictamente con el contrato actual de diseño (evitar polución de notificaciones). La necesidad de una vista de auditoría para correos pasivos/ruidosos es una **fricción de observabilidad** de interfaz, desacoplada de la aduana y fuera del gate de este PBI.

### 3.3 F-TRIAGE-04 — Ruido de Boletines Clasificado como `passive`

| Dimensión | Detalle Técnico |
|-----------|-----------------|
| **Síntoma** | UIDs 5817 (Computrabajo) y 5818 (Codely) resultaron `passive` degradados en lugar de `noise`. |
| **Causa A (Sesgo de Ensayo)** | Fueron reenviados desde una cuenta Gmail personal, perdiendo las cabeceras estándar RFC (`List-Id`, `List-Unsubscribe`, `Precedence: bulk`). El muro `C-LIST` no tuvo oportunidad de activarse. |
| **Causa B (Defecto de Producto)** | Al estar el LLM inoperativo, no existió la segunda vía canónica para clasificar boletines como `noise` por contenido semántico. |
| **Efecto Colateral** | El digest heurístico (`PBI-EMAIL-NOISE-HEURISTIC-DIGEST`) solo agrega veredictos `verdict=noise ∧ decision_path=deterministic ∧ matched_rule=C-*`. Estos correos no habrían entrado en la cuarentena ni en el digest. |
| **Acción** | Protocolizar la inyección de fixtures con cabeceras RFC reales y habilitar inferencia semántica para boletines sin cabeceras de lista. |

### 3.4 F-TRIAGE-05 — Actos de Negocio sin Patrón L-GUARD

- **Síntoma:** UID 5815 (requerimiento fiscal con fecha límite en el cuerpo) quedó en `passive` degradado.
- **Laudo D3:** No se añadirán palabras clave («factura», «plazo», «impuesto») a la regla L-GUARD. L-GUARD queda restringido a reuniones/citas con fecha en el asunto.
- **Comportamiento requerido:** Con inferencia activa, el modelo debe analizar el prompt (que incluye el `snippet`) y si extrae `title` y `datetime` válidos, emitir `actionable`. Si no hay fecha o la extracción es incompleta, la degradación a `passive` es la regla invariante de la norma §3.

### 3.5 F-TRIAGE-06 — Agenda en Fecha Pasada y Falta de Deduplicación

- **Síntoma:** UID 5819 asienta `28/08/2026` el `07/09/2026` y no colapsa con UID 5792 (mismo asunto, lote 26/08).
- **Veredicto SSOT:** `persist_agenda` en `email_triage.rs` (no cápsula `agenda-manager`). Fuera del gate Slice 1.

---

## 4. Invariantes y Contrato de Verdad

Reglas obligatorias según `email-triage-matrix` v1.1.0 y arquitectura Core:

### 4.1 Modelo de Tres Vías y Precedencia

| Veredicto | Condición de Cierre | Efecto en Dominio |
|-----------|---------------------|-------------------|
| `noise` | `C-*` concluye **o** `P-MUTE-SENDER` activo **o** LLM clasifica `noise` | Proof durable. Cero pokes en Telegram. Cero agenda. Candidato a digest si es `deterministic` + `C-*`. |
| `passive` | Contenido puramente informativo; o candidato `actionable` con extracción incompleta (sin `datetime`); o degradación documentada. | Proof durable. Cero pokes. Cero agenda. |
| `actionable` | Acto explícito con `title` y `datetime` extraíbles (vía LLM o L-GUARD). **Prohibido inventar fecha**. | Proof durable. Entrada en `agenda:persist`. Notificación poke en Telegram. |

**Desempate canónico:**
- Muro `C-*` prevalece sobre cualquier otra señal (salvo exención explícita `P-EXEMPT-C`).
- Tras Clasificación LLM, la extracción estructural de asunto (L-GUARD) eleva a `actionable` aunque el LLM haya retornado `passive` o `noise`.
- Un `C-*` concluido como `noise` **jamás se reabre** para Clasificación LLM (Peaje G5).

### 4.2 Atribución de `decision_path`

- `deterministic`: Cerrado por reglas deterministas del Triaje-C (`C-LIST`, `C-NOREPLY`, `C-SUBJECT-NOISE`).
- `preference`: Cerrado por hábito activo del usuario (`P-MUTE-SENDER`).
- `llm`: Cerrado tras pasar por la fase de Clasificación (incluyendo elevaciones posteriores por L-GUARD).

### 4.3 Confidencialidad en `Email_Triaged`

- `from` y `subject` son **OPTIONAL** (para display táctico).
- `body` y `snippet` son **FORBIDDEN** (terminantemente prohibidos en el payload del evento; la auditoría reside en los ficheros `.eml` locales y en los proofs).

### 4.4 Protocolo de Fixtures de Ensayo

SSOT de casos, inyección y oráculos: `docs/features/kaizen-email-triage-infer-20260907/baterias-correo-kalma2.md` (`LAB-EMAIL-TRIAGE-BATTERY`).

| Tipo de Correo | Requisitos de Inyección para Validación | Resultado Esperado |
|----------------|-----------------------------------------|--------------------|
| Boletín / Newsletter | Debe portar cabeceras RFC reales (`List-Id: <...>` o `List-Unsubscribe: <...>`). No usar reenvío plano de Gmail. | `verdict: noise`, `decision_path: deterministic`, `matched_rule: C-LIST`. Clasificación skipped. |
| Notificación automática | Remitente con prefijo de sistema (`noreply@...`, `notifications@...`). | `verdict: noise`, `decision_path: deterministic`, `matched_rule: C-NOREPLY`. |
| Reunión / Cita | Asunto con acto (`reunión`, `cita`, `meeting`) + fecha `DD/MM/YYYY` [hora]. | `verdict: actionable`, elevación por L-GUARD. Asiento en agenda y poke Telegram. |
| Trámite / Plazo en cuerpo | Remitente profesional o personal sin cabeceras de lista. Fecha límite en cuerpo/snippet. | Invocación LLM obligatoria. `actionable` si extrae fecha; `passive` si no la extrae. |
| Informativo (factura) | Notificación sin requerimiento de acción ni fecha de vencimiento. | `verdict: passive` tras inferencia LLM. |

---

## 5. Alcance y Desglose en Slices Operativos

### Slice 1 — Corrección de Aduana y Peaje Cognitivo (Gate Crítico)

1. **Unificación del contrato de telemetría en `email_triage.rs`:**
   - Sustituir la búsqueda ingenua en `body.get("tokens_in")` / `body.get("tokens_out")` por la extracción conforme al SSOT de `mayeuta-llm`: inspeccionar `data.telemetry_receipt` o integrar `telemetry_receipt::extract_from_capsule_body(&body, "skill:mayeuta-llm")`.
   - Asignar `prompt_tokens` → `tokens_in` y `completion_tokens` → `tokens_out` en el `thermodynamic_cost`.
2. **Gestión robusta de invocación de cápsula en `classify_llm`:**
   - Validar `r.exit_code == 0` y la presencia de salida válida antes de parsear texto.
   - En caso de fallo del subproceso o salida vacía de `mayeuta-llm`, registrar la causa explícita en `extras` y marcar `classification-degraded: true` sin deglutir el error silenciosamente.
3. **Alineación de variables de entorno de inferencia:**
   - Verificar la compatibilidad de `mayeuta-llm` con las variables inyectadas por la instancia (`SDDIA_LLM_CLI_COMMAND`, `SDDIA_LLM_CHAT_COMMAND`).
4. **Pruebas unitarias de regresión:**
   - Test en `email_triage.rs` validando que una respuesta simulada de `mayeuta-llm` con sobre `{success: true, data: {text: ..., telemetry_receipt: {prompt_tokens: 10, completion_tokens: 5}}}` asigna correctamente los tokens y **no** activa `classification-degraded`.

### Slice 2 — Protocolo de Ensayo y Fixtures Limpios (Calidad de Pruebas)

- Batería canónica: `docs/features/kaizen-email-triage-infer-20260907/baterias-correo-kalma2.md`.
- Documentación de fixtures `.eml` canónicos con cabeceras RFC preservadas (`List-Id`, etc.) para verificar la batería de triaje en frío sin distorsión por reenvíos de Gmail.
- Smoke de validación local mediante `./sddia-run.sh --process email-triage-gateway`.

### Slice 3 — Deuda Diferida (Fuera del Gate de Inferencia)

- **F-TRIAGE-03:** Extensión del endpoint `/api/email-inbox` o nuevo endpoint de observabilidad para correos pasivos/ruidosos.
- **F-TRIAGE-06:** `persist_agenda` — no asentar ciegamente fechas pasadas; idempotencia por `message_uid`. No es gate Slice 1.

---

## 6. Criterios de Aceptación Formales

- [x] **CA-1 (Extracción de Telemetría):** `email_triage.rs` extrae `tokens_in` y `tokens_out` desde `data.telemetry_receipt` de `mayeuta-llm`. Un test unitario específico verifica que tokens > 0 no activan `classification-degraded`.
- [x] **CA-2 (Diagnóstico sin Deglución Silenciosa):** Si `mayeuta-llm` retorna error de ejecución (`exitCode != 0` o `success: false`), `classify_llm` documenta el fallo en `extras` (`classification_error` o similar) junto a `classification-degraded: true`, manteniendo la degradación fail-open a `passive`.
- [x] **CA-3 (Preservación de L-GUARD):** Asunto con acto (`reunión`/`cita`/`meeting`/`llamada`) + `dd/mm/yyyy` [hora] → `actionable`, `decision_path: llm`, agenda + poke, con LLM sano **y** con cápsula degradada. La matriz no exige fecha futura.
- [x] **CA-4 (Muro C-LIST Determinista):** Un fixture con cabecera `List-Id` o `List-Unsubscribe` concluye en `verdict: noise`, `decision_path: deterministic`, `matched_rule: C-LIST`; la fase Clasificación LLM se marca como `skipped` (peaje 0 tokens garantizado).
- [ ] **CA-5 (Clasificación Semántica sin L-GUARD):** Ante un correo ambiguo sin cabeceras `List-*` y con LLM operativo, el sistema emite el veredicto semántico correspondiente con `tokens_in + tokens_out > 0`, sin bandera de degradación. **PENDIENTE-GATED** (L-SLICE; no bloquea merge).
- [x] **CA-6 (Preservación Estricta de Laudo D3):** Queda prohibida la inclusión de palabras clave comerciales (`factura`, `documentación`, `computrabajo`) en la lógica de L-GUARD. Ningún correo concluye `actionable` sin fecha/hora verificable.
- [x] **CA-7 (Invariante IMAP):** La ejecución del triaje bajo ninguna circunstancia ejecuta comandos `STORE`, `EXPUNGE` ni altera el buzón IMAP remoto. Todo veredicto genera su correspondiente proof durable en `.SddIA/proofs/email-triaged/`.
- [x] **CA-8 (Desacople de Gates):** F-TRIAGE-03 (visualización en WUI) y F-TRIAGE-06 (calidad de agenda) no bloquean la aprobación ni el merge del Slice 1.

---

## 7. Referencias de Ejecución del Lote

| Recurso | Identificador / Ruta Física |
|---------|-----------------------------|
| Fixtures locales | `.SddIA/inbox/581{4,5,6,7,8,9}.eml` |
| Asiento de agenda (UID 5819) | `.SddIA/agenda/2b49036a-c8a8-4835-bc91-4d38b121ec5e.json` |
| Proof UID 5814 | `.SddIA/proofs/email-triaged/d5305406-6021-40e6-a646-4f6bb88a334a.json` |
| Proof UID 5815 | `.SddIA/proofs/email-triaged/45c1f26a-a954-4836-99de-a8172e05c8c9.json` |
| Proof UID 5816 | `.SddIA/proofs/email-triaged/678370e9-c1dc-4015-992a-7fd002fbe07e.json` |
| Proof UID 5817 | `.SddIA/proofs/email-triaged/b9221595-7835-443b-bdf8-dfa5490c898b.json` |
| Proof UID 5818 | `.SddIA/proofs/email-triaged/66da9fab-d3c2-4211-af0b-33ae5785cdc2.json` |
| Proof UID 5819 | `.SddIA/proofs/email-triaged/e215ebf7-bb18-4330-aab2-61e8e50be071.json` |
| API Inbox | `GET http://127.0.0.1:8765/api/email-inbox` (lista UID 5819 en cabecera) |
| Antecesor G5 (Paciente 0 / `SddIA_AP`) | UID `104579` / proof histórico `6e552199-416d-4043-91c9-fb1bae9e2057` — **no** presente en proofs de esta forja |

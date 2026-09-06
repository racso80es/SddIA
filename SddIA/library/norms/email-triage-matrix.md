---
uuid: "3d8c7e09-6d98-422d-909f-5b233ba7fcf2"
name: "email-triage-matrix"
version: "1.1.0"
nature: "tactical-norm"
author: "norm-creator"
scope: "agnostic"
category: "workflow"
dependencies: []
hash_signature: "sha256:82acf00d3e934b68fb26da5b3c0e9d5aef8a008ab2f8fb77b94f2cfaaf39dbc3"
---

## Directriz Core

Matriz de triaje de correo de tres vias para el asistente Kalma2. La semantica vive en la Libreria, no en el Core.

### 1. Matriz de tres vias

- `noise`: estimulo sin valor para la consciencia (listas, bulk, marketing, bounce). No eleva. Deja constancia en `Email_Triaged`.
- `passive`: informativo; no exige acto. No asienta agenda.
- `actionable`: exige acto con fecha/titulo extraibles. Unica via que dispara `agenda:persist`.
- Desempate: si una regla determinista marca `noise`, prevalece sobre cualquier otra senal. Si hay `actionable` y `passive` sin `noise`, gana `actionable` solo con extraccion completa; si la extraccion falla, degrada a `passive`. Tras Clasificacion LLM, si la extraccion estructural del asunto es completa, el veredicto se eleva a `actionable` aunque el LLM haya emitido `passive` o `noise`; Triaje-C `noise` ya cerrado no se reabre.

### 2. Reglas deterministas del Triaje-C

Cada regla tiene `matched_rule` estable. Evaluacion en orden; primera coincidencia concluye la fase. Si hay exencion `P-EXEMPT-C`, este muro no se aplica al mensaje.

- `C-LIST`: cabeceras `List-Id`, `List-Unsubscribe`, `Precedence: bulk|list`, `X-Mailer` de newsletter, `Auto-Submitted` distinto de `no` → `noise`.
- `C-NOREPLY`: remitente coincide con `no-reply@`, `noreply@`, `mailer-daemon@`, `notifications@` → `noise`.
- `C-SUBJECT-NOISE`: asunto con patrones `unsubscribe`, `viagra`, `newsletter`, `view in browser` (case-insensitive) → `noise`.
- Sin coincidencia: Triaje-C no concluye; pasa a mute P o Clasificacion (`llm:interact`).

### 2.1 Reglas P (habitos del portador)

Consulta previa `memory:pref-query` (`subject_key` = hash del remitente; `include_proposed: false`). Fail-open: bloque `{schema_version: "1.0.0", preferences: []}`.

- `P-EXEMPT-C`: `status: active` ∧ `authority: explicit_user` ∧ `predicate: priority` ∧ `value.level` ∈ {`max`,`high`}. Impide que C cierre. No cierra `actionable`. `inferred` y `proposed` no eximen.
- `P-MUTE-SENDER`: si C no concluyo y `predicate: mute` activo (`value.muted: true`; `until` ausente/null o futuro) → `noise`, `decision_path: preference`.

### 3. Contrato de extraccion (via `actionable`)

Campos: `title`, `datetime`, `source_ref`. Extraccion incompleta (sobre todo `datetime` ausente) → degradar a `passive`. Prohibido inventar fecha.

### 4. Blindaje antiverbosidad

Longitud del cuerpo, tono comercial, mayusculas o urgencia declarada (`URGENT`, `!!!`, `act now`) **no** elevan el veredicto. Solo elevan senales estructurales (regla C-* o clasificacion semantica de acto real).

### 5. Prioridad de conflicto

Triaje-C determinista prevalece sobre mute P y sobre el veredicto LLM cuando C concluye. `decision_path` del evento refleja el camino que **cerro**: `deterministic` si Triaje-C concluyo; `preference` si mute P cerro; `llm` solo si Clasificacion se ejecuto. `P-EXEMPT-C` no es cierre.

## Restricciones Duras (Aduana de Fricción)

- Prohibido veredicto `actionable` por verbosidad, tono comercial o urgencia declarada.
- Prohibido ejecutar Clasificacion LLM si Triaje-C concluyo o mute P cerro.
- Prohibido inventar `datetime` en extraccion; incompleto ⇒ `passive`.
- Prohibido alojar esta matriz en `SddIA/process/` o en la capsula del Centinela.
- Prohibido mutar el buzon IMAP como efecto de un veredicto.
- Prohibido que `inferred` / `proposed` eximan Triaje-C. Solo `explicit_user`+`priority` max|high (`P-EXEMPT-C`).

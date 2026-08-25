---
uuid: "3d8c7e09-6d98-422d-909f-5b233ba7fcf2"
name: "email-triage-matrix"
version: "1.0.1"
nature: "tactical-norm"
author: "norm-creator"
scope: "agnostic"
category: "workflow"
dependencies: []
hash_signature: "sha256:d03e37d17b215b202eac2e57d8366988194af030cab0a1c8cac538c0df6ccab2"
---

## Directriz Core

Matriz de triaje de correo de tres vías para el asistente Kalma2. La semántica vive en la Librería, no en el Core.

### 1. Matriz de tres vías

- `noise`: estímulo sin valor para la consciencia (listas, bulk, marketing, bounce). No eleva. Deja constancia en `Email_Triaged`.
- `passive`: informativo; no exige acto. No asienta agenda.
- `actionable`: exige acto con fecha/título extraíbles. Única vía que dispara `agenda:persist`.
- Desempate: si una regla determinista marca `noise`, prevalece sobre cualquier otra señal. Si hay `actionable` y `passive` sin `noise`, gana `actionable` solo con extracción completa; si la extracción falla, degrada a `passive`. Tras Clasificacion LLM, si la extracción estructural del asunto es completa, el veredicto se eleva a `actionable` aunque el LLM haya emitido `passive` o `noise`; Triaje-C `noise` ya cerrado no se reabre.

### 2. Reglas deterministas del Triaje-C

Cada regla tiene `matched_rule` estable. Evaluación en orden; primera coincidencia concluye la fase.

- `C-LIST`: cabeceras `List-Id`, `List-Unsubscribe`, `Precedence: bulk|list`, `X-Mailer` de newsletter, `Auto-Submitted` distinto de `no` → `noise`.
- `C-NOREPLY`: remitente coincide con `no-reply@`, `noreply@`, `mailer-daemon@`, `notifications@` → `noise`.
- `C-SUBJECT-NOISE`: asunto con patrones `unsubscribe`, `viagra`, `newsletter`, `view in browser` (case-insensitive) → `noise`.
- Sin coincidencia: Triaje-C no concluye; pasa a Clasificacion (`llm:interact`).

### 3. Contrato de extracción (vía `actionable`)

Campos: `title`, `datetime`, `source_ref`. Extracción incompleta (sobre todo `datetime` ausente) → degradar a `passive`. Prohibido inventar fecha.

### 4. Blindaje antiverbosidad

Longitud del cuerpo, tono comercial, mayúsculas o urgencia declarada (`URGENT`, `!!!`, `act now`) **no** elevan el veredicto. Solo elevan señales estructurales (regla C-* o clasificación semántica de acto real).

### 5. Prioridad de conflicto

Triaje-C determinista prevalece sobre el veredicto LLM cuando ambos concluyen. `decision_path` del evento refleja el camino que **cerró**: `deterministic` si Triaje-C concluyó; `llm` solo si Clasificacion se ejecutó.

## Restricciones Duras (Aduana de Fricción)

- Prohibido veredicto `actionable` por verbosidad, tono comercial o urgencia declarada.
- Prohibido ejecutar Clasificacion LLM si Triaje-C concluyó.
- Prohibido inventar `datetime` en extracción; incompleto ⇒ `passive`.
- Prohibido alojar esta matriz en `SddIA/process/` o en la cápsula del Centinela.
- Prohibido mutar el buzón IMAP como efecto de un veredicto.

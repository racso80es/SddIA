---
document_id: PBI-KAIZEN-EVENT-BUS-AUDIT-31867981
uuid: "46dde226-6672-420f-8d2a-a5f3b49cdea8"
title: "[KAIZEN] Triaje de alerta de auditoría de bus EDA (PENDING_AUDIT_DOC_31867981)"
format: markdown
version: "1.3.0"
created: "2026-08-28"
updated: "2026-09-05"
status: done
refinement_status: implemented
priority: media
process: bug-fix
executor_vehicle: bug-fix
type: kaizen
dispatch: true
scope: diagnostic-satellite-and-sensor-tuning
follow_on_executor_vehicle: bug-fix
suggested_branch: fix/kaizen-event-bus-audit-sensor-tuning
persist_ref: docs/fixes/kaizen-event-bus-audit-sensor-tuning
persist_ref_suggested: docs/fixes/kaizen-event-bus-audit-sensor-tuning
branch: fix/kaizen-event-bus-audit-sensor-tuning
execution_id: "f9830175-0405-42fd-9e0c-e6de1c26201d"
derived_from: PENDING_AUDIT_DOC_31867981
origin_review_id: "46dde226-6672-420f-8d2a-a5f3b49cdea8"
alert_kind: event-bus-audit
census_date: "2026-09-05"
census_method: file-count-and-payload-sample
depends_on: []
related:
  - SddIA/tools/event-bus-audit.md
  - SddIA/tools/event-bus-audit/src/main.rs
  - SddIA/process/event-bus-audit.md
  - SddIA/actions/materialize-kaizen-alert-doc.md
  - SddIA/engine/execute-process/src/engine/materialize_kaizen_alert_doc.rs
  - SddIA/events/domain/kaizen-alert-required.md
  - SddIA/daemons/github-bridge-watcher.md
  - docs/fixes/event-bus-dead-letter-remediation/spec.md
  - docs/fixes/kaizen-audit-doc-dedupe-ola-20260716/spec.md
  - docs/todos/done/PENDING_AUDIT_DOC_06755dde.md
  - docs/todos/done/[FIX] kaizen audit-doc — dedupe ola event-bus-audit 2026-07-16.md
refinement_notes: "v1.3.0 Laudo 2026-09-05: ejecutar bug-fix (execution_id f9830175). Follow-on absorbido en este ciclo. v1.2.0 Filtro A intacto."
---

# [KAIZEN] Triaje de alerta de auditoría de bus EDA (PENDING_AUDIT_DOC_31867981)

> Origen: evento `Kaizen_Alert_Required` emitido por la cápsula `tool:event-bus-audit` (`emitter_agent: event-bus-audit`, `review_id: 46dde226-6672-420f-8d2a-a5f3b49cdea8`) y materializado por `action:materialize-kaizen-alert-doc` (Cúmulo) el 2026-08-28. Stub git `2fb7bd4`.

---

## Mandato

Este ítem es **cicatriz diagnóstica**. No ejecuta `bug-fix` ni muta genoma.

1. **Depuración ontológica:** la prosa DIA («fuga de conocimiento documental», checklist `spec.md`) es plantilla hardcodeada; `alert_kind` real es `event-bus-audit`.
2. **Censo verificable del bus:** sustituir métricas y causas raíz no reproducidas por un recuento de archivos a fecha `census_date`.
3. **Partición de alcance:** archivar esta cicatriz no equivale a tunear el sensor. Mutar `SddIA/tools/event-bus-audit` o `materialize-kaizen-alert-doc` exige PBI/ciclo `bug-fix` aparte (`follow_on_executor_vehicle`), vía orquestador (DA-2/DA-4).
4. **No purga:** `.events/dead-letter/` permanece intocable en este ítem.

---

## 0. Dictamen (Filtro A)

### 0.1 Stub original (2026-08-28) — veredicto sobre el bruto

| # | Afirmación del stub | Verdad objetiva | Veredicto |
|---|---|---|---|
| **H1** | Origen «sensor DIA» | `build_todo_body` en `materialize_kaizen_alert_doc.rs` (aprox. L157) hardcodea `sensor DIA`. El emisor del evento es `event-bus-audit` (`qa:probe`). | Alucinación de sensor. |
| **H2** | «Posible fuga de conocimiento documental» | Los 50 `implicated_files` del stub son `.events/dead-letter/github-bridge-*.json` (48) y testigos `.capability-di-gate.json` (2). Cero rutas de `docs/` / README. | Falso positivo DIA. Texto de plantilla. |
| **H3** | Checklist DIA (`spec.md`, README, `impacts_doc`) | `persist_ref` / `pr_branch` / `impacts_doc` = `—`. No hay `spec.md` de esta alerta. | Checklist N/A para `alert_kind: event-bus-audit`. |
| **H4** | 50 archivos = lote de trabajo | `state.anomalies.iter().take(50)` en `event-bus-audit/src/main.rs` (aprox. L856–860). Truncamiento sobre anomalías, no conjunto cerrado de código. | Inexactitud de alcance. |
| **H5** | Métricas 2026-08-28: 12320 DL, 707 testigos KO, 4970 estructurales, 13 huérfanos, 0 stale | Snapshot del `alert_justification` original. No es el estado actual. | Deriva temporal. Conservar como histórico. |
| **H8** | `needs_kaizen` si `dead_letter_count > 0` (y OR de más condiciones) | Condición real (aprox. L836–841): `dead_letter_count > 0 \|\| dead_letter_witness_count > 0 \|\| structural_error_count > 0 \|\| orphan_witness_count > 0 \|\| stale_pending_count > 0 \|\| circuit_alert`. Con DL histórico no vacío, `needs_kaizen` es perenne. El `take(50)` cambia con el orden del directorio → nuevo `hash8` → nuevo `PENDING_AUDIT_DOC_*` (la ola 2026-07-16 solo deduplica huella `alert_kind`+files idénticos). | Defecto de disparo + de muestreo. |

### 0.2 Correcciones al refinamiento v1.1.0 (este Filtro A)

| # | Afirmación v1.1.0 | Verdad objetiva (censo 2026-09-05) | Veredicto |
|---|---|---|---|
| **H6** | «95%+ de 12.900 DL = IOTA `127.0.0.1:8787` + Telegram DNS; resto esquemas legacy» | Universo **12.901 cabeceras** en `.events/dead-letter/` (no recursivo): **6.452 `Email_Received`** (50,0 %) con `delivery_state.cumulo.email-triage-gateway=failed`; **5.386 dumps `github-bridge-*`** (41,7 %) no-ECST, *todos* `error: iota-relay-unreachable` + `flag: FALLBACK_LOCAL_SIGNATURE`; resto (≈8 %) fracturas, PR, entidades, etc. Testigos **508**: IOTA 172 (33,9 %), Telegram/DNS 123 (24,2 %), otros 213 (42,0 %: RBAC, merkle-anchor, fases de proceso, …). IOTA es **el 100 % del subconjunto github-bridge** y **≈34 % de testigos**, no el 95 % del universo. | Alucinación cuantitativa. Confunde muestra `take(50)` (github-bridge) con el directorio entero. |
| **H7** | `event-bus-dead-letter-remediation/spec.md` § Fuera de alcance = laudo dogmático perpetuo contra purga **y** contra mutar `SddIA/tools/` | Ese «fuera de alcance» rige **aquel** ciclo 2026-07-11. No prohíbe un `bug-fix` futuro sobre `event-bus-audit`. Invocarlo para bloquear Ola 2 y a la vez proponer parche en `SddIA/tools/` es autocontradicción. | Sobre-generalización. Conservar: no purgar DL en *este* ítem. No convertir en veto de genoma. |
| **H9** | Snippet `needs_kaizen = stale_pending \|\| orphan \|\| circuit_alert` erradica el spam | A 2026-09-05 hay **30 pending >24 h** (todos `System_Fracture_Detected`) y **≥5.386 estructurales** por dumps github-bridge (filename ≠ ECST: faltan `event_id` / `event_type` / `emitter_agent`). Aunque se quite `dead_letter_count > 0`, el OR actual (y el snippet v1.1.0 vía `stale_pending`) sigue en `true`. | Parche ineficaz. No copiar ese snippet como solución. |
| **H10** | Flag nuevo `alert_on_dead_letter` | Ya existe `emit_kaizen_alert` (default `true`) en proceso y cápsula. Ver `SddIA/process/event-bus-audit.md`. | Inexactitud. No inventar input duplicado. |
| **H11** | Métricas v1.1.0 (12.900 / 508 / 5.776 estructurales / 14 huérfanos / 28 stale) como «event-bus-audit en `main`» | No hay `audit-report.md` en workspaces que las respalde. Censo de archivos: DL 12.901; testigos 508 (coincide); pending 32 / stale>24h **30**; processing 23; telemetry **687**; estructurales *en cabeceras DL* ≥5.386 (solo github-bridge) + 13 más; huérfanos testigo ≈12 (aprox. frente a DLT+pending+processing+processed). 5.776 / 14 / 28 / 686 **no reproducidos**. | Inexactitud de método. Este PBI cita censo, no una corrida de la cápsula (corrida con default emitiría otro Kaizen). |
| **H12** | Fila bus: «12.900 JSON inertes (686 telemetry, 32 pending, 23 processing)» | Mezcla el sink `dead-letter` con **otros** buckets del bus. | Incoherencia de agregación. |
| **H13** | CA5: `verify-process-integrity` / `audit-eda-coverage` con 0 huérfanos | Huérfanos de cobertura EDA ≠ `orphan_witness` del bus. Criterio ajeno a esta cicatriz. | CA inválido. Eliminado. |
| **H14** | CA1–CA3 marcados `[x]` por existir prosa en el PBI | Criterios de aceptación del *trabajo*; no se autocumplen al redactar el dictamen. | Incoherencia metodológica. |
| **H15** | `Kaizen_Alert_Required` «emisor legítimo = event-bus-audit» sin matiz | Clase `SddIA/events/domain/kaizen-alert-required.md` lista emisores autorizados: `pull-request-review` y `emit-kaizen-alert-required-event`. El proceso `event-bus-audit` (v1.0.1) **sí declara** emisión Kaizen. Deriva contrato de clase vs proceso/cápsula. | Hallazgo de follow-on; no afirmar autorización plena de la Clase. |
| **H16** | Archivar esta cicatriz cierra el spam | `find_open_kaizen_audit_doc` solo reutiliza TODO si el cuerpo tiene `- [ ] Revisar \`spec.md\`` **sin marcar** y la misma huella files. Checklist `[x]` o `take(50)` distinto → nuevo archivo `PENDING_AUDIT_DOC_{hash8}.md`. | Archivo ≠ inmunidad. |
| **H17** | Precedent `PENDING_AUDIT_DOC_06755dde`: ambas casillas «N/A event-bus-audit» | Solo la segunda casilla dice N/A; la primera dice «consolidado». Los `implicated_files` de ese satélite eran `.events/pending/`, no dead-letter. | Cita inexacta. |

---

## 1. Superficie de afectación

| Componente | Rol | Estado (censo 2026-09-05) | Este PBI |
|---|---|---|---|
| Esta cicatriz (`PENDING_AUDIT_DOC_31867981.md`) | Deuda documental en `docs/todos/pending/` | Refinado v1.2.0; `dispatch: false` | Dictamen + archivo a `done/` tras laudo |
| `tool:event-bus-audit` | Emisor empírico; `needs_kaizen` perenne | Genoma (`directories.tools`) | **Fuera de alcance** — follow-on |
| `action:materialize-kaizen-alert-doc` | Plantilla DIA monomórfica | Genoma (`directories.actions`) + handler en `SddIA/engine/` | **Fuera de alcance** — follow-on |
| Clase `kaizen-alert-required` | Emisores autorizados no listan la cápsula | Deriva vs `process/event-bus-audit.md` | **Fuera de alcance** — follow-on |
| `.events/dead-letter/` | Sink terminal | 12.901 cabeceras + 508 testigos | No mutar |
| `.events/pending/` | Cola dominio | 32 JSON; 30 stale = `System_Fracture_Detected` (ya hay PBI de fractura) | No reabrir como deuda de este ítem |

---

## 2. Censo empírico (2026-09-05)

Método: recuento de `*.json` en buckets de `cumulo.paths.json` → `eda_bus` / `eda_fractal`. **No** se invocó `./sddia-run.sh --process event-bus-audit` (default `emit_kaizen_alert: true`).

### 2.1 Buckets

| Bucket | Cabeceras `*.json` (no recursivo) |
|---|---|
| `dead-letter` | 12.901 |
| `dead-letter/subscribers` | 508 |
| `pending` | 32 (30 con `timestamp` >24 h) |
| `processing` | 23 |
| `processed` | 697 |
| `telemetry` | 687 |
| `orchestration` | 0 |
| `domain` | 0 |

### 2.2 Composición de cabeceras dead-letter

| Clase / artefacto | N | Nota |
|---|---|---|
| `Email_Received` | 6.452 | ECST válido; `cumulo.email-triage-gateway=failed` en los 6.452 |
| Dump `github-bridge-*` | 5.386 | **No** es evento ECST (`error`/`flag`/`pr`/`source`/`timestamp`). 5.386/5.386 = `iota-relay-unreachable` + `FALLBACK_LOCAL_SIGNATURE`. Explica el `take(50)` del stub. |
| `System_Fracture_Detected` | 378 | Residuo de fracturas ya materializadas como PBI |
| Otros (`PullRequest_*`, `Domain_Entity_*`, `Process_Execution_Completed`, …) | 1.085 | Mix de fallos Telegram/IOTA/TQM y anclas merkle |

### 2.3 Testigos dead-letter (`error_trace`)

| Cubo | N | % de 508 |
|---|---|---|
| IOTA / relay local | 172 | 33,9 |
| Telegram / DNS | 123 | 24,2 |
| Resto (RBAC, merkle-anchor, fases, cápsulas, …) | 213 | 42,0 |

### 2.4 Pending estancados

Los 32 pending son `System_Fracture_Detected`. El umbral default `stale_threshold_hours=24` marca 30. Esa deuda tiene PBI propios en `docs/todos/pending/` (`[FIX] … fractura sistémica`). No es «rotura de contratos del bus» ni lote de este satélite.

---

## 3. Línea de este ítem (solo cicatriz)

1. Conservar este dictamen v1.2.0 en el archivo.
2. Checklist DIA del stub: N/A (no ejecutar trabajo documental de paridad).
3. Tras laudo del Vértice Biológico: mover a `docs/todos/done/` **sin** PR de genoma y **sin** purga de `.events/`.
4. No marcar inmunidad al spam: el siguiente `event-bus-audit` con `emit_kaizen_alert: true` puede nacer otro `PENDING_AUDIT_DOC_*`.

### Follow-on (no se despacha aquí)

Semilla para un `bug-fix` posterior (`persist_ref_suggested` / `suggested_branch`), solo si hay laudo explícito:

- Discriminar dumps no-ECST (`github-bridge-watcher` / `FALLBACK_LOCAL_SIGNATURE`) del censo de anomalías estructurales.
- `needs_kaizen`: no OR-acumulado histórico; no alertar por `Email_Received` crónico ni por fracturas ya materializadas.
- Usar `emit_kaizen_alert` existente; no duplicar flags.
- Plantilla `build_todo_body` por `alert_kind`.
- Alinear emisores de la Clase `Kaizen_Alert_Required` con el proceso `event-bus-audit`, o dejar de emitir esa Clase desde la cápsula.
- Toda escritura bajo `directories.tools` / `directories.actions` / `directories.events` vía `entity-manager` + topología `bug-fix`/`feature` (DA-2, DA-4).

Prohibido tratar el snippet de `needs_kaizen` de v1.1.0 como parche aceptado.

---

## 4. Criterios de aceptación

- [x] **CA1 (Filtro A):** El cuerpo desmiente DIA, el 95 % IOTA/Telegram, el snippet ineficaz y las métricas no reproducidas; cita censo 2026-09-05.
- [x] **CA2 (Partición / laudo 2026-09-05):** El follow-on de sensor/plantilla/clase se ejecutó en este `bug-fix` (`execution_id` `f9830175`). L-NO-GENOME del v1.2.0 queda superado por laudo explícito.
- [x] **CA3 (Precedentes):** Ola 2026-07-16 = dedupe de huella, no licencia para reabrir purga DL. `event-bus-dead-letter-remediation` § fuera de alcance = de *aquel* ciclo.
- [x] **CA4 (Archivo):** Tras laudo, el archivo vive en `docs/todos/done/` con el mismo `document_id`.
- [x] **CA5 (Inmutabilidad DL):** Ningún cambio en `.events/dead-letter/` atribuible a este ítem.

---

## 5. Laudos de este ítem

1. **L-SATELLITE:** Esta cicatriz se cierra por diagnóstico, no por remediación del bus.
2. **L-NO-PURGE:** Prohibida purga u edición ad-hoc de `.events/dead-letter/` aquí.
3. **L-NO-GENOME (v1.2.0, superado 2026-09-05):** El laudo de implementación absorbe el follow-on en este ciclo `bug-fix`.
4. **L-NO-FALSE-FIX:** No se acepta un `needs_kaizen` que siga verdadero ante DL histórico, dumps github-bridge o pending de fractura.

---

## Datos históricos de la alerta (snapshot 2026-08-28, stub git)

| Campo | Valor original |
|---|---|
| `review_id` | `46dde226-6672-420f-8d2a-a5f3b49cdea8` |
| `alert_justification` | `Auditoría event-bus-audit: 12320 dead-letter cabeceras, 707 testigos KO, 4970 anomalías estructurales, 13 huérfanos, 0 pending estancados` |
| `alert_kind` | `event-bus-audit` |
| `persist_ref` | — |
| `pr_branch` | — |
| `impacts_doc` | — |
| `implicated_files` | 50 rutas; 48× `.events/dead-letter/github-bridge-*.json` + 2× `*.capability-di-gate.json` (`take(50)` sobre `anomalies`) |

### Checklist DIA original (N/A)

- [x] Revisar `spec.md` § Impacto en Documentación *(N/A — `alert_kind: event-bus-audit`)*
- [x] Actualizar README/manuales o `impacts_doc` *(N/A — sin `persist_ref`)*

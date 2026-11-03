---
feature_name: aut-63-lifecycle-genome-parity
created: "2026-11-02"
process: bug-fix
phase: blueprint
agents: dedalo
phases: T0-T7
branch_name: Fix_Events_Multiple_02
sighting_branch: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-63-lifecycle-genome-parity
pbi_ref: AUT-63
document_id: PBI-b0d4af60
uuid: 72169010-d232-41f9-a877-44beb83dafc2
olas:
  - D0
  - A1
  - A2
  - A3
  - A4
execution_id: "a6a8c74c-89df-46a7-ab0d-74d3ddb18814"
---
# Plan — aut-63-lifecycle-genome-parity

Orden: T0 auditoria diseno → T1 A1 dual Kaizen → T2 A2 done_gate intents+hash → T3 A3 HU_Forged stamp → T4 A4 sumidero Idea_Captured → T5 tests → T6 evolution + auditoria v1.2.0 → T7 cascada Argos. Esta parada Dedalo solo sella spec.md + plan.md (D0 ya en arbol). Mutacion genoma/codigo = Ejecucion (Tekton).

## T0 — Entregable de diseno: par de auditoria (D0)

1. Verificar docs/audits/ciclo-vida-software-sddia-v1.0.0.md: version 1.0.0, status frozen.
2. Verificar docs/audits/ciclo-vida-software-sddia.md: version 1.1.0, tabla A1-A5, tracker_ref AUT-63.
3. Anotar baseline en execution.md (Baseline auditoria): paths + versiones. Prohibido editar la v1.0.0.
4. Si el par falta o el frontmatter diverge: blocked — no inventar exito.

## T1 — A1 paridad documental dual (Kaizen_Alert_Required)

1. event-domain-subscriptions.json: intent del suscriptor Cumulo → dual tracker issue / file PENDING_AUDIT_DOC.
2. materialize-kaizen-alert-doc.md: proposito, outputs (target_path / issue_ref), pasos = dual; bump version accion si el contrato lo exige.
3. cumulo.md seccion 6 + cumulo.instructions.json mandato KAIZEN_ALERT → mismo dual.
4. Barrido de prosa que afirme solo docs/todos/pending/ en PPR / kaizen-alert-required / indices; corregir sin cambiar routing.
5. Commit: fix(aut-63): A1 dual Kaizen_Alert genome.

## T2 — A2 intents fase cierre + hash_signature

1. Editar intents + secciones Cierre documental en rama en feature.md, bug-fix.md, refactorization.md (codex SE).
2. Texto objetivo: ramificar por done_gate / provider segun features-documentation-pattern v1.3.1 (file → move+pbi_archived; linear → gate Linear + proofs, sin nuevos docs/todos/).
3. Refrescar hash_signature (capsula/crypto-broker / procedimiento canonico). Si no se puede calcular: documentar bloqueo — no inventar digest.
4. Actualizar process/index.md del codex si el resumen miente el cierre.
5. Commit: fix(aut-63): A2 done_gate closure intents.

## T3 — A3 HU_Forged + tracker-stamp

1. Anadir clave HU_Forged en event-domain-subscriptions.json → tekton / tracker-stamp (sello backlog HU + comentario document_id).
2. Crear codex-software-engineering/events/hu-forged.md (espejo pbi-forged.md); actualizar indices / EDA coverage segun aduana.
3. Opcional: entrada en subscriptions.json del codex si el SSOT debe listar el sello (sin auto-feature).
4. tracker_stamp.rs: file sink HU_Forged → COLUMN_BACKLOG; match remoto espejo PBI_Forged (backlog, comentario HU forjada, sync column).
5. Tests mock (sin red) en modulo stamp / lab existente.
6. Commit: fix(aut-63): A3 HU_Forged tracker-stamp.

## T4 — A4 Kaizen_Idea_Captured contrato de sumidero

1. Reforzar events/domain/kaizen-idea-captured.md: sumidero/traza intencional; no suscriptor de forja en v1.
2. Mantener [] en catalogo salvo anotacion machine-readable compatible con el schema.
3. Si event-bus-audit marca EMPTY_SUBSCRIBERS accionable sobre este event_type: allowlist/excepcion documentada + test.
4. Test execute-process: emitir/route Kaizen_Idea_Captured → success / sweep sin fractura; asercion politica L-A4-SINK.
5. Commit: fix(aut-63): A4 Kaizen_Idea_Captured sink contract.

## T5 — Verificacion agregada

1. Correr tests unitarios tocados (tracker_stamp, materialize_kaizen_alert_doc, route/audit segun T4).
2. Lab opcional: sddia-run.sh acotado a handlers nativos (forge-hu → stamp HU; materialize kaizen con/sin tracker mock).
3. Confirmar cero paths nuevos bajo docs/todos/ en el diff del fix.
4. Si un test no puede ejecutarse: evidencia explicita en execution.md — no marcar AC como OK.

## T6 — Evolution + auditoria v1.2.0

1. Crear SddIA/evolution/72169010-d232-41f9-a877-44beb83dafc2.md (AUT-63, PBI-b0d4af60, olas A1-A4).
2. Actualizar docs/audits/ciclo-vida-software-sddia.md → version 1.2.0: seccion 9 refleja A1-A4 alineados; A5 sin cambio; no tocar ciclo-vida-software-sddia-v1.0.0.md.
3. Commit(s) dedicados documentacion/evolution.

## T7 — Cascada bug-fix / cierre

1. implementation.md + execution.md con touchpoints y evidencia T0-T6.
2. Argos → validacion.md (APTO solo con AC1-AC7 evidentes; done_gate linear → sin archivar PBI en docs/todos/).
3. Tracker AUT-63 Done: fuera de Tekton (operador Linear).
4. delivery-close-cycle del propio fix tras validacion APTO.

## Contingencias

| Riesgo | Accion |
|--------|--------|
| Hash de proceso no calculable sin capsula | Bloquear T2; no inventar hash_signature |
| EDA orphan al crear hu-forged.md | Emit/backfill canonico o manifiesto Fase C; no saltar aduana |
| Allowlist audit A4 rechazada | Preferir contrato evento + test route antes de mutar umbrales Kaizen |
| Cuerpo Linear AUT-63 inaccesible | Criterios siguen semilla + seccion 9; no fabricar DoR |

## Cierre de esta parada Dedalo

Sellados: docs/fixes/aut-63-lifecycle-genome-parity/spec.md, docs/fixes/aut-63-lifecycle-genome-parity/plan.md.
Diseno D0 (par auditoria 1.0.0/1.1.0): ya en arbol; verificado por lectura MCP.
Pendiente Ejecucion: T0 evidencia en execution → T1-T7.

---
feature_name: aut-63-lifecycle-genome-parity
created: "2026-11-02"
process: bug-fix
phase: design
agents: dedalo
base: main
scope: A1-A4 genome/code lifecycle parity
branch_name: Fix_Events_Multiple_02
sighting_branch: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-63-lifecycle-genome-parity
pbi_ref: AUT-63
document_id: PBI-b0d4af60
uuid: 72169010-d232-41f9-a877-44beb83dafc2
version_spec: "1.0.0"
status: dedalo_locked
parent: AUT-20
audit_base: docs/audits/ciclo-vida-software-sddia.md
audit_frozen: docs/audits/ciclo-vida-software-sddia-v1.0.0.md
audit_current_version: "1.1.0"
olas:
  - A1
  - A2
  - A3
  - A4
  - D0
execution_id: "a6a8c74c-89df-46a7-ab0d-74d3ddb18814"
---
# Spec — aut-63-lifecycle-genome-parity

## 1. Misión

Cerrar las asimetrías **genoma ↔ código** del ciclo de vida listadas en `docs/audits/ciclo-vida-software-sddia.md` §9 (**A1–A4**). Padre: AUT-20. Proceso: `bug-fix`. Document id: `PBI-b0d4af60`.

Fuentes consumidas en diseño: semilla del orquestador, `objectives.md`, auditoría v1.1.0 (+ copia congelada v1.0.0), touchpoints de código/genoma verificados vía MCP, forge-state `.SddIA/state/forge-pbi/…` (`tracker_ref: AUT-63`, `uuid: 72169010-…`). **Cuerpo Linear AUT-63:** no hay fichero local `AUT-63.md` bajo el piloto; no se inventó el YAML del issue — el criterio de cierre se toma de la semilla + §9.

## 2. Problema verificado

| ID | Hecho (foto código/genoma) | Evidencia |
|----|----------------------------|-----------|
| **A1** | Catálogo/acción dicen solo `docs/todos/pending/`; Rust crea issue `Kaizen {hash8}` si hay tracker | `event-domain-subscriptions.json` intent Kaizen_Alert; `materialize-kaizen-alert-doc.md` §1–§3 + outputs `target_path` TODO; `materialize_kaizen_alert_doc.rs` rama `tracker_config_for_inputs` → `create_issue` título `Kaizen {hash8}` |
| **A2** | Fase «Cierre documental en rama» en `feature` / `bug-fix` / `refactorization` solo describe move a `docs/todos/done/` + `pbi_archived: true` (norma citada v1.2.0) | Intents + secciones cuerpo de los tres `.md`; handler real `phase_capsules` / `done_gate` ya ramifica `file` \| `linear` \| `both`; norma vigente `features-documentation-pattern` **v1.3.1** |
| **A3** | `HU_Forged` ausente del catálogo de suscripciones y del match de `tracker-stamp` (sí existe espejo `PBI_Forged`) | `event-domain-subscriptions.json` tiene `PBI_Forged`→`tracker-stamp`, no `HU_Forged`; `tracker_stamp.rs` match/`COLUMN_BACKLOG` solo `PBI_Forged`; `forge-hu` sí emite `HU_Forged`; existe `events/pbi-forged.md` en códice SE, **no** `hu-forged.md` |
| **A4** | `Kaizen_Idea_Captured`: suscriptores `[]`; evento declara «sin suscriptores obligatorios en MVP» | Catálogo `[]`; `kaizen-idea-captured.md`; dispatch vacío → sweep OK (`route_domain_core`); audit puede marcar `EMPTY_SUBSCRIBERS` |
| **A5** | Lab `success: true` ≠ agentes `executed` | **Fuera de alcance de código** (solo lectura del `execution_report`) |

## 3. Entregable de diseño ya materializado (auditoría)

| Artefacto | Rol |
|-----------|-----|
| `docs/audits/ciclo-vida-software-sddia-v1.0.0.md` | Congelada (`version: 1.0.0`, `status: frozen`). **No reescribir.** |
| `docs/audits/ciclo-vida-software-sddia.md` | Vigente **1.1.0**: conserva fotografía del código + tabla A1–A5 con estado objetivo AUT-63. |

Tras implementar A1–A4, Tekton/Argos publican **v1.2.0** del informe vigente sustituyendo en §9 solo los hechos que dejen de ser ciertos. La v1.0.0 permanece intacta.

## 4. Laudos

| Ref | Decisión |
|-----|----------|
| **L-SCOPE** | Solo A1–A4 + sellado documental de auditoría (D0 diseño / v1.2.0 post-fix). A5 sin mutación de código. |
| **L-A1-DUAL** | Alinear **texto** de catálogo, acción, `cumulo.md` / `cumulo.instructions.json` (y menciones en `pull-request-review` si afirman solo `docs/todos/`) al dual real: **tracker configurado → issue `Kaizen {hash8}`**; **sin tracker / provider file → `PENDING_AUDIT_DOC_{hash8}.md` en `docs/todos/pending/`**. No cambiar la semántica Rust salvo tests de regresión del dual. |
| **L-A2-DONE-GATE** | Reescribir intent + prosa de «Cierre documental en rama» en `feature.md`, `bug-fix.md`, `refactorization.md` según `done_gate` / `features-documentation-pattern` **v1.3.1** (file → move+`pbi_archived`; linear → gate Linear/proofs, **sin** nuevos ficheros en `docs/todos/`). Refrescar `hash_signature` de cada proceso (vía contrato/cápsula canónica; no inventar digest). Bump de versión de proceso coherente con mutación de fases. |
| **L-A3-HU-STAMP** | Suscribir `HU_Forged` → `tekton.tracker-stamp` en `event-domain-subscriptions.json` (y códice `subscriptions.json` si aplica). Extender `tracker_stamp.rs` (file sink + remoto) con rama espejo de `PBI_Forged`: estado backlog, `tracker_column: backlog`, comentario idempotente `HU forjada: {document_id}`. Crear `hu-forged.md` en el códice SE (paridad `pbi-forged.md`) + índice/EDA según aduana. |
| **L-A4-SINK** | **No** enganchar `forge-pbi` automático. Contrato explícito de **sumidero/traza**: mantener `[]` en catálogo **o** anotar sumidero documentado; reforzar `kaizen-idea-captured.md`; si `EMPTY_SUBSCRIBERS` genera ruido accionable sobre este tipo, allowlist/excepción documentada en audit. Test(s) en `execute-process` que demuestren: dispatch sin fractura + contrato de sumidero (y, si aplica, que el audit no trate el vacío como deuda abierta). |
| **L-AUDIT** | Par v1.0.0/v1.1.0 es entregable de **este** diseño. v1.2.0 solo tras A1–A4 verdes. |
| **L-KAIZEN** | Prohibido escribir semillas bajo `docs/todos/` desde Tekton/Argos/Dedalo. Solo Cúmulo / `Kaizen_Alert_Required`. |
| **L-GIT** | Mutaciones git solo `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |
| **L-TRACKER-DONE** | AUT-63 → Done en Linear **fuera de Tekton** (operador / canal tracker). Este fix no crea markdown de ítem en `docs/todos/`. |
| **L-EVOL** | Evolution canónica `SddIA/evolution/72169010-d232-41f9-a877-44beb83dafc2.md` vinculada a AUT-63 / `PBI-b0d4af60`. |

## 5. Solución (olas)

### D0 — Auditoría (diseño)

Confirmar presencia y frontmatter del par congelada/vigente (ya en árbol). No mutar v1.0.0.

### A1 — Paridad documental Kaizen_Alert dual

1. Actualizar intent en `event-domain-subscriptions.json` (`Kaizen_Alert_Required`).
2. Reescribir `materialize-kaizen-alert-doc.md` (propósito, outputs, límites) al dual tracker/file.
3. Alinear `cumulo.md` §6, `cumulo.instructions.json` mandato KAIZEN_ALERT, y prosa PPR/DIA que afirme solo `docs/todos/pending/`.
4. Tests existentes del path file se mantienen; añadir/ajustar cobertura del path tracker si falta (mock, sin red).

### A2 — Intents fase cierre + hash

1. Editar los tres procesos del códice SE: intent fase cierre + secciones «Cierre documental» → dual `done_gate` / v1.3.1.
2. Refrescar `hash_signature` (y versión) por proceso.
3. Actualizar filas de `process/index.md` del códice si describen el cierre solo-file.

### A3 — HU_Forged → tracker-stamp

1. Entrada catálogo Core (+ códice subscriptions si procede).
2. Evento `hu-forged.md` + sync índice/EDA.
3. Handler `tracker_stamp`: `HU_Forged` ≈ `PBI_Forged` (backlog + comentario + column sync); file sink incluye `HU_Forged` → `COLUMN_BACKLOG`.
4. Tests unitarios/lab stamp HU (mock outbound).

### A4 — Kaizen_Idea_Captured contrato explícito

1. Endurecer contrato evento (sumidero intencional; no forja automática).
2. Allowlist audit / nota de catálogo si hace falta para no generar Kaizen espurio por `EMPTY_SUBSCRIBERS`.
3. Test(s) execute-process: emit/dispatch con `[]` → success + sin `System_Fracture`; aserción de política de sumidero.

### Post-fix documental

1. Evolution AUT-63.
2. Auditoría vigente → **v1.2.0** (§9 actualiza A1–A4 cumplidos; A5 intacto).
3. Cascada `implementation.md` / `execution.md` / `validacion.md`.

## 6. Touchpoints

| Artefacto | Mutación |
|-----------|----------|
| `SddIA/core/event-domain-subscriptions.json` | A1 intent; A3 `HU_Forged`; A4 nota/política si aplica |
| `SddIA/actions/materialize-kaizen-alert-doc.md` (+ index si texto) | A1 dual |
| `SddIA/agents/cumulo.md`, `cumulo.instructions.json` | A1 dual |
| `…/process/{feature,bug-fix,refactorization}.md` (+ index) | A2 intents + hash |
| `SddIA/engine/…/tracker_stamp.rs` (+ tests) | A3 |
| `…/codex-software-engineering/events/hu-forged.md` (+ index) | A3 |
| `…/events/domain/kaizen-idea-captured.md` | A4 |
| `SddIA/tools/event-bus-audit` (si allowlist) | A4 opcional |
| `SddIA/engine/execute-process/tests/*` | A1/A3/A4 |
| `docs/audits/ciclo-vida-software-sddia.md` | D0 (1.1.0 ya); v1.2.0 post-fix |
| `docs/audits/ciclo-vida-software-sddia-v1.0.0.md` | **sin cambio** |
| `SddIA/evolution/72169010-d232-41f9-a877-44beb83dafc2.md` | Alta |
| `docs/fixes/aut-63-lifecycle-genome-parity/*` | Cascada bug-fix |

## 7. Fuera de alcance

- A5 (código).
- Auto-forja desde `Kaizen_Idea_Captured`.
- Cerrar AUT-63 en Linear desde Tekton.
- Escribir TODOs bajo `docs/todos/`.
- Reescritura de la auditoría v1.0.0.
- Rehabilitar procesos Cerbero ajenos (AUT-62).

## 8. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC1 | Catálogo + acción (+ Cúmulo) describen el dual tracker/file de `Kaizen_Alert_Required` sin contradecir `materialize_kaizen_alert_doc.rs`. |
| AC2 | Intents/prosa de cierre en `feature` / `bug-fix` / `refactorization` alineados a `done_gate` + norma v1.3.1; `hash_signature` refrescado en los tres. |
| AC3 | `HU_Forged` suscrito a `tracker-stamp`; stamp confirma backlog + comentario idempotente; test verde. |
| AC4 | `Kaizen_Idea_Captured`: suscriptor real **o** contrato explícito de sumidero + test execute-process verde. |
| AC5 | Par auditoría 1.0.0 frozen + 1.1.0 vigente en diseño; tras fix, §9 de la vigente refleja A1–A4 cerrados en **v1.2.0** sin tocar v1.0.0. |
| AC6 | Ningún fichero nuevo bajo `docs/todos/` creado por este ciclo. |
| AC7 | Evolution `72169010-…` referencia AUT-63 / `PBI-b0d4af60`. |

## 9. Métrica de mejora

Cero contradicciones documentadas genoma↔código en A1–A4; operadores/agentes leen el mismo dual que ejecuta el Rust; HU recibe el mismo sello de forja que el PBI.

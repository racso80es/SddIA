---

feature_name: man-46-hito-no-cierra-padre
created: "2026-11-21"
updated: "2026-11-21"
process: feature
purpose: Plan ejecutable — hito ≠ Done del padre / tracker (KZ-3 / MAN-46)
branch_name: feat/man-46-hito-no-cierra-padre
persist_ref: docs/features/man-46-hito-no-cierra-padre
pbi_ref: MAN-46
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-74cd5db0
agent: dedalo
phase: diseno-blueprint
version_plan: "1.0.0"
status: blueprint-ready
audit_ref: docs/audits/man-43-h1-milestone-razonamiento-20261111.md
kaizen_ref: KZ-3
norm_ref: features-documentation-pattern@1.3.2
phases:
  - H0
  - H1
  - H2
  - H3
  - H4
  - H5
execution_id: "c680c700-3963-432f-9b34-71c9ea8cc226"
---
# Plan de ejecución — man-46-hito-no-cierra-padre

Consumir `spec.md` + `clarify.md` en solo lectura. No reabrir L-SUCCESS…L-SCOPE-CUT.
Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`.
Orden: **H0 → H1 → H2 → H3 → H4 → H5**.

## H0 — Preflight

| # | Acción | Criterio |
|---|--------|----------|
| 0.1 | Rama `feat/man-46-hito-no-cierra-padre` vía git-manager | Creable / existente desde `main` |
| 0.2 | Confirmar huecos vivos: roll-up HU incondicional; `ReviewCiGreen`→Done; `delivery_state: success` sin gate AC; `apto_coherence` no cubre Done/narración success | Snapshot en `execution.md` |
| 0.3 | Inventariar call sites: `tracker_stamp.rs` (`maybe_complete_hu_when_children_done`, `stamp_done_with_hu_roll_up`), `lifecycle_phase_action.rs`, `accept_pr.rs`, writers `validacion`/`delivery_close`, frontera con `apto_coherence` | Lista en `implementation.md` |

## H1 — Módulo `parental_close_gate` + roll-up

| # | Acción | Paths |
|---|--------|-------|
| 1.1 | Crear `parental_close_gate.rs` con `assert_parental_close_allowed` + `ParentalCloseFault` | `engine/parental_close_gate.rs` |
| 1.2 | Implementar chequeo AC abiertos del persist_ref padre vs refine sellado previo | **D-API** / **D-REFINE-PRIOR** |
| 1.3 | Condicionar `maybe_complete_hu_when_children_done`: sin roll-up si padre AC abiertos sin laudo | `tracker_stamp.rs` / **D-ROLLUP** |
| 1.4 | Mapear errores → friction_ids del spec §3.6 | **D-FRICTION-IDS** |
| 1.5 | Tests unitarios `ka_mile_1` (tempdirs, sin red) | roll-up denegado |
| 1.6 | Registrar `mod` en `engine/mod.rs` | Compila |

**Gate H1:** unitario KA-MILE-1 (roll-up) verde en aislamiento.

## H2 — Done lifecycle + success parental

| # | Acción | Paths |
|---|--------|-------|
| 2.1 | Auditar/cortar side-effects que muevan el padre a Done al cerrar el hijo (`ReviewCiGreen` / accept-pr) | **D-LIFECYCLE** |
| 2.2 | Gate `delivery_state: success` del persist_ref padre: AC verdes **o** refine previo | **D-SUCCESS-GATE** |
| 2.3 | Rechazar narración «ciclo cerrado» / success con tokens hito sin refine | **D-NARRATIVE** / KA-MILE-2 |
| 2.4 | Honor `SDDIA_LAB_SKIP_PARENTAL_CLOSE_GATE` | **D-LAB** |
| 2.5 | Sin `System_Fracture_Detected` | **D-NO-FRACTURE** |

**Gate H2:** fixture KA-MILE-2 rechaza narración/success parental prematuro.

## H3 — Refine previo + frontera MAN-44

| # | Acción | Paths |
|---|--------|-------|
| 3.1 | Detector de refine sellado previo (timestamp/event_id &lt; merge); fail-closed si inestable | **D-REFINE-PRIOR** / **L-TRUTH** |
| 3.2 | Fixture KA-MILE-3: laudo previo permite success acotado; rewrite misma pasada denegado | **AC-3** |
| 3.3 | Helper compartido de tokens hito solo si hace falta; no debilitar `apto_coherence` | **D-OVERLAP-44** |
| 3.4 | Tests `ka_mile_3` | unit/integration |

**Gate H3:** **AC-3** cubierto; MAN-44 intacto.

## H4 — Códex / norma + fixtures documentales

| # | Acción | Nota |
|---|--------|------|
| 4.1 | Nota en `feature.md` (hito ≠ Done padre; success = AC\|refine) + refresh `hash_signature` | **D-CODEX-NORM** |
| 4.2 | Parche mínimo norma `features-documentation-pattern` si el párrafo falta | bump menor solo con materialización real |
| 4.3 | Materializar `_fixtures/ka-mile-{1,2,3}/` (padre/hijo, validacion, refine stubs) | Reproducibles en PR |
| 4.4 | Redactar contrafactual KA-MILE-4 bajo persist_ref | Citar auditoría KZ-3; no revert #453 |
| 4.5 | `implementation.md` touchpoints reales | Tras forja |

**Gate H4:** **AC-2…AC-4** documentados / cubiertos.

## H5 — Verificación y handoff Argos

| # | Acción | Criterio |
|---|--------|----------|
| 5.1 | `cargo test -p execute-process --lib` filtro `parental_close` / `ka_mile_` | Verde |
| 5.2 | Clippy acotado al crate si el régimen lo exige | Verde |
| 5.3 | Refresh process integrity tras bump YAML | `verify-process-integrity` OK |
| 5.4 | Evidencia en `execution.md` (sin inventar éxito) | **L-TRUTH** |
| 5.5 | Handoff Argos / `validacion.md` | Checks AC-1…4 |

**Gate H5:** ACs del `objectives.md` verdes con evidencia.

## Matriz AC ↔ hijos

| AC | Hijos | Verificación |
|----|-------|--------------|
| AC-1 (KA-MILE-1) | H1, H2 | padre ≠ Done; sin success parental |
| AC-2 (KA-MILE-2) | H2, H4 | narración/hito rechazados |
| AC-3 (KA-MILE-3) | H3, H4 | refine previo vs rewrite ad hoc |
| AC-4 (KA-MILE-4) | H4 | contrafactual doc |

## Stop conditions (blocked)

- Imposible distinguir issue hijo vs padre en lifecycle de forma estable → **blocked** con evidencia; no cerrar el padre «por si acaso».
- Detector de refine sellado no materializable → **blocked** o fail-closed documentado; no inventar lineage (**L-TRUTH**).
- Process hash / integrity falla tras edición YAML → corregir o **blocked**; no skipear integridad en productivo.
- Tentación de emitir `System_Fracture_Detected` o de tocar MAN-44/45/KZ-4 → rechazo de diseño.
- Bypass productivo vía lab env → **blocked**.

## Handoff Tekton

1. Leer `spec.md` §§1–5 y este plan; no reabrir laudos Mayeuta.
2. Ejecutar H0→H5; materializar `implementation.md` + `execution.md`.
3. No tocar hermanos MAN-44/45 ni deuda KZ-4 / EventManager de MAN-43.
4. No escribir bajo `docs/todos/`.
5. Al verde local, cierre vía Argos + DCC; la norma parental debe estar activa antes de narrar éxito de este feature (AC del propio `persist_ref`).

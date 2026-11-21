---

feature_name: man-45-accept-pr-ppr-tree-sha
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Plan ejecutable — aduana PPR tree_sha en accept-pr (KZ-2 / MAN-45)
branch_name: feat/man-45-accept-pr-ppr-tree-sha
persist_ref: docs/features/man-45-accept-pr-ppr-tree-sha
pbi_ref: MAN-45
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-75d12fd0
agent: dedalo
phase: diseno-blueprint
version_plan: "1.0.0"
status: blueprint-ready
audit_ref: docs/audits/man-43-h1-milestone-razonamiento-20261111.md
kaizen_ref: KZ-2
norm_ref: features-documentation-pattern@1.3.2
phases:
  - H0
  - H1
  - H2
  - H3
  - H4
  - H5
execution_id: "52da40f9-4dde-44a6-88d6-ce6efb0a1ec8"
---
# Plan de ejecución — man-45-accept-pr-ppr-tree-sha

Consumir `spec.md` + `clarify.md` en solo lectura. No reabrir L-ATTEST…L-DEDUPE.
Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`.
Orden: **H0 → H1 → H2 → H3 → H4 → H5**.

## H0 — Preflight

| # | Acción | Criterio |
|---|--------|----------|
| 0.1 | Rama `feat/man-45-accept-pr-ppr-tree-sha` vía git-manager | Creable / existente desde `main` |
| 0.2 | Confirmar huecos vivos: Auditoría Genómica no bloquea; `validate_for_accept_pr(..., None)`; sin `issuer_process` | Snapshot en `execution.md` |
| 0.3 | Inventariar call sites: `accept_pr.rs`, `qa_attestation.rs`, `residual_runner`, dedupe en `route_domain_core` | Lista en `implementation.md` |

## H1 — Contrato atestación + emisor

| # | Acción | Paths |
|---|--------|-------|
| 1.1 | Añadir `issuer_process: "pull-request-review"` en `try_write_from_ppr` | `qa_attestation.rs` |
| 1.2 | Exigir emisor en `validate_attestation_value` | **D-EMITTER** / **D-COMPAT** |
| 1.3 | Resolver `expected_tree_sha` pre-merge vs post-merge | **D-TREE-PRE** |
| 1.4 | Pasar `delta_class` real (fail-closed → `active` si ambiguo) | **D-DELTA** |
| 1.5 | Mapear errores → friction_ids del spec §3 | **D-FRICTION-IDS** |
| 1.6 | Tests unitarios `ka_ppr_1`…`ka_ppr_3` (tempdirs, sin red) | `qa_attestation` tests |

**Gate H1:** unitarios de validación/emisión verdes en aislamiento.

## H2 — Puerta bloqueante en `accept-pr`

| # | Acción | Paths |
|---|--------|-------|
| 2.1 | Invocar `validate_for_accept_pr` (API endurecida) en «Auditoría Genómica» de forma **bloqueante** | `accept_pr.rs` |
| 2.2 | Presented solo trazabilidad; fallo atestación ⇒ `abortado` sin merge | **D-GATE** / **L-GENOMIC** |
| 2.3 | Rechazar caminos que acrediten solo `skipped-ppr-dedupe` | **D-DEDUPE** / KA-PPR-4 |
| 2.4 | Honor `SDDIA_LAB_SKIP_PPR_ATTESTATION_ACCEPT` | **D-LAB** |
| 2.5 | Sin `System_Fracture_Detected` | **D-NO-FRACTURE** |

**Gate H2:** fixture KA-PPR-1 aborta accept-pr antes de Fusión Soberana.

## H3 — Códex + paridad emisión

| # | Acción | Paths |
|---|--------|-------|
| 3.1 | Nota en `accept-pr.md` (Auditoría Genómica exige atestación PPR) + refresh `hash_signature` | process YAML |
| 3.2 | Verificar `residual_runner` → `try_write_from_ppr` materializa `issuer_process` | `residual_runner.rs` |
| 3.3 | Test de integración mínimo: PPR aprobado escribe atestación aceptada por el gate | `ka_ppr` / accept-pr |

**Gate H3:** integrity de process OK; emisión canónica pasa el gate.

## H4 — Fixtures documentales + contrafactual

| # | Acción | Nota |
|---|--------|------|
| 4.1 | Materializar `_fixtures/ka-ppr-{1,2,3,4}/` (atestaciones / stubs dedupe) | Reproducibles en PR |
| 4.2 | Redactar contrafactual KA-PPR-5 bajo persist_ref | Citar auditoría KZ-2; no revert #453 |
| 4.3 | `implementation.md` touchpoints reales | Tras forja |
| 4.4 | Evaluar **D-EMITTER-LINEAGE**; si no estable, documentar drift y no inventar | **L-TRUTH** |

**Gate H4:** **AC-2…AC-5** documentados / cubiertos.

## H5 — Verificación y handoff Argos

| # | Acción | Criterio |
|---|--------|----------|
| 5.1 | `cargo test -p execute-process --lib` filtro `qa_attestation` / `ka_ppr_` / accept-pr gate | Verde |
| 5.2 | Clippy acotado al crate si el régimen lo exige | Verde |
| 5.3 | Refresh process integrity tras bump YAML | `verify-process-integrity` OK |
| 5.4 | Evidencia en `execution.md` (sin inventar éxito) | **L-TRUTH** |
| 5.5 | Handoff Argos / `validacion.md` | Checks AC-1…5 |

**Gate H5:** ACs del `objectives.md` verdes con evidencia.

## Matriz AC ↔ hijos

| AC | Hijos | Verificación |
|----|-------|--------------|
| AC-1 (KA-PPR-1) | H1, H2 | aborto sin atestación |
| AC-2 (KA-PPR-2) | H1, H2, H4 | docs-only / forjado |
| AC-3 (KA-PPR-3) | H1, H2, H4 | tree mismatch |
| AC-4 (KA-PPR-4) | H2, H4 | dedupe ≠ QA |
| AC-5 (KA-PPR-5) | H4 | contrafactual doc |

## Stop conditions (blocked)

- Imposible resolver `tree_sha` de `source_branch` de forma estable → **blocked** con evidencia; no saltar a Presented.
- Process hash / integrity falla tras edición YAML → corregir o **blocked**; no skipear integridad en productivo.
- Tentación de aceptar JSON forjado «con issuer escrito a mano» sin cadena PPR → rechazo de diseño; reforzar **D-EMITTER** / lineage si hace falta.
- Cualquier bypass productivo vía lab env → **blocked**.

## Handoff Tekton

1. Leer `spec.md` §§1–5 y este plan; no reabrir laudos Mayeuta.
2. Ejecutar H0→H5; materializar `implementation.md` + `execution.md`.
3. No tocar hermanos MAN-44/46 ni deuda KZ-4.
4. No escribir bajo `docs/todos/`.
5. Al verde local, cierre vía Argos + DCC; la aduana PPR debe estar activa en accept-pr antes del merge de este feature.

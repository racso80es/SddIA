---
feature_name: man-44-rechazar-apto-incoherente
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Plan ejecutable — aduana coherencia APTO (KZ-1 / MAN-44)
branch_name: feat/man-44-rechazar-apto-incoherente
persist_ref: docs/features/man-44-rechazar-apto-incoherente
pbi_ref: MAN-44
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-8205539f
agent: dedalo
phase: diseno-blueprint
version_plan: "1.0.0"
status: blueprint-ready
audit_ref: docs/audits/man-43-h1-milestone-razonamiento-20261111.md
kaizen_ref: KZ-1
norm_ref: features-documentation-pattern@1.3.1
execution_id: "e049f4db-4e0e-41d0-b651-e9b9de645f71"
phases:
  - H0
  - H1
  - H2
  - H3
  - H4
  - H5
---
# Plan de ejecución — man-44-rechazar-apto-incoherente

Consumir `spec.md` + `clarify.md` en solo lectura. No reabrir L-COHERENCE…L-RESOLUTION.
Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`.
Orden: **H0 → H1 → H2 → H3 → H4 → H5**.

## H0 — Preflight

| # | Acción | Criterio |
|---|--------|----------|
| 0.1 | Rama `feat/man-44-rechazar-apto-incoherente` vía git-manager | Creable / existente desde `main` |
| 0.2 | Confirmar huecos vivos: `validacion_ready_for_seal` sin execution/plan; `validate_for_accept_pr_merge` solo digest | Snapshot en `execution.md` |
| 0.3 | Inventariar call sites DCC sello + accept-pr fusión | Lista en `implementation.md` |

## H1 — Módulo `apto_coherence`

| # | Acción | Paths |
|---|--------|-------|
| 1.1 | Crear `apto_coherence.rs` con `validate_apto_coherence` + `CoherenceFault` | `engine/apto_coherence.rs` |
| 1.2 | Implementar C2 (execution blocked/partial) | §3.2 spec |
| 1.3 | Implementar C3 (fases abiertas) | §3.3 spec |
| 1.4 | Implementar C4 (denylist resolución / elevadores) | §3.4 spec |
| 1.5 | Tests unitarios con tempdirs (sin red) | `ka_close_1`, `ka_close_2`, `ka_close_3` |
| 1.6 | Registrar `mod` en `engine/mod.rs` | Compila |

**Gate H1:** tests unitarios del módulo verdes en aislamiento.

## H2 — Puerta pre-sello (DCC)

| # | Acción | Paths |
|---|--------|-------|
| 2.1 | Invocar contraste en `capsule_diff_seal_validacion` antes de sellar | `delivery_close.rs` |
| 2.2 | Mapear fault → `status: blocked` + friction_id + `operator_hint` | Envelope DCC |
| 2.3 | Extender `dcc_pre_pr_qa_block_suppresses_fracture` con los 3 friction_id | Sin `System_Fracture_Detected` |
| 2.4 | Honor `SDDIA_LAB_SKIP_APTO_COHERENCE` | Lab only |
| 2.5 | Actualizar nota de fase en `delivery-close-cycle.md` + refresh `hash_signature` | process YAML |

**Gate H2:** fixture KA-CLOSE-2 (o unit simulado) bloquea sello sin fractura.

## H3 — Puerta pre-fusión (`accept-pr`)

| # | Acción | Paths |
|---|--------|-------|
| 3.1 | Invocar contraste en `validate_for_accept_pr_merge` (o wrapper) cuando hay APTO | `diff_seal.rs` / `accept_pr.rs` |
| 3.2 | Asegurar aborto sin merge (`diff_seal_mismatch` no sustituye; error propio coherencia) | Fusión Soberana |
| 3.3 | Test KA-CLOSE-1: blocked execution ⇒ accept-pr no fusiona | `ka_close_1` |
| 3.4 | Nota en `accept-pr.md` + refresh hash | process YAML |

**Gate H3:** **AC-1** + **AC-5** (mismo módulo en ambas puertas).

## H4 — Norma + fixtures documentales

| # | Acción | Nota |
|---|--------|------|
| 4.1 | Parche `features-documentation-pattern` (§ resoluciones no elevan APTO) | **D-NORM-BUMP** si aplica |
| 4.2 | Materializar fixtures `_fixtures/ka-close-{1,2,3}/` (validacion/execution/plan mínimos) | Reproducibles en PR |
| 4.3 | Redactar contrafactual KA-CLOSE-4 bajo persist_ref | Citar auditoría KZ-1; no revert #453 |
| 4.4 | `implementation.md` touchpoints reales | Tras forja |

**Gate H4:** **AC-3** + **AC-4** documentados.

## H5 — Verificación y handoff Argos

| # | Acción | Criterio |
|---|--------|----------|
| 5.1 | `cargo test -p execute-process --lib` filtro `apto_coherence`/`ka_close_` | Verde |
| 5.2 | Clippy acotado al crate si el régimen del repo lo exige | Verde |
| 5.3 | Refresh process integrity tras bumps YAML | `verify-process-integrity` OK |
| 5.4 | Evidencia en `execution.md` (sin inventar éxito) | **L-TRUTH** |
| 5.5 | Handoff Argos / `validacion.md` | Checks AC-1…5 |

**Gate H5:** ACs del `objectives.md` verdes con evidencia.

## Matriz AC ↔ hijos

| AC | Hijos | Verificación |
|----|-------|--------------|
| AC-1 (KA-CLOSE-1) | H1, H3 | accept-pr aborta |
| AC-2 (KA-CLOSE-2) | H1, H2, H3 | fricción sin fractura |
| AC-3 (KA-CLOSE-3) | H1, H4 | denylist resolución |
| AC-4 (KA-CLOSE-4) | H4 | contrafactual doc |
| AC-5 (Puertas) | H2, H3 | doble call site |

## Stop conditions (blocked)

- Imposible parsear frontmatter de forma estable en fixtures reales → **blocked** con evidencia; no debilitar C2.
- Process hash / integrity falla tras edición YAML → corregir o **blocked**; no skipear integridad en productivo.
- Cualquier tentación de emitir `System_Fracture_Detected` → rechazo de diseño; solo friction_id.

## Handoff Tekton

1. Leer `spec.md` §§1–5 y este plan; no reabrir laudos Mayeuta.
2. Ejecutar H0→H5; materializar `implementation.md` + `execution.md`.
3. No tocar hermanos MAN-45/46 ni deuda KZ-4.
4. No escribir bajo `docs/todos/`.
5. Al verde local, cierre vía Argos + DCC con la aduana ya activa.

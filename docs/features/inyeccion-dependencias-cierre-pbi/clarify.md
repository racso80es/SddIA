---
feature_name: inyeccion-dependencias-cierre-pbi
created: "2026-07-22"
purpose: Estabilización Done global PBI-042 — archivo PBI padre + cascada documental de cierre (R15 / AC-DONE)
branch_name: feat/inyeccion-dependencias-cierre-pbi
persist_ref: docs/features/inyeccion-dependencias-cierre-pbi
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-CIERRE-PBI
execution_id: d4e8f1a3-6c7b-4d9e-a2f0-3b4c5d6e7f8a
phase: mayeuta-stabilization
agents: mayeuta
---

# Clarificación — PBI-042 Done global (archivo PBI + cierre documental)

## D0 — Semilla

- **PBI:** `docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md` (v1.2.0; Hito 6 entregado PR #140 merge `4203848`).
- **Ciclo:** feature `inyeccion-dependencias-cierre-pbi` · rama `feat/inyeccion-dependencias-cierre-pbi`.
- **Alcance declarado:** Done global — **R15** (archivo PBI-042 padre). Criterio **producto:** **AC-DONE**. Regresión: no reabrir R1–R14; smoke documental trazabilidad MVP→H6.
- **Precedente cerrado Hito 6:** `docs/features/inyeccion-dependencias-barrido-creators` — R14 creators residuales; finalize remite «Archivo PBI-042 padre = Done global / laudo Racso».
- **Precedente cerrado Hito 5:** `docs/features/inyeccion-dependencias-migracion-catalogo` — R11 sello; R12 ola; R13 omitido (Q6-A).
- **Precedente cerrado Hito 4:** `docs/features/inyeccion-dependencias-envelope-homologacion` — envelope Cerbero; baseline 8 ED.
- **Precedente cerrado Hito 3:** `docs/features/inyeccion-dependencias-gobernanza-asincronia` — Cerbero RBAC, piloto EDA, `proc:git-sync`, output validator.
- **Precedente cerrado Hito 2 / MVP:** resolución ciega + gate DI + taxonomía + DLQ.
- **Laudo Racso (inicio ciclo):** **L-PBI-LOC se levanta** — archivo del PBI padre es el vector soberano de este ciclo (no diferido).
- **Normas cierre:** `features-documentation-pattern` v1.2.0 · `task-closure-documental` — Done = un PR + `validacion.md` APTO (`pbi_archived: true`) + PBI en `docs/todos/done/`.
- **Runtime DI:** **intacto** — sin mutación de gate/resolver/Cerbero/envelope/output validator/taxonomía/bindings.
- **Fuera de alcance:** GesFer / Paciente 0; Fractura Core F1; EDA-only total sync→async; ola H7+ ED residuales; altas al Códice; reescritura runtime DI.

## D1 — Matriz de validación (residual × estado post-Hito 6)

| Afirmación / residual | Estado actual | Evidencia |
|------------------------|---------------|-----------|
| MVP R1–R3 / AC-P1–P3 | **Hecho (main)** | `inyeccion-dependencias-capacidades` |
| Hito 2 R1–R4 / AC-R1/R2 | **Hecho (main)** | PR #127 merge `60c4635` |
| Hito 3 R5–R8 / AC-R5–R8 | **Hecho (main)** | PR #128 merge `51fd434` |
| Hito 4 R9–R10 / AC-R9/R10 | **Hecho (main)** | PR #136 merge `6b0e98c` |
| Hito 5 R11–R12 / AC-R11/R12 | **Hecho (main)** | PR #138 merge `66a0f71` |
| Hito 5 R13 | **Omitido (Q6-A)** | No reabrir |
| Hito 6 R14 / AC-R14 | **Hecho (main)** | PR #140 merge `4203848` |
| Archivo PBI-042 padre (R15) | **Ausente** | PBI en `pending/`; `status: abierto`; H6 `pbi_archived: false` |
| Cascada documental cierre bajo este `persist_ref` | **Parcial** | Solo `objectives.md` semilla orquestador |
| Evolution cierre multi-hito MVP→H6 | **Ausente** | Pendiente este ciclo |
| Ola H7+ ED residuales | **Fuera** | Finalize H6: diferidas salvo laudo |
| Mutación genoma DI | **Prohibida** | Solo docs / evolution / PBI |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-HIT-CLOSE-SCOPE** | Este ciclo = **R15** + **AC-DONE** + cascada documental + evolution de cierre. Sin blast-radius de genoma DI. Sin ola H7. |
| **L-PBI-LOC-LIFT** | Laudo Racso levanta **L-PBI-LOC**: archivar PBI-042 es obligatorio para Done. Mover `pending/` → `done/` (mismo `document_id`); `status: cerrado`; frontmatter hito6+cierre coherente. |
| **L-R15-ARCHIVE** | Archivo = movimiento físico del `.md` PBI + metadatos de cierre; **no** reescribir historia R1–R14 como «pendiente». |
| **L-DOC-CASCADE** | Cascada bajo `persist_ref`: clarify → objectives → spec → plan → implementation → execution → validacion (`global: APTO`, `pbi_archived: true`, `branch` coherente). Norma v1.2.0 + `task-closure-documental`. |
| **L-SINGLE-PR** | Un único PR de cierre incluye código documental + PBI en `done/` + `validacion.md` APTO. Prohibido segundo PR `docs/cerrar-pbi-*`. |
| **L-NO-GENOME** | Prohibida mutación de gate/resolver/Cerbero/envelope/output validator/taxonomía/`capability-bindings`/ED DI salvo paths de documentación, evolution y PBI. |
| **L-NO-H7** | ED residuales no listadas (entity-manager, audits, routes, …) = **fuera**; Ola H7+ solo con laudo Racso posterior. |
| **L-R13-SEAL** | R13 permanece **omitido** (Q6-A); no reabrir en cierre. |
| **L-REG-DOC** | Regresión = smoke documental de trazabilidad MVP→H6 en `objectives`/`clarify` + no reabrir R1–R14. Sin exigir re-ejecución de suite DI salvo que Dedalo/Argos lo acoten como evidencia de no-regresión documental. |
| **L-EVOLUTION** | Registrar en `SddIA/evolution/` vinculando `execution_id` / UUID del ciclo + cierre multi-hito MVP→H6. |
| **L-GESFER** | Ortogonal; no absorber Paciente 0 / Fractura Core en este `persist_ref`. |

## D3 — Ambigüedades acotadas (handoff Dedalo — no diseño Mayeuta)

| # | Pregunta | Opciones admisibles | Criterio de cierre |
|---|----------|---------------------|--------------------|
| **Q1** | Frontmatter PBI al archivar | Campos mínimos vs auditoría extendida (`hito6_*` ya presentes; añadir `closed_at` / `close_feature` / `close_pr` opc.) | `document_id` inmutable; `status: cerrado`; trazabilidad MVP→H6 legible |
| **Q2** | Estructura `spec.md` / `plan.md` (ciclo docs-only) | (A) spec=contrato documental AC-DONE + plan=fases archivo/cascada/evolution; (B) spec mínimo + plan checklist único | Sin fases de mutación genoma; Dedalo elige densidad |
| **Q3** | Evolution | (A) un registro de cierre multi-hito; (B) índice + entradas por hito referenciadas | Vincula `execution_id` `d4e8f1a3-…`; auditable |
| **Q4** | Orden de materialización en rama | (A) cascada docs → mover PBI → validacion; (B) mover PBI temprano + validacion al final | `pbi_archived: true` solo si PBI ya en `done/` en la misma rama |
| **Q5** | Evidencia Argos AC-DONE | (A) asserts paths + frontmatter; (B) checklist manual + diff | Reproducible; sin Shell IDE crudo como SSOT |
| **Q6** | Mencionar residual H7 en PBI archivado | (A) sección «fuera / diferido» explícita; (B) omitir (solo Done alcanzado) | No dejar ambigüedad de «PBI incompleto»; H7 ≠ blocker de Done |

## D4 — Criterios producto estabilizados (este ciclo)

| ID | Criterio | Verificación esperada (Argos) |
|----|----------|-------------------------------|
| **AC-DONE** | PBI-042 en `docs/todos/done/` (mismo `document_id`) + `status: cerrado` + `validacion.md` con `global: APTO` y `pbi_archived: true` en el **mismo PR** de cierre; cascada documental completa bajo `persist_ref`; evolution de cierre presente | Diff: PBI movido; frontmatter coherente; `validacion.md` gate; sin genoma DI tocado |

Regresión obligatoria (documental / no-reapertura):

| ID | Criterio | Origen |
|----|----------|--------|
| **AC-REG-R1-R14** | No reabrir ni marcar incompletos R1–R14 (R13 sigue omitido Q6-A) | MVP→H6 |
| **AC-REG-TRACE** | Smoke documental: `objectives`/`clarify` trazan cadena MVP→H2→H3→H4→H5→H6→cierre | Este ciclo |

## D5 — Veredicto

**ok** — Requisitos de Done global termodinámicamente estables. Handoff a Dedalo: diseño cierre documental (Q1–Q6) → `spec.md` / `plan.md` **sin** blast-radius de genoma.

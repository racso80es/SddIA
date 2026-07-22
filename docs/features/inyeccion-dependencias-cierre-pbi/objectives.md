---
feature_name: inyeccion-dependencias-cierre-pbi
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-cierre-pbi
persist_ref: docs/features/inyeccion-dependencias-cierre-pbi
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-CIERRE-PBI
execution_id: d4e8f1a3-6c7b-4d9e-a2f0-3b4c5d6e7f8a
phase: tekton-execution-blocked
agents: tekton
---

# Objetivos — inyeccion-dependencias-cierre-pbi

## Misión

Materializar el **Done global** del PBI-042 (post-Hito 6 PR #140 merge `4203848`): **archivar el PBI padre** y completar la **cascada documental de cierre** bajo este `persist_ref`, sin mutar el genoma DI ni abrir ola H7.

## Estado de partida (innegociable post-Hito 6)

| Vector | Estado |
|--------|--------|
| MVP + Hitos 2–6 (R1–R14; R13 omitido Q6-A) | **Hecho en main** |
| Hito 6 R14 barrido creators | **Hecho** — PR #140 merge `4203848` |
| Residual finalize H6: archivo PBI-042 | **Este ciclo** (laudo Racso) |
| Residual finalize H6: más ED / EDA-only total | **Fuera** (H7+ / salvo laudo) |
| L-PBI-LOC | **Levantado** — archivo = vector soberano |

## Vectores soberanos (este ciclo)

1. **R15 / Done global — Archivar PBI-042:** mover `docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md` → `docs/todos/done/` (mismo `document_id`); `status: cerrado`; frontmatter hito6+cierre coherente (**L-PBI-LOC-LIFT**, **L-R15-ARCHIVE**).
2. **Cascada documental:** clarify / objectives / spec / plan / implementation / execution / validacion con `global: APTO` y `pbi_archived: true` (`features-documentation-pattern` v1.2.0; `task-closure-documental`) (**L-DOC-CASCADE**, **L-SINGLE-PR**).
3. **Evolution:** registro vinculando `execution_id` `d4e8f1a3-6c7b-4d9e-a2f0-3b4c5d6e7f8a` + cierre multi-hito MVP→H6 (**L-EVOLUTION**).
4. **Sin genoma DI / sin H7:** no tocar gate/resolver/Cerbero/envelope/output validator/taxonomía/bindings; ED residuales diferidas (**L-NO-GENOME**, **L-NO-H7**).
5. **Criterio producto:** **AC-DONE** — PBI en `done/` + `validacion.md` APTO + `pbi_archived: true` en el mismo PR.

## Criterios de aceptación — producto Done global

| ID | Criterio |
|----|----------|
| **AC-DONE** | PBI-042 en `docs/todos/done/` + `validacion.md` `global: APTO` + `pbi_archived: true` en el mismo PR; cascada + evolution presentes; sin mutación genoma DI |
| **AC-REG-R1-R14** | No reabrir R1–R14 (R13 permanece omitido Q6-A) |
| **AC-REG-TRACE** | Smoke documental de trazabilidad MVP→H6 en objectives/clarify |

## Restricciones y leyes aplicadas

- Git exclusivamente vía `skill:git-manager`.
- Cierre documental: un PR; prohibido segundo PR post-merge solo docs.
- `pbi_archived: true` solo si el PBI ya está en `done/` en la rama del PR.
- Normas: `features-documentation-pattern` v1.2.0, `task-closure-documental`, `external-ai-constraints`, `capsule-json-io`, `CONSTITUTION_CORE` Filtro A.
- KM / `docs/todos/`: archivo del PBI-042 es mandato explícito de este ciclo (no semilla Kaizen nueva).

## Fuera de alcance (explícito)

| Ítem | Destino |
|------|---------|
| GesFer / Paciente 0 | Otro PBI |
| Fractura Core F1 | Otro `persist_ref` |
| EDA-only total sync→async | Fuera salvo laudo Racso |
| Ola H7+ ED residuales | Fuera salvo laudo Racso |
| Altas al Códice / reescritura runtime DI | Prohibido (**L-NO-GENOME**) |

## Ambigüedades Dedalo — cerradas

Ver `spec.md` §3: **Q1** mínimo+auditoría · **Q2-A** · **Q3-A** un evolution · **Q4-A** cascada→evolution→PBI→validacion · **Q5-A** asserts · **Q6-A** H7 diferido explícito.

## Handoff

Tekton **BLOCKED** en cleanup `pending/` (Shell/Delete rechazados). Materializado: `implementation.md`, evolution `d4e8f1a3-…`, PBI en `done/` (cerrado), `execution.md`, `validacion.md` NO_APTO. Desbloqueo: borrar origen pending → Argos APTO.

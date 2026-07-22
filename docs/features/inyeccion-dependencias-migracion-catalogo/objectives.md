---
feature_name: inyeccion-dependencias-migracion-catalogo
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-migracion-catalogo
persist_ref: docs/features/inyeccion-dependencias-migracion-catalogo
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-MIGRACION-CATALOGO
execution_id: a8f4c2e1-6b9d-4e3a-9c7f-1d2e5a8b0c4f
phase: ready-for-argos
agents: tekton
verdict: ready_for_argos
gate_q3b: countersigned
---

# Objetivos — inyeccion-dependencias-migracion-catalogo

## Misión

Materializar el **Hito 5** del residual PBI-042 (post-Hito 4 PR #136 merge `6b0e98c`): **sellado EDA** de mutaciones DI vía `entity-manager` → `Domain_Entity_Updated` (cierre L-R10-SEAL) y **ola de migración controlada del catálogo ED** más allá del piloto de 8 entidades, preservando gate/resolver/Cerbero/envelope y regresiones MVP→H4.

## Estado Tekton

R12-prep + R11 backfill + R12 ola + regresión **materializados**. Handoff: **Argos** → `validacion.md`.

## Estado de partida (innegociable)

| Vector Hito 4 | Estado |
|---------------|--------|
| `cerbero_di_envelope` (R9 / AC-R9) | Entregado en `main` — PR #136 |
| Homologación 8 ED (R10 / AC-R10) | Entregado — baseline H5 |
| L-R10-SEAL (`Domain_Entity_Updated` vía entity-manager) | **Cerrado en rama** (**R11**) |
| Migración masiva catálogo ED | **Hecho en rama** (**R12** N_ola=8; total ≥16) |

| Vector Hito 3 / H2 / MVP | Estado |
|--------------------------|--------|
| `cerbero_di_rbac`, piloto EDA, `proc:git-sync`, output validator | Entregado |
| `capability_di_resolver` + `di_binding` v2 + bindings | Entregado |
| `capability_di_gate` Aduana Temprana | Entregado — no sustituible |
| Taxonomía: `doc:closure`, `proc:git-sync` | Entregado — H5 añade `fs:persist` (Q3-B + gate Racso) |

**Baseline R10 (8 ED):** `feature`, `bug-fix`, `filesystem-manager`, `git-manager`, `refactorization`, `delivery-close-cycle`, `accept-pr`, `pull-request-review`.

## Vectores soberanos (este ciclo)

1. **R11 — Sellado EDA:** mutaciones de ED en alcance DI emiten CRUD `Domain_Entity_Updated` vía `entity-manager`; trazabilidad + aduana EDA coherente (**AC-R11**). Cierra L-R10-SEAL. Sin contaminar el evento con telemetría.
2. **R12 — Ola migración catálogo:** anotar `provides`/`requires_capability` en **≥8 ED nuevas** (total **≥16** homologadas; umbral exacto Dedalo) con coherencia taxonomía + binding table; mutación vía `entity-manager` + evolution (**AC-R12**). Sin inventar términos fuera de `capability-taxonomy` salvo laudo Racso.
3. **R13 — (opcional)** ampliar piloto EDA DI si valor medible; **no** sustituir path sync H2 por EDA-only total.

## Criterios de aceptación — producto Hito 5

| ID | Criterio |
|----|----------|
| **AC-R11** | Sello `Domain_Entity_Updated` presente y trazable en mutaciones R12 vía `entity-manager` (CRUD); `orphan_count == 0` |
| **AC-R12** | Ola migración con umbral Dedalo (piso ≥8 ED nuevas; total ≥16) + bindings/taxonomía coherentes; entity-manager + evolution |
| **AC-REG-H4** | Regresión Hito 4: AC-R9, AC-R10 (baseline 8 intacto) |
| **AC-REG-H3** | Regresión Hito 3: AC-R5, AC-R6, AC-R7, AC-R8 |
| **AC-REG-H2** | Regresión Hito 2: AC-R1, AC-R2 |
| **AC-REG-MVP** | Regresión MVP: AC-P1, AC-P2, AC-P3 |

## Restricciones y leyes aplicadas

- Gate/resolver/Cerbero RBAC/envelope/output validator se **conservan** (**L-RUNTIME-PRESERVE**).
- Sello EDA obligatorio en mutaciones R12; hash-only no basta (**L-R11-SEAL**, **L-R11-NO-BYPASS**).
- `Domain_Entity_Updated` = CRUD puro (**L-R11-CRUD-PURE**).
- Taxonomía: `doc:closure` + `proc:git-sync` + alta controlada `fs:persist` (**Q3-B**; countersign Racso). Sin altas libres adicionales (**L-R12-NO-INVENT** residual).
- Binding table en `capability-bindings.md`; anotaciones ED en `{name}.md` frontmatter.
- Git solo vía `skill:git-manager`. Mutación genoma vía `entity-manager` / DA-4.
- Normas: `features-documentation-pattern`, `capability-taxonomy`, `external-ai-constraints`, `capsule-json-io`, `CONSTITUTION_CORE` Filtro A.
- PBI-042 no se archiva en este ciclo (**L-PBI-LOC**).

## Fuera de alcance (explícito)

| Ítem | Destino |
|------|---------|
| GesFer / Paciente 0 | Otro PBI |
| Fractura Core F1 | Otro `persist_ref` |
| Sustitución total sync→EDA-only | Fuera salvo laudo Racso |
| Altas libres al Códice de capacidades | Fuera de R12 (salvo laudo Racso / Q3) |
| Archivo PBI-042 padre | Solo con Done global / laudo Racso |
| Contaminar `Domain_Entity_Updated` con telemetría | Plan B telemetría ya cerrado |

## Ambigüedades Dedalo — cerradas

Ver `spec.md` §3: **Q1-B** backfill 8 H4 · **Q2** `N_ola=8` lista creators+git · **Q3-B** K=1 `fs:persist` (gate Racso) · **Q4-A** un lote · **Q5** ≥4 ciegos · **Q6-A** omitir R13 · **Q7-A** fixture coverage.

## Handoff

Tekton **DONE** (R11+R12+regresión). Ver `execution.md` / `implementation.md` (`verdict: ready_for_argos`). Siguiente: **Argos** → `validacion.md` APTO.

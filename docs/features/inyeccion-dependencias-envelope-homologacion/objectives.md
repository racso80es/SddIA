---
feature_name: inyeccion-dependencias-envelope-homologacion
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-envelope-homologacion
persist_ref: docs/features/inyeccion-dependencias-envelope-homologacion
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-ENVELOPE-HOMOLOGACION
execution_id: c3d8f1a2-9e4b-4c7d-8f6a-1b2e3d4c5f6a
phase: dedalo-blueprint
agents: dedalo
---

# Objetivos — inyeccion-dependencias-envelope-homologacion

## Misión

Materializar el **Hito 4** del residual PBI-042 (post-Hito 3 PR #128 merge `51fd434`): **revalidación Cerbero del envelope `di_binding` empaquetado** y **homologación ampliada del catálogo ED**, preservando resolución ciega, gate pre-ignición, RBAC, piloto EDA y validación de salida de hitos anteriores.

## Estado de partida (innegociable)

| Vector Hito 3 | Estado |
|---------------|--------|
| `cerbero_di_rbac` post-gate (RBAC-only) | Entregado en `main` — AC-R5 |
| Piloto EDA `CapabilityDi_*` + `capability_di_reactor` | Entregado — AC-R6 |
| `proc:git-sync` en taxonomía + binding → `git-manager` | Entregado — AC-R7 |
| `capability_di_output_validator` post-cápsula | Entregado — AC-R8 |
| Revalidación schema envelope `di_binding` (Q2 Hito 3) | **Diferida** → este ciclo (**R9**) |

| Vector Hito 2 / MVP | Estado |
|---------------------|--------|
| `capability_di_resolver` + `di_binding` v2 | Entregado |
| `capability_di_gate` Aduana Temprana | Entregado — no sustituible |
| `capability-bindings.md` (`doc:closure`, `proc:git-sync`) | Entregado |
| Orden `resolve → gate → cerbero_rbac → inject` | Entregado — se extiende con paso envelope |
| Piloto homologación (4 ED) | `feature`, `bug-fix`, `filesystem-manager`, `git-manager` |

## Vectores soberanos (este ciclo)

1. **R9 — Cerbero envelope:** tras gate APTO y RBAC allow, contrastar el objeto `di_binding` empaquetado contra contrato/schema del envelope y coherencia con binding resuelto (**AC-R9**). Sin duplicar la aduana pre-ignición del gate sobre la declaración de fase.
2. **R10 — Homologación catálogo ED:** anotar `provides`/`requires_capability` en ≥**4 ED nuevas** (total **≥8** homologadas) con coherencia taxonomía + binding table; migración controlada vía `entity-manager` (**AC-R10**). Sin inventar términos fuera de `capability-taxonomy`.

## Criterios de aceptación — producto Hito 4

| ID | Criterio |
|----|----------|
| **AC-R9** | Cerbero rechaza inject si `di_binding` empaquetado incumple contrato/schema aunque gate DI y RBAC hayan pasado |
| **AC-R10** | ≥8 entidades ED homologadas (`provides`/`requires_capability` + bindings coherentes); ≥4 nuevas respecto al piloto H2/H3 |
| **AC-REG-H2** | Regresión Hito 2: AC-R1, AC-R2 |
| **AC-REG-H3** | Regresión Hito 3: AC-R5, AC-R6, AC-R7, AC-R8 |
| **AC-REG-MVP** | Regresión MVP: AC-P1, AC-P2, AC-P3 |

## Restricciones y leyes aplicadas

- Orden DI mínimo: `resolve` → `capability_di_gate` → `cerbero_di_rbac` → **revalidación envelope** → inject → `capability_di_output_validator` (**L-CERBERO-ORDER**, **L-ENVELOPE-DELTA**).
- Gate `capability_di_gate` se **conserva**; Cerbero no lo reemplaza (**L-GATE-PRESERVE**).
- Taxonomía vigente: solo `doc:closure` y `proc:git-sync`; sin altas R7 adicionales en R10 (**L-R10-NO-INVENT**).
- Binding table en `capability-bindings.md`; anotaciones ED en `{name}.md` frontmatter (**L-CODEX-ROLE**).
- Git solo vía `skill:git-manager`. Mutación genoma vía procesos canónicos (`entity-manager` / DA-4).
- Normas: `features-documentation-pattern`, `capability-taxonomy`, `external-ai-constraints`, `capsule-json-io`, `CONSTITUTION_CORE` Filtro A.

## Fuera de alcance (explícito)

| Ítem | Destino |
|------|---------|
| GesFer / Paciente 0 | Otro PBI |
| Fractura Core F1 | Otro `persist_ref` |
| Migración masiva catálogo ED completo | Post-Hito 4 / backlog |
| Sustitución total sync→EDA-only | Fuera de piloto R6 |
| Altas nuevas al Códice de capacidades | Fuera de R10 (salvo laudo Racso) |
| Archivo PBI-042 padre | Solo con Done global / laudo Racso |

## Ambigüedades para Dedalo (cerrar en spec/plan)

Ver `clarify.md` §D3: **Q1** locus módulo envelope · **Q2** schema `di_binding` · **Q3** profundidad contraste · **Q4** lista piloto R10 · **Q5** paths ciegos nuevos · **Q6** coherencia piloto EDA.

## Handoff

Blueprint Dedalo cerrado (`spec.md`, `plan.md`). Siguiente: **Tekton** — implementación R9/R10 según plan.

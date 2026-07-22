---
feature_name: inyeccion-dependencias-barrido-creators
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-barrido-creators
persist_ref: docs/features/inyeccion-dependencias-barrido-creators
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-BARRIDO-CREATORS
execution_id: c9d1e4f2-7a8b-4c5d-9e0f-1a2b3c4d5e6f
phase: ready-for-delivery
agents: argos
verdict: apto
---

# Objetivos — inyeccion-dependencias-barrido-creators

## Misión

Materializar el **Hito 6** del residual PBI-042 (post-Hito 5 PR #138 merge `66a0f71`): **barrido de creators restantes** con DI por capacidades (`requires_capability` / path ciego preferente), preservando taxonomía+bindings post-H5, sellos EDA y regresiones MVP→H5.

## Estado de partida (innegociable post-H5)

| Vector | Estado |
|--------|--------|
| R11 `Domain_Entity_Updated` vía entity-manager | **Hecho en main** — merge `66a0f71` |
| R12 ola `N_ola=8` + alta `fs:persist` (total ≥16) | **Hecho en main** |
| R13 ampliación piloto EDA DI | **Omitido** (Q6-A) |
| Taxonomía | `doc:closure`, `proc:git-sync`, `fs:persist` |
| Bindings | `capability-bindings.md` v1.1.0 |
| Runtime DI (gate/resolver/Cerbero RBAC/envelope/output validator) | **Preservar** |

**Creators ya homologados (H5, no recontar):** `process-creator`, `skill-creator`, `action-creator`, `event-creator`, `agent-creator`, `tool-creator`.

**Creators residuales (piso R14):** `norm-creator`, `codex-creator`, `daemon-creator`, `suite-creator` — solo `delegates_to`, sin `requires_capability`.

## Vectores soberanos (este ciclo)

1. **R14 — Barrido creators residuales:** homologar con `provides`/`requires_capability` coherentes a taxonomía+bindings; preferir path ciego (`fs:persist` y/o `proc:git-sync`) en fases FS/git; mutación vía `entity-manager` + `Domain_Entity_Updated` + evolution (**AC-R14**).
2. **Umbral:** `N_ola` y lista exacta = Dedalo; piso Mayeuta = **≥4** (los residuales listados). Sin bajar piso sin laudo Racso.
3. **Sin altas libres** al Códice salvo laudo Racso (**L-R14-NO-INVENT**).
4. **PBI-042 no se archiva** en este ciclo (**L-PBI-LOC**) salvo Done global / laudo Racso.

## Criterios de aceptación — producto Hito 6

| ID | Criterio |
|----|----------|
| **AC-R14** | Creators residuales homologados (mín. norm/codex/daemon/suite-creator) con DI coherente + sellos EDA + `orphan_count == 0` |
| **AC-REG-H5** | Regresión Hito 5: AC-R11, AC-R12 |
| **AC-REG-H4** | Regresión Hito 4: AC-R9, AC-R10 |
| **AC-REG-H3** | Regresión Hito 3: AC-R5, AC-R6, AC-R7, AC-R8 |
| **AC-REG-H2** | Regresión Hito 2: AC-R1, AC-R2 |
| **AC-REG-MVP** | Regresión MVP: AC-P1, AC-P2, AC-P3 |

## Restricciones y leyes aplicadas

- Gate/resolver/Cerbero RBAC/envelope/output validator se **conservan** (**L-RUNTIME-PRESERVE**).
- Mutación genoma vía `entity-manager` + sello `Domain_Entity_Updated` + evolution (**L-R14-MUTATION**).
- Capacidades solo del catálogo vigente; default sin altas (**L-R14-NO-INVENT**).
- Preferencia path ciego en fases FS/git; mixto crypto+FS permitido (**L-R14-BLIND-PREF**).
- Git solo vía `skill:git-manager`. Mutación genoma vía `entity-manager` / DA-4.
- Normas: `features-documentation-pattern`, `capability-taxonomy`, `external-ai-constraints`, `capsule-json-io`, `CONSTITUTION_CORE` Filtro A.
- PBI-042 no se archiva en este ciclo (**L-PBI-LOC**).

## Fuera de alcance (explícito)

| Ítem | Destino |
|------|---------|
| GesFer / Paciente 0 | Otro PBI |
| Fractura Core F1 | Otro `persist_ref` |
| Sustitución total sync→EDA-only | Fuera salvo laudo Racso |
| Altas libres al Códice de capacidades | Fuera salvo laudo Racso |
| Archivo PBI-042 padre | Solo con Done global / laudo Racso |
| Recontar creators H5 como progreso R14 | Prohibido (**L-BASELINE-H5**) |

## Ambigüedades Dedalo — abiertas

Ver `clarify.md` §D3: **Q1** `N_ola`+lista · **Q2** fases ciegas · **Q3** taxonomía (default A) · **Q4** lotes · **Q5** `proc:git-sync` · **Q6** evidencia sello/orphan · **Q7** smoke regresión H5.

## Handoff

Mayeuta **DONE** (requisitos estables). Siguiente: **Dedalo** → `spec.md` / `plan.md` consumiendo este cuerpo como `refined_requirements`.

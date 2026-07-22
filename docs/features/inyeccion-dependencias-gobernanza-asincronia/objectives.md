---
feature_name: inyeccion-dependencias-gobernanza-asincronia
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-gobernanza-asincronia
persist_ref: docs/features/inyeccion-dependencias-gobernanza-asincronia
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-GOBERNANZA-ASINCRONIA
execution_id: f8b2c4d1-6e3a-4f7b-9c2d-1a0e5f8b3c7d
phase: blueprint-design
agents: dedalo
---

# Objetivos — inyeccion-dependencias-gobernanza-asincronia

## Misión

Materializar el **Hito 3** del residual PBI-042 (post-Hito 2 PR #127 merge `60c4635`): **gobernanza Cerbero** sobre inject DI, **composición piloto EDA**, **expansión del Códice de la Lengua** y **validación runtime del payload de salida**, preservando resolución ciega e inyección del Hito 2.

## Estado de partida (innegociable)

| Vector Hito 2 | Estado |
|---------------|--------|
| `capability_di_resolver` + `di_binding` v2 | Entregado en `main` |
| `capability-bindings.md` + `capability_di.bindings` | Entregado |
| Orden `resolve → gate → inject` | Entregado |
| `capability_di_gate` Aduana Temprana | Entregado — valida pre-ignición; **no** sustituye Cerbero |
| Cerbero | RBAC vía `cerbero.md` / `revoked_entities`; **no** bloquea post-gate DI hoy como AC-R5 |
| Composición DI | Path **síncrono** en `execute-process` (L-SYNC-PATH H2) |
| Taxonomía | Solo `doc:closure` |
| Validación contrato | Pre-ignición: `required` schema vs outputs **declarados** del proveedor |

## Vectores soberanos (este ciclo)

1. **R5 — Cerbero en cadena DI:** cruce RBAC sobre cápsula destino resuelta; puede rechazar inject aunque el gate DI haya pasado (**AC-R5**). Revalidación schema sobre `di_binding` empaquetado = opcional en MVP Hito 3.
2. **R6 — Piloto EDA DI:** evento de dominio + reacción async en `./.events/`; ECST post-cápsula; **sin** bloquear orquestador de fases (**AC-R6**). Path síncrono H2 se conserva para regresión.
3. **R7 — Expansión Códice:** al menos un término nuevo en `capability-taxonomy` (≠ `doc:closure`); alta vía `entity-manager` update + `SddIA/evolution/`; contrato schema asociado + fila binding si aplica piloto.
4. **R8 — Schema salida runtime:** validar payload **real** JSON de salida de cápsula contra contrato (no solo firma declarada de `outputs` en frontmatter).

## Criterios de aceptación — producto Hito 3

| ID | Criterio |
|----|----------|
| **AC-R5** | Cerbero puede rechazar inject por RBAC aunque el gate DI haya pasado |
| **AC-R6** | Flujo piloto DI vía evento de dominio + reacción async sin bloquear el orquestador de fases |
| **AC-R7** | ≥1 término nuevo homologado en Códice + evolution |
| **AC-R8** | Validación JSON Schema del payload real de salida vs contrato |
| **AC-REG-H2** | Regresión Hito 2: AC-R1, AC-R2 |
| **AC-REG-MVP** | Regresión MVP: AC-P1, AC-P2, AC-P3 |

## Restricciones y leyes aplicadas

- Orden DI mínimo: `resolve` → `capability_di_gate` → Cerbero RBAC → inject (**L-CERBERO-ORDER**).
- Gate `capability_di_gate` se **conserva**; Cerbero no lo reemplaza (**L-GATE-PRESERVE**).
- Bus Core = `./.events/`; no `.SddIA/events/` para composición DI (**L-BUS**).
- Binding table en `capability-bindings.md`; taxonomía en `capability-taxonomy.md` (**L-CODEX-ROLE**).
- Git solo vía `skill:git-manager`. Mutación genoma vía procesos canónicos (`entity-manager` / DA-4).
- Normas: `features-documentation-pattern`, `capability-taxonomy`, `external-ai-constraints`, `capsule-json-io`, `CONSTITUTION_CORE` Filtro A.

## Fuera de alcance (explícito)

| Ítem | Destino |
|------|---------|
| GesFer / Paciente 0 | Otro PBI |
| Fractura Core F1 | Otro `persist_ref` |
| Migración masiva catálogo ED | Post-Hito 3 |
| Sustitución total del hilo síncrono de fases por EDA | Fuera de piloto R6 |
| Cerbero como única aduana DI (sin gate) | Prohibido — O3 MVP |

## Ambigüedades para Dedalo (cerrar en spec/plan)

Ver `clarify.md` §D3: **Q1** intercepción Cerbero · **Q2** revalidación schema opcional · **Q3–Q4** evento/reactor EDA · **Q5** coexistencia sync/async · **Q6** locus R8 · **Q7** término R7 + binding.

## Handoff

Estabilización Mayeuta cerrada. Siguiente: **Dedalo** — diseño gobernanza Cerbero / EDA / códice / schema-salida → `spec.md` + `plan.md`.

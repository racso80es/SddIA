---
feature_name: inyeccion-dependencias-h8-familia-route
created: "2026-07-22"
updated: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-h8-familia-route
persist_ref: docs/features/inyeccion-dependencias-h8-familia-route
document_id: PBI-043-H8-FAMILIA-ROUTE
execution_id: a7c3e91f-2b84-4d6e-9f01-5c8a2e7d4b63
phases: 7
agent_planificador: dedalo
target_executor: tekton
rbac_ok: true
scope: "Hito 2 (H8) — Familia route (R4–R5 / AC-H8) · Rama A bus:route"
q1_laudo: alta-bus-route
ac_h8_branch: A
---

# Plan / Blueprint — H8 Familia route (Rama A)

## Fases

```yaml
phases:
  - name: "Countersign Racso Q1=(A)"
    intent: "Materializar laudo alta bus:route; abort si ausente."
  - name: "R4 Alta Códice"
    intent: "Write taxonomía+schema+bindings; provides bus-operator; sellos EDA."
  - name: "R5 Ola N_ola=3"
    intent: "entity-manager update route-domain|orchestration|telemetry con bus:route mixto."
  - name: "Q8 Revalidar RDE"
    intent: "Confirmar fs:persist×3; noop si sin drift."
  - name: "Índice skills"
    intent: "Alinear fila bus-operator v1.1.0 en skills/index.md."
  - name: "Evidencia + Regresión"
    intent: "orphan_count==0; cargo capability_di + cerbero_di."
  - name: "Documentación"
    intent: "implementation + execution + evolution; handoff Argos."
```

## Orden R4

1. `capability-taxonomy.md` v1.0.3 (+ `bus:route`) + sello/coverage
2. `bus.route.schema.json`
3. `capability-bindings.md` v1.2.0 → `skill:bus-operator`
4. `bus-operator.md` v1.1.0 `provides` += `bus:route` + sello

## Orden R5

`entity-manager` update ×3 (patch SemVer 1.0.1) → `Domain_Entity_Updated` ×3.

---
feature_name: aut-50-backlog-alta
created: "2026-11-01"
process: bug-fix
phases:
  - id: T1
    title: forge-pbi padre usable + success honesto
  - id: T2
    title: list_issues projectId ID! + complejidad
  - id: T3
    title: Blueprint forge-hu (HU raíz)
  - id: T4
    title: refine-item lock accionable + fractura + budget
execution_id: "82abfda6-47a2-41f6-855f-bddbc8546bda"
pbi_ref: AUT-50
branch_name: Fix_Events_Multiple_02
---

# Plan

Descomposición Tekton (incluye blueprint de proceso `forge-hu`). Un commit por tarea vía `skill:git-manager`. Sin TODOs bajo `docs/todos/`.

| Id | Tarea | Hecho |
|----|--------|--------|
| T1 | `forge_pbi`: padre = `parent_ref`\|`historia_ref`; tracker + sin padre → `success: false` / `PARENT_REF_REQUIRED`; tests CA1/CA2; ajustar test legado ruta-fichero. Opcional bump menor `forge-pbi.md`. | done |
| T2 | `linear-tracker-adapter`: `$projectId: ID!`; default `first=20`; quitar `children` del selection set default; tests query + lab AUT50-CA3/CA4. | done |
| T3 | Blueprint `forge-hu` en códice SE (process-creator/entity-manager): contrato, índice, handler nativo create_issue hu sin padre, evento/EDA, tests lab `tracker_ref`. Cierra hueco AUT-19 slice 2. | done |
| T4 | `refine-item` + store: `STORE_LOCK_CONTENTION`, `System_Fracture_Detected` si colapsa; pending si description ya sellada; default open budget 10000 ms; fix `StoreError` Display; evolution AUT-50. | done |

## Blueprint (T3) — mínimo

- **Nombre:** `forge-hu`
- **Packing:** `SddIA/library/codexes/codex-software-engineering/process/forge-hu.md`
- **Inputs:** `raw_idea`, `project_slug`
- **Fase registro:** un `create_issue` vía `tracker:lifecycle`, labels `[hu]`, sin `parent_ref`
- **Output:** `tracker_ref` (obligatorio si tracker en manifiesto)
- **Handler:** nativo bajo `execute-process` (paralelo a `forge_pbi`)
- **EDA:** alta indexada + emit dominio; nota evolution ligada a AUT-50

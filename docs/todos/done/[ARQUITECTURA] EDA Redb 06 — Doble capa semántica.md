---
document_id: PBI-EDA-REDB-06-SEMANTIC-LAYER
uuid: "5a9c3e17-2f84-4d6b-9c08-1b7e4a6d5f32"
title: "[ARQUITECTURA] EDA Redb 06 — Doble capa semántica"
format: markdown
version: "1.0.0"
created: "2026-10-21"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: false
historia_ref: "docs/todos/pending/HU-SDDIA-EDA-REDB-001: Evolución del Bus de Eventos a Redb, Jurisdicción Cognitiva y Estandarización del Marco SddIA.md"
historia_document_id: HU-SDDIA-EDA-REDB-001
historia_uuid: "7c4e9a12-8b3d-4f61-a9e0-2d5c6b8f1a47"
suggested_branch: feat/eda-redb-06-semantic-layer
persist_ref_suggested: docs/features/eda-redb-06-semantic-layer
blocked_by: []
unblocks:
  - PBI-EDA-REDB-07-CUTOVER
laudos:
  - L-REDB-08
  - L-REDB-09
related:
  - SddIA/agents/agents-contract.md
  - SddIA/agents/index.md
  - SddIA/agents/cerbero.md
  - SddIA/agents/cumulo.md
  - SddIA/agents/radamanto.md
  - SddIA/agents/argos.md
  - SddIA/agents/tekton.md
  - SddIA/agents/dedalo.md
  - SddIA/agents/mayeuta.md
  - SddIA/CONSTITUTION_CORE.md
  - README.md
acceptance:
  - AC-11
---

# Doble capa semántica

Historia madre: §6, L-REDB-08, L-REDB-09. Puede forjarse en paralelo al PBI 02. No depende del crate `redb`.

Genoma de agentes vía `entity-manager`. `CONSTITUTION_CORE.md` y `README.md` son parámetros de restricción: se editan en este PR, no se rebautizan los filtros.

## 1. Entrega

`agents-contract` 1.1.0 → 1.2.0. Campo YAML opcional:

```yaml
cognitive_role_descriptor: "una línea, vocabulario industrial"
```

No sustituye a `llm_profile`. Los siete agentes del catálogo (`SddIA/agents/index.md`) lo declaran. `uuid`, `allowed_policies` y `llm_profile.tier` no cambian. `index.md` gana la columna del descriptor en la misma pasada de `entity-manager`.

| Agente | Tier que debe permanecer | Descriptor |
| :--- | :--- | :--- |
| cerbero | `none` | Peaje RBAC determinista |
| cumulo | `none` | SSOT de topología e índices |
| radamanto | `none` | Actuario de umbrales sobre telemetría |
| argos | `medium` | Juez de artefactos; síntesis solo tras evidencia de cápsulas |
| tekton | `low` | Ejecución táctica según especificación |
| dedalo | `high` | Diseño de especificación y blueprint de proceso |
| mayeuta | `high` | Estabilización de requisitos y auditoría Kaizen de fracturas |

La Aiúa no entra en el catálogo. Sigue en `paths.directories.conscience` (`aiua_core.md`).

`CONSTITUTION_CORE.md` §3.1: la instancia de evento deja de describirse como JSON en `.events/` + `pending/` + `subscribers/`. Pasa a store `redb` (`events` + `subscriber_delivery`), con el modo fichero vigente hasta el cutover del PBI 07 citado como transición. Permanecen en el documento, sin reescritura de fondo: Filtro C (Necesidad), Filtro A (Verdad), Filtro B, Protocolo de Acero, Cicatriz Rúnica.

`README.md`: la sección de eventos nombra el store y la frontera DLT (local frente a `PullRequest_Merged` y `System_Immunity_Certified`). La tabla de descriptores de §1 se reproduce ahí. El diagrama de centinelas conserva `event-sweeper` hasta el PBI 07.

## 2. Fuera de alcance

- Implementar `sddia-event-store`.
- Dar de baja `event-sweeper` en `start-sddia.sh` o en `cumulo.paths.json`.
- Cambiar el tier de Argos o autorizar LLM en Cerbero, Cúmulo o Radamanto.

## 3. Criterios

| ID | Criterio |
| :--- | :--- |
| AC-11 | Los 7 `{name}.md` tienen `cognitive_role_descriptor`. Los tier de la tabla §1 coinciden con el YAML. `agents-contract` es 1.2.0. `CONSTITUTION_CORE.md` sigue nombrando los Filtros C, A y B. |
| CA-08 | Cero fila nueva en `SddIA/agents/index.md`. La Aiúa no aparece como agente. |

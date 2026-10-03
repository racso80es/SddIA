---
feature_name: tracker-operations-context
created: "2026-10-02"
process: feature
---

# Especificación — tracker-operations-context

## RBAC

- Nueva sección **2.10 `tracker-operations`** en `SddIA/norms/execution-contexts.md` (Cerbero SSOT).
- Alcance: lectura/listado, transiciones y comentarios vía adaptadores; sin crear/borrar issues.

## Agentes

- `allowed_policies` incluye `tracker-operations` en `SddIA/agents/tekton.md` y `SddIA/agents/argos.md`.

## Base

Implementación ya integrada en rama HU Tracker (PR #316 / #317); este backfill solo documenta el estado desplegado.

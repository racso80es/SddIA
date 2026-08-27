---
feature_name: bug-fix-revoked-registry-rehab-ppr194
created: "2026-08-27"
process: refactorization
phase: execution
agents: tekton
items:
  - T0-type-verify
  - T1-instance-rehab
  - T2-docs-evolution
branch_name: refactor/bug-fix-revoked-registry-rehab-ppr194
persist_ref: docs/features/bug-fix-revoked-registry-rehab-ppr194
pbi_ref: docs/todos/done/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #194).md
document_id: PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY
uuid: 8a4b0d3f-5c2e-4f9b-8d6a-7e8f9a0b1c2d
olas:
  - A1
---

# Implementation — bug-fix-revoked-registry-rehab-ppr194

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `.SddIA/cerbero/revoked_entities.json` | A1: eliminar `revoked.bug-fix` (fósil `entity_type: tool`). **Fuera del PR.** |
| `.SddIA/radamanto/stats.json` | A1: materializar bucket raíz `bug-fix` healthy + `entity_type: process` + laudo. **Fuera del PR.** |
| `SddIA/evolution/8a4b0d3f-5c2e-4f9b-8d6a-7e8f9a0b1c2d.md` | Registro UUID ciclo |
| `persist_ref` | `implementation.md` / `execution.md` |

## Genoma / motor

**Intacto.** Sin touchpoints en `radamanto_batch_core.rs`, hollow, thresholds, ni `bug-fix.md` (L-TYPE-VERIFY PASS / L-NO-A2 / L-THRESH).

## Tipología (T0)

`resolve_entity_type` bare id → `resolve_process_path` OK ⇒ `process`. Presencia: `SddIA/library/codexes/codex-software-engineering/process/bug-fix.md`. Motor no estampa `tool` para esta entidad; el `tool` en Cerbero era fósil.

---
feature_name: kaizen-dia-parity-rust-capsule-structure
created: "2026-07-11"
process: bug-fix
branch_name: fix/kaizen-dia-parity-rust-capsule-structure
persist_ref: docs/fixes/kaizen-dia-parity-rust-capsule-structure
pbi_ref: docs/todos/pending/PENDING_AUDIT_DOC_8ce19304.md
related_feature: docs/features/kaizen-rust-capsule-structure
origin_pr: 93
---

# Especificación — cierre deuda DIA kaizen-rust-capsule-structure

## Problema

Durante la aduana del PR #93 (`feat/kaizen-rust-capsule-structure`), el sensor DIA emitió `Kaizen_Alert_Required` con razón `impacts_doc_true_empty_section`: mutaciones en rutas monitorizadas (`SddIA/core/`, `SddIA/scripts/qa/`) mientras `spec.md` declaraba `impacts_doc: true` sin sección `### Impacto en Documentación`.

Cúmulo materializó `PENDING_AUDIT_DOC_8ce19304.md`. El PR se mergeó con certificación K6 APTO, pero la paridad DIA quedó incompleta y la cicatriz Kaizen en `pending/`.

## Causa raíz

El frontmatter `impacts_doc: true` se añadió en la refactorización Kaizen sin completar la sección DIA obligatoria (`norma-paridad-documental` / plantilla `spec-template`).

## Cambio (solo documentación)

| Artefacto | Modificación |
|-----------|--------------|
| `docs/features/kaizen-rust-capsule-structure/spec.md` | § DIA: rutas core, cápsulas Rust, runtime lab QA, contratos |
| `docs/fixes/kaizen-dia-parity-rust-capsule-structure/` | Topología fix + validación |
| `docs/todos/pending/PENDING_AUDIT_DOC_8ce19304.md` | → `docs/todos/done/` |

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| DIA-KZ1 | Sensor DIA `dia_declared_ok` sobre diff PR #93 | `audit-doc-parity.py --base-ref 8e611bc^1 --head-ref 8e611bc` |
| DIA-KZ2 | `impacts_doc: true` + § DIA no vacía | inspección `spec.md` |
| DIA-KZ3 | Rutas monitorizadas trazadas en § DIA | inspección manual vs `implicated_files` |
| DIA-KZ4 | PBI Kaizen archivado en `done/` | movimiento archivo |
| DIA-KZ5 | `validacion.md` APTO + `pbi_archived: true` | este fix |

## No objetivos

- Reabrir PR #93 ni mutar código Rust/Python.
- Alterar sensor DIA ni flujo EDA Kaizen.

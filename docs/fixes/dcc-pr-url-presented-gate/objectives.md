---
feature_name: dcc-pr-url-presented-gate
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-pr-url-presented-gate
pbi_ref: AUT-31
tracker_ref: AUT-31
execution_id: "705a9de9-556d-4f5b-945a-de5f8f8f02a5"
document_id: PBI-FIX-FRACTURE-186c785103d8
fracture_hash: 186c785103d8
---

# Objetivos — dcc-pr-url-presented-gate

## Misión

Cerrar AUT-31: `delivery-close-cycle` no debe colapsar en fractura sistémica cuando **Sello Presentación ECST** carece de `pr_url`. Recuperar la URL o devolver `blocked` con firma DCC; nunca `PullRequest_Presented` incompleto.

## Alcance

`capsule_delivery_emit_presented`. `gh pr view` del sello solo si **Apertura en forja** salió antes de `gh`. Firma `F-DCC-PR-URL-MISSING` en el catálogo y en `excludes` de `F-DCC-HOOK-RECURSION`. Corpus `186c785103d8`. Traza histórica: `pr_url obligatorio para Sello Presentación ECST…`.

## Ley aplicada

- Git y `gh` solo vía cápsulas (`skill:git-manager`, shell executor acotado en fase forja). Sin bypass raw para cerrar la fractura.
- Cambios en `SddIA/engine/execute-process/` (no genoma indexado de tools/skills/process).
- Firma vía Cúmulo (`SddIA/core/fracture-signatures.json`).
- Rama `Fix_Events_Multiple`; init etiquetó `fix/dcc-pr-url-presented-gate` — no se abre esa rama.
- Provider `linear`: cierre en issue, sin fichero en `docs/todos/pending/` ni `done/`.

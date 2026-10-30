---
feature_name: dcc-pre-pr-qa-gate
created: "2026-10-30"
process: feature
branch_name: feat/dcc-pre-pr-qa-gate
persist_ref: /home/racso/Proyectos/SddIA/docs/features/dcc-pre-pr-qa-gate
pbi_ref: AUT-38
execution_id: "5246a313-9bfc-447c-a34e-6c44417ac49f"
---

# Objetivos — dcc-pre-pr-qa-gate

## Misión

AUT-38: bloquear `delivery-close-cycle` antes del push si falta atestación QA válida para el `tree_sha` de HEAD o si el árbol no está cerrado (excepto `validacion.md` en `persist_ref`).

## Alcance (manifiesto)

- Fase `Gate pre-PR QA` en DCC v1.5.0.
- Fricciones `qa_attestation_required` y `worktree_not_closed`.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

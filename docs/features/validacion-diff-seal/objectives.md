---
feature_name: validacion-diff-seal
created: "2026-10-30"
process: feature
branch_name: feat/validacion-diff-seal
persist_ref: docs/features/validacion-diff-seal
pbi_ref: AUT-39
execution_id: "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
---

# Objetivos — validacion-diff-seal

## Misión

AUT-39: tras gate pre-PR QA, sellar en `{persist_ref}/validacion.md` el diff que viaja en el push inicial (`validated_*_sha`, `diff_digest` excluyendo el propio fichero).

## Alcance

- Módulo `diff_seal.rs` y fase **Sello diff validacion** en `delivery-close-cycle` v1.6.0.
- Commit de `validacion.md` vía `skill:git-manager` antes de Publicación remota.
- Fricciones `validacion_not_apto` y `diff_seal_commit_failed` sin fractura.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Informe Argos (`global: APTO`) en fase Verificación de `feature`; no post-`PullRequest_Presented`.

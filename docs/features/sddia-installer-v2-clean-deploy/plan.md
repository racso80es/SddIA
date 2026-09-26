---
feature_name: sddia-installer-v2-clean-deploy
created: "2026-09-26"
process: feature
phases:
  - L0-design-commit
  - L1-motor-registry-vault-port
  - L2-systemd-precond-templates
  - L3-instance-creator-bridge
  - L4-verify-process-events-norm
  - L5-facade-qa-dcc-ci-accept
branch_name: feat/sddia-installer-v2-clean-deploy
persist_ref: docs/features/sddia-installer-v2-clean-deploy
pbi_ref: docs/todos/pending/[ARQUITECTURA] Installer determinista v2 — despliegue limpio y anti-fricción (deploy + teardown).md
document_id: PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO
uuid: "bb30e934-7f1f-44cb-a51e-21c28ccf426b"
execution_id: "8ff78e98-6e4f-4859-9ca9-60ba718703ea"
---

# Plan — sddia-installer-v2-clean-deploy

Corte L0: clarify + objectives + spec + plan + **commit** antes de mutar motor/genoma.

Init: `execution_id` `8ff78e98-6e4f-4859-9ca9-60ba718703ea`.

## L0 — Diseño (esta parada)

Artefactos bajo `persist_ref` + PBI referenciado. Commit planificación.

## L1 — Motor: registro, bóveda, puerto

`instances.json`, `stage_vault` compuesto, mailbox exit 4, plan JSON extendido, registry en deploy/teardown.

## L2 — systemd

`_precondition.sh`, plantilla factory, `enable_units` condicional + sha overwrite, teardown plantillas última instancia.

## L3 — Creator + bridge

`materialize_domain_profile` engineering; `--codex` en deploy; kalma2 exit 78.

## L4 — Verify + eventos + norma

Handler + EM process/eventos; norm 1.1.0; suscripciones Cúmulo.

## L5 — Fachada, tests, DCC, CI, accept-pr

`implementation.md`, `execution.md`, `validacion.md` PENDIENTE-CI → APTO con run_id; PBI `done/` pre-merge; `delivery-close-cycle`.

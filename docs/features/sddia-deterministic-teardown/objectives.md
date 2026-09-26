---
feature_name: sddia-deterministic-teardown
created: "2026-09-26"
process: feature
branch_name: feat/sddia-deterministic-teardown
persist_ref: docs/features/sddia-deterministic-teardown
execution_id: "0e8d3078-8bcb-4d45-91b9-5dade5b6b641"
---

# Objetivos — sddia-deterministic-teardown

## Misión

Herramienta de tierra quemada determinista sobre el ROOT del installer (mismos defaults que deploy). Ceguera: sin LLM, sin prompts.

## Alcance

1. Endurecer `do_teardown` (protocolo deuda, defaults installer).
2. Atajo host `SddIA_Teardown.sh`.
3. Smoke: exit 3 sin `--force`; abort forja `/`; dry-run esc.

## Fuera

`paciente0-undeploy`. Default `SddIA_AP`. Segundo binario.

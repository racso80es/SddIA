---
feature_name: delivery-close-shell-executor-wasm-fallback
created: "2026-06-11"
process: bug-fix
branch_name: fix/delivery-close-shell-executor-wasm-fallback
persist_ref: docs/fixes/kaizen-delivery-close-shell-executor-wasm-fallback
pbi_ref: docs/todos/pending/Kaizen_delivery-close_shell-executor-wasm-fallback.md
related_todo: docs/todos/pending/Kaizen_delivery-close_shell-executor-wasm-fallback.md
origin: docs/todos/pending/Kaizen_delivery-close_shell-executor-wasm-fallback.md
---

# Objetivos — delivery-close-shell-executor-wasm-fallback

## Misión

delivery-close-cycle aborta al invocar shell-executor.wasm — working_directory invalid (WASI canonicalize); falta fallback nativo Python paralelo a git-manager; capsule_delivery_impact_assessment usa git via shell-executor (violacion allowlist).

## Alcance (manifiesto)

Inicialización de contexto vía intérprete dinámico `execute-process.py` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

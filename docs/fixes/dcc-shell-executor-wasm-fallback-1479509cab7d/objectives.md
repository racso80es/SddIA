---
feature_name: dcc-shell-executor-wasm-fallback-1479509cab7d
created: "2026-09-05"
process: bug-fix
branch_name: fix/dcc-shell-executor-wasm-fallback-1479509cab7d
persist_ref: docs/fixes/dcc-shell-executor-wasm-fallback-1479509cab7d
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (1479509cab7d).md
execution_id: "4fef455f-9155-42e2-b39d-f5085e167607"
---

# Objetivos — dcc-shell-executor-wasm-fallback-1479509cab7d

## Misión

Fractura 1479509cab7d: Apertura en forja fuga 'shell-executor wasm fallback marker' y emite System_Fracture_Detected. PBI v1.2.0 (uuid ca61b900-e474-4ebb-a623-4baf8ffd5f22). Causa: invoke_shell_executor re-ejecuta WASM si native es None y run_shell convierte marcadores a centinela también en vía nativa; dcc_lab_binary_missing_trace no casa el centinela. Mayeuta v1.0.0 escribió cubo head-sha ajeno.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

---
feature_name: start-sddia-native-contract
created: "2026-07-13"
process: bug-fix
branch_name: fix/start-sddia-native-contract
persist_ref: docs/fixes/start-sddia-native-contract
---

# Objetivos — start-sddia-native-contract

## Misión

PBI-FIX-FRACTURE-fcca5016574d: adecuar start-sddia.sh al contrato start-sddia.md. Exigir explícitamente event-watcher y event-sweeper; hacer observable y validar la resolución de binarios nativos Rust para centinelas, kalma2-bridge y execute-process; alinear el uso documentado con el script en raíz; mantener separación Core/instancia y apagado limpio.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

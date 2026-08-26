---
feature_name: bundle-consumer-telegram-gateway
created: "2026-08-26"
process: bug-fix
branch_name: fix/bundle-consumer-telegram-gateway
persist_ref: docs/fixes/bundle-consumer-telegram-gateway
---

# Objetivos — bundle-consumer-telegram-gateway

## Misión

F-BUNDLE-06: build-release-bundle.sh no empaqueta telegram-gateway aunque incluye telegram-watcher. Gateway rc=1 en Paciente 0; cápsula tool telegram-gateway no encontrada bajo SddIA/target.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

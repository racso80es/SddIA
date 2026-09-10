---
feature_name: kalma2-wui-agy-network-sanitize
created: "2026-09-10"
process: bug-fix
branch_name: fix/kalma2-wui-agy-network-sanitize
persist_ref: docs/fixes/kalma2-wui-agy-network-sanitize
pbi_ref: docs/todos/done/[FIX] Kalma2 WUI — red agy sanitizada en epidermis (sin retry).md
execution_id: "2dab4bf4-408b-4dc6-919a-680c92588cf7"
---

# Objetivos — kalma2-wui-agy-network-sanitize

## Misión

Kalma2 WUI #aiua-pulse: agy exit=1 network issue connecting recortado a 240. sanitize_bridge_message no clasifica red agy (sí 503/auth/timeout). Clasificar 'network issue connecting' y 'dial tcp' → canónico castellano. Cero retry DA-5. Cero skill/app.js. PBI PBI-FIX-KALMA2-AGY-NETWORK-SANITIZE.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

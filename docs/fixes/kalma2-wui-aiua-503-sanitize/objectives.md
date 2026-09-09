---
feature_name: kalma2-wui-aiua-503-sanitize
created: "2026-09-09"
process: bug-fix
branch_name: fix/kalma2-wui-aiua-503-sanitize
persist_ref: docs/fixes/kalma2-wui-aiua-503-sanitize
pbi_ref: docs/todos/pending/[OPERATIVO] Kalma2 WUI — 503 Gemini sanitizado en epidermis (sin retry).md
execution_id: "8abff97c-9ce3-4d83-bdd8-4b08ac0d75d0"
---

# Objetivos — kalma2-wui-aiua-503-sanitize

## Misión

PBI-OPERATIVO-KALMA2-AIUA-503-SANITIZE v1.2.0. POST /api/aiua/interact HTTP 500 con message crudo http-status-503 + JSON Google (188 chars). sanitize_bridge_message no clasifica 503. Epidermis kalma2-bridge: mensaje canónico castellano 91 chars. Cero retry DA-5. Intactos app.js y gemini-http-infer. Prefijo http-status-503 suficiente; no casar 503 suelto.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

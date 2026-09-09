---
feature_name: kalma2-bridge-aiua-interact-stale-elf
created: "2026-09-09"
process: bug-fix
branch_name: fix/kalma2-bridge-aiua-interact-stale-elf
persist_ref: docs/fixes/kalma2-bridge-aiua-interact-stale-elf
pbi_ref: docs/todos/pending/[FIX] kalma2-bridge ELF release fósil — POST api-aiua-interact 404.md
execution_id: "4ccdf721-f1d4-40e1-b7ca-40002e9d5da5"
---

# Objetivos — kalma2-bridge-aiua-interact-stale-elf

## Misión

PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF (bd611423-eea8-446a-a405-79cc08685e38) v1.3.0. Split-brain: daemon kalma2-bridge PID 6151 servía estáticos frescos pero ELF release 2026-09-06 anterior a PR #276; POST /api/aiua/interact devolvía 404 ruta desconocida. Causa: proceso longevo no reevalúa _sddia_resolve_daemon_binary. Remediación física host ya hecha (PID 1244544, ELF 2026-09-09 15:17, ruta

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

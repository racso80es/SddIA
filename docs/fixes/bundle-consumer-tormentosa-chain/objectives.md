---
feature_name: bundle-consumer-tormentosa-chain
created: "2026-09-25"
process: bug-fix
branch_name: fix/bundle-consumer-tormentosa-chain
persist_ref: docs/fixes/bundle-consumer-tormentosa-chain
pbi_ref: docs/todos/pending/[FIX] bundle consumer — cadena Tormentosa (13 ELF), llm-registry y active-domain-profile en instance-creator.md
execution_id: "4480e892-4647-48c5-a1de-934800e768d7"
---

# Objetivos — bundle-consumer-tormentosa-chain

## Misión

Consumer bundle omite cadena Tormentosa (llm-router, gemini-http-infer, antigravity-cli-executor, thought-graph-access). instance-creator v1.3.0 no materializa llm-registry.json ni active-domain-profile.json. Plantilla email-watcher sin LogRateLimit. Laudos Q1=a Q2=si. Ver PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

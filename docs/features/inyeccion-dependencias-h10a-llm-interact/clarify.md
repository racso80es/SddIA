---
feature_name: inyeccion-dependencias-h10a-llm-interact
created: "2026-07-23"
purpose: PBI-043 H10-A — alta operativa llm:interact + homologación kalma2-interact
branch_name: feat/inyeccion-dependencias-h10a-llm-interact
persist_ref: docs/features/inyeccion-dependencias-h10a-llm-interact
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md
document_id: PBI-043-H10A-LLM-INTERACT
execution_id: 6eb3c394-be0e-4c93-ad10-bd0c14cf3b2e
phase: closed-apto
agents: tekton,argos
racso_countersign: "2026-07-23T06:53:00Z"
inventory_with_capability: 35
inventory_without_capability: 7
---

# Clarificación — H10-A llm:interact

## Laudo

| ID | Veredicto |
|----|-----------|
| H10-A | APROBADO — `llm:interact` → `skill:mayeuta-llm` + consumidores (kalma2-interact / fases) |
| H10-B | DEFER total §3.4 resto |

## Alcance

1. `provides: llm:interact` en `mayeuta-llm` v1.1.0 (Códice ya catalogado en H9).
2. Homologar `kalma2-interact` fases Clasificación + Síntesis con `requires_capability` → `llm:interact`.
3. Fuera: Cerbero, Radamanto, Telegram, Suites, memory-evolution, capsule-invoke-smoke.

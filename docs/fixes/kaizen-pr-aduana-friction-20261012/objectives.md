---
feature_name: kaizen-pr-aduana-friction-20261012
created: "2026-10-13"
process: bug-fix
branch_name: fix/kaizen-pr-aduana-friction-20261012
persist_ref: docs/fixes/kaizen-pr-aduana-friction-20261012
pbi_ref: docs/todos/pending/[KAIZEN] Aduana PR — corte LLM, dedupe Presented, persist_ref docs y sellos post-merge.md
document_id: PBI-KAIZEN-PR-ADUANA-FRICTION-20261012
execution_id: "37dc1867-9940-499b-99a7-1e4d2d933a0d"
correlation_id: "834d80af-cd3f-49b5-958a-c4a7d62f6975"
---

# Objetivos — kaizen-pr-aduana-friction-20261012

## Misión

Eliminar fricción operativa de la aduana PR (PPR + accept-pr + PEC Telegram) sin debilitar `FAIL_F2_DOC` ni L-HYGIENE-SOFT.

Fuente: `PBI-KAIZEN-PR-ADUANA-FRICTION-20261012` (criterios ADU-CA1–ADU-CA14).

## Alcance

| ID | Objetivo |
|----|----------|
| F-PPR-LLM-AFTER-F2-BLOCK | Tras Triaje documental `blocked`, Veredicto determinista sin segunda sesión LLM |
| F-PPR-PRESENTED-FANOUT | Un solo PPR por `(rama, commit)`; commit nuevo no silenciado |
| F-PERSIST-REF-DOCS-PREFIX | Heurística `docs/{slug}` → `docs/features\|fixes/{slug}` si existe |
| F-PRESENTED-SCAN-PENDING-ONLY | Escaneo Presented en pending+processed+dead-letter |
| F-HYGIENE-REMOTE-ABSENT-REF | Ref remoto ausente = delete idempotente OK; `closed_branch` solo si local∧remoto OK |
| F-PEC-TELEGRAM-OPAQUE | Mensaje PEC con fase/motivo/rama/notice/correlation; sin `failed/failed` |

## Fuera de alcance

- Suavizar peaje F2 / L-HYGIENE-SOFT.
- Reintroducir Argos LLM en Auditoría Genómica del camino `merge_already_done` (O4).

## Ley aplicada

- Git vía `skill:git-manager`.
- Genoma solo vía `entity-manager` si ADU-CA3 exige clase ECST nueva.
- Cascada documental: `features-documentation-pattern`.

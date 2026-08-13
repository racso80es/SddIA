---
feature_name: kalma2-post-ev-aud-005-ola
created: "2026-08-13"
process: bug-fix
branch_name: fix/kalma2-post-ev-aud-005-ola
persist_ref: docs/fixes/kalma2-post-ev-aud-005-ola
pbi_ref: docs/todos/done/[OPERATIVO] Kalma2 — ola mejora post-auditoría EV-AUD-005 (K1–K5).md
document_id: b2e4c891-3f7a-4d2e-9c1b-8a5f6e0d2c47
---

# Objetivos — ola Kalma2 post EV-AUD-005

## Misión

Cerrar la fricción de orquestación/UX Kalma2 observada en el ciclo `dcb9efed-…` **sin** reabrir el veredicto EV-AUD-005 (PR hermano #170).

## Alcance

K4 `suggested_branch` → K5 single-flight `correlation_id` → K3 watcher async → K2 early PEC `awaiting_agents` → K1 poll UI → K6 PEC más reciente.

## Fuera de alcance

`phase_terminal` / agregación EV-AUD-005. Genoma. `evolution-contract-index-v11`.

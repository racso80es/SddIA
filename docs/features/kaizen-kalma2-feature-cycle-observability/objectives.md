---
feature_name: kaizen-kalma2-feature-cycle-observability
created: "2026-07-21"
process: feature
branch_name: feat/kaizen-kalma2-feature-cycle-observability
persist_ref: docs/features/kaizen-kalma2-feature-cycle-observability
document_id: PBI-KAIZEN-KALMA2-FEATURE-CYCLE-OBS
pbi_ref: docs/todos/pending/[Kaizen] ciclo Kalma2-feature — correlación EDA, estados terminales y aduana PPR.md
correlation_id: 6ae1b7be-54e5-4750-8888-5f19ac76551f
status: stabilized
verdict: ok
---

# Objetivos — Kaizen observabilidad ciclo Kalma2-feature

## Misión

Cerrar el lazo abierto Kalma2 → EDA → estados terminales UI y alinear la aduana `pull-request-review` con `pr_url` opcional hasta que exista forja.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-O1** | Tras despacho TQM con `correlation_id`, existe PEC en `.events/orchestration/` recuperable por correlación |
| **AC-O2** | El bridge puede proyectar `initialized` (early PEC) y `failed` (PEC fallo) sin depender solo del domain no purgado |
| **AC-O3** | Checklist de entrega: member Cargo ⇒ `Cargo.lock` + evidencia `--locked` documentada en este `persist_ref` |
| **AC-O4** | `validate_process_inputs` no exige `pr_url` (DEFAULTABLE); no DL `INPUT_VALIDATION` por ausencia |
| **AC-O5** | Cascada Kaizen separada del residual F1 Fractura Core |

## Fuera de alcance

F1 GesFer, extracción Nodos, FIX prótesis kalma2-bridge, push/PR de otras ramas.

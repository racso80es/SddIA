---
feature_name: eda-domain-entities-splus
branch: feat/eda-domain-entities-splus
created: "2026-05-20"
global: pass
checks:
  - name: py_compile scripts QA
    result: pass
  - name: entity-manager tool create smoke
    result: pass
    evidence: event_id 6ff2d3ce-992d-40de-9b6d-a344f1522a95, origin_topology core
  - name: audit --scan JSON
    result: pass
    evidence: orphan_count 40 (esperado pre-backfill Fase C)
  - name: E2E tool pending to processed
    result: pass
    evidence: run-eda-e2e-lab.py, event 7dfa528e-cc36-4b29-81ea-e932577c1459
  - name: delivery-close-cycle Argos block
    result: pass
    evidence: orphan_count 40, argos_verdict block
  - name: Fase C backfill emit
    result: pass
    evidence: 40 emits, orphan_count_after 0
  - name: Fase C anchor-merkle
    result: pass
    evidence: transaction_digest lab-simulated-2880300d857094a3
  - name: delivery-close-cycle post-backfill
    result: pass
    evidence: argos_verdict pass, orphan_count 0
  - name: origin_topology local no muta index core
    result: pending
git_changes: uncommitted
---

# Validación — EDA Domain Entities S+

## Criterios Fase 0

| Criterio | Estado |
|----------|--------|
| ECST con `origin_topology` REQUIRED | ✅ documentado |
| Fan-out filtrado por topología | ✅ watcher |
| Idempotencia sello | ✅ código |
| `--scan` produce JSON | ⏳ verificar en ejecución |

## Criterios Fase A

| Criterio | Estado |
|----------|--------|
| `entity-manager` + `tool` + create → pending + topology | ⏳ smoke |
| 8 clases en PILOT | ✅ |

## Notas

Validación E2E completa (Fase B) y backfill Merkle (Fase C) quedan para ciclo siguiente tras smoke local.

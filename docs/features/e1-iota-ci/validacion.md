---
feature_name: e1-iota-ci
created: "2026-05-24"
process: feature
branch: feat/e1-iota-ci
global: APTO
pbi_archived: false
checks:
  E1-CA1: pass
  E1-CA2: pass
  E1-CA3: pass
  E1-CA4: pass
  E1-CA5: pass
  E1-CA6: pass
  verify-process-integrity: pass
git_changes:
  - SddIA/scripts/qa/run-iota-ci-smoke.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - .github/workflows/sddia-index-qa.yml
  - docs/features/e1-iota-ci/
---

# Validación — E.1 IOTA CI (Argos)

**Veredicto global: APTO**

## Checks

| ID | Check | Estado | Evidencia |
|----|-------|--------|-----------|
| E1-CA1 | Job simulate CI | ✅ | `eda-iota-smoke-simulate` en workflow |
| E1-CA2 | Aborto simulate en `--require-physical` | ✅ | lógica `run-iota-ci-smoke.py` |
| E1-CA3 | Exige wallet/secret en físico | ✅ | `_wallet_available()` |
| E1-CA4 | Digest simulate `lab-sim-*`; físico distinto | ✅ | route + smoke local simulate |
| E1-CA5 | `verify-process-integrity` OK | ✅ | salida 2026-05-24 |
| E1-CA6 | PBI § E.1 actualizado | ✅ | manifiesto operativo |

## Smoke local

| Escenario | Comando | Resultado |
|-----------|---------|-----------|
| Simulate | `run-iota-ci-smoke.py --simulate` | ✅ `success: true`, digest `lab-sim-*` |
| Físico local | Sin wallet en entorno | ⏭ omitido — configurar secret en GitHub/local |

## Nota operador

Configurar `IOTA_WALLET_SECRET` en GitHub repo secrets para activar anclaje Testnet real en job `eda-iota-physical`. Sin secret, el job registra skip y no bloquea merge.

## Backlog operativo

PBI `[OPERATIVO] Backlog pendiente post-PR11` permanece en `pending/` — **L1-O5** abierto. Solo track **E.1** cerrado en este PR.

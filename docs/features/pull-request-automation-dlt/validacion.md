---
feature_name: pull-request-automation-dlt
created: "2026-05-23"
process: feature
branch: feat/pull-request-automation-dlt
global: APTO
pbi_archived: true
checks:
  CA-1: pass
  CA-2: pass
  CA-3: pass
  CA-4: pass
  CA-5: pass
  CA-6: pass
  CA-7: pass
  verify-process-integrity: pass
git_changes:
  - SddIA/scripts/daemons/github_bridge_watcher.py
  - SddIA/scripts/qa/dlt_bus_materializer.py
  - SddIA/scripts/qa/simulate_remote_pr.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/events/pull-request-presented.md
  - SddIA/evolution/pull-request-automation-dlt-oraculo-20260523.md
  - docs/features/pull-request-automation-dlt/
  - docs/todos/done/Activacion_Validacion_PR_DLT.md
---

# Validación — Oráculo Sensor DLT (Argos)

**Veredicto global: APTO**

Perfil lab: `SDDIA_LAB_SIMULATE_REMOTE_PR=1`, `SDDIA_LAB_SIMULATE_IOTA=1`, `SDDIA_LAB_ROUTE_SYNC=1`.

## Criterios spec §6

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| CA-1 | Demonio detecta PR (simulado) agnóstico autor | ✅ | `github_bridge_watcher.py --once` + fixture Jules |
| CA-2 | Filtro A validación GitHub REST | ✅ | `fetch_open_prs` descarta mismatch; lab bypass fixture |
| CA-3 | Anclaje IOTA → digest no vacío | ✅ | `lab-sim-c872a0a12f964dd0b0f6aaf5` |
| CA-4 | Bus idempotente `event_id == digest` | ✅ | Re-run bridge no duplica pending |
| CA-5 | Aduana 7 fases invocable desde evento bridge | ✅ | `_smoke-pr-review-from-bridge.json` → `verdict: aprobado` |
| CA-6 | Fallback dead-letter tras fallo IOTA | ✅ | `write_fallback_dead_letter` + flag `FALLBACK_LOCAL_SIGNATURE` |
| CA-7 | Simulador sin acceso wallet | ✅ | `simulate_remote_pr.py` solo escribe fixture |

## Guard IOTA (H5)

| event_id | cumulo subscriber | result_status |
|----------|-------------------|---------------|
| `lab-sim-c872a0a12f964dd0b0f6aaf5` | `iota-immutable-publisher` | `skipped-pre-anchored` |

## Aduana directa (7 fases)

```powershell
$env:SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF="1"
$env:SDDIA_LAB_SKIP_GIT_CHECKOUT="1"
python SddIA/scripts/qa/execute-process.py --process pull-request-review `
  --inputs-file docs/features/pull-request-automation-dlt/_smoke-pr-review-from-bridge.json
```

Resultado: `success: true`, `verdict: aprobado`, 7 fases `executed`.

## Smoke E2E cadena bridge → bus → watcher

```powershell
$env:SDDIA_LAB_SIMULATE_REMOTE_PR="1"
$env:SDDIA_LAB_SIMULATE_IOTA="1"
$env:SDDIA_LAB_ROUTE_SYNC="1"
$env:SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF="1"
Remove-Item -Force -ErrorAction SilentlyContinue .SddIA/.dev/github_bridge_state.json
python SddIA/scripts/qa/simulate_remote_pr.py --inputs-file docs/features/pull-request-automation-dlt/_smoke-remote-pr-dlt.json
python SddIA/scripts/daemons/github_bridge_watcher.py --once
python SddIA/scripts/daemons/event-watcher.py --once
```

Evidencia: materialización `.events/pending/lab-sim-c872a0a12f964dd0b0f6aaf5.json`; watcher purga pending tras enrutar.

## Aduanas transversales

| Check | Resultado |
|-------|-----------|
| `verify-process-integrity.py` | ✅ OK |

## Cierre documental

PBI `PBI-ACTIVACION-VALIDACION-PR-DLT` archivado en `docs/todos/done/Activacion_Validacion_PR_DLT.md`; retirado de `pending/` (2026-05-23).

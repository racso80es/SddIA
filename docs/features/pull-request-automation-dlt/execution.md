---
feature_name: pull-request-automation-dlt
created: "2026-05-23"
process: feature
items_applied:
  - github_bridge_watcher H1
  - dlt_bus_materializer H2-H3
  - simulate_remote_pr H4
  - route guard skipped-pre-anchored
  - pull-request-presented v1.2.0
---

# Ejecución — Oráculo Sensor DLT

## Hitos aplicados

| Hito | Entregable | Estado |
|------|------------|--------|
| H1 | `SddIA/scripts/daemons/github_bridge_watcher.py` | ✅ |
| H2 | Puente firma + `publish_with_retries` | ✅ |
| H3 | `materialize_to_bus` → `.events/pending/<digest>.json` | ✅ |
| H4 | `simulate_remote_pr.py` + `_smoke-remote-pr-dlt.json` | ✅ |
| H5 | ECST v1.2 + guard route | ✅ |

## Smoke E2E (lab)

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

## Evidencia primera corrida (2026-05-23)

| Paso | Resultado |
|------|-----------|
| Bridge materializa | `.events/pending/lab-sim-b24c3053d9e64072a99cf720.json` |
| IOTA suscriptor | `skipped-pre-anchored` (guard activo) |
| Aduana PR review | Invocada; fallo documental pre-`implementation.md` (esperado en iteración 1) |

## Variables de entorno

| Variable | Propósito |
|----------|-----------|
| `SDDIA_LAB_SIMULATE_REMOTE_PR` | Fixture lab sin GitHub |
| `SDDIA_LAB_SIMULATE_IOTA` | Digest simulado sin Testnet |
| `SDDIA_GITHUB_BRIDGE_POLL_SECONDS` | Intervalo polling (default 30) |
| `SDDIA_GITHUB_REPOSITORY` | Slug repo (default `racso80es/SddIA`) |
| `GITHUB_TOKEN` | API GitHub (bóveda) |

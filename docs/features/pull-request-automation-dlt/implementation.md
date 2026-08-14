---
feature_name: pull-request-automation-dlt
created: "2026-05-23"
process: feature
items:
  - SddIA/scripts/daemons/github_bridge_watcher.py
  - SddIA/scripts/qa/dlt_bus_materializer.py
  - SddIA/scripts/qa/simulate_remote_pr.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/events/pull-request-presented.md
  - SddIA/evolution/05d3d2f9-8b67-4e51-a215-03a15c4efd06.md
  - docs/features/pull-request-automation-dlt/_smoke-remote-pr-dlt.json
---

# Implementación — Oráculo Sensor DLT

## Touchpoints

| # | Artefacto | Cambio |
|---|-----------|--------|
| 1 | `github_bridge_watcher.py` | Demonio H1: polling GitHub / lab fixture, Filtro A, ciclo `--once` |
| 2 | `dlt_bus_materializer.py` | H2–H3: firma IOTA, materialización idempotente, dead-letter fallback |
| 3 | `simulate_remote_pr.py` | H4: fixture Jules sin wallet → `.SddIA/.dev/remote_pr_simulation.json` |
| 4 | `route_domain_event_core.py` | Guard `skipped-pre-anchored` si `dlt_anchor_address` en payload |
| 5 | `pull-request-presented.md` | v1.2.0 — campos oráculo + emisor `github-bridge-watcher` |
| 6 | `evolution/…20260523.md` | Registro transmutación |

## Decisiones de implementación

- **Wallet:** `load_wallet_secret()` lee `.SddIA/.dev/wallet.key` solo si `IOTA_WALLET_SECRET` no está en bóveda.
- **Idempotencia:** estado en `.SddIA/.dev/github_bridge_state.json` por `pr_url`; bus por `event_id == digest`.
- **Lab IOTA:** `SDDIA_LAB_SIMULATE_IOTA=1` genera digest `lab-sim-<hex>` sin red.

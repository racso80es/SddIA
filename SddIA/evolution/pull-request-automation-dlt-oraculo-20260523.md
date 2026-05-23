---
uuid: pull-request-automation-dlt-oraculo-20260523
name: pull-request-automation-dlt-oraculo
version: "1.0.0"
context: ecosystem-evolution
created: "2026-05-23"
feature_ref: docs/features/pull-request-automation-dlt
---

# Evolución — Oráculo Sensor DLT (PullRequest_Presented ruta remota)

## Transmutación

| Artefacto | Antes | Después |
|-----------|-------|---------|
| `pull-request-presented.md` | v1.1.0 — solo ruta local Cursor | v1.2.0 — payload oráculo + emisor `github-bridge-watcher` |
| Bus remoto | Ceguera transaccional (Jules sin sello) | `github_bridge_watcher` → IOTA → `.events/pending/` |
| `route_domain_event_core` | IOTA siempre en fan-out | Skip si `dlt_anchor_address` presente |

## Componentes nuevos

- `SddIA/scripts/daemons/github_bridge_watcher.py`
- `SddIA/scripts/qa/dlt_bus_materializer.py`
- `SddIA/scripts/qa/simulate_remote_pr.py`

## Invariantes preservados

- Flujo local `delivery-close-cycle` → `emit-pr-presented-event` intacto.
- Aduana `pull-request-review` v2 sin cambios de fase.

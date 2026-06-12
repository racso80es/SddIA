---
feature_name: kaizen-eda-domain-iota-routing
created: "2026-06-12"
process: bug-fix
---

# Implementación — kaizen-eda-domain-iota-routing

## Archivos tocados

| Archivo | Cambio |
|---------|--------|
| `SddIA/core/event-domain-subscriptions.json` | Suscriptores IOTA domain |
| `scripts/skills/bus-operator.sh` | Launcher cápsula Python |
| `SddIA/scripts/qa/env_loader.py` | Precedencia vault flags IOTA |
| `SddIA/scripts/qa/route_domain_event_core.py` | `ts-node` local |
| `SddIA/scripts/qa/dlt_bus_materializer.py` | Idem |
| `SddIA/scripts/qa/audit-entity-eda-coverage.py` | Idem |
| `SddIA/scripts/daemons/event-watcher.{sh,bat,py}` | Testnet real + Node `.tools/` |
| `SddIA/scripts/tools/iota-immutable-publisher/install-deps.sh` | Bootstrap npm |

## Evidencia runtime

- `SDDIA_LAB_SIMULATE_IOTA=0` tras `load_hierarchical_env`.
- Digest real verificado: prefijo `GByziMh3…` (no `lab-sim-`).
- Watcher `--once`: 15/15 eventos `domain/` enrutados.

---
feature_name: event-sweeper-heartbeat-fracture-8b1ed140e48d
created: "2026-06-20"
process: bug-fix
branch: fix/event-sweeper-heartbeat-fracture-8b1ed140e48d
global: APTO
pbi_archived: true
closed: "2026-06-20"
checks:
  CA1-build: pass
  CA2-once-json: pass
  CA3-keepalive-continuous: pass
  CA4-audit-sweep: pass
  CA5-pbi-archived: pass
git_changes:
  - SddIA/daemons/event-sweeper/src/main.rs
  - docs/fixes/event-sweeper-heartbeat-fracture-8b1ed140e48d/
  - docs/todos/done/[FIX] event-sweeper — fractura sistémica (8b1ed140e48d).md
  - docs/todos/done/[FIX] event-sweeper — fractura sistémica (ff0989e5b8c0).md
---

# Validación — event-sweeper heartbeat fracture

**Veredicto global: APTO**

| ID | Criterio | Estado |
|----|----------|--------|
| CA1 | Build | ✅ |
| CA2 | `--once --json` | ✅ |
| CA3 | Keepalive continuo | ✅ |
| CA4 | Audit sweep | ✅ |
| CA5 | PBIs archivados | ✅ |

**Causa raíz:** `sweep_once` bloquea el hilo principal sin latido intermedio (intervalo heartbeat 30s).

**PBIs consolidados:** `8b1ed140e48d`, `ff0989e5b8c0`.

**Distinción:** `docs/fixes/event-pending-sweeper/` aborda lógica de purga post-route; no es este incidente.

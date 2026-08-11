---
feature_name: heartbeat-circuit-regimen-20260811
created: "2026-08-11"
process: refactorization
branch_name: refactor/heartbeat-circuit-regimen-20260811
persist_ref: docs/features/heartbeat-circuit-regimen-20260811
pbi_ref: docs/todos/done/[REFACTOR] fix recurrentes eventos.md
document_id: PBI-REFACTOR-HEARTBEAT-CIRCUIT-20260811
uuid: 83bbfdeb-4715-4915-88be-751532dc268a
laudo_c3: A+B+C+D
status: cerrado
---

# Objetivos — Circuito Daemon_Heartbeat (régimen)

Erradicar fracturas recurrentes por inanición de fan-out: la prueba de vida debe actualizar `heartbeat-audit.json` en **régimen**, no solo en ignición (PR #155).

## Vías (laudo)

| ID | Entrega |
|----|---------|
| C | Side-channel `daemons_instance.state/heartbeats/<daemon>.json` en cada emit |
| A | `ingest_regime` nativo antes de cada sweep + sweeper cada 30s |
| B | `event-watcher` prioriza telemetry / `Daemon_Heartbeat` |
| D | Crash-Only tras 5 fallos consecutivos de side-channel |

## Relacionados

- PBI-FIX-FRACTURE-{b8e3c0e97eb4,d47d7767e23b,23c58000e252,63c439de23d0}
- docs/fixes/daemon-heartbeat-ingest-ignition

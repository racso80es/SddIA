---
feature_name: thermal-resilience-heartbeat-mayeuta
created: "2026-09-07"
process: bug-fix
branch_name: fix/thermal-resilience-heartbeat-mayeuta
persist_ref: docs/fixes/thermal-resilience-heartbeat-mayeuta
document_id: PBI-FIX-THERMAL-RESILIENCE-HEARTBEAT-MAYEUTA
execution_id: "0e0f6614-f1bd-40c5-9f63-af8e0a482786"
items:
  - daemon_heartbeat/host_boot_gate
  - enrich_fracture_pbi_kaizen/orphan_lock_cube
  - process/daemon-heartbeat-audit-1.2.0
  - action/enrich-fracture-pbi-kaizen-1.3.0
---

# Implementation — puerta btime y cubo orphan_lock

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `daemon_heartbeat.rs` | `parse_proc_stat_btime`, `lock_predates_host_boot`, `host_reboot_stale_lock`; `remove_lock` si PID muerto; L3 intacto |
| `enrich_fracture_pbi_kaizen.rs` | `is_orphan_lock_trace`; cubo EDA genómica acotada |
| `daemon-heartbeat-audit.md` | v1.2.0 vía `entity-manager` (`bef79b9b` / body `ba981cd8`) |
| `enrich-fracture-pbi-kaizen.md` | v1.3.0 vía `entity-manager` (`6c191abb` / índice `a6212176`) |
| `SddIA/evolution/c142dc19-b3d9-4810-bad2-734c1910606e.md` | Alta registro |

## Contrato

- `btime` de `/proc/stat` es el comparable de `lock.started_at`. Sin `/proc/stat`: fail-open L3.
- Traza `lock huérfano` → ciclo de vida de daemon, no `Domain_Entity_Created`.
- Test `orphan_lock_dead_pid_emits_fracture_once` verde.

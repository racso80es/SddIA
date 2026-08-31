---
feature_name: latido-ontologico-vitalidad-organos
created: "2026-08-31"
updated: "2026-08-31"
process: feature
branch_name: feat/latido-ontologico-vitalidad-organos
persist_ref: docs/features/latido-ontologico-vitalidad-organos
execution_id: "cb141830-b5e3-4b9e-904d-014922254734"
pbi_ref: docs/todos/done/[OPERATIVO] Latido Ontológico (System Heartbeat).md
document_id: PBI-OPER-LATIDO-ONTOLOGICO-001
uuid: "cafd87eb-817f-4eee-a169-f9cd6019e931"
global: APTO
pbi_archived: true
branch: feat/latido-ontologico-vitalidad-organos
approval_status: aprobado
verdict: aprobado
checks:
  VIT_CA1_EMISORES: APTO
  VIT_CA2_KALMA2_CENSO: APTO
  VIT_CA3_ORPHAN_LOCK: APTO
  VIT_CA4_VITALITY_EVENT: APTO
  VIT_CA5_SWEEPER: APTO
  VIT_CA6_NO_FORBIDDEN_CLASSES: APTO
  CA_CI: PENDIENTE-CI
git_changes:
  - SddIA/engine/execute-process/src/engine/handlers/system_vitality.rs
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/daemons/kalma2-bridge.md
  - SddIA/events/telemetry/system-vitality-probed.md
  - SddIA/process/system-vitality-probe.md
  - SddIA/events/telemetry/daemon-heartbeat.md
  - SddIA/daemons/event-sweeper.md
  - SddIA/daemons/event-sweeper/src/main.rs
  - SddIA/interfaces/kalma2-bridge/
  - docs/features/latido-ontologico-vitalidad-organos/
  - docs/todos/done/[OPERATIVO] Latido Ontológico (System Heartbeat).md
---

# Validación — latido-ontologico-vitalidad-organos (Argos)

## Veredicto

**APTO** — criterios de aceptación locales verdes; genoma vía entity-manager; PBI archivado en la misma rama. `CA_CI` queda `PENDIENTE-CI` hasta `run_id` de GitHub Actions (patrón CA de CI).

## Checks

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `VIT_CA1_EMISORES` | APTO | `daemon-heartbeat.md` lista 7 stems; uuid `9c5190ac-ac8a-46b6-b61d-67d45ff7caf1` |
| `VIT_CA2_KALMA2_CENSO` | APTO | `SddIA/daemons/kalma2-bridge.md` uuid `abdafa2f-…`; fila índice; crate `DaemonRuntime` |
| `VIT_CA3_ORPHAN_LOCK` | APTO | `orphan_lock_dead_pid_emits_fracture_once` |
| `VIT_CA4_VITALITY_EVENT` | APTO | clase `System_Vitality_Probed` uuid `380e11c3-…`; proceso `b215b373-…`; sondas cerbero red/green |
| `VIT_CA5_SWEEPER` | APTO | 3 tests cadencia 300/piso 30; capability `vitality-probe-sweep` |
| `VIT_CA6_NO_FORBIDDEN_CLASSES` | APTO | cero `System_Heartbeat_Emitted` / `System_Degraded` en el diff de entrega |
| `CA_CI` | PENDIENTE-CI | sin `run_id` al sello documental |

## Fuera

Panel Espejo. Metabolismo (Fase 4). Re-forja launcher/systemd Kalma2. Mover crate a `daemons/`.

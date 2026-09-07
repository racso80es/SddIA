---
feature_name: thermal-resilience-heartbeat-mayeuta
created: "2026-09-07"
process: bug-fix
base: main
scope: daemon-heartbeat-audit+enrich-fracture-pbi-kaizen
branch_name: fix/thermal-resilience-heartbeat-mayeuta
persist_ref: docs/fixes/thermal-resilience-heartbeat-mayeuta
pbi_ref: docs/todos/pending/[FIX] Resiliencia Térmica en Heartbeat Audit y Poda Ontológica en Mayeuta.md
document_id: PBI-FIX-THERMAL-RESILIENCE-HEARTBEAT-MAYEUTA
execution_id: "0e0f6614-f1bd-40c5-9f63-af8e0a482786"
laudo: A
---

# Spec — puerta `btime` y cubo `orphan_lock`

## Problema

Tras apagado/reinicio del host, `daemon-heartbeat-audit` trata locks persistidos con PID muerto como colapso en caliente y emite `System_Fracture_Detected`. Mayeuta (`analyze_fracture_kaizen`) asigna a esa traza la causa raíz de **EDA genómica** porque `"huérfan"` ⊂ `"huérfano"`.

Especímenes: `3b71cec5263b`, `a10b5364a1c7`, `4543ba08b905`, `ca6c00385158`. Traza:

```text
Centinela {daemon_id} lock huérfano: PID {pid} muerto. last_heartbeat={ISO}
```

## Diagnóstico (causa raíz)

| ID | Defecto | Evidencia |
|----|---------|-----------|
| F0 | `audit_running_daemon` emite huérfano **antes** de baseline / `host_suspend` / boot | `daemon_heartbeat.rs`: `if !pid_alive { return emit_orphan_lock_fracture }` |
| F1 | Auditor sin timestamp de arranque de host | No hay lectura de `/proc/stat` `btime` |
| F2 | Cubo Mayeuta `orphan\|huérfan` sobre blob concatenado | Líneas `has_any(&["orphan", …, "huérfan"])` → `Domain_Entity_Created` |
| F3 | Cubo `heartbeat_starvation` no cubre esta familia | Exige `omitió` + `ciclos consecutivos de Daemon_Heartbeat` |

**No es:** umbral `missed_cycles`; PR GitHub `#118` (no existe; evolución real `c6931c73`); prompt/LLM; keepalive de centinela.

`host_suspend` ya existe y **no** es la puerta de reboot: el ramal `pid_alive` lo adelanta. `CLOCK_BOOTTIME` / `sysinfo.uptime` son duraciones; el comparable con `lock.started_at` ISO es `btime`.

Invariante L3 (`latido-ontologico-vitalidad-organos`): lock + PID muerto + `started_at >= boot_time` **sigue** emitiendo fractura. Test `orphan_lock_dead_pid_emits_fracture_once` no se relaja.

Launchers: `DaemonRuntime::bootstrap` ya recupera lock huérfano. Fuera de alcance mutar `start-sddia.sh`.

## Cambio requerido

### Vector A — motor `daemon_heartbeat.rs` (no DA-2)

1. `host_boot_time_utc() -> Option<DateTime<Utc>>` lee `btime` de `/proc/stat`.
2. `lock_predates_host_boot(started_at, last_hb, boot_time) -> bool`.
3. En `audit_running_daemon`, **antes** de `emit_orphan_lock_fracture`: si predates → `classification=host_reboot_stale_lock`, sin evento; `remove_lock` solo si `!pid_alive`.
4. Tests: inyectar `boot_time` (no depender del `btime` real de la máquina de CI). CA-2 inhibición; CA-3 L3 intacto.

### Vector B — motor `enrich_fracture_pbi_kaizen.rs` (no DA-2)

1. `is_orphan_lock_trace(error_trace)` con anclas del `format!` de `emit_orphan_lock_fracture`.
2. Cubo antes del genómico y del catch-all. Texto: ciclo de vida de daemon / lock; **sin** `Domain_Entity_Created`.
3. Cubo EDA genómica: exigir contexto genómico y `!is_orphan_lock_trace`.
4. `heartbeat_starvation` intacto.

### Genoma (DA-2, fase implementación)

| Entidad | Clase | Bump |
|---------|-------|------|
| `daemon-heartbeat-audit` | process | 1.1.0 → 1.2.0 |
| `enrich-fracture-pbi-kaizen` | action | 1.2.0 → 1.3.0 |

Vía `entity-manager`. Documentar `host_reboot_stale_lock`, puerta `btime`, cubo `orphan_lock`, acote genómico.

### Deuda satélite

Archivar 4 PBI en `done/` con `status: invalidado_por_falso_positivo`, `subsumed_by` de este `document_id`, `fix_ref` este `persist_ref`. No fusionar `document_id`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-1 | Puerta `btime` inhibe `emit_orphan_lock_fracture` si `started_at` &lt; boot |
| CA-2 | Test lock pre-boot + PID muerto → 0 fracturas + `host_reboot_stale_lock` |
| CA-3 | `orphan_lock_dead_pid_emits_fracture_once` verde |
| CA-4 | Traza canónica lock huérfano → cubo daemon; sin EDA genómica |
| CA-5 | Trampas: genómica real intacta; `timeout in worker` + acción audit no entra a `orphan_lock` ni starvation |
| CA-6 | `cargo test -p execute-process -- daemon_heartbeat enrich_fracture_pbi_kaizen` verde |
| CA-7 | Bumps vía `entity-manager` |
| CA-8 | 4 satélites + PBI madre en `done/`; `validacion.md` `pbi_archived: true` |
| CA-9 | Checks GitHub del PR verdes antes de `accept-pr` |

## Fuera de alcance

Umbrales Argos, keepalive, `start-sddia.sh`, red teaming, `CLOCK_BOOTTIME` como `started_at`, `SDDIA_PHAGOCYTE_APPLY=1` como cierre, forja manual de genoma.

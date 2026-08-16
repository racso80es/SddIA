---
feature_name: tekton-fire-and-forget
created: "2026-08-16"
process: feature
base: main
scope: tekton-fire-and-forget
branch_name: feat/tekton-fire-and-forget
persist_ref: docs/features/tekton-fire-and-forget
pbi_ref: docs/todos/pending/ARQUITECTURA] Erradicación de esperas síncronas en Tekton (Patrón Fire-and-Forget).md
document_id: PBI-TEKTON-FIRE-AND-FORGET
uuid: 3ad2901a-aaf4-4631-b5df-11386b3ea997
status: blueprint_locked
laudo: L-CLI-DETACH-ALLOWLIST
agents: dedalo
execution_id: 57dc7e51-9a48-4b98-a717-191da9070903
---

# Especificación — tekton-fire-and-forget

## 1. Misión técnica

Castrar la supervisión síncrona de Tekton post-Aduana y hacer que el CLI operador (`./sddia-run.sh` / `execute-process`) devuelva control al depositar señal en el bus fractal, sin join a centinelas ni a la carga larga. Sin reabrir PBI-044 (HTTP) ni el canal PTC.

## 2. Auditoría empírica (touchpoints)

| ID | Hecho | Implicación |
|----|-------|-------------|
| **T1** | `run_process` ejecuta handlers/fases en el mismo proceso y emite PEC **al final** (`thermodynamic.rs`) | El bloqueo de consola es el trabajo, no el watcher |
| **T2** | `event-watcher` invoca `execute-process` con `.output()` (join) para `route-*` y `radamanto-batch` | Detach por defecto de `radamanto-batch` **rompería** el contrato del watcher (doble consumo) |
| **T3** | `pull-request-review` es proceso largo (agentes Kalma2/IDE); el PBI lo cita como prueba de fricción | Allowlist de detach = este process (no el batch de telemetría) |
| **T4** | `emit_initialized_pec` ya escribe `Process_Execution_Completed` + `cycle_phase=awaiting_agents` en `eda_fractal.orchestration` | Reutilizar PEC; **no** forjar Clase ECST nueva |
| **T5** | PBI-044: spawn/detach en **kalma2-bridge** (proceso servidor vivo + reaper) | CLI debe **salir**; no copiar reaper-in-parent. Hijo huérfano → init reaps |
| **T6** | `entity-manager` `norm` → `directories.library_norms`, no `directories.norms` | Core `external-ai-constraints.md` no tiene creator; mutación in-ciclo + evolution (precedente ABSTRACT-03) |
| **T7** | `invoke_orchestrator` hace `.output()` (join) en subprocesos | Hijo detached lleva `SDDIA_CLI_FOREGROUND=1` para no re-detach |
| **T8** | `load_fractal_dirs` lee `eda_fractal` de Cúmulo | Depósito = `./.events/orchestration/`; nunca `.SddIA/events/` |

## 3. Laudos Dedalo

| Ref | Pregunta | Laudo |
|-----|----------|-------|
| **L1** | ¿Mecanismo V2? | **`L-CLI-DETACH-ALLOWLIST`.** El padre escribe PEC temprano, hace `Command::spawn` del mismo binario, imprime acuse JSON y **sale**. El hijo ejecuta el proceso en foreground. No watcher-as-executor (recursión + PEC sin suscriptores). No HTTP. |
| **L2** | ¿Allowlist? | Default: `pull-request-review`. **Excluido** `radamanto-batch` (consumidor del watcher, T2). Overlay: `SDDIA_CLI_DETACH_PROCESSES` (csv). `--detach` fuerza cualquier process; `--foreground` / `SDDIA_CLI_FOREGROUND=1` gana siempre. |
| **L3** | ¿Ciclos `feature`/`bug-fix`/`refactorization`? | **Foreground por defecto** (relevo IDE / workspace-init). Fuera del veto Mayeuta D3. |
| **L4** | ¿Evento de acuse? | **Reusar** `Process_Execution_Completed` con `cycle_phase: awaiting_agents` y `payload.detach: true`. Emisor `execute-process`. Misma hoja `eda_fractal.orchestration`. Prohibido Clase nueva en esta ola. |
| **L5** | ¿Zombies / SIGHUP? | Padre **no** espera. Unix: `process_group(0)` / setsid. stdio hijo → log bajo workspace (`paths.workspacesRoot/{process}/{execution_id}/detached.log`). Al salir el padre, init reaps. |
| **L6** | ¿Watcher? | Si el entorno es invocación de centinela, no detach. Heurística: `SDDIA_CLI_FOREGROUND=1` (watcher/daemons/TQM deben exportarla al invocar allowlist). `event-watcher` no invoca PPR hoy; defensa en profundidad: documentar export en daemons si en el futuro despachan allowlist. |
| **L7** | ¿Acuse JSON? | `success: true`, `exitCode: 0`, `data.status: accepted`, `data.detached: true`, `data.execution_id`, `data.correlation_id`, `data.event_id`, `data.pid`, `data.log_path`. Timing: **después** de write fractal + spawn, **antes** de cualquier wait. |
| **L8** | ¿Genoma? | `agents/tekton.md` → `entity-manager` update. `external-ai-constraints.md` → edición Core in-ciclo + evolution (T6). Touchpoints `.cursorrules` y `.cursor/rules/` **no** son genoma. |
| **L9** | ¿Kalma2 / PTC? | **Veto de toque** a `kalma2-bridge` y hoja `progress`. |
| **L10** | ¿Prueba de fricción? | Lab: allowlist + hijo que duerme ≥2 s; padre retorna p99 &lt; 500 ms; JSON de acuse presente; fichero ECST bajo `eda_fractal.orchestration`; log hijo muestra continuación. No exigir PPR real (GitHub/agentes) en CI. |

## 4. Contrato de acuse (CLI)

```json
{
  "success": true,
  "status_code": 0,
  "exitCode": 0,
  "data": {
    "status": "accepted",
    "detached": true,
    "process_name": "pull-request-review",
    "execution_id": "<uuid>",
    "correlation_id": "<uuid>",
    "event_id": "<uuid>",
    "pid": 0,
    "log_path": ".SddIA/workspaces/pull-request-review/<execution_id>/detached.log"
  }
}
```

`correlation_id`: input si existe; si no, `execution_id`. PEC `event_id` = sello fractal.

## 5. Algoritmo (padre)

```text
canonical = resolve(process)
if domain_authority deny → envelope fail (sync)
if SDDIA_CLI_FOREGROUND or --foreground → run sync
if not (--detach or canonical ∈ allowlist ∪ env overlay) → run sync
execution_id = uuid
write PEC (cycle_phase=awaiting_agents, detach=true) @ eda_fractal.orchestration
spawn self with SDDIA_CLI_FOREGROUND=1, same --process/--inputs, stdio→log, new process group
print ack JSON; exit 0
```

Hijo: `run_process` normal; peaje PEC terminal al cierre (T1).

## 6. Norma DA-5 (texto mínimo)

Tras acuse del CLI (`success` + JSON), queda **prohibido**: `sleep`/`wait`/`timeout` de shell, bucles de polling sobre `./.events/` o status, y `AwaitShell`/equivalentes post-acuse para vigilar watcher o artefactos. Éxito de Tekton = inyección acusada. Siguiente estímulo = Racso o Kalma2. Fuera: ticks de daemons; wait del propio CLI hasta el acuse; relevo IDE de fases `simulated` del ciclo activo.

## 7. Criterios de aceptación (técnicos)

| ID | Criterio | Evidencia |
|----|----------|-----------|
| **AC1** | DA-5 + cláusula Tekton + touchpoints | Diff genoma/touchpoints |
| **AC2** | PPR (o smoke allowlist) → consola &lt;500 ms + `detached:true` | Test lab + envelope |
| **AC3** | Evento en `./.events/orchestration/`, no en `.SddIA/events/` | Path Cúmulo |
| **AC4** | Hijo continúa; padre no join | log + timing |
| **AC5** | Watcher/`radamanto-batch` sigue join | no-regresión |
| **AC6** | Cierre documental en rama | `validacion.md` + PBI `done/` |

## 8. Fuera de alcance

Pasarela HTTP; PTC/SSE; Clase ECST nueva; watcher-as-executor; suciedad Radamanto en el working tree.

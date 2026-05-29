---
feature_name: fix-bucle-fantasma-sistema-nervioso
created: "2026-05-29"
process: bug-fix
branch_name: fix/bucle-fantasma-sistema-nervioso
persist_ref: docs/fixes/fix-bucle-fantasma-sistema-nervioso
index_prefix: "[ARQUITECTURA]"
pbi_ref: docs/todos/pending/PBI-FIX-BUCLE-FANTASMA-SISTEMA-NERVIOSO.md
related_incident: "Re-enrutamiento fantasma en event-watcher con JSON estancados en colas fractal (.events/) tras stress core-full-stress en Windows"
---

# Objetivos — [ARQUITECTURA] fix-bucle-fantasma-sistema-nervioso

## Misión

Blindar el **Sistema Nervioso Central** (bus EDA runtime bajo `/.events/`) contra la latencia y el bloqueo de E/S en Windows: eliminar el bucle fantasma del `event-watcher`, endurecer la purga física de instancias ECST y proveer triaje de laboratorio para colas activas contaminadas tras colapsos previos.

## Contexto

| Señal | Detalle |
|-------|---------|
| Programa | Inmunidad/Caos Fase 4–5 (`Suite_Execution_Requested` → `System_Immunity_Certified`) |
| Topología | Bus fractal: `domain/`, `telemetry/`, `orchestration/` vigilados por watcher (`SDDIA_LAB_WATCH_FRACTAL=1`) |
| Mandato PBI | Tres fases: idempotencia en caliente, micro-sleeps en archivado, purga zona cero |

## Objetivos medibles

| ID | Fase | Objetivo | Criterio |
|----|------|----------|----------|
| **O1** | F1 | Idempotencia en caliente | Mismo `event_id` no dispara segundo `execute-process` mientras UUID ∈ `processing_uuids` o política D3 activa |
| **O2** | F1 | Liberación determinista | UUID sale de `processing_uuids` solo al retorno del subproceso (código de salida oficial) |
| **O3** | F2 | Absorción latencia E/S | `safe_remove_path`: hasta 3 intentos, 50 ms entre intentos, en rutas de purga fractal y utilidades bus |
| **O4** | F2 | Telemetría de fallo | Si agota reintentos: log explícito; no declarar `purged: true` si el archivo persiste |
| **O5** | F3 | Purga zona cero (lab) | `purge_stale_events.py --dry-run` lista candidatos; `--apply` reduce colas activas sin tocar dead-letter sin flag explícito |
| **O6** | — | No regresión sweeper / pending | `try_sweep_event`, `event-sweeper.py` y flujo `pending/` V3+ sin cambio de semántica Kaizen |
| **O7** | — | UX operador | Logs `[WATCHER]` distinguen skip-idempotente vs nuevo evento vs purga fallida |
| **O8** | — | Trazabilidad histórica | Commits y PR con prefijo documental **`[ARQUITECTURA]`** |

## No objetivos

- Implementar gobernanza reactiva post-`Telemetry_Compliance_Breached`.
- Modificar genoma ECST (`SddIA/events/`) ni `event-*-subscriptions.json` salvo enlace roto.
- Automatizar `purge_stale_events` en CI (solo documentación de uso lab en `execution.md`).
- Sustituir `event-sweeper` como recolector periódico de `pending/`.

## Ley aplicada

- Proceso `bug-fix` v1.4.0
- `SddIA/events/events-contract.md` § instancias ECST y familias fractal
- `SddIA/process/bug-fix.md` — cierre documental en rama (`validacion.md`, PBI → `done/`)
- Regla workspace `task-closure-documental` — un único PR

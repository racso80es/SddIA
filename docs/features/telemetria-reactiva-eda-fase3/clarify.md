---
feature_name: telemetria-reactiva-eda-fase3
created: "2026-05-27"
purpose: Decisiones Fase 3 y herencia Fases 0–2
---

# Clarificación — Fase 3

## Precondición (gates Fases 1–2)

- **Fase 1** mergeada (PR #52): genoma fractal, `event_family`, `Raw_Execution_Finished` en `SddIA/events/telemetry/`.
- **Fase 2** mergeada (PR #53): `workspace_template`, `paths.workspacesRoot`, instanciación CLI, `workspace_path` en contexto de agente.
- No se reabren Fases 1–2 salvo hallazgo bloqueante durante Tekton.

## Decisiones heredadas (aplican en Fase 3)

| ID | Resolución | Uso en Fase 3 |
|----|------------|---------------|
| D0.2 | Coexistencia V3+ + bus fractal | `route-domain-event` + `eda_bus.pending` permanecen; nuevas familias usan rutas `./.events/{family}/` |
| D0.4 | `event-watcher` evoluciona a multi-ruta | Watcher observa `pending/` (legacy) **y** rutas fractales según familia |
| D0.5 | Peaje Termodinámico solo CLI | Cronómetro y emisión `Raw_Execution_Finished` en `execute_process_capsules`; ED prohibidas |
| D2.7 | `workspace_path` en contexto | Formalizar en payload ECST de eventos `orchestration` |
| Axioma §0.3 | Simetría fractal genoma ↔ runtime | SSOT declara rutas homólogas a `SddIA/events/{family}/` |

## Decisiones cerradas — Fase 3

| ID | Pregunta | Resolución |
|----|----------|------------|
| D3.1 | ¿Dónde declarar rutas fractales? | Bloque **`eda_fractal`** en `cumulo.paths.json` (v1.2.0): `telemetry`, `orchestration`, `domain` bajo `./.events/`; convive con bloque `eda_bus` V3+ intacto |
| D3.2 | ¿Migración de `event-subscriptions.json`? | **Split físico:** crear tres archivos nuevos; el monolito actual se renombra lógicamente a **`event-domain-subscriptions.json`** (mismo contenido dominio + PR + Kaizen); referencia legacy en `eda_bus.subscriptions` hasta acta de retirada |
| D3.3 | ¿Enrutadores nuevos o extensión de `route-domain-event`? | **Tres procesos hermanos:** `route-telemetry`, `route-orchestration`, `route-domain` — cada uno clona el patrón V3+ de `route-domain-event` pero lee su archivo de suscripción y ruta fractal; **`route-domain-event` permanece** como alias operativo sobre dominio legacy `pending/` |
| D3.4 | ¿Alcance del Peaje Termodinámico? | Toda invocación de **`execute_process_capsules`** que ejecute al menos una fase no simulada (proceso completo o acción indexada vía CLI lab); excluir handlers puros de git-only sin cápsula de ejecución |
| D3.5 | ¿Origen de `asset_id`? | UUID v4 generado al **inicio** de la ejecución del proceso (mismo ciclo que `execution_id` Fase 2); registrar en `state["asset_id"]` |
| D3.6 | ¿Evento de orquestación post-éxito? | Forjar Clase **`Process_Execution_Completed`** en `SddIA/events/orchestration/` vía `event-creator`; payload mínimo: `process_name`, `asset_id`, `workspace_path`, `status`, `phase_count`; emisión solo si veredicto global `success` |
| D3.7 | ¿Radamanto sin agente? | Suscripción telemetría apunta a proceso **`telemetry-batch-stub`** (lab no-op que registra consumo y purga archivo); contrato Radamanto → Fase 4 |
| D3.8 | ¿Escritura por familia? | Nueva función `write_fractal_event(repo, event, family)` — resuelve carpeta vía `eda_fractal.{family}`; **prohibido** escribir telemetría en `pending/` |
| D3.9 | ¿Validación ECST telemetría? | Reutilizar validador existente extendido con `event_family`; telemetría sin gate Argos síncrono (ruido físico) |
| D3.10 | ¿Watcher por familia? | Un watcher (`event-watcher.py`) con **tres watchers internos** o poll unificado sobre lista de rutas; cada JSON nuevo despacha al proceso enrutador según carpeta origen |
| D3.11 | ¿Instancias dominio existentes? | Los 7 ECST en `domain/` ya cumplen `event_family: domain`; instancias nuevas en `./.events/domain/` usan mismo esquema payload; legacy sigue en `pending/` hasta migración voluntaria |
| D3.12 | ¿Persistencia encapsulada? | Documentar en spec §3.F el flujo `filesystem-manager` + `capsule-json-io`; **sin** refactor masivo de agentes obreros — smoke en `execution.md` con delegación existente |

## Payload Peaje Termodinámico (`Raw_Execution_Finished`)

| Campo | Origen | Obligatorio |
|-------|--------|:-----------:|
| `asset_id` | CLI UUID inicio ejecución | Sí |
| `exit_code` | Retorno última cápsula / veredicto proceso | Sí |
| `duration_ms` | `time.monotonic()` delta | Sí |
| `process_name` | Definición de proceso | Sí |
| `execution_id` | Herencia Fase 2 | Recomendado |
| `workspace_path` | Herencia Fase 2 | Recomendado |
| `telemetry_receipt` | stdout cápsula | No (Fase 5) |

## Jurisdicciones (Panteón — §3.E)

| Agente | Rol Fase 3 | Alcance esta feature |
|--------|------------|----------------------|
| **Argos** | Inspector de materia | Sigue en suscripciones dominio/orquestación (`pull-request-review`); sin cambio de contrato |
| **Radamanto** | Actuario (stub) | Solo suscripción cableada; sin DLT ni umbrales |

## Referencias

- Gate Fase 0: `impact-analysis.md` (H04–H09, H13, D0.2, D0.4, D0.5)
- Gate Fase 1: `docs/features/telemetria-reactiva-eda-fase1/validacion.md`
- Gate Fase 2: `docs/features/telemetria-reactiva-eda-fase2/validacion.md`
- PBI: § Fase 3 (3.A–3.F)
- Origen consolidado: `docs/todos/tmp/Telemetría Reactiva SddIA_V2.md`

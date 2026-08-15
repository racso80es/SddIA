---
feature_name: kalma2-canal-telemetria-progreso
created: "2026-08-15"
process: feature
branch_name: feat/kalma2-canal-telemetria-progreso
persist_ref: docs/features/kalma2-canal-telemetria-progreso
pbi_ref: docs/todos/pending/[OPERATIVO] PBI: Canal Asíncrono de Telemetría de Progreso y Observabilidad Activa para Interfaces Externas (Kalma2).md
document_id: PBI-KALMA2-CANAL-TELEMETRIA-PROGRESO
uuid: c8f4a2e1-9d3b-4f67-a1c5-8e2b6d09f4a7
status: blueprint_locked
mayeuta_verdict: ok
dedalo_verdict: ok
laudo: C-ephemeral-progress-leaf
depends_on:
  - docs/features/kalma2-event-bus-integration
  - docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
adjacent_not_merged:
  - docs/todos/pending/[Kaizen] Process_Execution_Completed — suscriptores y auditoría de circuito EDA.md
---

# Objetivos — kalma2-canal-telemetria-progreso

## Misión

Dotar al ecosistema de un **canal efímero fire-and-forget** de trazas de progreso para interfaces externas (Kalma2) durante `execute-process`, sin violar Ceguera Espacial, sin contaminar el bus de dominio y sin colisionar ontológicamente con la familia `telemetry` de peaje termodinámico / compliance.

## Punto objetivo

> **O-PROGRESO-KALMA2:** El operador ve chispazos de fase (Spec→Closure) en la WUI mientras el Core emite sin conocer clientes; el veredicto terminal sigue siendo proyección de correlación (`GET /api/status` / PEC), no sustituido por el stream de progreso.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Contrato semántico de traza de progreso (canal efímero distinto) | Reutilizar familia `telemetry` peaje (`Raw_Execution_Finished`, `Daemon_Heartbeat`) |
| Emisión no bloqueante desde orquestación de fases | Escritura bajo `.SddIA/events/` o capability-contract como schema de progreso |
| Proyección hacia bridge/WUI (lector; SSE de progreso ≠ SSE chat) | Sustituir polling de veredicto terminal |
| Consola cromática WUI + poda de huérfanos | Liquidar Kaizen PEC / 404 post-purge de `GET /api/status` |
| Dualidad veredicto vs progreso | Latencia &lt;100 ms como invariante Core |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Ontología aislada | Trazas de progreso fuera del fan-out peaje (`route-telemetry` → Radamanto/compliance) y fuera de `eda_fractal.domain` |
| **O2** | Topología correcta | Ninguna ruta de emisión/documentación usa `.SddIA/events/` como bus; resolución vía Cúmulo (`eda_fractal` / leaf que Dedalo proponga sin romper I1) |
| **O3** | Fire-and-forget | Bridge caído ⇒ ejecución completa sin error ni reintento bloqueante por progreso |
| **O4** | Dual-canal UI | `GET /api/status` (o equivalente de veredicto) permanece; progreso es canal adicional |
| **O5** | Consumo WUI | Consola reactiva cromática por severidad + badge de agente; sin reusar SSE de chat como bus |
| **O6** | Higiene | Sweeper/poda elimina trazas huérfanas o de ejecuciones cerradas/expiradas |
| **O7** | AC interfaz | Refresco WUI &lt;100 ms desde escritura en el canal = criterio Kalma2, no gate Core |
| **O8** | Grade S+ perímetro | Touchpoints Rust del cambio sin panics/warnings; validación alineada a `clarify.md` AC1–AC7 |

## No objetivos

- Fusionar con `PBI-KAIZEN-PEC-SUBSCRIBERS-CIRCUIT-AUDIT`.
- Homologar envelope PBI crudo como ECST sin `event_type` / `emitter_agent` / `payload`.
- Declarar schema de progreso como capability-contract.
- Invalidar laudos `kalma2-event-bus-integration` / PBI-044 sobre status=veredicto.

## Ley aplicada

- Ceguera espacial del orquestador y del bridge (emisión/lectura sin business coupling).
- `SddIA/core/cumulo.paths.json` — `eda_fractal`, `eda_instance.customization`, `directories.capability_contracts`.
- `SddIA/events/events-contract.md` (ECST) — solo si Dedalo elige Clase; peaje `SddIA/events/telemetry/` aislado.
- `features-documentation-pattern` v1.2.1 / proceso `feature`.
- Clarificaciones D0–D7 en `clarify.md` (laudo canal = **opción C**, efímero distinto).

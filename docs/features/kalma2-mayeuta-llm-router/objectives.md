---
feature_name: kalma2-mayeuta-llm-router
created: "2026-06-19"
process: feature
branch_name: feat/kalma2-mayeuta-llm-router
persist_ref: docs/features/kalma2-mayeuta-llm-router
pbi_ref: docs/todos/pending/[FEATURE] kalma2-mayeuta-llm-router — síntesis LLM y enrutamiento a procesos.md
uuid: def280fd-73a3-42fe-b485-3258f1e5e426
status: pre-implementation
---

# Objetivos — kalma2-mayeuta-llm-router

## Misión

Evolucionar la síntesis Mayeuta de `kalma2-interact` de **eco determinista** a **intérprete LLM con enrutamiento a procesos**: (a) usar un LLM real vía cápsula inyectable y (b) cuando el prompt exprese una intención ejecutable que corresponda a un proceso del Core, despacharlo mediante `action:execute-process` (procesos cortos) o emisión de evento de dominio (procesos largos).

## Punto objetivo (añadido)

> **O-MAYEUTA-LLM:** Adecuar `synthesize_mayeuta_response` para que use un LLM real y, si el prompt corresponde a un proceso del Core, lo ejecute vía `SddIA/actions/execute-process.md`.

## Invariante de arquitectura

El binario `kalma2-bridge` permanece **inerte** (ceguera espacial intacta). Toda la inteligencia reside en el genoma: handler `kalma2-interact` + nueva cápsula `mayeuta-llm`. La ejecución de lógica vive en cápsulas (`.cursorrules` §4); el motor de inferencia es agnóstico por inyección de configuración (§5).

## Clarificaciones vinculantes (C1–C3)

| Ref | Resolución | Efecto en objetivos |
|-----|------------|---------------------|
| **C1** | `mayeuta-llm` es **Skill** (no tool). | O7 fija tipología Skill. |
| **C2** | Enrutamiento a procesos de ciclo de vida **estrictamente asíncrono** (bus EDA, evento `Kalma2_Process_Requested`). | O10 emite evento y acuse inmediato; sin despacho síncrono. O14 cierra el lazo con suscriptor ejecutor. |
| **C3** | Motor de inferencia: **CLI de Cursor local** vía `std::process::Command`, comando por `SDDIA_LLM_CLI_COMMAND`. | O7/O8 sin cliente HTTP ni SDK; fallback si falta el comando. |

Detalle en `spec.md §2`.

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O7** | Cápsula LLM como **Skill** (C1) | Skill `mayeuta-llm` (Rust) con contrato JSON stdin/stdout; motor CLI Cursor por `SDDIA_LLM_CLI_COMMAND` (sin hardcode, C3) |
| **O8** | Síntesis LLM con fallback | `kalma2-interact` usa el CLI; si falta `SDDIA_LLM_CLI_COMMAND` o falla, degrada a `synthesize_mayeuta_response` determinista sin romper contrato |
| **O9** | Clasificación de intención | El handler obtiene del CLI `{intent: chat\|execute, process_name?, process_inputs?, confidence}` con esquema estricto |
| **O10** | Enrutamiento **asíncrono** (C2) | `intent=execute` + `process_name` en allowlist → emitir evento `manual-task-requested` al bus EDA + acuse inmediato; nunca síncrono |
| **O11** | Allowlist de procesos | Solo procesos en allowlist explícita son solicitables desde Kalma2 |
| **O12** | Paridad preservada | `telegram-fallback-responder` mantiene su síntesis determinista (sin regresión) |
| **O13** | Cerbero / contexto subproceso | La cápsula declara contexto de **ejecución de subproceso local** (CLI) y pasa el gate de políticas |
| **O14** | Cierre del lazo EDA | Evento dedicado `Kalma2_Process_Requested` + suscriptor `process` + rama en `dispatch_subscriber`, de modo que el evento emitido **ejecute** el proceso (no solo se ancle en DLT) |

## Brecha EDA detectada (verificada en código)

El watcher (`event-watcher`) y el enrutador (`route-domain-event`) **ya existen y son genéricos**, pero `Manual_Task_Requested` solo tiene suscriptor `iota-immutable-publisher` (anclaje DLT); **nadie ejecuta la tarea**. Además `dispatch_subscriber` solo mapea `telegram-fallback-responder` o payloads de PR (`branch`), y el ECST de `Manual_Task_Requested` rechazaría `{process, pbi_ref}` con emisor `kalma2-interact`. O14 cierra el lazo con tres entregables (P1–P3, ver `spec.md §6.bis`):

| # | Entregable |
|---|------------|
| **P1** | Evento `kalma2-process-requested.md` + suscriptor `process` en `event-domain-subscriptions.json` |
| **P2** | Rama en `dispatch_subscriber` que construye `process_inputs` desde `{process, pbi_ref}` |
| **P3** | Esquema ECST propio + emisor autorizado `kalma2-interact` + cobertura `eda-coverage.json` |

## No objetivos

- Cablear un proveedor LLM concreto en el código (motor por `SDDIA_LLM_CLI_COMMAND`, C3).
- Introducir cliente HTTP / SDK de red para inferencia en el Genoma Rust (C3).
- Ejecutar procesos de ciclo de vida de forma síncrona dentro del POST HTTP (C2).
- Permitir despacho de cualquier proceso sin allowlist.
- Conversación con estado/historial (cada POST sigue siendo atómico; "Estado Cero").
- Modificar el binario `kalma2-bridge`.

## Ley aplicada

- `.cursorrules` §4 (cápsulas Rust, JSON stdin/stdout), §5 (agnosticismo Core)
- `SddIA/actions/execute-process.md` v1.2.0 (ejecutor canónico, invocado por el Sistema Nervioso vía EDA — C2)
- `SddIA/norms/external-ai-constraints.md`
- `SddIA/core/cumulo.paths.json` (SSOT rutas)
- `features-documentation-pattern` v1.2.0 / proceso `feature` v1.3.0
- Clarificaciones C1 (Skill), C2 (asincronía EDA), C3 (CLI Cursor / `SDDIA_LLM_CLI_COMMAND`)

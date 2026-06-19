---
document_id: PBI-FEATURE-KALMA2-MAYEUTA-LLM-ROUTER
title: "[FEATURE] kalma2-mayeuta-llm-router — síntesis LLM y enrutamiento a procesos"
format: markdown
version: "1.0.0"
created: "2026-06-19"
status: done
priority: alta
process: feature
branch_name: feat/kalma2-mayeuta-llm-router
feature_ref: docs/features/kalma2-mayeuta-llm-router
validacion_ref: docs/features/kalma2-mayeuta-llm-router/validacion.md
uuid: c58db5d4-0521-4798-8344-dd490aed91d0
---

# PBI-FEATURE: kalma2-mayeuta-llm-router

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-FEATURE-KALMA2-MAYEUTA-LLM-ROUTER` |
| **Estatus** | 🔨 Forja física completada — pendiente PR |
| **Feature** | [`docs/features/kalma2-mayeuta-llm-router/`](../../features/kalma2-mayeuta-llm-router/) |
| **Rama** | `feat/kalma2-mayeuta-llm-router` |

## Objetivo táctico

Adecuar `synthesize_mayeuta_response` para usar un LLM real (CLI de Cursor) y, cuando el prompt corresponda a un proceso del Core, despacharlo **asíncronamente** vía evento de dominio que el Sistema Nervioso enruta a `SddIA/actions/execute-process.md`. El binario `kalma2-bridge` permanece inerte; la inteligencia vive en el handler `kalma2-interact` + Skill `mayeuta-llm`.

## Entregable de esta iteración

**Documentación pre-implementación** (DA-4 RAW Kernel: topología `docs/features/` instanciada antes de mutar genoma):

| Doc | Estado |
|-----|--------|
| `objectives.md` (O7–O13) | ✅ |
| `spec.md` | ✅ |
| `plan.md` | ✅ |
| `implementation.md` (diseño + blueprint) | ✅ |

## Pendiente (forja física — fase tekton)

| Fase | Entregable | Estado |
|------|------------|--------|
| A | Skill `mayeuta-llm` (andamiaje, C1) | ✅ |
| B | Transductor CLI Cursor + SYNTHESIZE + fallback (C3) | ✅ |
| C | CLASSIFY_INTENT + umbral + heurística | ✅ |
| D | Handler kalma2 con Skill + degradación | ✅ |
| E | Allowlist + emisión `Kalma2_Process_Requested` (C2) | ✅ |
| E2 | Cierre lazo EDA (P1–P3, O14) | ✅ |
| F | Cerbero/contexto + smokes + validacion.md | ✅ |
| — | PR + merge + PBI → done/ | ⏳ |

## Deudas técnicas (registro)

| ID | Deuda | Impacto |
|----|-------|---------|
| D1 | `SDDIA_LLM_CLI_TIMEOUT_SECS` documentado pero no implementado en mayeuta-llm (wait bloqueante) | Riesgo de bloqueo UI si CLI cuelga |
| D2 | Skill `mayeuta-llm` solo binario nativo; sin artefacto WASI | Paridad WASM pendiente |
| D3 | `task-queue-manager` no tiene rama nativa Kalma2; despacho vía inputs genéricos (`process`, `pbi_ref`) | Triaje puede requerir ampliación dedicada |
| D4 | `hash_signature` pending-forge en `mayeuta-llm.md` y `kalma2-process-requested.md` | Gate Cerbero post-merge |
| D5 | Validación E2E con Cursor CLI real no ejecutada en CI (solo echo mock + heurística) | Calidad inferencia en prod |
| D6 | Gate Cerbero `local-subprocess` declarado pero no verificado en pipeline CI | O13 parcial |

## Cierre del lazo EDA (O14 — 3 puntos verificados)

El watcher (`event-watcher`) y el enrutador (`route-domain-event`) ya existen y son genéricos, pero hoy ningún suscriptor **ejecuta** la tarea (`Manual_Task_Requested` solo ancla en DLT). Entregables:

| # | Brecha | Implementación |
|---|--------|----------------|
| **P1** | Sin suscriptor ejecutor | Evento `SddIA/events/domain/kalma2-process-requested.md` (`Kalma2_Process_Requested`, uuid `458c34a8-9ad5-4a40-88c4-0be1e5d9598e`) + suscriptor `process` (recomendado `task-queue-manager`) en `event-domain-subscriptions.json` |
| **P2** | `dispatch_subscriber` exige `branch` (`route_domain_event_core.py:154`) | Rama para `Kalma2_Process_Requested`: `process_inputs` desde `{process, pbi_ref, raw_text}` + gate allowlist |
| **P3** | ECST de `Manual_Task_Requested` rechazaría el payload/emisor | Esquema propio (REQUIRED `process`,`raw_text`) + emisor `kalma2-interact` + `eda-coverage.json` |

**Decisión:** evento dedicado `Kalma2_Process_Requested` en lugar de reutilizar `Manual_Task_Requested` (semántica sensorial Telegram, emisor restringido). Detalle en `spec.md §6.bis`.

## Decisiones resueltas (C1–C3 — vinculantes)

1. **C1** — `mayeuta-llm` se forja como **Skill** (descartada tool): `CLASSIFY_INTENT` exige consciencia de enrutamiento.
2. **C2** — Enrutamiento a procesos de ciclo de vida **estrictamente asíncrono** vía bus EDA (evento `manual-task-requested` + acuse inmediato); "Estado Cero".
3. **C3** — Motor de inferencia: **CLI de Cursor local** vía `std::process::Command`; comando por `SDDIA_LLM_CLI_COMMAND` (sin red/SDK en el Genoma).
4. Allowlist: `{bug-fix, feature, refactorization, task-queue-manager}`.

## Hallazgos verificados (base del diseño)

- `synthesize_mayeuta_response` compartida kalma2/telegram-fallback → no mutar firma.
- `telegram-gateway` ya emite eventos al bus (`fractal::write_fractal_event`) → patrón para el despacho asíncrono.
- Sin infraestructura LLM previa en `engine/` → Skill como transductor CLI local.

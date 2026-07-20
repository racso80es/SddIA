---
document_id: PBI-KALMA2-LLM-LIVE
uuid: f0f1b1ec-4b79-47c6-85e2-a0ac2ca3164b
title: "[FEATURE] kalma2-llm-live — ejecución real de LLM/agentes Cursor desde Kalma2"
format: markdown
version: "1.0.0"
created: "2026-07-20"
status: "abierto"
priority: alta
process: feature
suggested_feature_name: kalma2-llm-live
suggested_branch: feat/kalma2-llm-live
depends_on:
  - docs/features/kalma2-full-cycle
  - docs/features/kalma2-mayeuta-llm-router
  - docs/features/kalma2-event-bus-integration
related:
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/skills/mayeuta-llm/src/main.rs
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - interfaces/kalma2/
  - .dev/.env.example
inherited_from: docs/todos/done/[FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre (527007fa).md
evidence:
  - "Bóveda activada 2026-07-20 (SDDIA_AGENT_RUNTIME_* + SDDIA_LLM_CLI_COMMAND) — no versionable (.gitignore)"
  - "Host lab: cursor-agent ausente de PATH → soft awaiting_agents"
---

# [FEATURE] kalma2-llm-live — ejecución real de LLM/agentes Cursor desde Kalma2

## Estado

**v1.0.0 abierto.** Tras `kalma2-full-cycle` (APTO + PR #122) y activación en bóveda local, faltan eslabones de **instancia/host** y endurecimiento para que Kalma2 ejecute LLM y fases de agentes de forma real (no mock / no soft-await).

## Ya entregado (no reabrir)

| Capacidad | Dónde |
|-----------|--------|
| Emisión EDA + poll status + `cycle_phase` | kalma2-event-bus-integration · kalma2-full-cycle A |
| Hook `SDDIA_AGENT_RUNTIME_COMMAND` | `agent_runtime.rs` |
| Wrapper Cursor CLI/SDK/mock | `kalma2-agent-runtime-cursor.{sh,py}` |
| `pbi_body` en despacho | TQM + workspace_init |
| Claves en bóveda local | `.dev/.env` + `.SddIA/.dev/.env` (ignoradas por git) |

## Huecos para ejecución real (checklist)

### H1 — Binario Cursor CLI en el host

| Ítem | Detalle |
|------|---------|
| Síntoma | `cursor-agent` no está en PATH → wrapper responde `awaiting_agents` |
| Acción | Instalar CLI Cursor Agent **o** fijar ruta absoluta en bóveda: `SDDIA_AGENT_RUNTIME_CLI=/ruta/cursor-agent --print` y `SDDIA_LLM_CLI_COMMAND=…` |
| Criterio | `command -v` / exec del CLI con prompt corto → stdout no vacío |

### H2 — Chat Kalma2 con LLM (mayeuta-llm)

| Ítem | Detalle |
|------|---------|
| Camino | `POST /api/interact` sin intent execute → Skill `mayeuta-llm` → `SDDIA_LLM_CLI_COMMAND` |
| Hueco | Mismo CLI ausente; sin él la UI marca `[degradado]` (fallback determinista) |
| Acción | Misma instalación H1; smoke chat en Kalma2 sin prefijo «inicia fix» |
| Criterio | Envelope `degraded≠true` y respuesta no-determinista |

### H3 — Fases agent: live tras despacho execute

| Ítem | Detalle |
|------|---------|
| Camino | Kalma2 execute → TQM → hijo `bug-fix`/`feature` → fases Dedalo/Tekton/Argos → `agent-runtime` → wrapper Cursor |
| Hueco | Depende de H1 (o H4 SDK); reiniciar `kalma2-bridge` / ecosistema tras cambiar bóveda |
| Acción | Prompt fix con PBI; verificar `_agent_handoff.md` con `backend: cli` y `status: executed`; artefactos bajo `persist_ref` |
| Criterio | `cycle_phase=completed` (sin `simulated`) o al menos fases `executed` con transcript |

### H4 — Backend SDK (alternativa a CLI)

| Ítem | Detalle |
|------|---------|
| Acción | `pip install cursor-sdk`; bóveda: `SDDIA_AGENT_RUNTIME_BACKEND=sdk` + `CURSOR_API_KEY` + `SDDIA_AGENT_RUNTIME_MODEL` |
| Criterio | AGENT_PHASE vía SDK → `executed` en host sin `cursor-agent` |

### H5 — Timeout LLM / runtime

| Ítem | Detalle |
|------|---------|
| Deuda heredada | `SDDIA_LLM_CLI_TIMEOUT_SECS` documentado pero **no** implementado en `mayeuta-llm` (wait bloqueante) |
| Acción | Implementar timeout en Skill mayeuta-llm; alinear con `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` del wrapper |
| Criterio | CLI colgado no bloquea el bridge más allá del timeout |

### H6 — Propagación de bóveda en procesos hijos

| Ítem | Detalle |
|------|---------|
| Hecho | `execute-process` llama `load_hierarchical_env` al arrancar |
| Verificar | Tras `start-sddia` / bridge, el hijo ve `SDDIA_AGENT_RUNTIME_COMMAND` y el CLI |
| Riesgo | Bridge arrancado fuera de `sddia-run`/`start-sddia` sin env cargado en el padre (el hijo sí recarga bóveda — validar empíricamente) |

### H7 — Sistema Nervioso operativo

| Ítem | Detalle |
|------|---------|
| Requisito | `event-watcher` (y daemons asociados) activos para consumir `Kalma2_Process_Requested` |
| Criterio | Evento no queda huérfano en pending; TQM corre; status UI avanza |

### H8 — Cierre de ciclo de negocio (post-agentes)

| Ítem | Detalle |
|------|---------|
| Contexto | L2 process-dispatch sigue skip archive/delivery salvo `SDDIA_TQM_FULL_CYCLE=1` |
| Acción | Solo tras H3 estable: valorar full-cycle o que agentes dejen artefactos listos y un operador/Argos cierre |
| Criterio | Definir laudo: ¿Kalma2 debe abrir PR solo o hasta `initialized`+agentes? |

### H9 — (Opcional) Evento EDA de handoff B2

| Ítem | Detalle |
|------|---------|
| Alcance | `Process_Agent_Handoff_Requested` + cola IDE — no bloquea LLM live mínimo |
| Ref | open_debt de kalma2-full-cycle |

## Fuera de alcance

- Re-forjar aduana bridge / emisión ECST (ya APTO).
- Remediación de fracturas daemon (`*-watcher`) salvo como smoke de H7.
- Versionar secretos o `.dev/.env` / `.SddIA/.dev/.env` (gitignore).

## Objetivo medible

Desde Kalma2 en `127.0.0.1:8765`:

1. **Chat:** respuesta LLM no degradada.
2. **Execute:** proceso allowlist encola, agentes Cursor ejecutan fases, status honesto (`awaiting_agents` solo si runtime espera; `completed` solo con trabajo real).
3. Runbook de instancia documentado (instalación CLI o SDK + reinicio servicios).

## Mandato

Forjar feature `kalma2-llm-live` (o bug-fix operativo de instancia) cerrando H1–H7 como mínimo; H8/H9 con laudo Racso.

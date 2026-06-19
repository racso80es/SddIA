---
feature_name: kalma2-mayeuta-llm-router
created: "2026-06-19"
process: feature
base: main
scope: kalma2-mayeuta-llm-router
version_spec: "1.0.0"
uuid: def280fd-73a3-42fe-b485-3258f1e5e426
status: pre-implementation
---

# Especificación — kalma2-mayeuta-llm-router

## 1. Topología de responsabilidades

```text
kalma2-bridge (Rust, inerte)          ← SIN cambios
   └─ POST /api/interact → execute-process --process kalma2-interact
        └─ handlers::kalma2::run                ← evoluciona
             ├─ filter_c_should_abort (pre-gate)
             ├─ 1. clasificar intención (cápsula mayeuta-llm: CLASSIFY_INTENT)
             ├─ 2a. intent=chat    → SYNTHESIZE LLM (o fallback determinista)
             └─ 2b. intent=execute → gate allowlist + Cerbero
                     ├─ proceso corto → invoke_orchestrator::invoke_process(execute-process)
                     └─ proceso largo → emitir evento dominio (route async / task-queue-manager)
```

## 2. Clarificaciones vinculantes (C1–C3)

Decisiones resueltas que fijan las opciones antes abiertas. Son **normativas** para la forja.

| Ref | Resolución | Justificación |
|-----|------------|---------------|
| **C1** | `mayeuta-llm` se forja **obligatoriamente como Skill** (descartada tipología *tool*). | `CLASSIFY_INTENT` exige comprensión de la semántica de enrutamiento y de la arquitectura del ecosistema; ese grado de consciencia la eleva a Skill frente a las *tools* de Ceguera Espacial absoluta (p. ej. `shell-executor`). |
| **C2** | Enrutamiento a procesos de ciclo de vida (`bug-fix`, `feature`, `refactorization`) **estrictamente asíncrono** vía bus EDA. | Preserva "Estado Cero" y baja latencia de la UI web. `kalma2-interact` transmuta el lenguaje natural en evento de dominio (`manual-task-requested`), lo inyecta en el bus y devuelve acuse inmediato. Los demonios del Sistema Nervioso despiertan a los agentes en segundo plano. |
| **C3** | Motor de inferencia: **CLI de Cursor invocado localmente** vía `std::process::Command`. Comando físico inyectado por bóveda `SDDIA_LLM_CLI_COMMAND`. | Evita alojar modelo local robusto (límite del sustrato) y acoplamiento tóxico (red/SDK cerrado de OpenAI en el Genoma Rust). La cápsula opera como **transductor local**: ensambla prompt → ejecuta CLI → captura stdout. Portabilidad y limpieza del código por inyección de configuración. |

## 3. Decisión clave — anti-bloqueo síncrono (C2)

El handler `kalma2-interact` se ejecuta **síncrono** dentro de la petición HTTP. Un proceso de ciclo de vida (`bug-fix`, `feature`, `refactorization`) NO debe ejecutarse en línea: colgaría el POST y violaría el "Estado Cero".

**Ley de enrutamiento reactivo (C2):** todo despacho a proceso de ciclo de vida emite un **evento de dominio** (`manual-task-requested` con `process` + `pbi_ref`/`task_text`) al bus EDA (patrón `telegram-gateway`), devolviendo acuse al operador. El centinela `event-watcher` despierta a los agentes en segundo plano. No hay acoplamiento síncrono de procesos pesados.

> Nota: en este modelo, la allowlist de despacho síncrono vía `action:execute-process` queda reservada a procesos triviales/idempotentes (si los hubiera). Por C2, los procesos de ciclo de vida nunca son síncronos.

## 4. Cápsula `mayeuta-llm` (Skill — C1)

| Aspecto | Definición |
|---|---|
| Tipo / ubicación | **Skill** Rust en `SddIA/skills/mayeuta-llm/` (C1; patrón skills existentes) |
| E/S | stdin JSON `{operation, prompt, schema?}` → stdout JSON `{success, data}` |
| Operaciones | `SYNTHESIZE` (texto libre) · `CLASSIFY_INTENT` (intención estructurada) |
| Motor de inferencia | **CLI de Cursor local** vía `std::process::Command` (C3); sin cliente HTTP, sin SDK |
| Configuración | `SDDIA_LLM_CLI_COMMAND` (comando físico del CLI) desde `.dev/.env` / `.SddIA/.dev/.env` |
| Agnosticismo | Comando inyectado por bóveda; cero proveedor hardcode; sin dependencias de red en el Genoma |
| Fallback | Si falta `SDDIA_LLM_CLI_COMMAND` o el CLI falla / stdout no parseable → `success:false` con causa; el handler degrada |
| Secretos | Prohibido loguear el comando resuelto si contiene credenciales |

## 4. Contrato CLASSIFY_INTENT (LLM → JSON estricto)

```json
{
  "intent": "chat | execute",
  "process_name": "task-queue-manager | bug-fix | feature | refactorization | null",
  "process_inputs": { "pbi_ref": "docs/todos/pending/[FIX] ...md" },
  "confidence": 0.0
}
```

Reglas de decisión:

- `confidence < UMBRAL` (def. 0.7) → tratar como `chat` (seguro por defecto).
- `process_name ∉ ALLOWLIST` → rechazo explícito, sin ejecutar.
- `intent=execute` + proceso de ciclo de vida → **emitir evento** (`manual-task-requested`) — nunca síncrono (C2).

## 5. Allowlist (O11)

```text
ALLOWLIST_KALMA2 = { bug-fix, feature, refactorization, task-queue-manager }
```

La allowlist gobierna **qué procesos puede solicitar** Kalma2; el despacho de todos ellos es **asíncrono por evento** (C2). El triaje/ejecución real lo gobiernan los demonios y agentes del Core en segundo plano. Procesos no listados → rechazo explícito.

## 6. Integración con el Core (despacho asíncrono — C2)

El handler **no** invoca `execute-process` de forma síncrona para procesos de ciclo de vida. En su lugar emite un **evento de dominio dedicado** `Kalma2_Process_Requested` al bus EDA (patrón `telegram-gateway` → `fractal::write_fractal_event`):

```json
{
  "event_type": "Kalma2_Process_Requested",
  "event_family": "domain",
  "emitter_agent": "kalma2-interact",
  "payload": { "process": "bug-fix", "pbi_ref": "docs/todos/pending/[FIX] ...md", "raw_text": "..." }
}
```

> **Decisión — evento dedicado (no reutilizar `Manual_Task_Requested`):** `Manual_Task_Requested` tiene semántica sensorial Telegram (REQUIRED `task_text`/`source`/`raw_text`; emisor autorizado solo `telegram-gateway`) y su único suscriptor ancla en DLT, sin ejecutar proceso. Reutilizarlo contaminaría esa semántica y rompería su ECST. Por eso `kalma2-interact` emite un tipo propio, `Kalma2_Process_Requested`, con su esquema y suscriptor ejecutor.

El centinela `event-watcher` consume el evento y delega en `action:execute-process` en segundo plano. `SddIA/actions/execute-process.md` sigue siendo el ejecutor canónico, pero **invocado por el Sistema Nervioso**, no por la petición HTTP.

## 6.bis Cierre del lazo EDA (3 puntos — análisis verificado)

La infraestructura de transporte ya existe (`event-watcher` → `route-domain-event` genérico), pero **hoy no hay despacho a proceso de ciclo de vida**: `Manual_Task_Requested` solo tiene suscriptor `iota-immutable-publisher`. Esta feature cierra el lazo con tres entregables:

| # | Brecha verificada | Implementación |
|---|-------------------|----------------|
| **P1** | No existe suscriptor que ejecute la tarea | Definir evento `SddIA/events/domain/kalma2-process-requested.md` (ECST propio) + registrar suscriptor `process` en `event-domain-subscriptions.json` para `Kalma2_Process_Requested` |
| **P2** | `dispatch_subscriber` solo mapea `telegram-fallback-responder` (lee `text`) o payload PR (`branch`); un `process: bug-fix` con `pbi_ref` falla con *"branch missing in payload"* (`route_domain_event_core.py:154-156`) | Añadir rama en `dispatch_subscriber`: si `event_type == Kalma2_Process_Requested`, construir `process_inputs` desde `{process, pbi_ref, raw_text}` (no desde `branch`) |
| **P3** | Esquema ECST de `Manual_Task_Requested` rechazaría el payload `{process, pbi_ref}` y el emisor `kalma2-interact` | Esquema propio del evento dedicado: REQUIRED `process`, `raw_text`; OPTIONAL `pbi_ref`; emisor autorizado `kalma2-interact`. Registrar en `eda-coverage` / clases ECST |

### P1 — Evento `Kalma2_Process_Requested`

```yaml
# SddIA/events/domain/kalma2-process-requested.md (frontmatter)
uuid: "458c34a8-9ad5-4a40-88c4-0be1e5d9598e"
name: "kalma2-process-requested"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Kalma2_Process_Requested"
context: "ecosystem-evolution"
```

Payload ECST: REQUIRED `process`, `raw_text`; OPTIONAL `pbi_ref`, `process_inputs`. Emisor autorizado: `kalma2-interact`.

Suscripción a añadir:

```json
"Kalma2_Process_Requested": [
  {
    "agent": "tekton",
    "process": "{payload.process}",
    "intent": "Despacho reactivo del proceso solicitado desde Kalma2 (bug-fix | feature | refactorization | task-queue-manager)."
  },
  {
    "agent": "cumulo",
    "tool": "iota-immutable-publisher",
    "intent": "Anclaje DLT IOTA de la solicitud de proceso desde Kalma2."
  }
]
```

> Si el registro de suscripciones no admite `process` dinámico (`{payload.process}`), alternativa: suscriptor fijo `process: task-queue-manager` que clasifica internamente y delega en `feature|bug-fix|refactorization` (coherente con `task-queue-manager.md`). **Recomendado** por simplicidad y por respetar el triaje soberano del Core.

### P2 — Rama en `dispatch_subscriber`

```python
# route_domain_event_core.py — dentro de dispatch_subscriber, rama process
if event.get("event_type") == "Kalma2_Process_Requested":
    proc = (payload.get("process") or "").strip()
    if proc not in ALLOWLIST_KALMA2:
        return sid, "failed", f"proceso no permitido: {proc}", 1
    process_inputs = {"correlation_id": event.get("event_id") or ""}
    if isinstance(payload.get("pbi_ref"), str) and payload["pbi_ref"].strip():
        process_inputs["pbi_ref"] = payload["pbi_ref"].strip()
    if isinstance(payload.get("raw_text"), str):
        process_inputs["task_text"] = payload["raw_text"].strip()
    # invoca execute-process --process <proc> --inputs <process_inputs>
    ...
```

Reutiliza el patrón de subprocess + parseo de envelope ya presente en las otras ramas `process`.

### P3 — Esquema ECST y cobertura

- Registrar la clase `Kalma2_Process_Requested` donde `ecst_validation.load_event_class_schemas` la resuelva (misma vía que el resto de eventos de dominio).
- Añadir entrada en `SddIA/core/eda-coverage.json` (cobertura del nuevo tipo).
- Validar que el payload emitido por `kalma2-interact` cumple REQUIRED `process`+`raw_text`.

## 7. Seguridad / Cerbero / Sandbox

- La cápsula `mayeuta-llm` introduce **contexto de ejecución de subproceso local** (CLI Cursor, C3) → declararlo en su `{name}.md` y pasar `execution-contexts` / Cerbero (O13). No introduce contexto de red en el Genoma.
- `filter_c_should_abort` (existente en `mayeuta.rs`) se aplica **antes** de invocar el CLI (no enviar `/`, `!`, vacío).
- `SDDIA_LLM_CLI_COMMAND` se resuelve desde bóveda; nunca persistir ni loguear el comando si porta credenciales.

## 8. Paridad (O12)

`synthesize_mayeuta_response` permanece como **fallback determinista compartido**. No se muta su firma ni salida; `telegram-fallback-responder` la sigue usando sin cambios. La ruta LLM se añade en el handler `kalma2`, no en la función compartida.

## 9. Criterios de aceptación (pre-impl)

1. `mayeuta-llm` (Skill, C1) compila y responde `SYNTHESIZE`/`CLASSIFY_INTENT` con contrato JSON.
2. Sin `SDDIA_LLM_CLI_COMMAND` → `kalma2-interact` degrada a determinista (O8) y sigue verde.
3. Prompt conversacional → respuesta del CLI de Cursor (C3).
4. Prompt "inicia fix …PBI" → emite evento `Kalma2_Process_Requested` con `process: bug-fix` + `pbi_ref`; acuse inmediato; **sin** ejecución síncrona (C2).
5. Proceso fuera de allowlist → rechazo explícito.
6. `telegram-fallback-responder` sin cambios de salida (O12).
7. **P1** — `event-watcher` enruta `Kalma2_Process_Requested` a un suscriptor `process` (no solo DLT).
8. **P2** — `dispatch_subscriber` construye `process_inputs` desde `{process, pbi_ref}` sin exigir `branch`.
9. **P3** — Validación ECST del nuevo evento APTO con emisor `kalma2-interact`.

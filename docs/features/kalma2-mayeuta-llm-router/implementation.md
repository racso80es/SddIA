---
feature_name: kalma2-mayeuta-llm-router
created: "2026-06-19"
process: feature
branch_name: feat/kalma2-mayeuta-llm-router
uuid: def280fd-73a3-42fe-b485-3258f1e5e426
status: pre-implementation
note: "Diseño pre-implementación. NO materializado. Forja física diferida a fase tekton."
---

# Implementación (diseño pre-implementación) — kalma2-mayeuta-llm-router

> **Estado:** documentación pre-implementación. El código siguiente es blueprint de forja, **no** materializado.

## Touchpoints planificados

| # | Artefacto | Acción | Tipo |
|---|-----------|--------|------|
| 1 | `SddIA/skills/mayeuta-llm/Cargo.toml` | crear (Skill, C1) | genoma |
| 2 | `SddIA/skills/mayeuta-llm/src/main.rs` | crear (transductor CLI, SYNTHESIZE/CLASSIFY_INTENT) | genoma |
| 3 | `SddIA/skills/mayeuta-llm.md` | crear (definición skill + contexto subproceso local) | genoma |
| 4 | `SddIA/engine/.../handlers/kalma2.rs` | integrar Skill + enrutamiento asíncrono + fallback | genoma |
| 5 | `SddIA/process/kalma2-interact.md` | actualizar fases (Síntesis LLM + Enrutamiento EDA) | genoma |
| 6 | `SddIA/events/domain/kalma2-process-requested.md` | crear evento dedicado (P1, C2) | genoma |
| 7 | `SddIA/core/event-domain-subscriptions.json` | suscriptor `process` para `Kalma2_Process_Requested` (P1) | genoma |
| 8 | `SddIA/scripts/qa/route_domain_event_core.py` | rama `dispatch_subscriber` para el evento (P2) | genoma |
| 9 | `SddIA/core/eda-coverage.json` + clases ECST | cobertura/esquema del nuevo evento (P3) | genoma |
| 10 | `.dev/.env.example` | documentar `SDDIA_LLM_CLI_COMMAND` | instancia |
| 11 | `interfaces/kalma2/app.js` | (opcional) mostrar acuse de tarea encolada | instancia |

## Estado verificado del código (pre-impl)

- `synthesize_mayeuta_response` es compartida por `kalma2` y `telegram-fallback` (`handlers/mayeuta.rs`). NO mutar su firma/salida → la ruta LLM se añade en `handlers/kalma2.rs`.
- `telegram-gateway` ya transmuta texto→evento y lo escribe al bus (`fractal::write_fractal_event`) → patrón para el enrutamiento asíncrono (C2).
- `invoke_orchestrator::invoke_process` existe pero **no se usa** para procesos de ciclo de vida (C2 prohíbe síncrono); el despacho real lo hace `event-watcher` en segundo plano.
- No existe infraestructura LLM en `engine/` → la Skill la introduce como **transductor CLI local** (C3), sin cliente HTTP ni SDK.

## Esquema de la Skill `mayeuta-llm` (C1)

```text
stdin  : {"operation":"SYNTHESIZE"|"CLASSIFY_INTENT","prompt":"...","schema":{...}?}
stdout : {"success":bool,"data":{...},"error":null|string}

SYNTHESIZE      -> data: {"text": "<respuesta libre>"}
CLASSIFY_INTENT -> data: {"intent","process_name","process_inputs","confidence"}
```

## Transductor CLI Cursor (C3) — blueprint cápsula

```rust
// Lee comando físico desde bóveda; ensambla prompt; ejecuta; captura stdout.
fn run_cli(prompt_assembled: &str) -> Result<String, String> {
    let raw = std::env::var("SDDIA_LLM_CLI_COMMAND")
        .map_err(|_| "SDDIA_LLM_CLI_COMMAND ausente".to_string())?;
    let parts = shell_words::split(&raw).map_err(|e| e.to_string())?;
    let (bin, args) = parts.split_first().ok_or("comando vacío")?;

    let mut child = std::process::Command::new(bin)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn CLI: {e}"))?;

    child.stdin.take().unwrap()
        .write_all(prompt_assembled.as_bytes())
        .map_err(|e| e.to_string())?;

    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}
```

> Sin dependencias de red en el Genoma (C3). El prompt se pasa por stdin; el CLI de Cursor responde por stdout. `CLASSIFY_INTENT` añade un prompt de sistema que exige JSON estricto y parsea la última línea.

## Blueprint handler `kalma2.rs` (referencia)

```rust
pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let prompt = /* extraer + trim como hoy */;

    if filter_c_should_abort(prompt) {
        return Ok(envelope_response(synthesize_mayeuta_response(prompt))); // pre-gate
    }

    // 1. Clasificar intención (cápsula; fallback a chat si falla)
    let intent = classify_intent(repo, prompt).unwrap_or(Intent::chat());

    match intent {
        // C2: procesos de ciclo de vida SIEMPRE asíncronos vía evento
        Intent::Execute { process_name, inputs, confidence }
            if confidence >= THRESHOLD && allowlisted(&process_name) =>
        {
            let ev = manual_task_event(&process_name, &inputs); // event_type: manual-task-requested
            write_fractal_event(repo, &ev)?;
            return Ok(envelope_response(ack_enqueued(&process_name))); // acuse inmediato
        }
        Intent::Execute { process_name, .. } if !allowlisted(&process_name) =>
            return Ok(envelope_response(rejected(&process_name))),
        _ => {} // chat
    }

    // 2. Chat: SYNTHESIZE vía Skill (CLI Cursor) con degradación determinista
    let text = synthesize_via_skill(repo, prompt)
        .unwrap_or_else(|_| synthesize_mayeuta_response(prompt)); // O8 fallback
    Ok(envelope_response(text))
}
```

> El `run` pasa a recibir `repo: &Path` (hoy solo `process_inputs`); ajustar la llamada en `engine::mod::run_process` (`handlers::kalma2::run(repo, process_inputs)`). El handler invoca la Skill `mayeuta-llm` vía `capsules::invoke_skill` (patrón cápsulas existente).

## Evento de dominio dedicado (C2 / P1)

```json
{
  "event_id": "<uuid>",
  "event_type": "Kalma2_Process_Requested",
  "event_family": "domain",
  "emitter_agent": "kalma2-interact",
  "timestamp": "<iso>",
  "payload": {
    "process": "bug-fix",
    "pbi_ref": "docs/todos/pending/[FIX] event-sweeper — fractura sistémica (8b1ed140e48d).md",
    "raw_text": "<prompt original>"
  },
  "delivery_state": {}
}
```

Escrito en `.events/pending/` (vía `fractal::write_fractal_event`); `event-watcher` lo enruta a `route-domain-event` → suscriptor `process` → `action:execute-process` en segundo plano.

## Cierre del lazo EDA — P1/P2/P3 (O14)

> Análisis verificado: hoy `Manual_Task_Requested` solo ancla en DLT; `dispatch_subscriber` exige `branch`; el ECST no admite `{process, pbi_ref}` con emisor `kalma2-interact`. Por eso se introduce un evento dedicado.

### P1 — Evento + suscripción

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

```json
// SddIA/core/event-domain-subscriptions.json (añadir)
"Kalma2_Process_Requested": [
  {
    "agent": "tekton",
    "process": "task-queue-manager",
    "intent": "Triaje soberano + despacho del proceso solicitado desde Kalma2."
  },
  {
    "agent": "cumulo",
    "tool": "iota-immutable-publisher",
    "intent": "Anclaje DLT IOTA de la solicitud de proceso desde Kalma2."
  }
]
```

> Recomendado suscriptor fijo `task-queue-manager` (no `{payload.process}` dinámico): respeta el triaje soberano del Core y evita inyección de nombre de proceso desde la UI.

### P2 — Rama en `dispatch_subscriber` (`route_domain_event_core.py`)

```python
# Antes de la rama PR (branch), interceptar el evento de Kalma2:
if event.get("event_type") == "Kalma2_Process_Requested":
    proc = (payload.get("process") or "").strip()
    if proc not in ALLOWLIST_KALMA2:
        return sid, "failed", f"proceso no permitido: {proc}", 1
    process_inputs = {"correlation_id": event.get("event_id") or ""}
    if isinstance(payload.get("pbi_ref"), str) and payload["pbi_ref"].strip():
        process_inputs["pbi_ref"] = payload["pbi_ref"].strip()
    if isinstance(payload.get("raw_text"), str) and payload["raw_text"].strip():
        process_inputs["task_text"] = payload["raw_text"].strip()
    proc_resolved = subscriber.get("process") or proc   # task-queue-manager si suscriptor fijo
    proc = _run_subprocess(resolve_orchestrator_cmd(
        repo, ["--process", proc_resolved, "--inputs",
               json.dumps(process_inputs, ensure_ascii=False)]),
        cwd=str(repo), shell=False)
    # parseo de envelope idéntico a las otras ramas process → (sid, status, err, code)
```

Esto evita el fallo *"branch missing in payload"* (`route_domain_event_core.py:154-156`) para este tipo.

### P3 — ECST + cobertura

- Registrar la clase `Kalma2_Process_Requested` donde `ecst_validation.load_event_class_schemas` la resuelve (misma vía que el resto de eventos de dominio): REQUIRED `process`, `raw_text`.
- Añadir entrada de cobertura en `SddIA/core/eda-coverage.json` para el nuevo UUID/tipo.
- Verificar `validate_ecst_instance` APTO para el payload emitido por `kalma2-interact`.

## Configuración (instancia)

```dotenv
# .dev/.env  (no versionar valores reales)
# Comando físico del CLI de Cursor; recibe el prompt por stdin y responde por stdout.
SDDIA_LLM_CLI_COMMAND=cursor-agent --print
```

## Verificación planificada

| Check | Comando |
|-------|---------|
| Build skill | `cd SddIA && cargo build -p mayeuta-llm` |
| Build engine | `cargo build -p execute-process` |
| Fallback sin comando | `unset SDDIA_LLM_CLI_COMMAND; execute-process --process kalma2-interact --inputs '{"prompt":"hola"}'` → respuesta determinista |
| Chat CLI | con `SDDIA_LLM_CLI_COMMAND` → respuesta del CLI Cursor |
| Enrutamiento fix | `--inputs '{"prompt":"inicia fix docs/todos/.../[FIX] event-sweeper...md"}'` → evento `Kalma2_Process_Requested` en `.events/pending/` + acuse |
| Lazo EDA (P1-P2) | `SDDIA_LAB_ROUTE_SYNC=1` + evento en bus → `route-domain-event` despacha suscriptor `process` (no solo DLT) |
| ECST (P3) | `validate_ecst_instance` APTO para `{process, raw_text}` con emisor `kalma2-interact` |
| Allowlist | proceso no permitido → rechazo |
| Paridad | `execute-process --process telegram-fallback-responder ...` sin cambios |

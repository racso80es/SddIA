---
uuid: "c4d5e6f7-a8b9-4012-c345-678901234567"
name: "enrich-fracture-pbi-kaizen"
version: "1.2.0"
contract: "actions-contract v1.2.0"
context: "knowledge-management"
capabilities:
  - "fracture-root-cause-analysis"
  - "kaizen-evolution-proposal"
  - "fracture-pbi-resolver"
  - "delegate-filesystem-manager"
inputs:
  - "process_name": "string; proceso o cápsula que colapsó"
  - "error_trace": "string; traza o mensaje de error"
  - "agent_emitter": "string; agente o runtime que emitió la fractura"
  - "attempted_action": "string; acción que falló"
  - "persist_ref": "string; carpeta de persistencia si aplica (opcional)"
  - "branch_name": "string; rama en contexto si aplica (opcional)"
  - "cumulo_pbi_path": "string; ruta PBI si conocida (opcional; resolución por genoma si omitido)"
outputs:
  - "success": "boolean"
  - "target_path": "string|null; ruta del PBI enriquecido"
  - "message": "string; resultado del análisis Kaizen o no_target"
  - "reason": "string; enriched | no_target"
  - "evolution_verdict": "string|null; new_norm | refactor_tool | prompt_adjustment | process_fix"
hash_signature: "sha256:eabe4edeede0f451df88d2c927503f2f2c0f45d562b666219306748f2773a368"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: enrich-fracture-pbi-kaizen

## 1. Propósito

Acción canónica del Agente **Mayeuta** ante `System_Fracture_Detected`. **No** crea el PBI (eso es Cúmulo); localiza el PBI abierto vía resolutor Core (`fracture_hash` / `fracture_process` en genoma YAML) y añade la sección **Conclusión Analítica y Propuesta Evolutiva**.

## 2. Orquestación

### Paso 1 — Resolución de target (Core `fracture_pbi`)

Cascada sin reconstruir ruta por nombre de fichero:

1. `cumulo_pbi_path` si el fichero existe.
2. PBI abierto en `pending/` con mismo `fracture_hash`.
3. PBI abierto en `pending/` con mismo `fracture_process` (deduplicación por proceso).
4. Si Cúmulo acaba de abrir regresión, enriquece esa ruta.
5. **No** escribe sobre `done/`.

Si no hay target: `success: true`, `reason: no_target`, sin dead-letter.

### Paso 2 — Análisis

Consumir `process_name`, `error_trace`, `attempted_action`, `agent_emitter` y contexto opcional (`persist_ref`, `branch_name`).

Cubo `heartbeat_starvation` (F-MAYEUTA-HB-BLIND): match **exclusivo** sobre `error_trace` con anclas literales de Argos `emit_system_fracture` (`Centinela `, `omitió`, `ciclos consecutivos de Daemon_Heartbeat`, `umbral=`, `last_heartbeat=`). Veredicto `refactor_tool`: inanición de latido con proceso vivo; prohibido «Auditar proceso {daemon_id}». Evaluar antes del catch-all `timeout|block|abort|failed|colaps`.

**F-MAYEUTA-HB-TOKEN-TRAP:** prohibido clasificar latido con tokens `heartbeat`, `daemon`, `audit` o `colaps` sobre el blob concatenado (`error_trace` + `attempted_action` + `process_name`). `attempted_action` es siempre `daemon-heartbeat-audit` en esta familia.

Cubos hook / bypass / huérfano EDA intactos (hook no concatena `process_name`).

### Paso 3 — Enriquecimiento

Reemplazar o completar `## Conclusión Analítica y Propuesta Evolutiva` con diagnóstico, veredicto (`new_norm` | `refactor_tool` | `prompt_adjustment` | `process_fix`) y propuesta accionable.

### Paso 4 — Cierre (stdout)

Envelope con `success`, `target_path`, `reason` (`enriched` | `no_target`), `evolution_verdict`, `message`.

## 3. Límites

* No repara la fractura ni autoriza bypass.
* No reconstruye ruta del PBI por hash de nombre.
* No diseña código ejecutable ni fases `delegates_to`.
* No mueve archivos del bus.
* No usa `heartbeat`/`daemon`/`audit` como tokens del blob general para el cubo de latido.

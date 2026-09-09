---
uuid: "c4d5e6f7-a8b9-4012-c345-678901234567"
name: "enrich-fracture-pbi-kaizen"
version: "1.4.0"
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
hash_signature: "sha256:ef22da114e0d5d1ef82d10eeb8950bd6014dc3e67fc3ce45f61bccdc7d176ec6"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: enrich-fracture-pbi-kaizen

## 1. Propósito

Acción canónica del Agente **Mayeuta** ante `System_Fracture_Detected`. **No** crea el PBI (eso es Cúmulo); localiza el PBI abierto vía resolutor Core (`fracture_hash` / `fracture_process` en genoma YAML) y añade la sección **Conclusión Analítica y Propuesta Evolutiva**.

**Laudo `L-ENRICH-KINTSUGI-DETERMINISTA`:** handler nativo + matcher léxico. Misma traza → misma sección. Prohibido `llm:interact` / `skill:mayeuta-llm` en este enrich. Canal LLM asíncrono = `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION`.

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

Cubo `heartbeat_starvation` (F-MAYEUTA-HB-BLIND): match **exclusivo** sobre `error_trace` con anclas literales de Argos `emit_system_fracture` (`Centinela `, `omitió`, `ciclos consecutivos de Daemon_Heartbeat`, `umbral=`, `last_heartbeat=`). Veredicto `refactor_tool`. Evaluar antes del catch-all.

**F-MAYEUTA-HB-TOKEN-TRAP:** prohibido clasificar latido con tokens `heartbeat`, `daemon`, `audit` o `colaps` sobre el blob concatenado.

Cubo `orphan_lock` (F-MAYEUTA-ORPHAN-TOKEN-TRAP): match **exclusivo** sobre `error_trace` (`Centinela `, `lock huérfano`, `PID `, `muerto`, `last_heartbeat=`). Veredicto `refactor_tool`.

Cubo EDA genómica: exigir contexto genómico **y** token huérfano/orphan; **no** disparar si `is_orphan_lock_trace`.

Cubo DLT (`iota-relay-publish-error` / `F-DLT-PUBLISH-ERROR`) **subtipado** (F-MAYEUTA-DLT-GENERIC):
- firma `is not available for consumption` ∧ `current version:` → colisión de gas/inputs; `process_fix`; prohibido afirmar transporte.
- tokens de red (`ENETUNREACH`/`ETIMEDOUT`/`ENOTFOUND`) → transporte; `process_fix`.
- resto → `process_fix` opaco; no afirmar transporte.

**F-MAYEUTA-CATCHALL-FAILED:** el catch-all **no** incluye el token `failed`. Tokens restantes: `timeout` | `block` | `abort` | `colaps`. `{acción} failed:` sin cubo de dominio → fallback `process_fix` + «requiere laudo humano», nunca `prompt_adjustment` por verbo de fallo.

Cubos hook / bypass / PAT / snapshot / shell intactos (hook no concatena `process_name`).

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
* No usa `orphan`/`huérfan` sobre el blob concatenado para clasificar lock de centinela.
* No invoca `llm:interact` / `mayeuta-llm` (laudo `L-ENRICH-KINTSUGI-DETERMINISTA`).
* No usa `failed` en el catch-all de operador.

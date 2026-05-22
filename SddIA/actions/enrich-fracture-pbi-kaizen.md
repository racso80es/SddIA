---
uuid: "c4d5e6f7-a8b9-4012-c345-678901234567"
name: "enrich-fracture-pbi-kaizen"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "knowledge-management"
capabilities:
  - "fracture-root-cause-analysis"
  - "kaizen-evolution-proposal"
  - "delegate-filesystem-manager"
inputs:
  - "process_name": "string; proceso o cápsula que colapsó"
  - "error_trace": "string; traza o mensaje de error"
  - "agent_emitter": "string; agente o runtime que emitió la fractura"
  - "attempted_action": "string; acción que falló"
  - "persist_ref": "string; carpeta de persistencia si aplica (opcional)"
  - "branch_name": "string; rama en contexto si aplica (opcional)"
  - "cumulo_pbi_path": "string; ruta PBI materializado por Cúmulo (opcional; se resuelve por hash si omitido)"
outputs:
  - "success": "boolean"
  - "target_path": "string; ruta del PBI enriquecido"
  - "message": "string; resultado del análisis Kaizen"
  - "evolution_verdict": "string; new_norm | refactor_tool | prompt_adjustment | process_fix"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: enrich-fracture-pbi-kaizen

## 1. Propósito

Acción canónica del Agente **Mayeuta** (Auditor de Fricción Kaizen) ante `System_Fracture_Detected`. **No** crea el PBI (eso es Cúmulo); analiza el `error_trace` y el contexto para diagnosticar la causa raíz (el *Por Qué*) y añade al PBI de Cúmulo la sección **Conclusión Analítica y Propuesta Evolutiva**.

Mayeuta dictamina si el fallo requiere nueva norma, refactor de herramienta, ajuste de prompt de sistema o corrección de proceso — transformando el error en vitalidad arquitectónica.

## 2. Orquestación

### Paso 1 — Precondición

El PBI debe existir en `docs/todos/pending/` (materializado por `materialize-fracture-pbi` en el mismo fan-out, **antes** que esta acción).

### Paso 2 — Análisis

Consumir `process_name`, `error_trace`, `attempted_action`, `agent_emitter` y contexto opcional (`persist_ref`, `branch_name`).

### Paso 3 — Enriquecimiento

Reemplazar o completar la sección `## Conclusión Analítica y Propuesta Evolutiva` con:

- Diagnóstico de causa raíz
- Veredicto evolutivo (`new_norm` | `refactor_tool` | `prompt_adjustment` | `process_fix`)
- Propuesta concreta accionable

### Paso 4 — Cierre (stdout)

Envelope con `success`, `target_path`, `evolution_verdict`, `message`.

## 3. Límites

* No repara la fractura ni autoriza bypass.
* No diseña código ejecutable ni fases `delegates_to`.
* No mueve archivos del bus.

---
uuid: "d5c792b5-33a1-4287-a66f-ea3b2ad5716d"
name: "append-mayeuta-hypothesis"
version: "1.0.0"
contract: "actions-contract v1.3.0"
context: "knowledge-management"
capabilities:
  - "fracture-semantic-hypothesis"
  - "delegate-mayeuta-llm"
inputs:
  - "fracture_pbi_path": "string; ruta relativa al PBI en pending"
  - "process_name": "string; proceso que originó la fractura"
  - "error_trace_hash": "string; 12 hex de fracture_trace_hash"
  - "attempted_action": "string; opcional"
  - "agent_emitter": "string; opcional"
outputs:
  - "success": "boolean; true en fallos operativos"
  - "synthesized": "boolean"
  - "reason": "injected | replaced | target_absent_or_closed | llm_unavailable | llm_timeout | llm_invalid_stdout"
  - "pbi_path": "string|null"
  - "hypothesis_chars": "integer|null"
hash_signature: "sha256:cbf23fc266bbe11c32fee49c91fd5b4b410b66a92b2f46f12dc3ccd15a16ff39"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: append-mayeuta-hypothesis

## 1. Propósito

Suscriptor de `Fracture_Clarification_Requested`. Inyecta hipótesis consultiva (`skill:mayeuta-llm` `SYNTHESIZE`) en el PBI de fractura inédita. No altera el sello determinista de Cúmulo/enrich.

Handler nativo en `execute-process` (`append_mayeuta_hypothesis`). Fail-open: fallos de CLI no emiten `System_Fracture_Detected` ni `success: false`.

## 2. Orquestación

1. Resolver `fracture_pbi_path` bajo `paths.todos.pending`. Si ausente o fuera de pending: `synthesized: false`, `reason: target_absent_or_closed`.
2. Extraer traza de `## Traza de error`.
3. Componer prompt Filtro C (MAX 15 LINES; no inventar rutas ausentes en CONTEXTO).
4. Invocar `skill:mayeuta-llm` con `operation: SYNTHESIZE` y `prompt` (sin `temperature`).
5. Recortar salida ≤15 líneas. Upsert H2 `## Hipótesis Semántica (Mayeuta) — Inferencia Asíncrona` entre Conclusión y `## Criterio de cierre`.

## 3. Límites

* No muta YAML (`document_id`, `fracture_hash`, `fracture_process`, `status`, `incident_ref`) ni `## Traza de error` ni Conclusión.
* No cambia el veredicto `process_fix` / «requiere laudo humano».
* No repara, no `git-manager`, no procesos de entrega.
* `success: true` ante CLI ausente, timeout o stdout inválido.

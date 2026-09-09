---
feature_name: async-fracture-clarification-mayeuta
created: "2026-09-09"
process: feature
base: main
scope: enrich_fracture_pbi_kaizen, append_mayeuta_hypothesis, event Fracture_Clarification_Requested, event-orchestration-subscriptions, mayeuta.md
---

# Spec — async-fracture-clarification-mayeuta

## Disparador (Tiempo 1)

`analyze_fracture_kaizen` retorna `unclassified: bool` = `root_causes.is_empty()` **antes** del fallback genérico. `run()`:

1. Upsert Conclusión (sin cambio de algoritmo).
2. Si `unclassified && target`: `write_fractal_event` hacia `load_fractal_dirs(repo).1` con envelope ECST §4 del PBI. Fail-soft I/O.
3. Cero `invoke_capsule_json`.

## Evento

Clase `SddIA/events/orchestration/fracture-clarification-requested.md`. Payload REQUIRED: `fracture_pbi_path`, `process_name`, `error_trace_hash`. Viñetas `- \`campo\`` para `ecst_validation`.

Suscriptor: `mayeuta` / `append-mayeuta-hypothesis` en `event-orchestration-subscriptions.json`.

## Acción (Tiempo 2)

Handler `append_mayeuta_hypothesis::run(repo, inputs)`:

| Condición | Resultado |
|----------|-----------|
| Path ausente o no en `paths.todos.pending` | `synthesized: false`, `reason: target_absent_or_closed` |
| CLI ausente/fallo | `synthesized: false`, `reason: llm_*`, PBI intacto |
| OK | upsert H2, `reason: injected\|replaced` |

## Inmutables del PBI Cúmulo

YAML: `document_id`, `fracture_hash`, `fracture_process`, `status`, `incident_ref`. Cuerpo: `## Traza de error`, `## Conclusión Analítica y Propuesta Evolutiva`.

## Tests

Filtro `--lib -- enrich_fracture_pbi_kaizen` / `append_mayeuta_hypothesis`. Fixture inédita vs cubo. Doble upsert. CLI mock fallido.

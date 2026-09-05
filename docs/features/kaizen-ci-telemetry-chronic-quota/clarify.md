---
feature_name: kaizen-ci-telemetry-chronic-quota
created: "2026-09-05"
process: feature
purpose: Estabilización Mayeuta — PBI-KAIZEN-CI-TELEMETRY-CHRONIC-QUOTA v1.2.0
version_clarify: "1.0.0"
execution_id: "18aec32c-f457-4330-819c-2366b959cf57"
pbi_ref: docs/todos/pending/[KAIZEN] Telemetría de CI — cuota crónica y degradación mapeada (CA8-CA9).md
document_id: PBI-KAIZEN-CI-TELEMETRY-CHRONIC-QUOTA
uuid: "166c91f9-7378-4766-b6fe-ff5e7eee382f"
---

# Clarificación — kaizen-ci-telemetry-chronic-quota

Transcript Mayeuta. Semilla: PBI v1.2.0 (Filtro A aplicado). Init `18aec32c-f457-4330-819c-2366b959cf57`. Relé IDE. Rama anclada en `main` local (2 commits sobre `origin/main`).

## D0 — Apertura

| Pregunta | Decisión |
|----------|----------|
| Vehículo | `--process feature`. Relé `SDDIA_AGENT_RELAY_IDE=1`. Skip archive/DCC en init. |
| Rama | `feat/kaizen-ci-telemetry-chronic-quota` |
| `persist_ref` | `docs/features/kaizen-ci-telemetry-chronic-quota` |
| MVP | B1 + B2 + CA9-NEG. CA9 positivo gated. |
| `radamanto.md` | **No EM.** `run_agent_forge` en update regenera UUID. Residual igual que el padre. |

## D1 — Clave de runtime (H8 / L-THRESH-PATH)

`load_radamanto_config`: `cfg["ci_failures"]` = ruta ledger. `cfg["thresholds"]` = objeto. Cuota = `thresholds.ci_failures.per_job_limit` + `job_entity_map`. Default in-code del objeto umbrales incluye el bloque (tests sin copiar el JSON).

## D2 — Orden emit vs sello (H7)

Append en memoria → emitir si cuota cruzada y sin sello → persistir (failures ± alerts) una vez. Si `write_fractal_event` falla: persistir el append **sin** sello y devolver error (el retried del mismo `check_run_id` reintenta emisión porque el sello no existe). Sello solo post-OK.

## D3 — Forja

Clase domain `ci-chronic-failure-detected` (`quality-assurance`, emisor `radamanto`). Acción `materialize-ci-chronic-failure-pbi` (`ecosystem-evolution`). Ambas vía `entity-manager`. `CONSUMER_SKIP_FORGE_ACTIONS`. Suscripción solo Cúmulo.

## D4 — CA9-NEG

`job_entity_map: {}`. Lookup existe; rama Degraded solo con par válido **y** entidad en genoma. Default `resolve_entity_type` → `tool` prohibido como SSOT.

## D5 — Fuera

Centinela. DIA. Kintsugi. Peaje `stats.json`. Pares reales. L-RESET. `radamanto.md`.

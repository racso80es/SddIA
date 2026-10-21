---
feature_name: eda-redb-02-writers
created: "2026-10-21"
base: main
scope: escritores del bus
---

# Spec — eda-redb-02-writers

La compuerta vive en `sddia-event-store::gate`. Lee `SDDIA_EVENT_STORE` y no consulta Cúmulo.

| Modo | Escritura | Lectura de despacho |
| :--- | :--- | :--- |
| `filesystem` | Solo fichero | Fichero |
| `dual` | Store y luego fichero | Store si hay fila; si no, fichero |
| `redb` | Solo store | Solo store |

Tablas usadas: `events`, `subscriber_delivery`, `progress_traces`, `dead_letter_index`, `dlt_audit_seals`.

`family` sale del nombre lógico (`telemetry`, `orchestration`, `domain`). `pending` de dominio se guarda como `domain`.

Puntos envueltos:

- `eda_bus::write_fractal_event`, `fractal::write_fractal_event`
- `write_pending_domain_event_file`
- `write_processing_witness`, `promote_witness`
- `stamp_fractal_delivery_state`
- `write_ptc_best_effort` (el error de store se traga)
- `persist_dlt_telemetry_proof` (copia el digest después del proof)
- `sddia-daemon-runtime::write_fractal_event`
- `github_bridge::materialize_to_bus`, `linear_bridge::materialize_pending`
- `try_sweep_event` en execute-process y en el daemon

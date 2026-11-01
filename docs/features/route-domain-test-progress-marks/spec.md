---
feature_name: route-domain-test-progress-marks
created: "2026-11-01"
process: feature
branch_name: Fix_Events_Multiple
persist_ref: docs/features/route-domain-test-progress-marks
tracker_ref: AUT-59
historia_ref: AUT-20
---

# Spec — AUT-59

## Problema

Los cinco tests `route_domain_*` de `eda_subscriber_ecst_redb.rs` entran en `route_domain_event` sin marca. `cargo test` no muestra la salida del test hasta que acaba. El enlace del binario ya lo imprime `cargo`. Lo que falta es la señal de que el cuerpo del test ya llamó al router.

## Contrato

Helper privado del fichero de test. No vive en `route_domain_core`.

Firma conceptual: fase (`enter` | `leave`), `event_type`, ruta lógica del pending.

Efectos, en este orden:

1. Una línea en stderr, con flush: `route_domain_test phase=<fase> event_type=<tipo> rel=<ruta>`.
2. La misma línea añadida a `{checkout}/.tmp/route-domain-test-progress.log`. El checkout es la raíz del repo que ya resuelve el test (`find_repo_root`), no el repo temporal del bus. El directorio `.tmp/` se crea si no existe. Append, sin truncar al empezar el test.

`enter` va en la línea anterior a `route_domain_event`. `leave` va en la línea siguiente, antes de leer `delivery_status`.

Tests cubiertos:

- `route_domain_tracker_markdown_apply_not_blocked_by_missing_branch`
- `route_domain_telegram_responder_receives_ecst_path`
- `route_domain_kalma2_task_queue_not_blocked_before_handler`
- `route_domain_telegram_empty_text_still_skipped`
- `route_domain_memory_evolution_not_blocked_by_missing_branch`

`.tmp/` sigue ignorado por git. El log no se commitea.

## Observación

Corrida de lectura: `cargo test -p execute-process --test eda_subscriber_ecst_redb route_domain_ -- --nocapture --test-threads=1` desde `SddIA/`. En paralelo, `tail -f .tmp/route-domain-test-progress.log` desde la raíz del checkout. Un cuelgue dentro del router deja `enter` sin `leave`.

## Fuera de alcance

`dispatch_subscriber`, `assemble_domain_process_subscriber_inputs`, tests `assemble_*` y `domain_process_catalog_*`, arreglar aserciones, cambiar el harness de `cargo`.

---
feature_name: kaizen-disk-thermodynamics-syslog
created: "2026-09-13"
process: feature
phases:
  - silence-hot-path
  - unit-tests
  - systemd-rate-limit
  - docs-execution
  - dcc-pr-ci-accept
branch_name: feat/kaizen-disk-thermodynamics-syslog
persist_ref: docs/features/kaizen-disk-thermodynamics-syslog
pbi_ref: docs/todos/pending/[KAIZEN] Termodinámica de disco — inundación syslog del event-watcher y techos de log de instancia.md
document_id: PBI-KAIZEN-DISK-THERMODYNAMICS-SYSLOG
pbi_uuid: "9454e7eb-f397-4e36-abaf-91fd91cba802"
pbi_version: "1.2.0"
execution_id: "e3bbb4f8-b27c-4833-9750-d855b504da8b"
---

# Plan — kaizen-disk-thermodynamics-syslog

Corte diseño: clarify + objectives + spec + plan + PBI v1.2.0. **Commit planificación** antes de mutar el crate y templates.

## L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. PBI Filtro A v1.2.0.

## L1 — Hot path

En `run_watcher` (batch y fractal): eliminar los `println!` de skip. Introducir `watcher_skip_emits_hot_path_trace` → `false`.

## L2 — Tests

`#[cfg(test)]` en `src/main.rs`. Fixtures tempdir. Verificar `watcher_skip_reason` y política de silencio.

```text
cd SddIA && cargo test -p event-watcher
```

## L3 — systemd

Fábrica + unidad instancia. Grep en `SddIA/scripts/qa/test-instance-root-resolver.sh`.

## L4 — Docs + DCC

`implementation.md` / `execution.md` / `validacion.md` (CI `PENDIENTE-CI` hasta verde). Evolution UUID. Runbook host en persist_ref. `delivery-close-cycle`. `accept-pr` solo con checks verdes.

## Fuera

Purger. `tracing`. `log_route_outcome`. Email-watcher template. journald de host.

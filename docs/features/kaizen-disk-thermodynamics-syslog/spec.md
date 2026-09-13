---
feature_name: kaizen-disk-thermodynamics-syslog
created: "2026-09-13"
process: feature
base: main
scope: core-daemon-event-watcher-hot-path-silence
branch_name: feat/kaizen-disk-thermodynamics-syslog
persist_ref: docs/features/kaizen-disk-thermodynamics-syslog
pbi_ref: docs/todos/pending/[KAIZEN] Termodinámica de disco — inundación syslog del event-watcher y techos de log de instancia.md
pbi_uuid: "9454e7eb-f397-4e36-abaf-91fd91cba802"
pbi_version: "1.2.0"
execution_id: "e3bbb4f8-b27c-4833-9750-d855b504da8b"
---

# Spec — kaizen-disk-thermodynamics-syslog

## Slice A — `event-watcher`

Archivo: `SddIA/daemons/event-watcher/src/main.rs`. Sin cambios de `Cargo.toml`.

### Política de skip (hot path)

Extraer función pura (o equivalente testable en el mismo crate):

```text
watcher_skip_emits_hot_path_trace(skip: &str) -> bool
```

Contrato: **siempre `false`** para los prefijos/cadenas que emite `watcher_skip_reason` (`in-flight`, `routed-ok`, `fractal-terminal`, `max attempts`, `dead-letter kaizen`).

En `run_watcher`, ambas ramas:

```rust
if let Some(_skip) = watcher_skip_reason(...) {
    continue;
}
```

Prohibido `println!`/`eprintln!` en ese `if`. El `continue` permanece.

### Semántica intacta

`watcher_skip_reason` no cambia. `RouteBook` (`processing`, `routed_ok`, `attempts`) no cambia. `eligible.push` sigue omitiendo skips. `MAX_ROUTE_ATTEMPTS = 3`. `POLL_SECONDS = 2`.

`log_route_outcome` y `apply_route_outcome_{pending,fractal}`: sin cambios de I/O.

### Tests (`#[cfg(test)]` en `main.rs`)

Tempdir + `BusTopology` mínimo (dirs vacíos o testigo DLQ).

| Caso | Expectativa |
|------|-------------|
| `processing` contiene UUID | `in-flight` |
| `routed_ok` + fichero existe | `routed-ok` |
| proceso ≠ `route-domain-event` + `delivery_state` todo terminal | `fractal-terminal` |
| `attempts[key] >= MAX_ROUTE_ATTEMPTS` | `max attempts` |
| proceso `route-domain-event` + testigo en `dead_letter_subscribers` | `dead-letter kaizen` |
| `watcher_skip_emits_hot_path_trace` para cada skip | `false` |
| skip `routed-ok` ⇒ no elegible (no despacho) | UUID permanece en `routed_ok` |

`cargo test -p event-watcher` debe ejecutar >0 tests y pasar.

## Slice B — systemd

`SddIA/templates/systemd/sddia-daemon@.service.template` y `.SddIA/systemd/sddia-event-watcher@.service`, sección `[Service]`:

```ini
LogRateLimitIntervalSec=30s
LogRateLimitBurst=500
```

Smoke existente `test-instance-root-resolver.sh`: añadir `grep` de ambas claves en la fábrica (no relajar `ExecStart=%f`).

## Fuera

WASI. `tracing`. Purger. `sddia-email-watcher@.service.template`. Mutar `event-watcher.md` genoma (no hace falta bump: contrato del daemon no cambia).

---
feature_name: email-watcher-heartbeat-keepalive
created: "2026-08-31"
process: bug-fix
branch: fix/email-watcher-heartbeat-keepalive
execution_id: "9dbcfea6-4df8-47ac-873a-cf9bce846929"
items_applied:
  - spawn_heartbeat_worker
  - run_loop_arc_mutex
  - once_no_keepalive
  - crate_tests
---

# Ejecución — Keepalive `email-watcher` (`6c0db1296181`)

## Comandos

```bash
cd SddIA && unset CARGO_TARGET_DIR && cargo test -p email-watcher
```

## Resultado

```text
running 21 tests
...
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Tests añadidos: `heartbeat_keepalive_matches_sibling_centinels`, `once_branch_does_not_spawn_keepalive_worker`. Contrato `once_envelope_json_io_contract` sigue verde.

## Rama

`fix/email-watcher-heartbeat-keepalive`

## PBI

`docs/todos/pending/[FIX] email-watcher — fractura sistémica (6c0db1296181).md` → archivo en `docs/todos/done/` en este ciclo.

## Relevo

Ciclo Kalma2 previo (`9dbcfea6`) falló en spawn Tekton (DNS `api2.cursor.sh`). Spec+plan Dedalo reutilizados. Ejecución IDE Tekton 2026-08-31.

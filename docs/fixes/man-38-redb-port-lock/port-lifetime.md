---
feature_name: man-38-redb-port-lock
process: bug-fix
persist_ref: docs/fixes/man-38-redb-port-lock
tracker_ref: MAN-38
---

# Ownership — EventManagerPort productivo (MAN-38)

## Quién abre Redb

Solo el host (`sddia-event-manager-host`) vía `resolve_productive_port` → factory `default` → `RedbEventAdapter` sobre `.SddIA/runtime/events.redb`.

## Lifetime productivo

1. `with_productive_port(repo, op_label, f)` abre el adapter.
2. Ejecuta **una** operación (o lote atómico acotado, p. ej. `claim_pending` en un tick).
3. Al salir del scope se dropea el `Arc` y se libera el lock del fichero.

Prohibido cachear el puerto productivo en `thread_local` más allá de la operación.

## Lifetime de test

`eda_port::install_port(mock)` deja un mock en thread-local (`PORT` sin `PORT_REPO`). `ParallelLabGuard` / `clear_port` higienizan el lab.

## Retry

`SDDIA_EVENT_MANAGER_OPEN_BUDGET_MS` (default 2000 ms) es escudo transitorio ante contención; no sustituye scoped ownership.

## Telemetría

En open con espera ≥100 ms o fallo de lock, stderr:

`[event-manager-host] open_contention process=… operation=… wait_ms=… budget_ms=… outcome=ok|locked|error err=…`

## Sesiones persistentes excepcionales (T4 — pendiente migración)

Callers que aún llaman `resolve_productive_port` y retienen `Arc` sin scope documentado:

| Locus | Justificación / siguiente paso |
|-------|-------------------------------|
| `sddia-daemon-runtime` (`github_bridge`, `eda_sweep`) | Daemon de sesión larga; migrar a scoped por tick en kaizen posterior |
| `kalma2-bridge` / `eda_productive.rs` | Bridge de sesión; idem |
| `tools/event-bus-audit`, `repo-retention-purger` | Herramientas CLI batch; abrir scoped por subcomando |
| `tools/sddia-qa/iota_ci_smoke.rs` | Smoke one-shot; candidato a `with_productive_port` |
| `lancedb_thought_repo` | Indexación; scoped por batch |

Hot path del incidente (execute-process, event-watcher, vitality probe) migrado en este fix.

---
feature_name: email-watcher-heartbeat-keepalive
created: "2026-08-30"
process: bug-fix
phases:
  - keepalive-run-loop
  - once-no-keepalive
  - crate-build-test
  - document-implementation-execution
  - argos-validacion-archive-pbi
  - delivery-close-cycle
branch_name: fix/email-watcher-heartbeat-keepalive
persist_ref: docs/fixes/email-watcher-heartbeat-keepalive
execution_id: "9dbcfea6-4df8-47ac-873a-cf9bce846929"
---

# Plan — Keepalive `email-watcher` (`6c0db1296181`)

Blueprint de ejecución para Tekton. Paridad mecánica con `telegram-watcher` / `event-watcher`. Sin forja de proceso nuevo.

## Fase 0 — Diseño (hecho, Dedalo)

- `spec.md` + este `plan.md` bajo `persist_ref`.
- Laudo A fijado en frontmatter de `spec.md`.

## Fase 1 — Keepalive en `run_loop` (Tekton)

**Target:** `SddIA/daemons/email-watcher/src/main.rs` (`directories.daemons` — fuera de DA-2 genoma).

**delegates_to (lectura/escritura local):** edición de cápsula daemon + `skill:git-manager` al cerrar commits.

1. Imports: `std::sync::Mutex` (ya hay `Arc`).
2. Constantes:
   - `HEARTBEAT_TICK_SECONDS: u64 = 10`
   - `HEARTBEAT_EMIT_FAIL_BUDGET: u32 = 5`
3. Función `spawn_heartbeat_worker(centinela: Arc<Mutex<DaemonRuntime>>, top: BusTopology) -> JoinHandle<()>` — copiar semántica de `telegram-watcher` (lock → `tick` → sleep 10 s → budget panic).
4. `run_loop`:
   - `bootstrap` sobre `DaemonRuntime` local.
   - `let shared = Arc::new(Mutex::new(centinela));`
   - `let _hb = spawn_heartbeat_worker(Arc::clone(&shared), top.clone());`
   - Bucle: `lock` → `poll_once` → `tick` → liberar guard → wait `poll_secs` con sleep 1 s **sin** exigir tick del hilo principal (el keepalive ya latía); opcional conservar ticks del wait si no reintroduce deadlock (no lock durante sleep del worker: el worker solo necesita el Mutex libre periódicamente).
   - Al salir: `lock` → `shutdown`.
5. Mensaje stdout de arranque con intervalo keepalive.

**Invariante de lock:** no retener el `MutexGuard` durante `thread::sleep` del intervalo de poll ni durante I/O IMAP si el worker debe emitir; patrón telegram: lock solo alrededor de `run_once`/`poll_once`, `drop` antes del sleep.

## Fase 2 — `--once` sin keepalive

Rama `main` con `--once`: **no** llamar `spawn_heartbeat_worker`. Sin cambios a `once_envelope` / schema JSON-IO.

## Fase 3 — Verificación crate

```text
cd SddIA && cargo build -p email-watcher
cd SddIA && cargo test -p email-watcher
```

CA1–CA2. No mutar umbrales ni genoma.

## Fase 4 — Documentación de ejecución (Tekton)

Bajo `persist_ref`:

- `implementation.md` — qué se cambió y dónde.
- `execution.md` — comandos/resultados de build/test.

## Fase 5 — Argos + cierre documental en rama

- `validacion.md`: `global: APTO`, `pbi_archived: true`, `branch: fix/email-watcher-heartbeat-keepalive`.
- Mover PBI `docs/todos/pending/[FIX] email-watcher — fractura sistémica (6c0db1296181).md` → `docs/todos/done/` (mismo `document_id`).
- Un solo PR (norma cierre documental).

## Fase 6 — Entrega

`./sddia-run.sh --process delivery-close-cycle` con inputs del ciclo (`persist_ref`, rama, correlation). Git solo vía `skill:git-manager`. Fire-and-forget tras acuse JSON.

## Orden

```text
spec/plan (Dedalo, este commit)
  → Fase 1–2 parche main.rs
    → Fase 3 cargo build/test
      → Fase 4 implementation.md + execution.md
        → Fase 5 validacion.md + archivo PBI
          → Fase 6 delivery-close-cycle → PR
```

## Fuera de este plan

Timeout IMAP, `uid_search("ALL")`, fagoctio apply, `iota-publish-relay`, umbrales Argos, emisores `daemon-heartbeat.md`.

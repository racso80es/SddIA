---
feature_name: iota-publish-relay-elf-fosil-r1
created: "2026-08-30"
updated: "2026-08-30"
process: bug-fix
branch_name: fix/iota-publish-relay-elf-fosil-r1
persist_ref: docs/fixes/iota-publish-relay-elf-fosil-r1
pbi_ref: docs/todos/pending/[REGRESIÓN] route-domain-event — fractura sistémica (701c77ebeab8)-R1.md
document_id: PBI-FIX-FRACTURE-701c77ebeab8-R1
uuid: ceec654f-e513-4c42-9e32-6e69bad713b1
scope: daemon-resolver-identity-and-dlt-drain
base: main
execution_id: "e0adfc87-d73f-4c08-8413-3d446823e5f6"
fracture_hash: 701c77ebeab8
---

# Spec — R1 ELF fósil + cola re-anclaje

SSOT de criterios: PBI R1 v1.2.0. Este spec no reabre Ola 0 (`GRACE_SECS`) ni Ola 1 (`tick_with_status` / espejo).

## Problema

El unit `sddia-iota-publish-relay@…` ejecuta `SddIA/target/debug/iota-publish-relay` (mtime 2026-08-28). HEAD ya contiene gracia + `degraded`. `:8787` nunca hace bind. `merkle-batch-preseal` → `iota-relay-unreachable` (os error 111). 19 UUID en `eda_instance.dlt_reanchor` con `path` a `pending/` ya purgado.

## Causa raíz

| ID | Hecho | Trabajo |
|----|-------|---------|
| D1 | `_sddia_resolve_daemon_binary` sirve el primer `-f && -x` (release → debug). Sin ELF mime. Sin ELF↔fuente. | Aduana en el resolutor compartido (CA1, CA6) |
| D2 | `event-watcher`, `telegram-watcher`, `github-bridge-watcher`, `event-sweeper` prefieren debug **aunque exista release**. No llaman al resolutor. | Convergencia de launchers (CA6) |
| D3 | `try_drain_dlt_reanchor_queue` exige que el `path` grabado siga siendo fichero. IOTA no mueve el padre; el sweeper sí. | Resolver por UUID en pending → processed → dead-letter (CA4) |
| D4 | Espejo verde: el ELF **vivo** emite `alive`. HEAD ya honra `degraded`. | Rebuild + restart; CA5 sobre el proceso systemd. Cero parche de diseño Ola 1. |

## Laudos

| ID | Decisión |
|----|----------|
| L-LUGAR-ADUANA | Aduana en `_sddia_resolve_daemon_binary`, no en `execute-process` ni en `iota-publish-relay/src/main.rs`. |
| L-MIME-INSUFICIENTE | `_sddia_is_native_elf` es prefiltro. El fósil **es** ELF. Gate duro = mtime ELF ≥ mtime máxima de fuente del crate (`Cargo.toml` + `*.rs` fuera de `target/`). |
| L-PERFILES | Orden de búsqueda: release luego debug. Primer candidato que cumple MIME **y** frescura vs fuente. Si release fósil y debug fresco → debug. Si ambos fósiles → no exec + error explícito. |
| L-CRATE | Crate: `SddIA/daemons/{name}/` o `SddIA/interfaces/{name}/` (kalma2-bridge). Si no hay crate → MIME-only no basta: error explícito (no silent). |
| L-DRAIN-UUID | Localizar `{uuid}.json` en `eda_bus.pending` → `processed` → `dead-letter`. No reinyectar a `pending/`. |
| L-NO-REABRIR | Prohibido mutar `GRACE_SECS`, `decide_supervisor_tick`, `record_heartbeat_at`, `color_daemon`. |

## Solución

### 1. Resolutor (`sddia_shell_lib.sh`)

`_sddia_daemon_crate_dir` + `_sddia_daemon_source_mtime` + `_sddia_resolve_daemon_binary` como arriba. Smoke: `SddIA/scripts/qa/test-daemon-binary-resolver.sh`.

### 2. Launchers

Los cuatro forked en `SddIA/daemons/{name}.sh` copian el patrón de `email-watcher.sh` (source lib + `_sddia_resolve_daemon_binary`). `email-watcher.sh` e `iota-publish-relay.sh` ya delegan; no reescribir salvo que el resolutor cambie la firma.

### 3. Drain (`route_domain_core.rs`)

Extraer `resolve_reanchor_event_path(repo, bus, uuid, stored_path) -> Option<PathBuf>`. Tests: path pending muerto + JSON en processed; ídem dead-letter; path vivo se respeta.

### 4. Deploy del ELF del sello

`cargo build -p iota-publish-relay` (release preferente). Restart del unit es ops de instancia (execution.md); no viaja en el PR.

## Criterios (Argos)

RELAY-R1-CA1…CA6 = PBI §8.

## Fuera de alcance

- Reabrir Ola 0 / Ola 1.
- Probe `/health` en centinelas sin hijo HTTP.
- Parser de cabeceras ELF como único gate.
- `DT-DLT-RELAY-NODE`.
- `SDDIA_LAB_SIMULATE_IOTA=1` como cierre.
- Jurisdicción `email-watcher` keepalive (`6c0db1296181`).

---
feature_name: iota-publish-relay-elf-fosil-r1
created: "2026-08-30"
process: bug-fix
branch_name: fix/iota-publish-relay-elf-fosil-r1
persist_ref: docs/fixes/iota-publish-relay-elf-fosil-r1
execution_id: "e0adfc87-d73f-4c08-8413-3d446823e5f6"
items_applied:
  - resolve_daemon_binary_identity
  - launcher_converge
  - resolve_reanchor_event_path
  - rebuild_iota_publish_relay_debug
---

# Ejecución — R1 ELF fósil + drain

## Fases

| Fase | Estado | Evidencia |
|------|--------|-----------|
| Inicialización | executed | `e0adfc87-d73f-4c08-8413-3d446823e5f6`. Fetch git-manager DNS-fail; rama vía cápsula `git-manager` checkout; `SDDIA_LAB_SKIP_GIT=1` en el segundo `bug-fix`. |
| Diseño | executed | `e3b742c` spec+plan |
| L0 resolutor | executed | `sddia_shell_lib.sh` |
| L1 launchers | executed | 4 `.sh` convergen |
| L2 drain | executed | `resolve_reanchor_event_path` |
| tests | executed | smoke resolver OK; `cargo test -p execute-process resolve_reanchor` 4/4; `cargo test -p iota-publish-relay` 7/7 |
| L3 rebuild | executed | `SddIA/target/debug/iota-publish-relay` mtime 2026-08-30 19:57. `systemctl --user restart` unit. `curl /health` → `{"ok":true}`. Hijo `server.mjs` pid 67195. |

## Cola re-anclaje (instancia)

`.SddIA/dlt/reanchor-queue/` sigue con entradas hasta el próximo `route-domain-event` con `/health` 200 (try_drain). Código listo; no se forzó el orquestador a mano.

## Comandos

```bash
SddIA/scripts/qa/test-daemon-binary-resolver.sh
cd SddIA && cargo test -p execute-process resolve_reanchor
cd SddIA && cargo test -p iota-publish-relay
unset CARGO_TARGET_DIR && cd SddIA && cargo build -p iota-publish-relay
```

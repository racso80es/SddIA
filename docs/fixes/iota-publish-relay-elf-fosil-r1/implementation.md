---
feature_name: iota-publish-relay-elf-fosil-r1
created: "2026-08-30"
process: bug-fix
branch_name: fix/iota-publish-relay-elf-fosil-r1
persist_ref: docs/fixes/iota-publish-relay-elf-fosil-r1
items:
  - sddia_shell_lib/_sddia_resolve_daemon_binary
  - daemons/launchers-converge
  - route_domain_core/resolve_reanchor_event_path
---

# Implementation — R1 ELF fósil + drain

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/scripts/common/sddia_shell_lib.sh` | MIME ELF + crate dir + mtime ELF ≥ fuente. Release luego debug. Fósil ⇒ no exec. |
| `SddIA/daemons/{event-watcher,telegram-watcher,github-bridge-watcher,event-sweeper}.sh` | Delegan en `_sddia_resolve_daemon_binary`. Fin del debug-first. |
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | `resolve_reanchor_event_path`: path vivo o UUID en pending → processed → dead-letter. |
| `SddIA/scripts/qa/test-daemon-binary-resolver.sh` | Smoke CA6. |

`iota-publish-relay/src/main.rs` **intacto** (L-NO-REABRIR).

## Contrato

- Identidad = ELF nativo **y** mtime ≥ fuente del crate. MIME solo no cierra el sello.
- Drain no reinyecta a `pending/`.

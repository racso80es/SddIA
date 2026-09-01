---
feature_name: email-watcher-elf-fosil-1933c0a0fe2c
created: "2026-09-01"
process: bug-fix
phases:
  - rebuild-release-elf
  - crate-tests
  - recycle-instance
  - document-implementation-execution
  - argos-validacion-archive-pbi
  - delivery-close-cycle
branch_name: fix/email-watcher-elf-fosil-1933c0a0fe2c
persist_ref: docs/fixes/email-watcher-elf-fosil-1933c0a0fe2c
execution_id: "a8e4d437-4c8c-42a4-888b-3fd1de477883"
---

# Plan — Reciclo ELF `email-watcher` (`1933c0a0fe2c`)

Blueprint Tekton. Laudo C. Sin mutación de `SddIA/daemons/email-watcher/src/main.rs`.

## Fase 0 — Diseño (hecho, Dedalo)

- `spec.md` + este `plan.md` bajo `persist_ref`.
- Laudo C fijado en frontmatter de `spec.md`.
- Commit de diseño vía `skill:git-manager` (mandato operador: detente tras este commit, luego continuar a PR).

## Fase 1 — Recompilar ELF (Tekton)

```text
cd SddIA && unset CARGO_TARGET_DIR && cargo build --release -p email-watcher
```

Verificar: `_sddia_resolve_daemon_binary` no marca fósil; `strings` del release contiene keepalive.

## Fase 2 — Tests crate

```text
cd SddIA && unset CARGO_TARGET_DIR && cargo test -p email-watcher
```

CA2. Regresión de tests ya presentes (`heartbeat_keepalive_matches_sibling_centinels`, `--once` sin spawn). Cero parche de fuente.

## Fase 3 — Reciclar instancia

1. Confirmar unidad: `sddia-email-watcher@home-racso-Proyectos-SddIA.service`.
2. `systemctl --user restart` de esa unidad (Restart=always; ExecStart = `email-watcher.sh` → resolutor ELF fresco).
3. Si el bus user no está disponible: `_sddia_stop_lock_pid` sobre `.SddIA/daemons/status/email-watcher.lock` y relanzar launcher.
4. Verificar: PID ≠ 7064; lock `started_at` nuevo; keepalive en el proceso; `missed_cycles=0`.

## Fase 4 — Documentación de ejecución

Bajo `persist_ref`: `implementation.md`, `execution.md`.

## Fase 5 — Argos + cierre documental en rama

- `validacion.md`: `global: APTO`, `pbi_archived: true`, `branch: fix/email-watcher-elf-fosil-1933c0a0fe2c`.
- Mover PBI pending → `docs/todos/done/` (mismo `document_id`).
- Un solo PR.

## Fase 6 — Entrega

`./sddia-run.sh --process delivery-close-cycle` con `source_process: bug-fix`, `persist_ref`, `branch_name`. Git vía `skill:git-manager`. Fire-and-forget tras acuse JSON.

## Orden

```text
spec/plan (Dedalo, commit de diseño)
  → Fase 1 cargo build --release
    → Fase 2 cargo test
      → Fase 3 reciclo systemd/lock
        → Fase 4 implementation.md + execution.md
          → Fase 5 validacion.md + archivo PBI
            → Fase 6 delivery-close-cycle → PR
```

## Fuera de este plan

Re-forja keepalive, timeout IMAP, fagoctio apply, `iota-publish-relay`, umbrales Argos, emisores `daemon-heartbeat.md`.

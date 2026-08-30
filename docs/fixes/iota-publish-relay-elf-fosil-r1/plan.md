---
feature_name: iota-publish-relay-elf-fosil-r1
created: "2026-08-30"
updated: "2026-08-30"
process: bug-fix
phases:
  - l0-resolver-identity
  - l1-launcher-converge
  - l2-drain-uuid
  - tests-unit-smoke
  - l3-rebuild-relay
  - doc-closure
branch_name: fix/iota-publish-relay-elf-fosil-r1
persist_ref: docs/fixes/iota-publish-relay-elf-fosil-r1
pbi_ref: docs/todos/pending/[REGRESIÓN] route-domain-event — fractura sistémica (701c77ebeab8)-R1.md
document_id: PBI-FIX-FRACTURE-701c77ebeab8-R1
uuid: 539eaf58-ebac-4811-9688-94c7108f006c
execution_id: "e0adfc87-d73f-4c08-8413-3d446823e5f6"
---

# Plan — R1 ELF fósil + drain (sella Diseño)

Orden: L0 → L1 → L2 → tests → L3 → cierre. **Este commit sella Diseño** (`spec.md` + `plan.md` + `objectives.md`). Código = fase Ejecución.

Prohibido: mutar `SddIA/tools|skills|actions|process|agents|events|norms` a mano; reabrir `main.rs` del relay.

## Fase L0 — Resolutor (CA6, CA1 parcial)

Archivo: `SddIA/scripts/common/sddia_shell_lib.sh`.

1. `_sddia_daemon_crate_dir`, `_sddia_daemon_source_mtime`, gate mtime ELF ≥ fuente.
2. Prefijo MIME `_sddia_is_native_elf`.
3. Smoke: `SddIA/scripts/qa/test-daemon-binary-resolver.sh` (copia `/bin/true` como ELF; crate falso más nuevo ⇒ no exec; ELF más nuevo ⇒ exec; release fósil + debug fresco ⇒ debug).

## Fase L1 — Launchers (CA6)

`SddIA/daemons/{event-watcher,telegram-watcher,github-bridge-watcher,event-sweeper}.sh` → mismo patrón que `email-watcher.sh`.

`grep` post-facto: los 6 launchers del índice llaman `_sddia_resolve_daemon_binary`. Cero `NATIVE_DEBUG` primero.

Material `directories.daemons` → evolution-register en el commit de código (no en este sello).

## Fase L2 — Drain (CA4)

Archivo: `SddIA/engine/execute-process/src/engine/route_domain_core.rs`.

1. `resolve_reanchor_event_path`.
2. `try_drain` usa esa fn; si `None`, `continue`.
3. Tests en el mismo módulo.

## Fase tests

```bash
SddIA/scripts/qa/test-daemon-binary-resolver.sh
cd SddIA && cargo test -p execute-process resolve_reanchor
cd SddIA && cargo test -p iota-publish-relay
```

CI: `cargo build --workspace` + `verify-compiled-capsules`. No reabrir umbrales de heartbeat.

## Fase L3 — Rebuild (CA1/CA2/CA5, instancia)

Ops en `execution.md`: `cargo build -p iota-publish-relay` (y `--release` si el unit resuelve release). Restart systemd no es el PR.

## Cierre documental (misma rama)

1. `implementation.md` + `execution.md`.
2. `validacion.md` APTO, `pbi_archived: true`.
3. PBI → `docs/todos/done/`.
4. `delivery-close-cycle` `source_process: bug-fix`.

## Delegación

| Fase proceso | Quién | Artefacto |
|--------------|-------|-----------|
| Diseño | Dedalo (este sello, relevo local) | `spec.md`, `plan.md` |
| Ejecución | Tekton | código + `implementation.md` + `execution.md` |
| Verificación | Argos | `validacion.md` |
| Cierre | `delivery-close-cycle` | PR único |

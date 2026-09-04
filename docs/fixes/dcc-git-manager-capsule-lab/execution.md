---
feature_name: dcc-git-manager-capsule-lab
created: "2026-09-04"
process: bug-fix
branch_name: fix/ignition-pre-push-guard
execution_id: "ad403cf3-94b5-45ac-9889-65692f34b2a1"
items_applied:
  - ignition-release-pkg-git-manager
  - ignition-debug-git-manager
  - seal-witness-no-genome
  - verify-ca1-elf
  - verify-ca2-tool-status
  - verify-ca3-dcc-snapshot
---

# Ejecución — Ola 1 cápsula `git-manager`

## Compilación lab

```bash
cd SddIA && cargo build --release -p git-manager && cargo build -p git-manager
execute-process --seal-capsules --inputs '{"profile":"release","write_genome":false,"write_witness":true,"names":["git-manager"]}'
```

Release ELF `application/x-pie-executable` 717376 bytes. Debug 8053784. `SddIA/skills/git-manager.md` intacto (sin `source_sha256`). Testigo `SddIA/target/release/git-manager.sha256`.

## DCC-GM-CA2

```bash
./sddia-run.sh --tool git-manager --inputs '{"operation_type":"status","repository_path":"/home/racso/Proyectos/SddIA","operation_payload_json":{}}'
```

Stdout: `success: true`, `exitCode: 0`. Sin literal `no encontrada bajo SddIA/target`.

## DCC-GM-CA3

`delivery-close-cycle` `execution_id` `ad403cf3-94b5-45ac-9889-65692f34b2a1`.

| Fase | Status |
|------|--------|
| Snapshot final | **executed** (`9c5b6a0`, `files_committed: 2`, `skill:git-manager`) |
| Publicación remota | **executed** (push `fix/ignition-pre-push-guard`) |
| Aduana evolution | failed — `sddia-qa no encontrado` (Ola 2) |
| Aduana integridad índices | failed — Ola 2 |
| Apertura en forja | failed — `shell-executor` ausente (fuera de Ola 1) |

`failed_phase`: `Aduana evolution`. No Snapshot.

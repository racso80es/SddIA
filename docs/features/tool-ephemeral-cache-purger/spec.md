---
feature_name: tool-ephemeral-cache-purger
created: "2026-09-09"
process: feature
base: main
scope: core-tool-native-jail
branch_name: feat/tool-ephemeral-cache-purger
persist_ref: docs/features/tool-ephemeral-cache-purger
pbi_ref: docs/todos/pending/[FEATURE] Tool: ephemeral-cache-purger (Saneamiento Termodinámico).md
pbi_uuid: "0987ac64-2c95-41c4-9ca7-d777446034cb"
pbi_version: "1.2.0"
execution_id: "7f37a724-5d10-41df-9cc9-cf22dc275951"
---

# Spec — tool-ephemeral-cache-purger

## Tool

Binario nativo `ephemeral-cache-purger`. stdin JSON (`request` o plano). stdout `SddiaResponse`.

### Jail

1. Path absoluto. Relativo / `..` léxico → `SECURITY_VIOLATION_PATH_OUT_OF_BOUNDS`.
2. Por componente: `symlink_metadata`; symlink → `SECURITY_VIOLATION_SYMLINK`.
3. Match `^/tmp/cursor-sandbox-cache(/[a-f0-9]{16,64}(/.*)?)?$`.
4. Prefijos vetados: `/`, `/bin`, `/usr`, `/etc`, `/var`, `/lib`, `/lib64`, `/boot`, `/dev`, `/proc`, `/sys`, `/home`, `/root`, `/opt`, `/sbin`, y `HOME` si existe.

### Inventario

Default `target_dir=/tmp/cursor-sandbox-cache`. Listar hijos hex 16–64. Candidato = `{dir}/{hash}/cargo-target` si existe (salvo `purge_sandbox_root`). `older_than_hours` filtra mtime. Recursión de bytes **sin** tope 4.

### Purga

`remove_dir_all` por candidato. `EACCES`/`ENOENT` → `errors[]`. Panic capturado → `emit_error`.

## Acción

Handler: `invoke_tool_capsule_json` nativo, `simulate: true`, validar cada candidato contra el regex, luego `simulate: false` con el mismo `target_dir`/flags. Abort si dry-run `success=false` o candidato fuera.

## Forja

`entity-manager` `create` tool + action. Creator escribe `{name}.md` + índice. Crate y handler = ejecución Tekton post-forja.

## Fuera

WASI v1. Escalada de privilegios. Borrado fuera del regex.

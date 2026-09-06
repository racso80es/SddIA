---
uuid: "4dac18fc-4cd1-4aa4-bdc3-faeb3bf762fc"
name: "git-manager"
version: "1.2.0"
contract: "skills-contract v1.1.0"
context: "source-control"
provides:
  - id: "proc:git-sync"
    contract: "proc.git_sync"
    version: "1.0.0"
capabilities:
  - "git-read-state"
  - "git-branching"
  - "git-commit"
  - "git-sync-remote"
hash_signature: "sha256:50f7ec30bf514c9153fead435f9c356a61d156f99d3979d4749e27eac1f34b48"
inputs:
  - "operation_type": "Enum congelado (minúsculas): status | checkout | commit | push | pull | fetch | branch_list | get_last_commit | merge | delete_branch | diff_name_only | commit_summary. SSOT: normative_documents.skill_io_git_manager_frozen v1.2.0 en cumulo.paths.json. Homónimo remote: string (push/pull/fetch) ≠ boolean (delete_branch)."
  - "repository_path": "string; ruta absoluta del repositorio Git, resuelta previamente por Cúmulo (sin path traversal)."
  - "operation_payload_json": "object; forma estricta según operation_type. Claves y semántica: SddIA/norms/skill-io-git-manager-frozen.md (v1.2.0)."
outputs:
  - "success": "boolean"
  - "exitCode": "integer; 0 éxito, distinto de 0 error"
  - "data": "objeto con gitStdout, gitStderr (camelCase); opcionalmente campos parseados según operation_type (p. ej. estructura para branch_list; commit_summary: commitHash, subject, files, totalFilesChanged, truncated)"
  - "error": "string de diagnóstico cuando exitCode != 0 o success es false"
---

# Skill: git-manager (definición + cápsula Fase B)

## 1. Propósito y naturaleza
Motor unificado y ciego para interacciones **exclusivas** con el binario nativo **`git`**. No aplica reglas de negocio ni nomenclatura de ramas; solo traduce un JSON determinista a invocaciones `git` permitidas. Mapeo de capabilities a operaciones: **git-read-state** (`status`, `branch_list`, `get_last_commit`, `diff_name_only`, `commit_summary`), **git-branching** (`checkout`, `delete_branch` local), **git-commit** (`commit`, `merge`), **git-sync-remote** (`fetch`, `pull`, `push`, `delete_branch` remoto).

## 2. Alcance y prohibidos
- **Solo** el ejecutable `git` del sistema. Queda prohibido enrutar aquí `gh`, `npm`, `docker`, scripts ad hoc, etc. (véase `SddIA/norms/pull-request-orchestration.md` y `skill-io-shell-executor-frozen.md`).
- La validación de política (p. ej. prefijos `feat/`) es previa a la invocación; norma: `SddIA/norms/git-operations.md`.

## 3. Motor de ejecución (cápsula física)
Cápsula nativa **`git-manager`** resuelta vía `cumulo.paths.json` → `compiled_capsules` → `SddIA/target/{release|debug}/git-manager` (preferente WASM en `compiled_capsules.wasm_root`). Un único objeto JSON por **stdin**; respuesta JSON en **stdout** (`success`, `exitCode`, `data`, `error` si fallo).

Invocación orientativa: `./sddia-run.sh --tool git-manager`

Invocación directa (tras `cargo build -p git-manager`): `SddIA/target/debug/git-manager`

Implementación: crate Rust `SddIA/skills/git-manager/` — patrón Command (argumentos en lista, sin shell); validación estricta de `operation_payload_json`; rutas de `files` en `commit` confinadas al repositorio.

## 4. Referencias normativas
- `SddIA/norms/skill-io-git-manager-frozen.md`
- `SddIA/skills/skills-contract.md` (§5 esquemas congelados)
- `SddIA/core/cumulo.paths.json` → `directories.norms`, `normative_documents`

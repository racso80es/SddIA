---
uuid: "4dac18fc-4cd1-4aa4-bdc3-faeb3bf762fc"
name: "git-manager"
version: "1.0.0"
contract: "skills-contract v1.1.0"
context: "source-control"
capabilities:
  - "git-read-state"
  - "git-branching"
  - "git-commit"
  - "git-sync-remote"
hash_signature: "sha256:e3c101d4bdfc8ee9d33b55393a4bb073069303987007946fa26806c7b34d6792"
inputs:
  - "operation_type": "Enum congelado (minúsculas): status | checkout | commit | push | pull | fetch | branch_list. SSOT: normative_documents.skill_io_git_manager_frozen en cumulo.paths.json."
  - "repository_path": "string; ruta absoluta del repositorio Git, resuelta previamente por Cúmulo (sin path traversal)."
  - "operation_payload_json": "object; forma estricta según operation_type. Claves y semántica: SddIA/norms/skill-io-git-manager-frozen.md."
outputs:
  - "success": "boolean"
  - "exitCode": "integer; 0 éxito, distinto de 0 error"
  - "data": "objeto con gitStdout, gitStderr (camelCase); opcionalmente campos parseados según operation_type (p. ej. estructura para branch_list)"
  - "error": "string de diagnóstico cuando exitCode != 0 o success es false"
---

# Skill: git-manager (definición + cápsula Fase B)

## 1. Propósito y naturaleza
Motor unificado y ciego para interacciones **exclusivas** con el binario nativo **`git`**. No aplica reglas de negocio ni nomenclatura de ramas; solo traduce un JSON determinista a invocaciones `git` permitidas. Mapeo de capabilities a operaciones: **git-read-state** (`status`, `branch_list`), **git-branching** (`checkout`), **git-commit** (`commit`), **git-sync-remote** (`fetch`, `pull`, `push`).

## 2. Alcance y prohibidos
- **Solo** el ejecutable `git` del sistema. Queda prohibido enrutar aquí `gh`, `npm`, `docker`, scripts ad hoc, etc. (véase `SddIA/norms/pull-request-orchestration.md` y `skill-io-shell-executor-frozen.md`).
- La validación de política (p. ej. prefijos `feat/`) es previa a la invocación; norma: `SddIA/norms/git-operations.md`.

## 3. Motor de ejecución (cápsula física)
Cápsula **`git-manager.py`** resuelta vía `cumulo.paths.json` → `execution_capsules.skills` → `git-manager.py`. Un único objeto JSON por **stdin**; respuesta JSON en **stdout** (`success`, `exitCode`, `data`, `error` si fallo). Invocación orientativa desde la raíz del workspace:

`python {paths.execution_capsules.skills}/git-manager.py`

Implementación: **patrón Command** (argumentos en lista, `subprocess` **sin** `shell=True`), validación estricta de `operation_payload_json` y tokens sin metacaracteres de shell; rutas de `files` en `commit` confinadas al repositorio. El proceso termina con código `0` si `git` devolvió `0`, `1` en caso contrario o error de validación.

## 4. Referencias normativas
- `SddIA/norms/skill-io-git-manager-frozen.md`
- `SddIA/skills/skills-contract.md` (§5 esquemas congelados)
- `SddIA/core/cumulo.paths.json` → `directories.norms`, `normative_documents`

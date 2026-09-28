---
uuid: "f4a5b6c7-d8e9-4f0a-1b2c-3d4e5f6a7b8c"
name: "filesystem-manager"
version: "2.0.0"
contract: "skills-contract v1.1.0"
context: "filesystem-ops"
capabilities:
  - "file-read"
  - "file-write"
  - "list-directory"
  - "delete-file"
  - "create-directory"
  - "move-file"
  - "patch-file"
provides:
  - id: "doc:closure"
    contract: "doc.closure"
    version: "1.0.0"
  - id: "fs:persist"
    contract: "fs.persist"
    version: "1.0.0"
inputs:
  - "operation": "Enum estricto: [READ_FILE, WRITE_FILE, LIST_DIR, DELETE_FILE, CREATE_DIR, MOVE_FILE, PATCH_FILE]"
  - "target_path": "Ruta relativa al directorio raíz del workspace (workspace_root o SDDIA_WORKSPACE_ROOT)"
  - "content": "(Opcional) Cadena para WRITE_FILE o diff unificado para PATCH_FILE"
  - "destination_path": "(Opcional) Ruta de destino para MOVE_FILE"
  - "workspace_root": "(Opcional) Raíz absoluta del proyecto; default: SDDIA_WORKSPACE_ROOT o repo Core"
outputs:
  - "exitCode": "0 para éxito, 1 para error"
  - "data": "Contenido del archivo (READ_FILE) o array de strings (LIST_DIR)"
  - "error_log": "Descripción detallada si exitCode es 1"
---

# Skill: Filesystem Manager (cápsula física)

## 1. Propósito
Interfaz física con el disco bajo `workspace_root`, sin depender del IDE. Operaciones acotadas por Cerbero (`filesystem-ops`).

## 2. Motor de ejecución
Cápsula nativa **`filesystem-manager`**: `SddIA/target/{debug,release}/filesystem-manager`. JSON por stdin; respuesta JSON en stdout (`success`, `exitCode`, `data`).

Invocación: `./sddia-run.sh` con DI hacia `skill:filesystem-manager`.

Implementación: `SddIA/skills/filesystem-manager/`.

## 3. Límites
- Path traversal (`..`, rutas absolutas fuera de raíz) → `PROJECT_SCOPE_ESCAPE`.
- `PATCH_FILE`: diff unificado; hunk que no casa → exit 1, fichero intacto.

## 4. Referencias
- `SddIA/norms/skill-io-filesystem-manager-frozen.md`
- `SddIA/skills/skills-contract.md`

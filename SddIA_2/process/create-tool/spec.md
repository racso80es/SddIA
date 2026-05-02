---
contract_ref: paths.processPath/process-contract.json
inputs:
  description: Descripción breve. Obligatorio.
  tool_id: kebab-case. Obligatorio.
name: Create Tool
paths:
  featurePath_ref: paths.featurePath (Cúmulo)
  toolCapsulesRef: paths.toolCapsules
  toolsDefinitionPath: ./SddIA/tools/
  toolsIndexPath_ref: paths.toolsIndexPath (Cúmulo)
  toolsPath_ref: paths.toolsPath (Cúmulo)
persist_ref: paths.featurePath/create-tool-<tool-id>
phases:
  - description: Ejecutar git-workspace-recon para validar entorno limpio. Tras confirmar, crear rama feat/create-tool-<tool-id> desde master usando git-branch-manager.
    id: '0'
    name: Preparar entorno
  - description: Objetivos y especificación (objectives.md, spec.md); definición en paths.toolsDefinitionPath.
    id: '1'
    name: Especificación y definición SddIA
  - description: Cápsula, manifest, índice, Cúmulo; hitos consolidados con git-save-snapshot. Ante fallo estructural, git-tactical-retreat (con confirmación explícita).
    id: '2'
    name: Implementación de cápsula e integración
  - description: Validación contractual y funcional de la tool.
    id: '3'
    name: Validar
  - description: Cierre. git-sync-remote; git-create-pr inyectando objectives.md y validacion.md en el cuerpo del PR.
    id: '4'
    name: Finalizar
process_doc_ref: paths.processPath/create-tool/
process_id: create-tool
process_interface_compliance: 'Genera en carpeta de la tarea un .md por acción con YAML Frontmatter (objectives.md, spec.md, implementation.md, validacion.md); no ficheros .json separados. Entrega ejecutable: cápsula en paths.toolCapsules[<tool-id>]. Norma: features-documentation-frontmatter.md.'
related_actions:
  - spec
  - implementation
  - validate
  - finalize-process
related_skills:
  - git-workspace-recon
  - git-branch-manager
  - git-save-snapshot
  - git-sync-remote
  - git-tactical-retreat
  - git-create-pr
spec_version: 2.0.0
tools_contract_ref: SddIA/tools/tools-contract.json
triggers:
  - Crear nueva herramienta en paths.toolsPath
  - Solicitud de creación de herramienta con tool-id
---
# Proceso: Creación de herramientas (create-tool) (spec_version 2.0.0)

Este documento define el **proceso de tarea** para crear una nueva herramienta (tool) (**spec_version 2.0.0**), con **Arsenal Táctico Git (S+)**: `git-workspace-recon`, `git-branch-manager`, `git-save-snapshot`, `git-sync-remote`, `git-tactical-retreat`, `git-create-pr`. Ubicación: paths.processPath/create-tool/ (Cúmulo). Rutas: **Cúmulo** (paths.toolsPath, paths.toolCapsules, paths.toolsIndexPath).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): la tarea de creación genera en la carpeta de la tarea (Cúmulo) un **`.md` por acción** con **YAML Frontmatter** (objectives.md, spec.md, implementation.md, validacion.md). No ficheros .json separados. El **resultado ejecutable** es la cápsula en **paths.toolCapsules[<tool-id>]** con todos los artefactos requeridos por el contrato de herramientas. Norma: SddIA/norms/features-documentation-frontmatter.md.

## Propósito

El proceso **create-tool** define el procedimiento para incorporar una nueva herramienta al ecosistema de paths.toolsPath (Cúmulo): desde la definición del objetivo hasta la cápsula lista, el índice actualizado y Cúmulo sincronizado. Garantiza que cada herramienta cumpla SddIA/tools/tools-contract.json y que las rutas queden registradas en Cúmulo y en scripts/tools/index.json.

## Alcance del procedimiento

- **Documentación de la tarea:** Cúmulo (paths.featurePath/create-tool-<tool-id>/).
- **Definición (SddIA):** paths.toolsDefinitionPath/<tool-id>/ con spec.md y spec.json (implementation_path_ref obligatorio).
- **Cápsula (implementación):** paths.toolCapsules[<tool-id>].

**Flujo Git (S+):** Fase 0 — **git-workspace-recon** y **git-branch-manager** (rama `feat/create-tool-<tool-id>`). Durante cápsula/índice/Cúmulo — **git-save-snapshot** por hitos; **git-tactical-retreat** solo ante emergencia y con confirmación. Cierre — **git-sync-remote** y **git-create-pr** con objectives.md y validacion.md en el cuerpo del PR.

Fases detalladas (orientativas, alineadas al frontmatter): 0 Preparar entorno | 1 Objetivos, spec y definición SddIA | 2–6 Cápsula, manifest, scripts, índice, Cúmulo (snapshots) | 7 Opcional Rust | 8 Validación | 9 Cierre (sync + PR).

## Restricciones

- toolId en kebab-case. Rama feat/create-tool-<tool-id>. Windows 11, PowerShell 7+. Contrato tools (salida JSON, feedback) obligatorio.

## Implementación de la Herramienta

La herramienta debe implementarse **únicamente como ejecutable Rust** (`.exe`).

**Estructura esperada de la cápsula:**

```
scripts/tools/<tool-id>/
├── <tool-name>.exe             # Ejecutable Rust compilado (OBLIGATORIO)
├── manifest.json               # Metadatos de la herramienta (OBLIGATORIO)
├── <tool-name>-config.json     # Configuración (si aplica)
├── <tool-name>.md              # Documentación de uso (OBLIGATORIO)
└── <Tool-Name>.bat             # Launcher que invoca el .exe
```

**Fuente Rust:**

El código fuente Rust debe ubicarse en:
```
scripts/tools-rs/src/<tool-name>.rs
```

O, si es complejo:
```
scripts/tools-rs/src/<tool-name>/
├── main.rs
├── lib.rs
└── ...
```

**Proceso de Compilación:**

1. Desarrollar en `scripts/tools-rs/src/<tool-name>.rs`
2. Compilar:
   ```powershell
   cargo build --release --manifest-path scripts/tools-rs/Cargo.toml
   ```
3. Copiar el `.exe` generado a `scripts/tools/<tool-id>/<nombre>.exe` (ruta de la tool)
4. Actualizar el índice: `scripts/tools/index.json`
5. Actualizar Cúmulo: `SddIA/agents/cumulo.paths.json` (campo `toolCapsules`)

**Prohibiciones:**

❌ **NO se deben crear:**
- Archivos `.ps1` (PowerShell scripts)
- Archivos `.bat` (Batch files) como implementación principal
- Scripts shell (`.sh`)
- Cualquier otro formato de script

✅ **Solo se debe generar:** Ejecutable `.exe` compilado desde Rust.

**Migración desde .ps1:**

Si estás migrando una herramienta existente desde `.ps1` a `.exe`:
1. Implementar en Rust
2. Validar funcionamiento del `.exe`
3. Eliminar el `.ps1`
4. Actualizar la spec de la herramienta con sección "Implementación"

## Referencias

- Contrato: SddIA/tools/tools-contract.json, tools-contract.md.
- Cúmulo: paths.toolsDefinitionPath, paths.toolsPath, paths.toolCapsules, paths.toolsIndexPath.
- Proceso machine-readable: paths.processPath/create-tool/spec.json.

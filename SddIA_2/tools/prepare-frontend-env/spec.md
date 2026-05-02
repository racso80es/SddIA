---
contract_ref: SddIA/tools/tools-contract.json
cumulo_ref: SddIA/agents/cumulo.json
depends_on_tools: []
env:
  - Windows 11
  - PowerShell 7+
  - Node.js 20+
implementation_path_ref: paths.toolCapsules.prepare-frontend-env
inputs:
  OutputPath: string (opcional). Fichero donde escribir el resultado JSON.
  Quiet: boolean (opcional). Suprimir salida JSON por stdout (por defecto: se emite).
output:
  exit_codes:
    '0': 'Exito: entorno preparado'
    '1': npm install fallo
    '2': Excepcion no controlada
    '3': Directorio src no encontrado
  phases_feedback:
    - init
    - install
    - env-check
    - done
    - error
  schema_ref: tools-contract.json output.required_fields y optional_fields
toolId: prepare-frontend-env
version: 1.0.0
---
# Especificación: prepare-frontend-env

**toolId:** `prepare-frontend-env`
**Definición (SddIA):** Este directorio.
**Implementación (scripts):** Ruta canónica en Cúmulo → **implementation_path_ref:** `paths.toolCapsules.prepare-frontend-env`.

## Objetivo

Herramienta que prepara el entorno de desarrollo frontend: instala dependencias npm en `src/` y verifica que exista configuración de entorno (`.env.local`).

## Entradas

| Parámetro | Tipo | Descripción |
|----------|------|-------------|
| OutputPath | string | Fichero donde escribir el resultado JSON. |
| Quiet | switch | Suprimir salida JSON por stdout (por defecto: se emite). |

## Salida

Cumple `SddIA/tools/tools-contract.json`: objeto JSON con toolId, exitCode, success, timestamp, message, feedback[], data (env_local_exists, env_created), duration_ms. **JSON por stdout por defecto.**

**Tabla codificada (tools-contract.output.output_codes_table):** [output-salida-json.md](./output-salida-json.md)

| exitCode | success | message_resumen | data_presente | descripción |
|----------|---------|-----------------|---------------|-------------|
| 0 | true | "Entorno frontend preparado" | Sí | Éxito |
| 1 | false | "npm install fallo" | Sí | npm install falló |
| 2 | false | "Error: &lt;excepción&gt;" | Sí | Excepción |
| 3 | false | "Directorio frontend no encontrado" | Sí | src/ no existe |

## Fases (feedback)

init → install → env-check → done (o error).

## Estado de implementación

**Formato principal:** Ejecutable Rust (`.exe`)
**Ubicación:** `scripts/tools/prepare-frontend-env/prepare_frontend_env.exe`
**Fuente:** `scripts/tools-rs/src/bin/prepare_frontend_env.rs`
**Fallback:** `Prepare-FrontendEnv.ps1` cuando no existe el .exe

---
contract_ref: SddIA/tools/tools-contract.json
cumulo_ref: SddIA/agents/cumulo.json
depends_on_tools: []
env:
  - Windows 11
  - PowerShell 7+
  - Node.js 20+
implementation_path_ref: paths.toolCapsules.run-tests-frontend
inputs:
  BaseUrl: string (opcional). URL base frontend para E2E; por defecto http://localhost:3000.
  OnlyTests: boolean (opcional). Solo ejecutar tests (sin npm install).
  OutputPath: string (opcional). Fichero donde escribir el resultado JSON.
  Quiet: boolean (opcional). Suprimir salida JSON por stdout (por defecto: se emite).
  TestScope: string (opcional). unit | e2e | build | lint | all. Por defecto all.
output:
  data_fields: tests_summary (scope, exit codes), duration_ms
  phases_feedback:
    - init
    - install
    - lint
    - build
    - unit
    - e2e
    - done
    - error
  schema_ref: tools-contract.json output.required_fields y optional_fields
toolId: run-tests-frontend
version: 1.0.0
---
# Especificación: run-tests-frontend

**toolId:** `run-tests-frontend`
**Definición (SddIA):** Este directorio (paths.toolsDefinitionPath/run-tests-frontend/).
**Implementación (scripts):** Ruta canónica en Cúmulo → **implementation_path_ref:** `paths.toolCapsules.run-tests-frontend`.

## Objetivo

Ejecutar tests del frontend (unitarios, E2E, build, lint) en condiciones de validación local sin invocar comandos npm directamente desde el agente. Ejecuta los scripts npm correspondientes en `src/` según el alcance indicado.

## Entradas

| Parámetro | Tipo | Descripción |
|-----------|------|-------------|
| TestScope | string | unit, e2e, build, lint, all (por defecto all). |
| OnlyTests | switch | Solo ejecutar tests (sin npm install previo). |
| BaseUrl | string | URL base del frontend para E2E (por defecto http://localhost:3000). |
| OutputPath | string | Fichero donde escribir el resultado JSON. |
| Quiet | switch | Suprimir salida JSON por stdout (por defecto: se emite). |

## Salida

Cumple `SddIA/tools/tools-contract.json`: toolId, exitCode, success, timestamp, message, feedback[], data (scope, lint_exit, build_exit, unit_exit, e2e_exit, duration_ms). **JSON por stdout por defecto.**

**Tabla codificada (tools-contract.output.output_codes_table):** [output-salida-json.md](./output-salida-json.md)

| exitCode | success | message_resumen | data_presente | descripción |
|----------|---------|-----------------|---------------|-------------|
| 0 | true | "Tests completados correctamente" | Sí | Éxito |
| 1 | false | "npm install fallo" | Sí | npm install falló |
| * | false | "Tests con fallos" | Sí | Algún test falló |
| 1 | false | "Error: &lt;excepción&gt;" | Sí | Excepción no controlada |

## Fases (feedback)

init → install (opcional) → lint → build → unit → e2e → done (o error).

## Estado de implementación

**Formato principal:** Ejecutable Rust (`.exe`)
**Ubicación:** `scripts/tools/run-tests-frontend/run_tests_frontend.exe`
**Fuente:** `scripts/tools-rs/src/bin/run_tests_frontend.rs`
**Fallback:** `Run-Tests-Frontend.ps1` cuando no existe el .exe

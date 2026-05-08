---
contract_ref: SddIA/tools/tools-contract.json
cumulo_ref: SddIA/agents/cumulo.json
depends_on_tools: []
env:
  - Windows 11
  - PowerShell 7+
  - Node.js 20+
implementation_path_ref: paths.toolCapsules.start-frontend
inputs:
  OutputJson: boolean (opcional). Emitir resultado JSON por stdout (por defecto: si).
  OutputPath: string (opcional). Fichero donde escribir el resultado JSON.
  Quiet: boolean (opcional). Suprimir salida JSON por stdout (solo con --output-path).
  Port: number (opcional). Puerto del dev server. Por defecto 3001.
output:
  exit_codes:
    '0': 'Exito: frontend responde'
    '1': Config no encontrado o invalido
    '2': Puerto ocupado
    '7': Frontend no respondio a tiempo
  phases_feedback:
    - init
    - port-check
    - launch
    - healthcheck
    - done
    - error
  schema_ref: tools-contract.json output.required_fields y optional_fields
  success_criterion: El frontend responde en http://localhost:<port> (HTTP 200).
toolId: start-frontend
version: 1.0.0
---
# Especificación: start-frontend

**toolId:** `start-frontend`
**Definición (SddIA):** Este directorio.
**Implementación (scripts):** Ruta canónica en Cúmulo → **implementation_path_ref:** `paths.toolCapsules.start-frontend`.

**Implementación Rust:** Ejecutable `start_frontend.exe` en la ruta de la tool (junto al .bat). Fuente: `scripts/tools-rs/src/bin/start_frontend.rs`. Launcher `.bat` invoca el .exe. Compilación: `cargo build --release` en tools-rs; despliegue: `install.ps1`.

## Objetivo

Herramienta que **levanta el dev server** del proyecto GesFer.Admin.Front (Next.js): ejecuta `npm run dev` en `src/`, comprueba que el puerto 3001 esté disponible y considera **éxito** si `http://localhost:3001` responde (HTTP 200).

## Entradas

| Parámetro     | Tipo   | Descripción |
|---------------|--------|-------------|
| Port          | number | Puerto del dev server (override). Por defecto 3001. |
| OutputPath    | string | Fichero donde escribir el resultado JSON (contrato). |
| OutputJson    | switch | Emitir el resultado JSON por stdout (por defecto: sí). |
| Quiet         | switch | Suprimir salida JSON por stdout (útil solo con --output-path). |

## Validación de éxito

La herramienta considera la ejecución **correcta** si y solo si `http://localhost:<port>` responde (HTTP 200) dentro del timeout configurado.

## Códigos de salida (exitCode)

Tabla codificada según tools-contract.output.output_codes_table. Detalle completo: [output-salida-json.md](./output-salida-json.md).

| exitCode | success | message_resumen | data_presente | descripción |
|----------|---------|-----------------|---------------|-------------|
| 0 | true | "Frontend levantado; health OK" | Sí | Éxito: frontend responde |
| 1 | false | "Config no encontrado o inválido" | No | Config inválido |
| 2 | false | "Puerto ocupado" | Sí | Puerto bloqueado |
| 3 | false | "No se pudo liberar el puerto" / "Puerto aún ocupado" | Sí/No | Fallo al liberar puerto |
| 4 | false | "Directorio frontend no encontrado" | No | Precondición fallida |
| 6 | false | "Error al lanzar frontend" | No | npm no ejecutó |
| 7 | false | "Health no respondió a tiempo" | Sí | Arrancó pero timeout health |

## Salida

Cumple `SddIA/tools/tools-contract.json`: objeto JSON con toolId, exitCode, success, timestamp, message, feedback[], data (url_base, port, pid), duration_ms.

**Especificación detallada:** [output-salida-json.md](./output-salida-json.md) — Análisis, especificación y clarificación de la salida JSON en función de la responsabilidad de la herramienta.

## Fases (feedback)

init → port-check → launch → healthcheck → done | error.

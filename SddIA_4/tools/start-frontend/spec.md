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
  OutputJson: boolean (opcional). Emitir resultado JSON por stdout (por defecto: no; flag CLI --output-json).
  OutputPath: string (opcional). Fichero donde escribir el resultado JSON.
  Quiet: boolean (opcional). Suprimir salida JSON por stdout (solo con --output-path).
  Port: number (opcional). Puerto del dev server. Por defecto 3000.
  PortBlocked: false | kill (opcional). Política si el puerto objetivo está en uso. Por defecto false. Fuentes: CLI --port-blocked, clave portBlocked en start-frontend-config.json (precedencia en spec). Ver «Puerto ocupado (PortBlocked)» y clarify.md.
output:
  exit_codes:
    '0': 'Exito: frontend responde'
    '1': Config no encontrado o invalido
    '2': Puerto ocupado
    '3': No se pudo liberar el puerto / Puerto aun ocupado
    '7': Frontend no respondio a tiempo
  phases_feedback:
    - init
    - port-check
    - port-kill
    - next-cache
    - launch
    - healthcheck
    - done
    - error
  schema_ref: tools-contract.json output.required_fields y optional_fields
  success_criterion: El frontend responde en http://localhost:<port> (HTTP 200).
toolId: start-frontend
version: 1.2.0
---
# Especificación: start-frontend

**toolId:** `start-frontend`
**Definición (SddIA):** Este directorio.
**Implementación (scripts):** Ruta canónica en Cúmulo → **implementation_path_ref:** `paths.toolCapsules.start-frontend`.

**Configuración (JSON):** `start-frontend-config.json` en la cápsula (`paths.toolCapsules.start-frontend`): directorio de trabajo, puerto por defecto, timeouts, y **`portBlocked`** (política ante puerto ocupado). Esquema de la clave: booleano `false` o cadena `"kill"` (misma semántica que **PortBlocked** en el spec).

**Implementación Rust:** Ejecutable `start_frontend.exe` en la ruta de la tool (junto al .bat). Fuente: `scripts/tools-rs/src/bin/start_frontend.rs`. Lee el JSON de configuración anterior; fusiona con flags CLI según precedencia documentada abajo. Launcher `.bat` invoca el .exe. Compilación: `cargo build --release` en tools-rs; despliegue: `install.ps1`. En **Windows**, el binario lanza `npm run dev` mediante `cmd /C npm …` porque `npm` es un `.cmd` y `CreateProcess` no lo resuelve igual que el shell (evita error «program not found»).

## Objetivo

Herramienta que **levanta el dev server** del proyecto GesFer.Product.Front (Next.js): ejecuta `npm run dev` en `src/`, comprueba el puerto configurado (por defecto 3000) y considera **éxito** si `http://localhost:<port>` responde (HTTP 200). Si el puerto está ocupado y **PortBlocked** es `kill`, puede intentar **liberar el puerto** terminando el proceso que lo escucha, solo cuando la implementación lo identifique como el dev server esperado (Node/Next); ver [clarify.md](./clarify.md) (decisiones D-02, D-03).

## Entradas

| Parámetro     | Tipo   | Descripción |
|---------------|--------|-------------|
| Port          | number | Puerto del dev server (override). Por defecto 3000. |
| PortBlocked   | `false` \| `kill` | Política ante puerto en uso. Por defecto **`false`**: si el puerto está ocupado, error (exit 2) sin matar procesos. **`kill`**: intentar terminar el proceso que escucha en `<Port>` solo si es identificable como el dev server de este frontend; si no se libera el puerto, exit 3. **CLI:** `--port-blocked false` \| `--port-blocked kill`. **Config JSON:** clave `portBlocked` en `start-frontend-config.json` (`false` booleano o `"kill"` como cadena). **Precedencia:** si el usuario pasa `--port-blocked` en la línea de comandos, ese valor **prevalece** sobre `portBlocked` del fichero; si no hay flag, se usa el del config; si la clave falta en el config, se asume `false`. |
| OutputPath    | string | Fichero donde escribir el resultado JSON (contrato). |
| OutputJson    | switch | Emitir el resultado JSON por stdout (por defecto: no). Integraciones suelen pasar `--output-json`. |
| Quiet         | switch | Suprimir salida JSON por stdout (útil solo con --output-path). |

## Puerto ocupado (PortBlocked)

1. **`false` (defecto):** Tras `port-check`, si el puerto no está libre → **exitCode 2** («Puerto ocupado»). No se invoca liberación del puerto.
2. **`kill`:** Si el puerto no está libre, la implementación resuelve el proceso en escucha (p. ej. API Windows), aplica el criterio de seguridad del [clarify.md](./clarify.md) (D-03) y, si procede, termina el proceso; feedback de fase **`port-kill`** cuando aplique (véase [output-salida-json.md](./output-salida-json.md)). Si el puerto sigue ocupado o no es seguro liberar → **exitCode 3**.

**Clarificaciones cerradas** (nombres, CLI, config JSON, criterio `kill`): [clarify.md](./clarify.md).

## Validación de éxito

La herramienta considera la ejecución **correcta** si y solo si la URL de salud (`http://127.0.0.1:<port>` + **`healthPath`** en `start-frontend-config.json`, por defecto **`/api/health`**, ruta dedicada que responde 200 sin depender del rewrite i18n) responde con HTTP **2xx** dentro del timeout configurado (`healthCheckTimeoutSeconds`).

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

init → port-check → [port-kill si PortBlocked kill y aplica] → [next-cache si aplica] → launch → healthcheck → done | error.

**Fase `next-cache` (opcional):** Antes de `launch`, si existe `.next` con artefactos que referencian el vendor chunk de TanStack (`@tanstack.js`) pero el fichero falta en disco (caché incoherente, p. ej. tras un build interrumpido), la implementación **elimina** el directorio `.next` del frontend y emite feedback informativo; si el borrado falla, emite advertencia. Así se evita el error de runtime `Cannot find module './vendor-chunks/@tanstack.js'` al arrancar `next dev`.

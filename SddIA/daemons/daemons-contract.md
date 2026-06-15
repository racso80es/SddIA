---
contract_version: "1.0.0"
entity_type: "daemon"
jurisdiction: "Core SddIA (Definición) / Workspace (Delivery periférico)"
capabilities:
  - "daemon-schema-governance"
  - "logical-blindness-enforcement"
  - "physical-anomaly-interception"
  - "heartbeat-telemetry-emission"
  - "eda-event-injection"
execution_substrate: "delivery-agnostic"
---

# Contrato de Centinelas / Daemons (S+ Grade)

Este documento rige los **Centinelas** (*daemons*, *watchers*): procesos periféricos con **Ceguera Lógica absoluta**. Su única función es interceptar anomalías **físicas** del entorno e inyectar instancias ECST en el bus. No orquestan, no mutan genoma, no interpretan intención de negocio.

**Principio rector (Invarianza del Core):** el Core SddIA dicta identidad, aislamiento, telemetría vital y contrato de emisión. El **Delivery** (lenguaje, runtime, layout del entrypoint) es workspace-local y se resuelve por topología (Cúmulo) mediante `execution.entrypoint` y `execution.runtime`.

## 1. Identidad Atómica (Innegociable)

Todo Centinela debe definirse mediante un archivo `{name}.md` bajo `directories.daemons` con cabecera YAML obligatoria:

| Campo | Obligatorio | Descripción |
|-------|:-----------:|-------------|
| `uuid` | Sí | UUID v4 inmutable |
| `name` | Sí | Identificador kebab-case (nombre del archivo sin extensión) |
| `version` | Sí | SemVer |
| `contract` | Sí | `daemons-contract v{contract_version}` |
| `context` | Sí | Política RBAC Cerbero (`execution-contexts.md`); típicamente `peripheral-sensing` o `ecosystem-evolution` |
| `hash_signature` | Sí | `sha256:` sobre canon §7 |
| `capabilities` | Sí | Array de strings para enrutamiento semántico (ej. `telegram-long-poll`, `github-webhook-bridge`, `eda-bus-watch`) |
| `execution` | Sí | Bloque de ejecución periférica (§4) |
| `jurisdiction` | Sí | Declaración de aislamiento; valor canónico: `Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus` |

Prohibido `spec.json` u otro formato paralelo. Entidades sin `{name}.md` conforme se marcan **Entropía/Código Fósil**.

## 2. Ceguera Lógica (Invariante ontológico)

| Permitido | Prohibido |
|-----------|-----------|
| Escuchar sockets, webhooks, FS watches, colas OS, stdin físico | Leer o mutar `SddIA/skills/`, `actions/`, `process/`, `agents/`, `tools/`, `events/` (genoma) |
| Emitir instancias ECST al bus (`eda_bus`, `eda_fractal`, `eda_instance`) | Invocar `execute-process`, `entity-manager` o acciones de orquestación |
| Escribir `.lock` de identidad física (§5) | Interpretar PBIs, features, rutas de negocio no inyectadas en su contrato |
| Terminar limpiamente ante SIGTERM/SIGKILL del actuador autorizado | Escalar privilegios (`sudo`, root, capabilities elevadas) |

El Centinela es **lógica y espacialmente ciego** respecto al dominio SddIA; solo conoce su contrato, su entrypoint y el envelope de eventos que debe emitir.

## 3. Consciencia Espacial (Obediencia al SSOT)

* Las definiciones catalogadas residen en `cumulo.directories.daemons` → `SddIA/daemons/{name}.md`.
* Los artefactos de delivery residen bajo `cumulo.execution_capsules.daemons` → `SddIA/daemons/{name}/` (objetivo Kaizen) o ruta resuelta por `execution.entrypoint` relativa al workspace.
* Estado runtime (locks, PIDs) bajo `cumulo.daemons_instance.status` → `.SddIA/daemons/status/` (fuera de Git).
* Prohibido hardcodear rutas absolutas del host; resolver exclusivamente vía topología inyectada.

## 4. Bloque `execution` (Innegociable)

Objeto YAML obligatorio en frontmatter de cada `{name}.md`:

```yaml
execution:
  entrypoint: "<ruta lógica o relativa al workspace del artefacto ejecutable>"
  runtime: "<identificador de runtime delivery: python3 | node | bash | wasmtime | ...>"
  heartbeat_interval_seconds: <entero positivo>
```

| Subcampo | Tipo | Reglas |
|----------|------|--------|
| `entrypoint` | string | Puntero al script/binario periférico; no normativo en Core |
| `runtime` | string | Intérprete o launcher autorizado en el workspace; delivery decide |
| `heartbeat_interval_seconds` | integer | Intervalo máximo entre emisiones `Daemon_Heartbeat`; ≥ 5 |

Ejemplo canónico (ilustrativo, no normativo en rutas):

```yaml
execution:
  entrypoint: "scripts/daemons/telegram-watcher.py"
  runtime: "python3"
  heartbeat_interval_seconds: 30
```

## 5. Identidad Física y Exclusión Mutua

Al **despertar** (arranque exitoso), el Centinela debe:

1. Crear `{daemons_instance.status}/{name}.lock` con el PID del proceso y timestamp ISO-8601 UTC.
2. Abortar arranque si el lock existe y el PID referenciado sigue vivo (evitar duplicados).
3. Eliminar el lock en shutdown limpio (SIGTERM manejada).

Formato mínimo del lock (JSON UTF-8):

```json
{
  "daemon_name": "<kebab-case>",
  "pid": <integer>,
  "started_at": "<ISO-8601 UTC>",
  "heartbeat_interval_seconds": <integer>
}
```

## 6. Obligaciones Termodinámicas (Innegociables)

### 6.1 Emisión `Daemon_Heartbeat`

El Centinela **debe** emitir periódicamente una instancia ECST de tipo **`Daemon_Heartbeat`** (familia `telemetry`) con cadencia ≤ `execution.heartbeat_interval_seconds`.

| Campo payload | Obligatorio | Descripción |
|---------------|:-----------:|-------------|
| `daemon_name` | Sí | Coincide con `name` del contrato |
| `daemon_uuid` | Sí | UUID v4 de la definición |
| `pid` | Sí | PID OS del proceso |
| `uptime_seconds` | Sí | Segundos desde arranque |
| `last_stimulus_at` | No | ISO-8601 del último estímulo físico procesado |
| `status` | Sí | `alive` \| `degraded` \| `shutting_down` |

**Destino de emisión:** preferente `eda_fractal.telemetry` (`./.events/telemetry/`). Si la saturación del bus principal lo exige, redirigir vía `eda_instance.customization` (`.SddIA/events/telemetry/`) sin contaminar el registro histórico DLT del bus V3+.

Omisión de **tres ciclos consecutivos** de heartbeat para un Centinela crítico habilita alerta `System_Fracture_Detected` (fan-out Argos — fuera de alcance CEN-01).

### 6.2 Idempotencia de estímulos

Debe poder **morir y reiniciar** sin reprocesar estímulos antiguos del entorno (cursor, offset, `update_id`, watermark persistido en `.SddIA/daemons/state/{name}.json` o equivalente delivery).

### 6.3 Purga (Kill-Switch)

Ante terminación del motor SddIA o señal del actuador autorizado (`governance-daemon-manager`), todos los Centinelas amparados deben apagarse limpiamente. Prohibidos procesos zombie. Señales: SIGTERM (graceful) → SIGKILL (timeout).

## 7. Canon de integridad (`hash_signature`)

```text
SHA-256(JSON UTF-8 canónico ordenado de:
  { name, version, context, capabilities, execution }
```

Separadores `(",", ":")`, `sort_keys: true`, `ensure_ascii: false`.

## 8. Interfaz de Emisión (ECST)

Los Centinelas **no** reciben stdin de orquestación SddIA en operación normal. Su salida al sistema es exclusivamente **instancias ECST** en el bus:

```json
{
  "event_id": "<uuid-v4>",
  "event_type": "<PascalCase_Snake de la Clase catalogada>",
  "timestamp": "<ISO-8601 UTC>",
  "emitter_agent": "<daemon_name>",
  "payload": { }
}
```

La forma del payload la gobierna la Clase de Evento correspondiente en `directories.events`. Para telemetría vital, la Clase **`daemon-heartbeat`** (event_type `Daemon_Heartbeat`) debe existir en `events/telemetry/`.

## 9. Frontera con Procesos y Acciones

| Rol | Entidad | Responsabilidad |
|-----|---------|-----------------|
| Periferia ciega | Centinela (`{name}.md`) | Escucha física + emisión ECST |
| Actuador OS | Proceso `governance-daemon-manager` (CEN-02) | `start` \| `status` \| `kill` sobre contratos indexados |
| Forja de definiciones | Proceso `daemon-creator` | Materializa `{name}.md` + índice bajo `directories.daemons` |
| Auditoría | Agente Argos | Latido térmico, locks huérfanos, fractura por omisión de heartbeat |

Queda **TERMINANTEMENTE PROHIBIDO** catalogar un Centinela como `action`, `skill` o `process` de negocio.

## 10. Termodinámica declarativa

Campos opcionales en frontmatter de `{name}.md` — paridad con `skills-contract.md` §6:

| Campo | Tipo | Default |
|-------|------|---------|
| `telemetry_provided` | boolean | `true` (implícito para Centinelas — heartbeat obligatorio) |
| `telemetry_schema` | string[] | `["uptime_seconds", "pid", "status"]` |

Cuando `telemetry_provided: true`, cada ciclo de heartbeat debe incluir recibo auditable cruzable por `telemetry-compliance-audit`.

## 11. Índice soberano

`SddIA/daemons/index.md` — columnas: **Archivo fuente** | **uuid** | **name** | **version** | **contract** | **context** | **Capabilities** | **heartbeat_interval_seconds**

El contrato de familia (`daemons-contract.md`) no es fila del catálogo.

## 12. Historial normativo (extracto)

- **v1.0.0** — Baseline CEN-01: identidad atómica `{name}.md`, bloque `execution`, Ceguera Lógica, `Daemon_Heartbeat`, locks `.SddIA/daemons/status/`, frontera con `daemon-creator` y `governance-daemon-manager`.

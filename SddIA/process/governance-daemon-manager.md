---
context:
- system-operations
- ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:5c1e999195271bc637127214c91d65f91e7c283ddab6a82ca67175da91190d54
inputs:
- operation: 'Enum estricto: `start` | `status` | `kill` — directriz física OS a ejecutar'
- daemon_id: Identificador kebab-case del Centinela (`name` en `{daemon_id}.md`; debe existir en `SddIA/daemons/index.md`)
- cumulo_topology: Topología SSOT inyectada (paths, contratos, directorios, daemons_instance); prohibido inferir rutas del host
- repository_path: Ruta absoluta del workspace raíz; inyectada por orquestador para resolución de `execution.entrypoint`
- kill_grace_seconds: Opcional; segundos entre SIGTERM y SIGKILL en `kill` (default `10`)
minteo_maximo: null
name: governance-daemon-manager
outputs:
- os_result: Objeto con resultado físico de la operación (pid, alive, signal, exit_code, lock_path, entrypoint_resolved, runtime)
- daemon_uuid: UUID v4 extraído del frontmatter de `{daemon_id}.md`
- operation_status: 'Enum: `succeeded` | `failed` | `noop` (ej. kill sobre PID inexistente)'
- orchestration_event_id: UUID v4 del `Process_Execution_Completed` emitido en `eda_fractal.orchestration`
- orchestration_event_path: Ruta relativa del JSON en `./.events/orchestration/`
phase_invocations:
- invocations:
  - bind:
      data.result: orchestration_event_id
    capsule: action:crypto-broker
    on_error: abort
    stdin_json:
      operation: GENERATE_UUID
      target_payload: null
  phase_name: Sello orchestration
phases:
- delegates_to:
  - agent:cumulo
  intent: Consultar `{directories.daemons}/index.md`; validar fila para `daemon_id`; leer `{daemon_id}.md`; extraer `execution.entrypoint`, `execution.runtime`, `uuid`, `name` vía cumulo.
  name: Resolución SSOT
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
- delegates_to:
  - agent:cerbero
  intent: 'Validar `operation` ∈ {start, status, kill}; RBAC Cerbero sobre contexto del Centinela; abortar si daemon_id ausente del índice o entrypoint/runtime inválidos. Ceguera lógica: no evaluar necesidad de arranque/parada.'
  name: Validación operativa
- delegates_to:
  - skill:shell-executor
  intent: 'Ejecutar directriz física sobre el SO mediante `skill:shell-executor`: spawn (start), lectura lock/PID (status), SIGTERM→SIGKILL (kill). Sin escalada root/sudo.'
  name: Actuación OS
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
- delegates_to:
  - action:crypto-broker
  - skill:bus-operator
  intent: Emitir instancia ECST `Process_Execution_Completed` en `eda_fractal.orchestration` con payload extendido incluyendo `os_result`, `operation`, `daemon_id`, `daemon_uuid`, `operation_status`.
  name: Sello orchestration
porcentaje_de_exito: null
uuid: 5a89793a-ba98-4b4f-9287-43c087e312df
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# governance-daemon-manager

## Directriz de ejecución obrera

Antes de ejecutar fases, el runtime IDE **debe** anteponer:

> [EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. DO NOT BYPASS EDA BUS. USE SddIA CLI.]

Actuador OS puro del ciclo de vida de **Centinelas** indexados. No interpreta necesidad de negocio; traduce `operation` + `daemon_id` en acciones físicas sobre el sistema operativo.

Invocación canónica:

```bash
```bash
./sddia-run.sh --process governance-daemon-manager --inputs '{"operation":"start","daemon_id":"telegram-watcher","repository_path":"<abs>","cumulo_topology":{...}}'
```
```

## Ceguera lógica (invariante)

| Permitido | Prohibido |
|-----------|-----------|
| Resolver rutas vía Cúmulo | Decidir si un Centinela «debe» arrancar o detenerse |
| Spawn / status / kill según `operation` | Mutar genoma (`SddIA/daemons/{name}.md`, índice) |
| Leer/escribir locks en `daemons_instance.status` | Invocar `entity-manager` o creators |
| Retornar estado OS factual | Escalar privilegios (`sudo`, root) |

## Fase 1 — Resolución SSOT

1. Resolver `directories.daemons` → `SddIA/daemons/` desde `cumulo_topology` (SSOT: `cumulo.paths.json` → `directories.daemons`).
2. Abrir `{directories.daemons}/index.md`; localizar fila cuya columna **name** coincide con `daemon_id`. Si ausente → abortar `status_code: 1`, `operation_status: failed`.
3. Leer `{directories.daemons}/{daemon_id}.md`; parsear frontmatter YAML.
4. Extraer obligatoriamente:
   - `uuid` → `daemon_uuid`
   - `execution.entrypoint` → ruta relativa al `repository_path`
   - `execution.runtime` → launcher (ej. `native-rust`, `bash`)
5. Resolver lock path: `{daemons_instance.status}/{daemon_id}.lock` (SSOT: `cumulo.daemons_instance.status`).

## Fase 2 — Validación operativa

1. Validar `operation` ∈ `{ start, status, kill }`; otro valor → abortar.
2. Delegar en **agent:cerbero** cruce RBAC: contexto del Centinela vs política del invocante.
3. Validar `execution.entrypoint` resuelto existe bajo `repository_path`.
4. **No** evaluar carga del sistema, prioridad de negocio ni estado del bus EDA.

## Fase 3 — Actuación OS

Todas las operaciones vía **skill:shell-executor** en espacio de usuario. Working directory: `repository_path`.

### `start`

1. Si `{daemon_id}.lock` existe y PID referenciado responde (`kill -0` o equivalente) → `operation_status: noop`, retornar PID existente.
2. Construir comando: `{execution.runtime} {entrypoint_resolved}` en background (`nohup` / `start_new_session` según delivery).
3. Persistir lock JSON en `daemons_instance.status` (§5 `daemons-contract.md`).
4. Poblar `os_result`: `{ "pid": <int>, "alive": true, "entrypoint_resolved": "<rel>", "runtime": "<str>", "lock_path": "<rel>" }`.

### `status`

1. Si lock ausente → `os_result.alive: false`, `operation_status: succeeded`.
2. Si lock presente: leer JSON; comprobar PID vivo en OS.
3. Poblar `os_result`: `{ "pid", "alive", "started_at", "lock_path", "heartbeat_interval_seconds" }`.

### `kill`

1. Leer lock; si ausente o PID muerto → limpiar lock huérfano, `operation_status: noop`.
2. Enviar **SIGTERM** al PID; esperar `kill_grace_seconds` (default 10).
3. Si persiste vivo → **SIGKILL**; verificar terminación.
4. Eliminar lock.
5. Poblar `os_result`: `{ "pid", "signal_sequence": ["SIGTERM","SIGKILL?"], "exit_code", "lock_removed": true }`.

Prohibidos procesos zombie: verificar que el PID no quede en estado `Z` tras kill.

## Fase 4 — Sello orchestration

Emitir **obligatoriamente** instancia ECST `Process_Execution_Completed` en `./.events/orchestration/` vía **skill:bus-operator** (`write_fractal_event`, familia `orchestration`).

### Envelope mínimo

```json
{
  "event_id": "<uuid-v4>",
  "event_type": "Process_Execution_Completed",
  "timestamp": "<ISO-8601 UTC>",
  "emitter_agent": "governance-daemon-manager",
  "payload": {
    "process_name": "governance-daemon-manager",
    "asset_id": "<uuid-v4 del workspace o invocación>",
    "workspace_path": "<rel o abs inyectado>",
    "status": "<succeeded|failed>",
    "operation": "<start|status|kill>",
    "daemon_id": "<kebab-case>",
    "daemon_uuid": "<uuid-v4>",
    "operation_status": "<succeeded|failed|noop>",
    "os_result": { }
  }
}
```

Campos `process_name`, `asset_id`, `workspace_path`, `status` son **REQUIRED** según Clase `process-execution-completed`. Los campos `operation`, `daemon_id`, `daemon_uuid`, `operation_status`, `os_result` son extensión CEN-02 auditable (payload OPTIONAL permitido).

Emisión **incondicional** al cierre de Fase 3 (éxito o fallo controlado). En abort temprano (Fase 1–2), emitir con `status: failed` y `os_result.error`.

Propagar `orchestration_event_id` y `orchestration_event_path` a outputs del proceso.

## Handler laboratorio

Implementación física: handler nativo `handlers::governance_daemon` en `execute-process` cuando `process_name == governance-daemon-manager`.

## Límites

* No sustituye al Centinela en emisión `Daemon_Heartbeat`.
* No modifica definiciones bajo `directories.daemons`.
* Kill-Switch global del Core (CEN-03) invocará este proceso en iteración sobre índice.

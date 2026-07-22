---
context:
- system-operations
- ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:08c4c7d6316441f36fdcdc1a87a136820b69c8c312d063bfd6d02e498b1056bf
inputs:
- kill_grace_seconds: Opcional; segundos SIGTERM→SIGKILL por Centinela (default `10`)
- cumulo_topology: Topología SSOT inyectada
- repository_path: Ruta absoluta del workspace raíz
- trigger: 'Opcional: `manual` | `sigterm` | `sigint` | `atexit` — origen de la purga'
minteo_maximo: null
name: daemon-kill-switch
outputs:
- purge_report: Array por daemon_id con resultado kill/noop
- stale_locks_removed: Lista de locks huérfanos eliminados
- orchestration_event_id: UUID v4 del Process_Execution_Completed
- orchestration_event_path: Ruta relativa en eda_fractal.orchestration
phases:
- delegates_to:
  - agent:cumulo
  intent: Listar daemon_id desde `{directories.daemons}/*.md` excluyendo contrato e índice; vía cumulo.
  name: Enumeración índice
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
- delegates_to:
  - action:execute-process
  intent: 'Por cada Centinela con lock/PID vivo: delegar kill en governance-daemon-manager (SIGTERM→SIGKILL).'
  name: Purga iterativa
- intent: Eliminar locks stale; verificar ausencia de PIDs zombie bajo daemons_instance.status.
  name: Verificación huérfanos
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
- delegates_to:
  - skill:bus-operator
  intent: Emitir Process_Execution_Completed con purge_report y stale_locks_removed.
  name: Sello orchestration
porcentaje_de_exito: null
uuid: b0de6585-11fc-4b3c-8b19-ad6b727d820e
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# daemon-kill-switch

Kill-Switch global (CEN-03). Al apagar el motor SddIA (`SIGTERM`, `SIGINT`, `atexit`) o invocación manual, purga **todos** los Centinelas indexados sin procesos huérfanos.

```bash
./sddia-run.sh --process daemon-kill-switch --inputs '{"repository_path":"<abs>"}'
```

## Purga manual

Invocación explícita vía `./sddia-run.sh --process daemon-kill-switch` o `--process governance-daemon-manager` con `operation: kill`. No depende de intérprete Python.

## Fase 1 — Enumeración índice

1. Resolver `directories.daemons` vía cumulo.
2. Listar `{name}.md` excluyendo `daemons-contract.md` e `index.md`.
3. Orden determinista lexicográfico por `daemon_id`.

## Fase 2 — Purga iterativa

Por cada `daemon_id`:

1. Leer lock en `daemons_instance.status/{daemon_id}.lock`.
2. Si PID vivo → invocar `governance-daemon-manager` con `operation: kill`.
3. Si lock stale → omitir kill OS; limpiar lock en Fase 3.

## Fase 3 — Verificación huérfanos

1. Escanear `daemons_instance.status/*.lock`.
2. Eliminar locks cuyo PID no responde.
3. Prohibido dejar procesos zombie (verificar post-SIGKILL).

## Fase 4 — Sello orchestration

Emitir `Process_Execution_Completed` con `purge_report[]` y `stale_locks_removed[]`.

## Handler laboratorio

Handler laboratorio: módulo nativo `handlers::daemon_kill_switch` en `execute-process`.

## Límites

* No arranca Centinelas.
* No muta definiciones en `directories.daemons`.
* Espacio de usuario; sin sudo/root.

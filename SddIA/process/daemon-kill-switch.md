---
uuid: "b0de6585-11fc-4b3c-8b19-ad6b727d820e"
name: "daemon-kill-switch"
version: "1.0.0"
contract: "process-contract v1.4.0"
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
  - "system-operations"
  - "ecosystem-evolution"
hash_signature: sha256:1a50df9c5c859f9f7ecd34ecdf13c7cc5586536fb89b810053023895a45a2f62
inputs:
  - "kill_grace_seconds": "Opcional; segundos SIGTERM→SIGKILL por Centinela (default `10`)"
  - "cumulo_topology": "Topología SSOT inyectada"
  - "repository_path": "Ruta absoluta del workspace raíz"
  - "trigger": "Opcional: `manual` | `sigterm` | `sigint` | `atexit` — origen de la purga"
outputs:
  - "purge_report": "Array por daemon_id con resultado kill/noop"
  - "stale_locks_removed": "Lista de locks huérfanos eliminados"
  - "orchestration_event_id": "UUID v4 del Process_Execution_Completed"
  - "orchestration_event_path": "Ruta relativa en eda_fractal.orchestration"
phases:
  - name: "Enumeración índice"
    intent: "Listar daemon_id desde `{directories.daemons}/*.md` excluyendo contrato e índice; vía cumulo."
    delegates_to:
      - "agent:cumulo"
      - "skill:filesystem-manager"
  - name: "Purga iterativa"
    intent: "Por cada Centinela con lock/PID vivo: delegar kill en governance-daemon-manager (SIGTERM→SIGKILL)."
    delegates_to:
      - "action:execute-process"
  - name: "Verificación huérfanos"
    intent: "Eliminar locks stale; verificar ausencia de PIDs zombie bajo daemons_instance.status."
    delegates_to:
      - "skill:filesystem-manager"
  - name: "Sello orchestration"
    intent: "Emitir Process_Execution_Completed con purge_report y stale_locks_removed."
    delegates_to:
      - "skill:bus-operator"
minteo_maximo: null
porcentaje_de_exito: null
---

# daemon-kill-switch

Kill-Switch global (CEN-03). Al apagar el motor SddIA (`SIGTERM`, `SIGINT`, `atexit`) o invocación manual, purga **todos** los Centinelas indexados sin procesos huérfanos.

```bash
python3 SddIA/scripts/qa/execute-process.py --process daemon-kill-switch --inputs '{"repository_path":"<abs>"}'
```

## Registro de hooks

El CLI `execute-process.py` registra `register_kill_switch_hooks(repo)` al arrancar. Ante señal de terminación del Core, ejecuta purga antes de salir.

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

`daemon_kill_switch_core.run_daemon_kill_switch()` — invocado desde `execute-process.py`.

## Límites

* No arranca Centinelas.
* No muta definiciones en `directories.daemons`.
* Espacio de usuario; sin sudo/root.

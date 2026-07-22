---
uuid: 608ae470-4db2-4ae6-8bb8-7aa5949c208a
name: task-queue-manager
version: 1.1.0
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
aliases:
- automatic_task
context:
- ecosystem-evolution
- filesystem-ops
hash_signature: sha256:2ced2d81886d826b739bfb9c39f03f0e7492352d1602b30e276d75d3e7439de1
inputs:
- tasks_path: Raíz de cola de tareas resuelta vía Cumulo
outputs:
- processed_task_ref: Referencia de la tarea despachada y cerrada
- evolution_entry: Entrada opcional en evolution/
phases:
- name: Triaje
  intent: Leer raíz de cola, priorizar y aplicar marcas KAIZEN según convención local.
  requires_capability:
  - id: fs:persist
    contract: fs.persist
    version: '>=1.0.0'
- name: Activación
  intent: Promover tarea a ACTIVE/ y snapshot git inicial.
  delegates_to:
  - skill:filesystem-manager
  - skill:git-manager
- name: Despacho
  intent: Según trigger semántico, invocar feature | bug-fix | refactorization vía
    execute-process.
  delegates_to:
  - action:execute-process
  - agent:tekton
- name: Finalización FS
  intent: Mover tarea a DONE/ (persistencia ciega).
  requires_capability:
  - id: fs:persist
    contract: fs.persist
    version: '>=1.0.0'
- name: Finalización Git
  intent: Consolidar estado en git (sync ciego).
  requires_capability:
  - id: proc:git-sync
    contract: proc.git_sync
    version: '>=1.0.0'
minteo_maximo: null
porcentaje_de_exito: null
---

# task-queue-manager

Meta-orquestación de **cola de tareas** para el Core SddIA. Expone el alias canónico legacy **`automatic_task`** (`process-contract v1.4.0`) hacia el mismo archivo físico.

## Identidad

* **Nombre soberano:** `task-queue-manager`
* **Alias:** `automatic_task` (resolución previa a ruta física por Cúmulo)

## Despacho

La fase **Despacho** debe clasificar la tarea (feature vs corrección vs refactor) antes de fijar el `process_name` pasado a **`action:execute-process`**, siempre resolviendo identidad canónica.

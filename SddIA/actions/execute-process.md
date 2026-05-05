---
uuid: "f1e2d3c4-b5a6-4789-b012-cdef34567890"
name: "execute-process"
version: "1.0.0"
contract: "actions-contract v1.1.0"
context: "ecosystem-evolution"
capabilities:
  - "process-load-ssot"
  - "phase-graph-resolution"
  - "cerbero-policy-gate"
  - "capsule-fanout-skills-tools"
inputs:
  - "process_name": "Identificador kebab-case del proceso a ejecutar (definición bajo `directories.process` en `cumulo.paths.json`)"
  - "process_inputs": "JSON con los parámetros exigidos por el proceso destino"
outputs:
  - "execution_report": "JSON agregado: fases recorridas, cápsulas previstas, resultados intermedios y cierre"
  - "status_code": "0 si todas las fases completaron; 1 si Cerbero bloquea, falla topología cumulo o error fatal de fase"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: execute-process

## 1. Propósito
Orquestar de extremo a extremo la ejecución autónoma de un **proceso** del Core SddIA: resolver su ley vía Cúmulo, recorrer sus fases con Tekton, someter cada intención ejecutiva a Cerbero y delegar la física exclusivamente en **skills** o **tools** autorizados.

## 2. Orquestación lógica (sin terminal nativa)

1. **Carga de ley (Cúmulo):** Tekton solicita al agente **Cúmulo** (o al manifiesto SSOT `cumulo.paths.json` inyectado en su `cumulo_topology`) la ruta canónica de `directories.process` y compone la ruta física del archivo `{process_name}.md`. Si el proceso no existe o el mapa es inválido, emitir `status_code: 1` y detener.
2. **Iteración de fases (Tekton):** Tekton lee el YAML y el cuerpo del proceso, materializa la lista ordenada de fases y, por cada fase, identifica qué **actions**, **skills** o **tools** deben invocarse según la ley del proceso. Resuelve siempre rutas y contratos vía claves cumulo (`directories.actions`, `directories.skills`, `directories.tools`, `execution_capsules`), nunca por rutas absolutas inventadas.
3. **Paso por el centinela (Cerbero):** Antes de cada ejecución física de cápsula, Tekton presenta a **Cerbero** la intención (`entity_request` con políticas de Tekton, `target_capsule` con el `context` de la cápsula). Cerbero cruza contra `execution-contexts.md` (ruta vía `directories.norms`). Si el `context` de la actividad **no** está en `allowed_policies` de Tekton, Cerbero responde `exitCode: 1`: abortar la acción y devolver `status_code: 1` con causa en `execution_report`.
4. **Ejecución y cierre:** Tras autorización, se emite la invocación estructurada (`skill_invocations` u homólogo para tools/actions hijas), se registra el resultado en `execution_report` y se avanza a la siguiente fase. Al completar la última fase sin abortos, `status_code: 0`.

## 3. Límites y referencias de agentes
* **Tekton:** definición operativa bajo `paths.directories.agents` — prohibición explícita de terminal nativa; obediencia a fases y topología inyectada.
* **Cerbero:** definición operativa bajo `paths.directories.agents` — intercepción pura; bloqueo binario si el contexto de la cápsula no está permitido para la entidad solicitante.
* **Contrato de acciones:** `paths.contracts.actions` — sin acceso directo al SO; solo delegación en skills/tools con ruteo cumulo.

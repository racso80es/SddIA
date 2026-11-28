---
context:
- chaos-engineering
- quality-assurance
- ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: "sha256:b02d2b43a9cbd77235f66224edf660362ce5ccdb7a34f279838bf27038b355f2"
inputs:
- suite_id: Identificador kebab-case de la Suite (required)
- execution_strategy: Override opcional fail_fast | run_all
minteo_maximo: null
name: execute-suite
outputs:
- survival_manifest_path: Ruta relativa al manifiesto compilado
- nodes_executed: Conteo de nodos ejecutados
phases:
- delegates_to:
  - agent:cumulo
  intent: Cargar spec Suite desde Cúmulo/directories.suites.
  name: Resolución Suite
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
  id: resolucion_suite
- delegates_to:
  - agent:tekton
  - action:execute-process
  intent: Por cada atomic_node, subproceso execute-process aislado.
  name: Orquestación nodos
  id: orquestacion_nodos
- delegates_to:
  - agent:argos
  intent: Argos escribe survival-manifest.md en workspace orquestador.
  name: Compilación manifiesto
  id: compilacion_manifiesto
- delegates_to:
  - agent:radamanto
  intent: Tras éxito global y manifiesto, emitir System_Immunity_Certified en bus domain.
  name: Certificación inmunidad
  id: certificacion_inmunidad
porcentaje_de_exito: null
uuid: a1b2c3d4-e5f6-4789-a012-3456789abcde
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# execute-suite

## Propósito

Orquestador de **Suites Caos** (Fase 3–4): resuelve `suite_id`, ejecuta nodos atómicos con sub-workspaces aislados, compila `survival-manifest.md` (D0.7) y, si la campaña es exitosa, emite `System_Immunity_Certified` (Fase 4).

## Operación

Invocación vía `./sddia-run.sh --process <name>`; fases en frontmatter YAML.

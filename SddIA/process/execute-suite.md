---
context:
- chaos-engineering
- quality-assurance
- ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:401b43cd1efc847636666bd8401855d114240baf9c4e7bf5e74dd2889cedc406
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
- delegates_to:
  - agent:tekton
  - action:execute-process
  intent: Por cada atomic_node, subproceso execute-process aislado.
  name: Orquestación nodos
- delegates_to:
  - agent:argos
  intent: Argos escribe survival-manifest.md en workspace orquestador.
  name: Compilación manifiesto
- delegates_to:
  - agent:radamanto
  intent: Tras éxito global y manifiesto, emitir System_Immunity_Certified en bus domain.
  name: Certificación inmunidad
porcentaje_de_exito: null
uuid: a1b2c3d4-e5f6-4789-a012-3456789abcde
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# execute-suite

Orquestador de **Suites Caos** (Fase 3–4): resuelve `suite_id`, ejecuta nodos atómicos con sub-workspaces aislados, compila `survival-manifest.md` (D0.7) y, si la campaña es exitosa, emite `System_Immunity_Certified` (Fase 4).

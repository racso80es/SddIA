---
uuid: "a1b2c3d4-e5f6-4789-a012-3456789abcde"
name: execute-suite
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- chaos-engineering
- quality-assurance
- ecosystem-evolution
hash_signature: sha256:64c697cf966ef458d5e7d4bc063ad0e1b2587ec90476cde53adfb87128429167
inputs:
- suite_id: Identificador kebab-case de la Suite (required)
- execution_strategy: Override opcional fail_fast | run_all
outputs:
- survival_manifest_path: Ruta relativa al manifiesto compilado
- nodes_executed: Conteo de nodos ejecutados
phases:
- name: Resolución Suite
  intent: Cargar spec Suite desde Cúmulo/directories.suites.
  delegates_to:
  - agent:cumulo
- name: Orquestación nodos
  intent: Por cada atomic_node, subproceso execute-process aislado.
  delegates_to:
  - agent:tekton
  - action:execute-process
- name: Compilación manifiesto
  intent: Argos escribe survival-manifest.md en workspace orquestador.
  delegates_to:
  - agent:argos
minteo_maximo: null
porcentaje_de_exito: null
---

# execute-suite

Orquestador de **Suites Caos** (Fase 3): resuelve `suite_id`, ejecuta nodos atómicos con sub-workspaces aislados y compila `survival-manifest.md` (D0.7).

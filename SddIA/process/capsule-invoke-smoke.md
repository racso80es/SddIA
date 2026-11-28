---
context:
- quality-assurance
contract: process-contract v1.4.0
hash_signature: "sha256:6b61755dd588327bfcec63cc567ce2d68082c6e80b264a0f85d8a6dff819d6f3"
inputs: []
minteo_maximo: null
name: capsule-invoke-smoke
outputs:
- capsule_invoked: Indica que io-choke respondió con éxito en el workspace inyectado
phases:
- delegates_to:
  - tool:io-choke
  intent: Invocar tool:io-choke sobre workspace_path inyectado (golden D-P5.2).
  name: Invocación io-choke
  requires_capability:
  - contract: qa.probe
    id: qa:probe
    version: '>=1.0.0'
  id: invocacion_io_choke
porcentaje_de_exito: null
uuid: f3a8c2d1-9e4b-4a7c-b6d5-1e0f9a8b7c6d
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# capsule-invoke-smoke

## Propósito

Golden D-P5.2: validar que `execute-process` materializa el workspace del proceso y ejecuta la fase que delega en `tool:io-choke` con payload `workspace_path` + `target_file: .capsule-smoke-target`, propagando el resultado de la cápsula en `phase_reports` y el booleano `capsule_invoked`.

## Orquestación (handler nativo)

1. `validate_process_inputs` contra el frontmatter vigente; exige `workspace_template` (por defecto `.SddIA/workspaces/{process_name}/{execution_id}/`).
2. `bootstrap_workspace` inyecta `workspace_path` en el estado de orquestación.
3. Para la fase `Invocación io-choke`, `invoke_tool(repo, "io-choke", …)` — implementación en `SddIA/engine/execute-process/src/engine/capsule_invoke_smoke.rs`.
4. Fases sin delegación explícita quedan `simulated`; el veredicto terminal agrega `phase_reports` vía `aggregate_execution_terminal`.

## Entradas y salidas

- Entradas declaradas: ninguna (`inputs: []`); el workspace es el único contexto inyectado.
- Salida de negocio: `capsule_invoked` (éxito de la invocación io-choke).

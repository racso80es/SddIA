---
uuid: "e7d5087c-6d47-4890-9602-34962496b3bb"
name: "agent-creator"
version: "1.0.0"
contract: "process-contract v1.1.0"
context: "ecosystem-evolution"
hash_signature: "sha256:c55caadddcfa03813059ebc6d40a76f17bd150b49148699d6b1e68ddbb231fb9"
inputs:
  - "agent_name": "Identificador kebab-case del agente (`{name}` del archivo de definición bajo `cumulo.directories.agents`)"
  - "allowed_policies": "Array de contextos S+ Grade (identificadores de `execution-contexts.md`) que el agente puede solicitar ante Cerbero"
  - "agent_inputs": "Definición estandarizada de datos requeridos para iniciar el ciclo operativo del agente"
  - "agent_outputs": "Artefactos o estructuras esperadas al cierre (p. ej. Markdown, JSON, delegación a cápsulas)"
  - "agent_purpose": "Lógica operativa, jurisdicción y límites de la identidad (prosa normativa y flujo)"
outputs:
  - "artifact_agent_md": "Archivo `{paths.directories.agents}/{agent_name}.md` con cabecera YAML conforme a `paths.contracts.agents` (incl. `allowed_policies`)"
  - "artifact_agents_index": "`{paths.directories.agents}/index.md` creado o actualizado con fila que expone `allowed_policies` para lectura rápida de Cerbero"
phases:
  - name: "Auditoría de Gobernanza"
    intent: "Validar estrictamente que todas las políticas solicitadas en allowed_policies existen en la normativa vigente dictada por Cerbero."
    delegates_to:
      - "agent:cumulo"
      - "agent:cerbero"
      - "action:policy-validator"
  - name: "Forja de Identidad"
    intent: "Generar uuid v4 y cabecera YAML (contract, allowed_policies, inputs, outputs, hash_signature si aplica) según agents-contract v1.0.0; cuerpo con agent_purpose bajo directories.agents; rutas solo vía cumulo."
    delegates_to:
      - "skill:cryptography-manager"
      - "skill:filesystem-manager"
  - name: "Indexación de Soberanía"
    intent: "Actualizar el catálogo de Agentes exponiendo explícitamente sus allowed_policies para lectura rápida de Cerbero."
    delegates_to:
      - "agent:cumulo"
      - "skill:filesystem-manager"
minteo_maximo: null
porcentaje_de_exito: null
---

# agent-creator

Proceso maestro para instanciar nuevas identidades operativas (Agentes) en el Core SddIA, validar políticas contra la matriz Cerbero, materializar la definición bajo `agents-contract` y mantener el índice soberano del directorio `agents`.

## Fase 1 — Auditoría de Gobernanza

1. Resolver `execution-contexts.md` y topología vía `agent:cumulo` (`paths.directories.norms`, `cumulo.paths.json`); auditar unicidad y kebab-case de `agent_name` bajo `directories.agents`.
2. Con `agent:cerbero`, verificar coherencia RBAC: cada valor de `allowed_policies` debe existir como contexto S+ en la matriz (`source-control`, `filesystem-ops`, `knowledge-management`, `quality-assurance`, `ecosystem-evolution`).
3. Delegar en `action:policy-validator` el dictamen normativo cruzado; abortar con causa auditable si falta algún contexto o hay divergencia con la normativa vigente.
4. Registrar el dictamen para la fase de forja; no avanzar ante bloqueo de Cerbero o policy-validator.

## Fase 2 — Forja de Identidad

1. Resolver rutas exclusivamente vía `cumulo.paths.json` (`directories.agents`, `contracts.agents`).
2. Invocar `skill:cryptography-manager` (cápsula bajo `paths.execution_capsules.skills`) con `{"operation":"GENERATE_UUID","target_payload":null}` por stdin; usar `data.result` como `uuid` v4 del nuevo agente. Prohibido UUID por heurística o código ad hoc.
3. Si `agents-contract` o la gobernanza del repositorio exigen `hash_signature` sobre un bloque canónico del agente, calcularlo exclusivamente con `GENERATE_SHA256` y `target_type` `STRING` o `FILE_PATH` según política; prefijo `sha256:` + `data.result`.
4. Asignar `contract` como `agents-contract v1.0.0`, `version` SemVer, `allowed_policies` idéntico al validado en Fase 1, `inputs` / `outputs` según parámetros del proceso.
5. Persistir `{paths.directories.agents}/{agent_name}.md` con cuerpo que documente `agent_purpose`, obediencia al SSOT (sin rutas hardcodeadas al host) y límites termodinámicos opcionales (`minteo_maximo`, `porcentaje_de_exito`) si aplican, mediante `skill:filesystem-manager` tras autorización Cerbero.

## Fase 3 — Indexación de Soberanía

1. Con `agent:cumulo`, auditar o crear `{paths.directories.agents}/index.md` con tabla de catálogo que incluya columna explícita **allowed_policies** (lectura rápida para Cerbero).
2. Insertar o actualizar la fila del agente creado alineada a la cabecera YAML (`uuid`, `name`, `version`, `contract`, políticas).
3. Ejecutar verificación cruzada índice ↔ `.md` fuente sin divergencia; persistir cambios vía `skill:filesystem-manager`.

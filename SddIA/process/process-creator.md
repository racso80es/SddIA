---
uuid: "7c2d9e41-88a3-4f6b-9c12-4def01a2b3c4"
name: "process-creator"
version: "1.0.0"
contract: "process-contract v1.1.0"
context: "ecosystem-evolution"
hash_signature: "sha256:17e0e599a8b9e85bf2eff64de5bfd0455ca100b61246e561fea2f32ff0981083"
inputs:
  - "process_name": "Identificador kebab-case del proceso (`{name}` del archivo `{name}.md`)"
  - "process_description": "Descripción operativa del propósito del proceso"
  - "process_context": "Contexto de ejecución válido según `execution-contexts.md`"
  - "process_phases": "Listado inmutable de fases (orden, id y criterio de salida por fase)"
  - "process_contract_version": "Versión del contrato process a materializar (ej. 1.0.0)"
outputs:
  - "artifact_process_md": "Archivo `{paths.directories.process}/{process_name}.md` generado con cabecera YAML conforme a contrato"
  - "artifact_process_index": "`{paths.directories.process}/index.md` actualizado con fila alineada a la cabecera del nuevo proceso"
phases:
  - name: "Validación de inputs"
    intent: "Comprobar unicidad y kebab-case de process_name, process_context en execution-contexts, SemVer y exhaustividad de inputs/phases/outputs."
    delegates_to:
      - "agent:cumulo"
      - "agent:cerbero"
  - name: "Forja del archivo"
    intent: "Generar uuid v4; calcular hash_signature canónico del array phases (JSON UTF-8, sort_keys); redactar YAML y cuerpo conforme a process-contract; persistir bajo directories.process."
    delegates_to:
      - "skill:cryptography-manager"
      - "skill:filesystem-manager"
  - name: "Auditoría y actualización del índice"
    intent: "Verificar process/index.md y fila Name|UUID|Versión|Context|Descripción alineada al YAML fuente."
    delegates_to:
      - "agent:cumulo"
      - "skill:filesystem-manager"
minteo_maximo: null
porcentaje_de_exito: null
---

# process-creator

Proceso maestro para instanciar nuevos procesos en el Core SddIA y mantener el índice del directorio `process`.

## Fase 1 — Validación de inputs

1. Verificar que `process_name` sea único en `{paths.directories.process}` y cumpla kebab-case.
2. Resolver `process_context` contra `SddIA/norms/execution-contexts.md`; abortar si no existe en la matriz S+.
3. Validar `process_phases` como secuencia ordenada sin saltos lógicos y con criterios de salida explícitos.
4. Confirmar `process_contract_version` alineada con `process-contract.md` vigente en `cumulo.contracts.process`.

## Fase 2 — Forja del archivo

1. Invocar `skill:cryptography-manager` (cápsula bajo `paths.execution_capsules.skills`) con un único JSON por stdin: `{"operation":"GENERATE_UUID","target_payload":null}`; tomar `data.result` como `uuid` v4 del proceso hijo (no reutilizar el de process-creator). Prohibido generar UUID por heurística o código ad hoc.
2. Serializar el array `phases` acordado a JSON UTF-8 canónico (objetos con `name`, `intent`, `delegates_to`; `json.dumps(..., sort_keys=True, separators=(',', ':'), ensure_ascii=False)`). Invocar `skill:cryptography-manager` con `{"operation":"GENERATE_SHA256","target_type":"STRING","target_payload":"<cadena canónica>"}`; asignar `hash_signature` como `sha256:` + `data.result` (hex minúsculas). Prohibido calcular SHA-256 fuera de esta cápsula.
3. Escribir `{paths.directories.process}/{process_name}.md` con cabecera YAML (uuid, name, version, contract, context, hash_signature, inputs, phases, outputs, métricas si aplican) y cuerpo que describa cada fase en prosa operativa.
4. Leer rutas físicas solo vía `cumulo.paths.json` (`directories.process`, `contracts.process`, `directories.norms`).

## Fase 3 — Auditoría y actualización del índice

1. Abrir `{paths.directories.process}/index.md` y comprobar encabezados de tabla exigidos por Gobernanza de Índices.
2. Insertar o actualizar la fila del proceso creado: **Name**, **UUID**, **Versión**, **Context**, **Descripción** copiados literalmente del YAML y la descripción operativa.
3. Verificación cruzada: cero divergencia entre fila del índice y cabecera del `.md` fuente.

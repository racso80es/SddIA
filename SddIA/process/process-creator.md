---
uuid: "7c2d9e41-88a3-4f6b-9c12-4def01a2b3c4"
name: "process-creator"
version: "1.0.0"
contract: "process-contract v1.3.0"
context: "ecosystem-evolution"
hash_signature: "sha256:b0b74db50d849219c315cd934ec299750bb87791a17fd3abed89bfccf4652730"
inputs:
  - "process_name": "Identificador kebab-case del proceso (`{name}` del archivo `{name}.md`)"
  - "process_description": "Descripción operativa del propósito del proceso"
  - "process_context": "Contexto de ejecución válido según `execution-contexts.md`"
  - "process_phases": "Listado inmutable de fases (orden, id y criterio de salida por fase)"
  - "process_contract_version": "Versión del contrato process a materializar (ej. 1.0.0)"
  - "process_aliases": "Opcional; array de strings kebab-case — aliases de identidad adicionales (v1.3.0); vacío si no aplica"
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
      - "action:crypto-broker"
      - "skill:filesystem-manager"
  - name: "Auditoría y actualización del índice"
    intent: "Verificar process/index.md y fila Name|UUID|Versión|Context|Descripción alineada al YAML fuente."
    delegates_to:
      - "agent:cumulo"
      - "skill:filesystem-manager"
phase_invocations:
  - phase_name: "Forja del archivo"
    invocations:
      - capsule: "action:crypto-broker"
        stdin_json:
          operation: "GENERATE_UUID"
          target_payload: null
        bind:
          "data.result": "child_process_uuid"
        on_error: abort
      - capsule: "action:crypto-broker"
        stdin_spec:
          operation: "GENERATE_SHA256"
          target_type: "STRING"
          target_payload:
            type: "canonical_json_utf8"
            from_process_input: "process_phases"
            json_dumps:
              sort_keys: true
              separators: [",", ":"]
              ensure_ascii: false
        bind:
          "data.result": "child_phases_sha256_hex"
        on_error: abort
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
5. Si `process_aliases` está presente y no vacío: validar cada elemento en **kebab-case**; prohibir duplicados internos; prohibir que cualquier alias sea igual al `process_name` canónico; delegar en **agent:cumulo** la comprobación de **no colisión** con los `name` y `aliases` ya catalogados bajo `directories.process` (mapa de identidad derivado). Si Cúmulo reporta colisión o formato inválido, abortar sin escribir artefactos.

## Fase 2 — Forja del archivo

1. Ejecutar `phase_invocations` de esta fase: `action:crypto-broker` emite UUID v4 (`child_process_uuid`) y digest SHA-256 del JSON canónico de `process_phases` (`child_phases_sha256_hex`). Prefijo de cabecera: `hash_signature: "sha256:" + child_phases_sha256_hex`. Prohibido UUID o digest fuera del broker + cápsula.
2. Escribir `{paths.directories.process}/{process_name}.md` con cabecera YAML (uuid, name, version, contract, context, hash_signature, inputs, phases, outputs, métricas si aplican; si `process_aliases` es no vacío tras validación, persistir **`aliases`** en el YAML alineado a `process-contract v1.3.0`) y cuerpo que describa cada fase en prosa operativa.
3. Leer rutas físicas solo vía `cumulo.paths.json` (`directories.process`, `contracts.process`, `directories.norms`).

## Fase 3 — Auditoría y actualización del índice

1. Abrir `{paths.directories.process}/index.md` y comprobar encabezados de tabla exigidos por Gobernanza de Índices.
2. Insertar o actualizar la fila del proceso creado: **Name**, **UUID**, **Versión**, **Context**, **Aliases** (vacío o lista si procede), **Descripción** copiados literalmente del YAML y la descripción operativa.
3. Verificación cruzada: cero divergencia entre fila del índice y cabecera del `.md` fuente.

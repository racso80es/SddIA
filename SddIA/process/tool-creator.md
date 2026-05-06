---
uuid: "c4355159-b6ea-4201-973a-a08db5ce8156"
name: "tool-creator"
version: "1.0.0"
contract: "process-contract v1.2.0"
context: "ecosystem-evolution"
hash_signature: "sha256:22b15bb2204ae9b1f5cbabc5734d646010108340b1ec95e620c44558850bac5e"
inputs:
  - "tool_name": "Identificador kebab-case de la tool (`{name}` del archivo `{name}.md` bajo `cumulo.directories.tools`)"
  - "domain_origin": "Proyecto o contexto de dominio al que pertenece la tool (alineado a `domain_origin` del contrato de tools)"
  - "tool_context": "Contexto RBAC Cerbero (debe existir en `execution-contexts.md`)"
  - "required_secrets": "Variables de entorno requeridas; lista o mapa auditable para cruce con el Vértice Biológico"
  - "dependencies": "Librerías externas necesarias para la ejecución de la cápsula"
  - "tool_outputs": "Definición de salidas esperadas (logs, códigos, artefactos) interpretables por acciones"
  - "execution_logic": "Especificación de la lógica ejecutable (cuerpo de script, puntos de entrada y contrato de invocación)"
outputs:
  - "artifact_tool_md": "Archivo `{paths.directories.tools}/{tool_name}.md` con cabecera YAML conforme a `paths.contracts.tools`"
  - "artifact_tool_script": "Script físico bajo `{paths.execution_capsules.tools}` cuando `execution_logic` exija cápsula ejecutable"
  - "artifact_tools_index": "`{paths.directories.tools}/index.md` actualizado con fila de catálogo (columna Capabilities obligatoria)"
phases:
  - name: "Validación de Dominio y Secretos"
    intent: "Verificar que el context exists, el domain_origin es válido y auditar si los required_secrets están declarados en el entorno del Vértice Biológico."
    delegates_to:
      - "action:policy-validator"
      - "skill:environment-reader"
  - name: "Resolución de Dependencias"
    intent: "Verificar si el proyecto destino cuenta con las librerías necesarias o si requieren inyección."
    delegates_to:
      - "skill:dependency-auditor"
  - name: "Forja del Contrato y Cápsula"
    intent: "Generar el archivo Markdown bajo contrato v1.0.0 y el script físico en el domain_origin."
    delegates_to:
      - "action:crypto-broker"
      - "skill:filesystem-manager"
  - name: "Indexación"
    intent: "Actualizar el catálogo de Tools con la nueva entidad exponiendo sus capabilities."
    delegates_to:
      - "skill:filesystem-manager"
phase_invocations:
  - phase_name: "Forja del Contrato y Cápsula"
    invocations:
      - capsule: "action:crypto-broker"
        stdin_json:
          operation: "GENERATE_UUID"
          target_payload: null
        bind:
          "data.result": "child_tool_uuid"
        on_error: abort
minteo_maximo: null
porcentaje_de_exito: null
---

# tool-creator

Proceso maestro para instanciar herramientas de dominio (Tools) complejas en el Core SddIA: validar política y secretos, auditar dependencias, materializar definición Markdown y cápsula bajo cumulo, y registrar el catálogo.

## Fase 1 — Validación de Dominio y Secretos

1. Presentar a **Cerbero** la intención de invocación hacia `action:policy-validator` y `skill:environment-reader` con `context` coherente con `tool_context` y políticas de Tekton para `ecosystem-evolution`.
2. Verificar que `tool_context` existe en la matriz S+ de `execution-contexts.md` (`paths.directories.norms`).
3. Validar `domain_origin` como declaración de proyecto coherente con el contrato de tools (origen de dominio explícito, sin rutas absolutas inventadas).
4. Auditar `required_secrets` contra el entorno del Vértice Biológico vía `skill:environment-reader`; registrar hallazgos para la fase siguiente.

## Fase 2 — Resolución de Dependencias

1. Delegar en `skill:dependency-auditor` el inventario de `dependencies` frente al árbol del `domain_origin`.
2. Emitir dictamen: dependencias satisfechas, requieren inyección, o bloqueo documentado con causa.

## Fase 3 — Forja del Contrato y Cápsula

1. Ejecutar `phase_invocations`: obtener `child_tool_uuid` vía `action:crypto-broker` (`GENERATE_UUID`); incluir en cabecera YAML de la tool.
2. Completar metadatos de tool según `tools-contract.md` (`paths.contracts.tools`); alinear versión de contrato declarada (p. ej. materialización bajo especificación vigente; la fase prevé generación compatible con baseline v1.0.0 donde aplique).
3. Redactar `{paths.directories.tools}/{tool_name}.md` con `capabilities`, `inputs`, `outputs` y vínculo a cápsula bajo `paths.execution_capsules.tools` si corresponde.
4. Persistir script físico o esqueleto ejecutable en la ruta resuelta por cumulo, sin escritura fuera del SSOT.

## Fase 4 — Indexación

1. Abrir `{paths.directories.tools}/index.md` y localizar la tabla de catálogo (columna **Capabilities** obligatoria).
2. Insertar o actualizar la fila asociada a `{tool_name}.md` copiando `uuid`, `name`, `version`, `contract`, `context` y `capabilities` desde el YAML fuente.
3. Verificación cruzada índice ↔ cabecera antes de cerrar la instancia del proceso.

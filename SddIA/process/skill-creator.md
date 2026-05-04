---
uuid: "b8c3d1e2-f4a5-4a6b-8c7d-0e1f2a3b4c5d"
name: "skill-creator"
version: "1.0.0"
contract: "process-contract v1.1.0"
context: "ecosystem-evolution"
hash_signature: "sha256:c75699b4ce67f188b7921005bf08bdbc9ea12b01877b9716fa16d1cc2e469811"
inputs:
  - "skill_name": "Identificador kebab-case de la skill (`{name}` del archivo `{name}.md` en `cumulo.directories.skills`)"
  - "skill_context": "Contexto RBAC Cerbero (debe existir en `execution-contexts.md`)"
  - "skill_description": "Descripción operativa de la capacidad y límites de la skill"
  - "skill_inputs_schema": "Definición de entradas esperadas (alineada a I/O JSON estricto del contrato de skills)"
  - "skill_outputs_schema": "Definición de salidas esperadas (`success`, `exitCode`, `data` o `error` según contrato)"
  - "skill_version": "SemVer de la nueva skill (ej. 1.0.0)"
  - "skills_contract_version": "Versión del contrato skills a materializar (ej. 1.0.0 según `skills-contract.md`)"
outputs:
  - "artifact_skill_md": "Archivo `SddIA/skills/{skill_name}.md` con cabecera YAML conforme a `skills-contract.md`"
  - "artifact_skills_index": "`SddIA/skills/index.md` actualizado con fila sincronizada a la cabecera YAML de la skill"
phases:
  - name: "Validación RBAC y topología"
    intent: "Verificar skill_context en execution-contexts; unicidad y kebab-case de skill_name bajo directories.skills; SemVer y esquemas I/O."
    delegates_to:
      - "agent:cumulo"
      - "agent:cerbero"
  - name: "Forja del Markdown"
    intent: "Generar uuid v4 y hash_signature de integridad según política de skills; YAML (contract, context, capabilities, inputs, outputs) y cuerpo conforme a contracts.skills; rutas solo vía cumulo."
    delegates_to:
      - "skill:filesystem-manager"
      - "skill:cryptography-manager"
  - name: "Indexación"
    intent: "Auditar skills/index.md (columna Capabilities obligatoria) e insertar fila idéntica a la cabecera de la skill creada."
    delegates_to:
      - "agent:cumulo"
      - "skill:filesystem-manager"
minteo_maximo: null
porcentaje_de_exito: null
---

# skill-creator

Proceso maestro para estandarizar y automatizar la creación de nuevas skills (definición física y lógica) en el Core SddIA.

## Fase 1 — Validación RBAC y topología

1. Cargar `SddIA/norms/execution-contexts.md` desde la clave `directories.norms` de `cumulo.paths.json` y comprobar que `skill_context` coincide con un identificador de sección 2.x (`source-control`, `filesystem-ops`, `knowledge-management`, `quality-assurance`, `ecosystem-evolution`).
2. Verificar que no exista `SddIA/skills/{skill_name}.md` y que `skill_name` cumpla kebab-case.
3. Validar `skill_version` y `skills_contract_version` frente a `skills-contract.md` vigente (`cumulo.contracts.skills`).
4. Auditar que `skill_inputs_schema` y `skill_outputs_schema` cubren el contrato de I/O JSON estricto (stdin / stdout).

## Fase 2 — Forja del Markdown

1. Emitir `uuid` v4 nuevo; asignar `contract` como `skills-contract v{skills_contract_version}`, `context` igual a `skill_context` validado y `capabilities` obligatorio según contrato.
2. Calcular `hash_signature` según política declarada en el contrato de skills (integridad del artefacto ejecutable o, en modalidad LLM-Native, del bloque canónico acordado para la definición).
3. Escribir `SddIA/skills/{skill_name}.md` con secciones de propósito, ejecución/cápsula (`execution_capsules.skills` si aplica) y límites termodinámicos coherentes con la descripción.
4. No hardcodear rutas absolutas del host; resolver `directories.skills` y `contracts.skills` exclusivamente desde el SSOT de cumulo.

## Fase 3 — Indexación

1. Abrir `SddIA/skills/index.md` y localizar la tabla de catálogo de definiciones (columna **Capabilities** obligatoria).
2. Insertar o actualizar la fila asociada a `{skill_name}.md` copiando `uuid`, `name`, `version`, `contract`, `context` y `capabilities` desde el YAML fuente sin divergencia.
3. Ejecutar verificación cruzada índice ↔ cabecera antes de cerrar la instancia del proceso.

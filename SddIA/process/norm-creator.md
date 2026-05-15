---
uuid: "a132a6fc-52c8-4795-8c68-a2897d456588"
name: "norm-creator"
version: "1.1.0"
contract: "process-contract v1.3.0"
context:
  - "ecosystem-evolution"
  - "knowledge-management"
hash_signature: "sha256:741da9dad9c37c411d950d09b02923d84ad618867fd6aa234c6495a5bd68dc0e"
inputs:
  - "tactical_norm_name": "Identificador kebab-case del archivo (`{name}.md` bajo `directories.library_norms`)"
  - "tactical_norm_version": "SemVer de la norma (p. ej. 1.0.0)"
  - "tactical_norm_friction": "Descripción de la fricción o patrón a cristalizar en una única norma atómica"
  - "tactical_norm_author": "Entidad creadora (persona o agente)"
  - "tactical_norm_dependencies": "Array de UUID v4 de normas prerequisito; `[]` si no aplica"
  - "norms_contract_version": "Versión del contrato a materializar (p. ej. 1.0.0 según `norms-contract.md` vía `cumulo.contracts.library_norms`)"
outputs:
  - "artifact_tactical_norm_md": "Archivo `{paths.directories.library_norms}/{tactical_norm_name}.md` con frontmatter y cuerpo conforme a `norms-contract.md`"
phases:
  - name: "Triaje de Entrada (Aduana Lógica)"
    intent: "Recibir tactical_norm_friction; abortar si viola el Principio de Atomicidad (dominios contradictorios o multi-vector no atomizable)."
    delegates_to:
      - "agent:cumulo"
      - "agent:cerbero"
  - name: "Clasificación Semántica"
    intent: "Deducir scope y category según enums de norms-contract; validar tactical_norm_dependencies; emitir UUID v4 para cabecera vía crypto-broker."
    delegates_to:
      - "action:crypto-broker"
      - "agent:dedalo"
      - "agent:cumulo"
  - name: "Destilación Rúnica"
    intent: "Separar conocimiento en Directriz Core (aséptica) y Restricciones Duras (prohibiciones explícitas para Filtro A)."
    delegates_to:
      - "agent:dedalo"
      - "agent:argos"
  - name: "Materialización"
    intent: "Ensamblar YAML y Markdown conforme a contracts.library_norms; persistir en directories.library_norms con nombre kebab-case."
    delegates_to:
      - "skill:filesystem-manager"
      - "agent:cumulo"
phase_invocations:
  - phase_name: "Clasificación Semántica"
    invocations:
      - capsule: "action:crypto-broker"
        stdin_json:
          operation: "GENERATE_UUID"
          target_payload: null
        bind:
          "data.result": "tactical_norm_uuid"
        on_error: abort
minteo_maximo: null
porcentaje_de_exito: null
---

# norm-creator

Proceso **creator** para la entidad **`tactical-norm`** (`Library_Norm`): orquesta la creación estandarizada de normas atómicas bajo el SSOT, innegociablemente alineado a **`norms-contract.md`** (`contracts.library_norms`).

## Fase 1 — Triaje de Entrada (Aduana Lógica)

1. Cargar `norms-contract.md` desde la ruta resuelta por `cumulo.contracts.library_norms`.
2. Analizar `tactical_norm_friction`: si la petición mezcla vectores técnicos incompatibles en un solo activo, exige división; si persiste la violación del **Principio de Atomicidad**, **abortar** sin escribir artefactos.
3. Verificar `tactical_norm_name` único bajo `directories.library_norms`, kebab-case y coherencia con `tactical_norm_version` / `norms_contract_version`.
4. Delegar gate **Cerbero** / topología **Cúmulo** según contextos declarados (`execution-contexts.md`).

## Fase 2 — Clasificación Semántica

1. Ejecutar `phase_invocations`: obtener `tactical_norm_uuid` vía `action:crypto-broker` (prohibido fabricar UUID fuera de la cápsula).
2. Asignar `scope` ∈ {`agnostic`, `frontend`, `backend`, `database`, `infrastructure`, `security`} y `category` ∈ {`architecture`, `workflow`, `code-smell`, `convention`, `testing`} deducidos de la fricción; rechazar valores fuera de enum.
3. Validar que cada elemento de `tactical_norm_dependencies` sea UUID v4 y exista en el catálogo que gobierne **Cúmulo** cuando aplique.

## Fase 3 — Destilación Rúnica

1. Redactar **Directriz Core**: texto aséptico, sin ambigüedad, describiendo el patrón o norma técnica.
2. Redactar **Restricciones Duras (Aduana de Fricción)**: lista explícita de prohibiciones o condiciones binarias para auditorías **Filtro A** futuras.
3. **Argos** valida que las restricciones sean comprobables (no metáforas ni criterios subjetivos sin ancla técnica).

## Fase 4 — Materialización

1. Ensamblar frontmatter obligatorio: `uuid` = `tactical_norm_uuid`, `name` = `tactical_norm_name`, `version` = `tactical_norm_version`, `nature` = `tactical-norm`, `author`, `scope`, `category`, `dependencies` = `tactical_norm_dependencies`; alinear claves y tipos a la sección 1 de `norms-contract.md`.
2. Escribir cuerpo Markdown con secciones **Directriz Core** y **Restricciones Duras (Aduana de Fricción)** según sección 2 del contrato.
3. Persistir `{paths.directories.library_norms}/{tactical_norm_name}.md` usando solo rutas resueltas desde `SddIA/core/cumulo.paths.json` post-fusión universal+local.

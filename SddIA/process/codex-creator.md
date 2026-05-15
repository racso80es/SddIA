---
uuid: "dd9e13b2-fc07-40d2-95f5-b50ebd535a9e"
name: "codex-creator"
version: "1.0.0"
contract: "process-contract v1.3.0"
context:
  - "ecosystem-evolution"
  - "knowledge-management"
hash_signature: "sha256:e2a4480fc9dbd98d8afd1df4d529a53e5cd5bd6da20642358b52cfa58f8a987d"
inputs:
  - "domain_codex_slug": "Identificador kebab-case del archivo (`{slug}.md` bajo `directories.library_codexes`)"
  - "domain_codex_name": "Nombre estratégico del paquete (campo `name` del frontmatter según `codex-contract.md`)"
  - "domain_codex_version": "SemVer inicial del códice"
  - "domain_codex_author": "Creador del paquete"
  - "target_environment": "Array de strings: entornos donde el códice tiene autoridad"
  - "tactical_norm_inventory": "Lista de referencias a normas atómicas: cada ítem `{ norm: <UUIDv4>, path: <ruta relativa canónica al .md bajo la cantera> }`"
  - "codex_contract_version": "Versión del contrato a materializar (p. ej. 1.0.0 según `codex-contract.md` vía `cumulo.contracts.library_codexes`)"
  - "domain_codex_certification_grade": "Opcional; por defecto `Pendiente` hasta auditoría Argos"
outputs:
  - "artifact_domain_codex_md": "Archivo `{paths.directories.library_codexes}/{domain_codex_slug}.md` conforme a `codex-contract.md`"
phases:
  - name: "Selección y Triaje (Inventario)"
    intent: "Recibir tactical_norm_inventory y target_environment; verificar bajo directories.library_norms que cada norma existe y es válida bajo norms-contract."
    delegates_to:
      - "agent:cumulo"
      - "agent:cerbero"
  - name: "Inyección de Identidad"
    intent: "Emitir UUID v4 del códice; fijar versión inicial y certification_grade por defecto Pendiente hasta auditoría Argos."
    delegates_to:
      - "action:crypto-broker"
      - "agent:cumulo"
  - name: "Forja de Estrategia (El Vibe)"
    intent: "Redactar cuerpo Markdown: Estrategia de Dominio e Instrucciones de Prioridad ante matices contradictorios entre normas."
    delegates_to:
      - "agent:dedalo"
      - "agent:argos"
  - name: "Materialización (Transmutación a Activo Físico)"
    intent: "Ensamblar YAML (composition con paths de normas) y Markdown en un solo flujo; nombre de archivo kebab-case; persistir en directories.library_codexes."
    delegates_to:
      - "skill:filesystem-manager"
      - "agent:cumulo"
phase_invocations:
  - phase_name: "Inyección de Identidad"
    invocations:
      - capsule: "action:crypto-broker"
        stdin_json:
          operation: "GENERATE_UUID"
          target_payload: null
        bind:
          "data.result": "domain_codex_uuid"
        on_error: abort
minteo_maximo: null
porcentaje_de_exito: null
---

# codex-creator

Proceso **creator** para la entidad **`domain-codex`** (`Library_Codex`): ensambla códices de dominio agrupando **Normas Atómicas** y fijando filosofía de ejecución para un entorno técnico concreto, innegociablemente alineado a **`codex-contract.md`** (`contracts.library_codexes`).

## Fase 1 — Selección y Triaje (Inventario)

1. Resolver `directories.library_norms` y `contracts.library_norms` exclusivamente vía `SddIA/core/cumulo.paths.json` (post-fusión universal+local).
2. Para cada entrada de `tactical_norm_inventory`, comprobar `stat` físico del `.md`, UUID en frontmatter y conformidad estructural con `norms-contract.md`.
3. Validar `target_environment` como array de strings no vacío donde aplique la política del runtime; gate **Cerbero** / **Cúmulo** según contextos.

## Fase 2 — Inyección de Identidad

1. Ejecutar `phase_invocations`: obtener `domain_codex_uuid` con `action:crypto-broker`.
2. Asignar `version` = `domain_codex_version`; `certification_grade` = `domain_codex_certification_grade` si viene informado, si no **`Pendiente`**.
3. Preparar `composition`: copia literal del inventario validado (`norm`, `path`) sin rutas inventadas.

## Fase 3 — Forja de Estrategia (El Vibe)

1. **Estrategia de Dominio:** justificar por qué el conjunto de normas es óptimo para `target_environment` y el objetivo de arquitectura.
2. **Instrucciones de Prioridad:** reglas explícitas de desempate si dos normas del códice chocan (orden de precedencia, criterio o norma gana).
3. **Argos** valida que las instrucciones sean accionables en auditoría (Filtro A), no eslóganes.

## Fase 4 — Materialización (Transmutación a Activo Físico)

1. Frontmatter obligatorio: `uuid` = `domain_codex_uuid`, `name` = `domain_codex_name`, `version`, `nature` = `domain-codex`, `author`, `target_environment`, `certification_grade`, `composition` según §1 de `codex-contract.md`.
2. Cuerpo con secciones **Estrategia de Dominio** e **Instrucciones de Prioridad** según §2 del contrato; un único flujo texto YAML+MD sin anexos externos.
3. Escribir `{paths.directories.library_codexes}/{domain_codex_slug}.md` donde `domain_codex_slug` es kebab-case y coincide con el soberano de fichero.

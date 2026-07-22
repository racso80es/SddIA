---
context:
- ecosystem-evolution
- knowledge-management
contract: process-contract v1.4.0
hash_signature: sha256:265e42fa9a88287d58a64095a19d00f0d920430e2b5fb737fda1c15068aec66f
inputs:
- domain_codex_slug: Identificador kebab-case del archivo (`{slug}.md` bajo `directories.library_codexes`)
- domain_codex_name: Nombre estratégico del paquete (campo `name` del frontmatter según `codex-contract.md`)
- domain_codex_version: SemVer inicial del códice
- domain_codex_author: Creador del paquete
- target_environment: 'Array de strings: entornos donde el códice tiene autoridad'
- tactical_norm_inventory: 'Lista de referencias a normas atómicas: cada ítem `{ norm: <UUIDv4>, path: <ruta relativa canónica al .md bajo la cantera> }`'
- codex_contract_version: Versión del contrato a materializar (p. ej. 1.0.0 según `codex-contract.md` vía `cumulo.contracts.library_codexes`)
- domain_codex_certification_grade: Opcional; por defecto `Pendiente` hasta auditoría Argos
minteo_maximo: null
name: codex-creator
outputs:
- artifact_domain_codex_md: Archivo `{paths.directories.library_codexes}/{domain_codex_slug}.md` conforme a `codex-contract.md`
- artifact_library_codexes_index: '`{paths.directories.library_codexes}/index.md` creado o actualizado con fila alineada a la cabecera YAML del códice'
- handoff_entity_uuid: UUID v4 del códice forjado; consumido por `entity-manager`
- handoff_hash_signature_new: Sello post-forja
- handoff_hash_signature_old: Sello previo en update; `null` en create
- handoff_version: SemVer resultante
phase_invocations:
- invocations:
  - bind:
      data.result: domain_codex_uuid
    capsule: action:crypto-broker
    on_error: abort
    stdin_json:
      operation: GENERATE_UUID
      target_payload: null
  phase_name: Inyección de Identidad
phases:
- delegates_to:
  - agent:cumulo
  - agent:cerbero
  intent: Recibir tactical_norm_inventory y target_environment; verificar bajo directories.library_norms que cada norma existe y es válida bajo norms-contract.
  name: Selección y Triaje (Inventario)
- delegates_to:
  - action:crypto-broker
  - agent:cumulo
  intent: Emitir UUID v4 del códice; fijar versión inicial y certification_grade por defecto Pendiente hasta auditoría Argos.
  name: Inyección de Identidad
- delegates_to:
  - agent:dedalo
  - agent:argos
  intent: 'Redactar cuerpo Markdown: Estrategia de Dominio e Instrucciones de Prioridad ante matices contradictorios entre normas.'
  name: Forja de Estrategia (El Vibe)
- delegates_to:
  - agent:cumulo
  intent: Ensamblar YAML (composition con paths de normas) y Markdown en un solo flujo; nombre de archivo kebab-case; persistir en directories.library_codexes.
  name: Materialización (Transmutación a Activo Físico)
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
- delegates_to:
  - agent:cumulo
  intent: Verificar library_codexes/index.md e insertar o actualizar fila Archivo fuente|uuid|name|version|target_environment|certification_grade alineada al YAML fuente.
  name: Indexación
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: dd9e13b2-fc07-40d2-95f5-b50ebd535a9e
version: 1.1.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# codex-creator

## Directriz de ejecución obrera

Antes de ejecutar fases de forja, el runtime IDE **debe** anteponer al contexto de Tekton el prefijo definido en `SddIA/norms/external-ai-constraints.md` § Prefijo creator:

> [EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. DO NOT BYPASS EDA BUS. USE SddIA CLI.]

Prohibido delegar forja manual en el agente cuando exista proceso creator aplicable.

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

## Fase 5 — Indexación

1. Abrir o crear `{paths.directories.library_codexes}/index.md` conforme a la sección 3 de `codex-contract.md` (cabecera YAML de índice + tabla de catálogo).
2. Insertar o actualizar la fila de `{domain_codex_slug}.md` copiando literalmente **uuid**, **name**, **version**, **target_environment** y **certification_grade** desde el frontmatter del códice recién materializado.
3. Excluir `codex-contract.md` de la tabla de definiciones.
4. Verificación cruzada: cero divergencia entre fila del índice y cabecera YAML del `.md` fuente antes de cerrar la instancia del proceso.

## Handoff `entity-manager`

Invocable desde **`entity-manager`** (piloto S+). Propagar handoff estándar al cierre de Fase 5; `origin_topology=core`.

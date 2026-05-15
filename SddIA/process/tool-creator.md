---
uuid: "c4355159-b6ea-4201-973a-a08db5ce8156"
name: "tool-creator"
version: "1.1.0"
contract: "process-contract v1.3.0"
context: "ecosystem-evolution"
hash_signature: "sha256:d93db300fd0598a158703f762b6c697f538a4c927c291c00c3caec8246f14ffb"
inputs:
  - "scope": "Enum estricto obligatorio: `core` | `local`. Resuelve el SSOT de definición e índice (simetría fractal)."
  - "tool_name": "Identificador kebab-case canónico de la tool (`{name}` del archivo `{tool_name}.md` bajo la raíz resuelta por `scope`)"
  - "tool_id": "Alias estricto de `tool_name` para compatibilidad con `interaction-triggers` (`intent.create_tool`). Si `tool_name` está ausente y `tool_id` está presente, resolver `tool_name := tool_id` (kebab-case) antes de Fase 1. Prohibido divergir si ambos están presentes."
  - "domain_origin": "Proyecto o contexto de dominio al que pertenece la tool (alineado a `domain_origin` del contrato de tools)"
  - "tool_context": "Contexto RBAC Cerbero (debe existir en `execution-contexts.md`)"
  - "required_secrets": "Variables de entorno requeridas; lista o mapa auditable para cruce con el Vértice Biológico"
  - "dependencies": "Librerías externas necesarias para la ejecución de la cápsula"
  - "tool_outputs": "Definición de salidas esperadas (logs, códigos, artefactos) interpretables por acciones"
  - "execution_logic": "Especificación de la lógica ejecutable (cuerpo de script, puntos de entrada y contrato de invocación)"
outputs:
  - "artifact_tool_md": "Si `scope=core`: `{paths.directories.tools}/{tool_name}.md`. Si `scope=local`: `.SddIA/tools/{tool_name}.md`. Cabecera YAML conforme a `paths.contracts.tools`."
  - "artifact_tool_script": "Script físico bajo `{paths.execution_capsules.tools}` cuando `execution_logic` exija cápsula ejecutable"
  - "artifact_tools_index": "Si `scope=core`: `{paths.directories.tools}/index.md`. Si `scope=local`: `.SddIA/tools/index.md`. Fila de catálogo con columna **Capabilities** obligatoria."
scope_topology:
  core:
    definitions_root: "SddIA/tools/"
    index_path: "SddIA/tools/index.md"
  local:
    definitions_root: ".SddIA/tools/"
    index_path: ".SddIA/tools/index.md"
phases:
  - name: "Validación de Dominio y Secretos"
    intent: "Cúmulo: unicidad y topología según `scope` antes de forja. Cerbero + policy-validator + shell-executor para contexto, dominio y secretos."
    delegates_to:
      - "agent:cumulo"
      - "agent:cerbero"
      - "action:policy-validator"
      - "skill:shell-executor"
  - name: "Resolución de Dependencias"
    intent: "Inventariar `dependencies` frente al árbol de `domain_origin` mediante comprobaciones autorizadas por Cerbero y ejecutadas con shell-executor."
    delegates_to:
      - "agent:cerbero"
      - "skill:shell-executor"
  - name: "Forja del Contrato y Cápsula"
    intent: "Generar el Markdown bajo contrato tools y la cápsula física en la topología resuelta por `scope`."
    delegates_to:
      - "action:crypto-broker"
      - "skill:filesystem-manager"
  - name: "Indexación"
    intent: "Cúmulo valida metadatos atómicos; solo tras autorización se escribe la fila en el `index.md` del ámbito `scope`."
    delegates_to:
      - "agent:cumulo"
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

Proceso maestro para instanciar tools con **simetría fractal** (`scope`: catálogo Core o catálogo local del workspace).

## Resolución de rutas (`scope`)

| `scope` | Definición `{tool_name}.md` | Índice |
|---------|----------------------------|--------|
| `core` | `SddIA/tools/{tool_name}.md` | `SddIA/tools/index.md` |
| `local` | `.SddIA/tools/{tool_name}.md` | `.SddIA/tools/index.md` |

Cualquier otro valor de `scope` → abortar con `exitCode: 1`. No escribir fuera de las rutas de la fila activa.

**Resolución `tool_id` / `tool_name`:** si `tool_name` ausente y `tool_id` presente → `tool_name := tool_id`. Si ambos presentes y distintos → `exitCode: 1`.

## Fase 1 — Validación de Dominio y Secretos

1. **Cúmulo** (obligatorio, previo a redactar `{tool_name}.md`): resolver `definitions_root` e `index_path` según `scope`; comprobar que `{definitions_root}{tool_name}.md` **no existe**; que ningún `uuid`/`name` del catálogo en `index_path` colisiona con la forja planificada; abortar con **Ruido de Sistema** si hay duplicado. Sin autorización de Cúmulo → prohibido iniciar Fase 3.
2. **Cerbero** autoriza `action:policy-validator` con `tool_context` y políticas Tekton para `ecosystem-evolution`.
3. `action:policy-validator`: dictamen sobre `tool_context` frente a `execution-contexts.md` (`paths.directories.norms`).
4. Validar `domain_origin` (explícito, sin rutas absolutas inventadas).
5. **Cerbero** autoriza `skill:shell-executor` para auditar `required_secrets` (presencia/ausencia en Vértice Biológico; sin volcar secretos en logs ni `execution_report`).

## Fase 2 — Resolución de Dependencias

1. **Cerbero** autoriza `skill:shell-executor` para contrastar `dependencies` con el árbol de `domain_origin` (comandos acotados; whitelist; sin terminal nativa fuera de la skill).
2. Dictamen: dependencias satisfechas, requieren inyección, o bloqueo documentado con causa.

## Fase 3 — Forja del Contrato y Cápsula

1. Ejecutar `phase_invocations`: `child_tool_uuid` vía `action:crypto-broker` (`GENERATE_UUID`).
2. Metadatos según `tools-contract.md` v1.2.0; campo canónico **`name`**.
3. Escribir `{definitions_root}{tool_name}.md` según tabla `scope` (cabecera YAML: `capabilities`, `inputs`, `outputs`, `implementation_path_ref` si aplica).
4. Cápsula bajo `paths.execution_capsules.tools` cuando `execution_logic` lo exija.

## Fase 4 — Indexación (gobernanza Cúmulo)

1. **Cúmulo** (obligatorio, previo a escritura): validar que `uuid`, `name`, `version`, `contract`, `context` y `capabilities` de la cabecera YAML coinciden con el contrato de tools, con kebab-case de `tool_name`, unicidad del fichero en `definitions_root` y coherencia con la tabla del `index_path` del `scope` activo. Si hay discordancia → **Ruido de Sistema**; abortar; **no** autorizar fila.
2. Tras autorización explícita de Cúmulo: `skill:filesystem-manager` abre `index_path`, localiza la tabla (columna **Capabilities** obligatoria) e inserta o actualiza la fila de `{tool_name}.md` copiando metadatos desde el YAML fuente.
3. **Cúmulo** verificación cruzada índice ↔ cabecera; cierre de instancia solo si sincronización exacta.

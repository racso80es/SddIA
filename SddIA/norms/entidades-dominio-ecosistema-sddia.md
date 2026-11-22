# Norma: Entidades de dominio (ecosistema SddIA)

**Fuente:** SddIA/norms. Aplicable a todo agente o IA que opere en el repositorio y a la gobernanza de artefactos SddIA.

## Definición

Se denominan **entidades de dominio** o **entidades del ecosistema SddIA** a aquellas que **integran el ítem o contrato de Token** (paths.tokensPath; Cúmulo). Es decir, son las entidades cuyos contratos o definiciones exigen operar bajo un contexto de Token (p. ej. Karma2Token) para trazabilidad y seguridad.

**Incluyen, según sus contratos:**

- **Skills** (paths.skillsDefinitionPath, paths.skillCapsules) — contrato: paths.skillsDefinitionPath/skills-contract.json (required_token: Karma2Token).
- **Tools** (paths.toolsDefinitionPath, paths.toolCapsules) — contrato: paths.toolsDefinitionPath/tools-contract.json (required_token: Karma2Token).
- **Actions** (paths.actionsPath) — contrato: actions-contract.json (required_token: Karma2Token).
- **Process** (paths.processPath) — contrato: paths.processPath/process-contract.json (required_token: Karma2Token).
- **Suites** (paths.directories.suites) — contrato: `SddIA/suites/suites-contract.md` (required_token: Karma2Token).
- **Patterns** (paths.patternsPath) — contrato: patterns-contract.json (required_token: Karma2Token).
- **Principles** (paths.principlesPath) — contrato: principles-contract.json (required_token: Karma2Token).
- **Templates** (paths.templatesPath) — contrato: templates-contract (required_token: Karma2Token).
- **Tokens** (paths.tokensPath) — contrato: SddIA/tokens/tokens-contract.md (definición de los propios tokens).
- **Capability Contracts** (`directories.capability_contracts`) — contrato: `contracts.capability_contracts` (`SddIA/library/norms/capability-contracts/capability-contracts-contract.md`, Sustrato B `{name}.schema.json` con `"x-sddia"`).

## Obligaciones de estructura (Norma de Identidad Polimórfica)

Todas las **entidades de dominio** han de respetar uno de los dos sustratos canónicos de la Cicatriz Digital (`.cursorrules` §2):

1. **Sustrato A (`{name}.md` con YAML Frontmatter):** Las entidades basadas en Markdown viven en un único archivo `{name}.md` bajo el directorio de su familia (resuelto vía Cúmulo), con los metadatos requeridos por su contrato (`uuid`, `name`, `version`, `hash_signature`, etc.) en bloque YAML Frontmatter en la cabecera del archivo.
2. **Sustrato B (`{name}.schema.json` con `"x-sddia"`):** La familia `capability-contract` (`directories.capability_contracts`) vive en un único archivo `{name}.schema.json` (JSON Schema Draft 2020-12) con cabecera de identidad atómica en la clave raíz `"x-sddia": { "uuid", "version", "hash_signature" }`, donde `hash_signature` es el SHA-256 del JSON canónico con claves ordenadas excluyendo `"x-sddia"`.
3. **Tokens (`paths.tokensPath`):** Cada token catalogado es un único `<token-id>.md` con YAML frontmatter según `SddIA/tokens/tokens-contract.md` (Sustrato A). En el Core no se usan `spec.json` ni subcarpetas por entidad para la definición.

La validación de esta estructura se realiza mediante la acción `validate` y las aduanas `sddia-qa`, asegurando que cada artefacto Sustrato A posee YAML Frontmatter válido y cada artefacto Sustrato B posee `"x-sddia"` íntegro y compilable según su contrato.

## Documentación de tarea (`paths.featurePath`, `paths.fixPath`) y workspace operativo

La **documentación de tarea** (output de las acciones spec, clarify, planning, implementation, execution, validate, finalize-process) sigue el mismo patrón que las entidades de dominio: **un solo `.md` por acción** (metadatos estructurados + cuerpo legible); no ficheros `.json` separados. Los agentes deben resolver y aplicar la norma táctica **`features-documentation-pattern`** vía Cúmulo (`directories.library_norms` → `features-documentation-pattern.md`).

El **territorio operativo** de mutación de artefactos durante una ejecución vive bajo `paths.workspacesRoot` + `workspace_template` del proceso (`process-contract v1.4.0`). Es ortogonal a `persist_ref` documental: Tekton, Dédalo y Argos limitan su visión al `workspace_path` inyectado por el CLI; prohibido asumir rutas absolutas del repositorio.

## Referencias

- **Token (Karma2Token):** paths.tokensPath; SddIA/tokens/karma2-token.md.
- **Contrato de tokens:** SddIA/tokens/tokens-contract.md (Cúmulo).
- **Documentación de tarea:** Cúmulo → `directories.library_norms` / `features-documentation-pattern.md` (`tactical-norm`).

---
*Definición canónica de entidades de dominio para gobernanza SddIA.*

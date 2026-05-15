# Norma: Entidades de dominio (ecosistema SddIA)

**Fuente:** SddIA/norms. Aplicable a todo agente o IA que opere en el repositorio y a la gobernanza de artefactos SddIA.

## Definición

Se denominan **entidades de dominio** o **entidades del ecosistema SddIA** a aquellas que **integran el ítem o contrato de Token** (paths.tokensPath; Cúmulo). Es decir, son las entidades cuyos contratos o definiciones exigen operar bajo un contexto de Token (p. ej. Karma2Token) para trazabilidad y seguridad.

**Incluyen, según sus contratos:**

- **Skills** (paths.skillsDefinitionPath, paths.skillCapsules) — contrato: paths.skillsDefinitionPath/skills-contract.json (required_token: Karma2Token).
- **Tools** (paths.toolsDefinitionPath, paths.toolCapsules) — contrato: paths.toolsDefinitionPath/tools-contract.json (required_token: Karma2Token).
- **Actions** (paths.actionsPath) — contrato: actions-contract.json (required_token: Karma2Token).
- **Process** (paths.processPath) — contrato: paths.processPath/process-contract.json (required_token: Karma2Token).
- **Patterns** (paths.patternsPath) — contrato: patterns-contract.json (required_token: Karma2Token).
- **Principles** (paths.principlesPath) — contrato: principles-contract.json (required_token: Karma2Token).
- **Templates** (paths.templatesPath) — contrato: templates-contract (required_token: Karma2Token).
- **Tokens** (paths.tokensPath) — contrato: SddIA/tokens/tokens-contract.md (definición de los propios tokens).

## Obligaciones de estructura

Todas las **entidades de dominio** han de:

1. **Respetar la estructura** definida en su contrato: la estructura canónica para las entidades es un único archivo `spec.md` en la carpeta de la entidad (paths según Cúmulo), con los metadatos requeridos por el contrato integrados en formato **YAML Frontmatter** en la parte superior del archivo.
2. **Tokens (`paths.tokensPath`):** Cada token catalogado es un único `<token-id>.md` con YAML frontmatter según `SddIA/tokens/tokens-contract.md`. En el Core no se usan `spec.json` ni subcarpetas por token para la definición.

La validación de esta estructura puede realizarse mediante la acción `validate`, asegurando que el documento `spec.md` contiene un bloque YAML Frontmatter bien formado y que los atributos en él corresponden con el esquema esperado por su contrato.

## Documentación de tarea (paths.featurePath, paths.fixPath)

La **documentación de tarea** (output de las acciones spec, clarify, planning, implementation, execution, validate, finalize-process) sigue el mismo patrón que las entidades de dominio: **un solo `.md` por acción** (metadatos estructurados + cuerpo legible); no ficheros `.json` separados. Los agentes deben resolver y aplicar la norma táctica **`features-documentation-pattern`** vía Cúmulo (`directories.library_norms` → `features-documentation-pattern.md`).

## Referencias

- **Token (Karma2Token):** paths.tokensPath; SddIA/tokens/karma2-token.md.
- **Contrato de tokens:** SddIA/tokens/tokens-contract.md (Cúmulo).
- **Documentación de tarea:** Cúmulo → `directories.library_norms` / `features-documentation-pattern.md` (`tactical-norm`).

---
*Definición canónica de entidades de dominio para gobernanza SddIA.*

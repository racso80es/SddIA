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
- **Tokens** (paths.tokensPath) — contrato: paths.tokensPath/tokens-contract.json (definición de los propios tokens).

## Obligaciones de estructura

Todas las **entidades de dominio** han de:

1. **Respetar la estructura** definida en su contrato: la estructura canónica para las entidades es un único archivo `spec.md` en la carpeta de la entidad (paths según Cúmulo), con los metadatos requeridos por el contrato integrados en formato **YAML Frontmatter** en la parte superior del archivo.
2. **Excepción de Tokens:** Las entidades de tipo Token (como `karma2-token`) pueden mantenerse en un formato de definición JSON puro (`spec.json`) si así lo requiere su especificación u operación.

La validación de esta estructura puede realizarse mediante la acción `validate`, asegurando que el documento `spec.md` contiene un bloque YAML Frontmatter bien formado y que los atributos en él corresponden con el esquema esperado por su contrato.

## Documentación de tarea (paths.featurePath, paths.fixPath)

La **documentación de tarea** (output de las acciones spec, clarify, planning, implementation, execution, validate, finalize-process) sigue el mismo patrón: **un solo .md por acción con YAML Frontmatter**; no ficheros .json separados. Norma específica: **SddIA/norms/features-documentation-frontmatter.md**.

## Referencias

- **Token (Karma2Token):** paths.tokensPath; SddIA/tokens/karma2-token/spec.json.
- **Contrato de tokens:** paths.tokensPath/tokens-contract.json (Cúmulo).
- **Arquitectura Frontmatter:** paths.featurePath/refactorization-arquitectura-frontmatter/.
- **Documentación de tarea:** SddIA/norms/features-documentation-frontmatter.md.

---
*Definición canónica de entidades de dominio para gobernanza SddIA. Ref: refactorization-arquitectura-frontmatter, features-documentation-frontmatter.*

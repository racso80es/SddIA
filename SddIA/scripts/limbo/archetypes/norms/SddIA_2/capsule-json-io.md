# Norma — E/S JSON en cápsulas (skills / tools)

**Ámbito:** binarios invocables desde **paths.skillCapsules** y **paths.toolCapsules** cuando el diseño exija **entrada y salida en JSON** (telemetría, registro evolution, orquestación por agentes).

**Fuente de rutas:** Cúmulo → `pathsContract` → `cumulo.paths.json`.

## Envelope mínimo

- **Entrada (stdin):** objeto JSON con `meta` (versión de contrato, identificador de skill/tool, correlación u origen) y `request` (payload específico del binario).
- **Salida exitosa (stdout):** objeto JSON con `meta` y `result`.
- **Salida de error:** objeto JSON con `meta` y `error` (código, mensaje, detalle opcional).

Los campos concretos por binario deben documentarse en **paths.skillsDefinitionPath** / **paths.toolsDefinitionPath** y alinearse con **SddIA/skills/skills-contract.md** y **SddIA/tools/tools-contract.md** (y sus `.json` asociados).

## Envelope versión 2.0 (skills, implementación gesfer-skills)

Contrato usado por los binarios en **paths.skillsRustPath** que consumen `gesfer_skills::capsule_json`. Campos en **camelCase** en JSON. Entrada alternativa: variable de entorno **`GESFER_CAPSULE_REQUEST`** (mismo JSON que stdin). Si no hay stdin fiable: **`GESFER_SKIP_STDIN=1`** y modo CLI documentado por skill.

**Petición (stdin o `GESFER_CAPSULE_REQUEST`):**

- `meta.schemaVersion`: literal `"2.0"`.
- `meta.entityKind`: `"skill"`.
- `meta.entityId`: kebab-case, debe coincidir con el skill.
- `meta.token`: opcional (contrato Karma2Token cuando aplique).
- `request`: objeto; campos por **paths.skillsDefinitionPath/&lt;skill-id&gt;/spec.md**.

**Respuesta (stdout, una línea JSON):**

- `meta`: eco coherente (`schemaVersion`, `entityKind`, `entityId`).
- `success`: boolean.
- `exitCode`: número; **debe cumplirse** `exitCode === 0` si y solo si `success === true`.
- `message`: texto breve.
- `feedback`: opcional, array de strings.
- `result`: opcional, objeto libre.
- `durationMs`: opcional, milisegundos.

## Referencias

- Contrato de cápsula y manifest: `paths.skillsDefinitionPath/skills-contract.md`, `paths.toolsDefinitionPath/tools-contract.md`.
- Registro evolution: `SddIA/norms/sddia-evolution-sync.md`, `paths.sddiaEvolutionContractFile`.

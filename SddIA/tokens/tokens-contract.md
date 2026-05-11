---
contract_version: "1.2.0"
entity_type: "token"
jurisdiction: "Core SddIA"
capabilities:
  - "token-schema-governance"
  - "interaction-audit-envelope"
  - "ssot-tokens-path-routing"
catalog_index: "SddIA/tokens/index.md"
paths_ref: "SddIA/core/cumulo.paths.json → directories.tokens + contracts.tokens"
---

# Contrato de Tokens (S+ Grade)

Este documento es el **único contrato de familia** del dominio **tokens** bajo `paths.tokensPath` (`SddIA/tokens/`).

## 1. Alcance

- **Ruta canónica:** `paths.tokensPath` → `SddIA/tokens/` (minúsculas).
- **Naturaleza:** motor (Core inyectable).
- **Propósito:** Toda definición de **token** que actúe como entidad de modelo con trazabilidad y contexto de seguridad debe cumplir este contrato y vivir como `{token-id}.md` en ese directorio.
- **Resolución de rutas:** solo desde **Cúmulo** (`SddIA/core/cumulo.paths.json` + fusión con `.SddIA/local.paths.json` cuando aplique).

## 2. Identidad atómica por token (innegociable)

Cada token catalogado debe tener **un único archivo** `SddIA/tokens/<token-id>.md` donde:

- **`<token-id>`** está en **kebab-case** y coincide con el **`name`** del frontmatter y con el nombre del fichero sin extensión.
- La cabecera **YAML** incluye como mínimo: `uuid`, `name`, `version`, `contract` (p. ej. `tokens-contract v1.2.0`), `nature`, `entity_family: token`, `description`, `contract_ref` (este documento), `token_definition`, `validation_rules`.

Queda **prohibido** usar `spec.json`, carpetas `<token-id>/spec.json` u otro duplicado machine-readable paralelo para definir tokens en el Core.

## 3. Artefactos del dominio

| Artefacto | Obligatoriedad | Uso |
|-----------|----------------|-----|
| `tokens-contract.md` (este fichero) | Sí | Contrato de familia + normativa + auditoría. |
| `index.md` | Sí | Catálogo tabular de tokens; enlaces a cada `<token-id>.md`. |
| `<token-id>.md` | Sí (por token catalogado) | Definición atómica del token. |

No existe `README.md` en `SddIA/tokens/`; la entrada humana al dominio es este contrato y el índice.

## 4. Restricciones

1. `token_id` / nombre de fichero en **kebab-case**.
2. Rutas canónicas solo vía **Cúmulo** (`paths.tokensPath`).
3. Un token sin su `<token-id>.md` **no se considera completo**.
4. Los tokens con **auditoría de interacciones** deben exponer `interaction_audit` coherente con la sección 5.

## 5. Auditoría de interacciones (normativa)

**Objetivo:** registrar quién utilizó qué entidad de modelo y cuándo.

**Campos obligatorios:** `entity_type`, `entity_id`, `invoked_by`, `timestamp`.

**Valores de `entity_type`:** `skill`, `tool`, `action`, `process`.

**Registro:** bajo `paths.auditsPath` (y convenciones de `accessLogFile` / subcarpetas según Cúmulo).

**Consumidor formal del contrato de Token:** el rol auditor (arquetipos bajo `SddIA/scripts/limbo/archetypes/agents/auditor/`) debe poder consultar y agregar datos de interacciones a partir de los registros asociados a cada token (`invoked_by` = `identity.issuer`, `entity_id` = `identity.subject`, `entity_type`, `context.timestamp`).

## 6. Consumidores típicos

- `paths.actionsPath`
- `paths.skillsDefinitionPath` (definición de skills en el Core)
- `paths.toolsDefinitionPath`
- `paths.processPath`
- `paths.patternsPath`
- `paths.principlesPath`
- `SddIA/scripts/limbo/archetypes/agents/auditor/`
- `SddIA/agents/security-engineer.json`
- `SddIA/security/`

## 7. Catálogo e ítems

El detalle de cada token (campos, reglas, tablas) reside en su **`SddIA/tokens/<token-id>.md`**. Resumen en **[index.md](index.md)**.

**Token catalogado actual:** [karma2-token.md](karma2-token.md) — Karma2Token.

## 8. Mantenimiento

- Cualquier cambio material en la familia o en un token debe actualizar **este contrato** (si afecta a la ley común), **`index.md`** y el **`<token-id>.md`** afectado en la misma revisión.
- Nuevos tokens: añadir `SddIA/tokens/<token-id>.md`, fila en `index.md`, y ampliar la sección 7 si hace falta un resumen explícito.

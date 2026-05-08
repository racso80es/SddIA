---
contract_version: 1.1.0
nature: motor
description: Contrato maestro de patrones — Core (motor) vs espacio local (.sddia). Los patrones del motor viven bajo SddIA/patterns; los de dominio de negocio bajo .sddia/patterns.
folder_structure: 'Patrón motor: SddIA/patterns/<uuid>/. Patrón producto: .sddia/patterns/<uuid>/ (misma forma de artefactos; resolución vía Cúmulo tras fusión de paths).'
scope: 'SddIA/patterns/ y, por extensión normativa, .sddia/patterns/'
json_schema:
  description: Esquema de spec.json por patrón; ver patterns-contract.json.
security_model:
  description: Aplicación o modificación de un patrón del motor bajo Karma2Token válido.
  required_token: Karma2Token
  token_ref: SddIA/tokens/karma2-token/spec.json
---

# Contrato de patrones (motor + espacio local)

## Invariante S+ (bifurcación espacial)

| Naturaleza | Ubicación canónica | Contenido |
|------------|-------------------|-----------|
| **Motor** (`nature: motor`) | `SddIA/patterns/<uuid>/` | Patrones de arquitectura y diseño **del ecosistema SddIA** (SSOT, orquestación, cápsulas, resolución de rutas, etc.). |
| **Producto / negocio** (`nature: product`) | `.sddia/patterns/<uuid>/` | Patrones propios del **software o dominio del cliente** (DDD de aplicación, integraciones de producto, etc.). |

Cúmulo resuelve `directories.patterns` del Core y la clave local equivalente (p. ej. `local_patterns` en `.sddia/local.paths.json`) y **fusiona** mapas; en colisión de claves rige **local wins** (véase `SddIA/agents/cumulo.instructions.json`).

## Estructura por patrón (ambos espacios)

Cada patrón reside en carpeta nombrada con **UUID v4** en su `spec.json` / frontmatter.

### Archivos obligatorios

1. **`spec.md`** — Descripción legible (es-ES): propósito, uso, trade-offs, ejemplos. Incluir en frontmatter al menos `uuid`, `name`, `version`, `nature` (`motor` | `product`), `contract_ref` apuntando a este contrato o a `SddIA/patterns/patterns-contract.md`.
2. **`spec.json`** — Metadatos según `patterns-contract.json` (id, title, category, tags, metadata, interested_agents). Se recomienda campo **`nature`** alineado con la fila de bifurcación anterior.

## Agentes interesados

La lista `interested_agents` debe reflejar el consumo real (p. ej. agentes del Core en `.md`: architect vía normas, tekton-developer, etc.).

## Referencias

- **Esquema JSON:** `SddIA/patterns/patterns-contract.json`
- **Proceso de creación (motor):** `SddIA/process/create-pattern.md` (si existe en el Core).
- **Triaje:** `SddIA/norms/triage-nature-protocol.md`

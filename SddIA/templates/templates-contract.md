---
contract_version: 1.1.0
nature: motor
description: Contrato maestro de plantillas — Core (motor) vs espacio local (.SddIA). Plantillas de procedimiento del motor en SddIA/templates; plantillas de entrega de negocio en .SddIA/templates.
scope: 'SddIA/templates/ y, por extensión normativa, .SddIA/templates/'
constraints:
  - template_id en kebab-case.
  - Rutas en input_sources deben poder resolverse vía Cúmulo cuando sean canónicas (Core + .SddIA/local.paths.json).
  - process_ref debe existir en paths.processPath del mapa fusionado cuando referencie un proceso del Core.
consumers:
  - paths.actionsPath
  - SddIA/agents/
  - paths.processPath
  - .cursor/rules
security_model:
  description: Ejecución de plantilla del motor bajo Karma2Token válido.
  required_token: Karma2Token
  token_ref: SddIA/tokens/karma2-token.md
---

# Contrato de plantillas (motor + espacio local)

## Invariante S+ (bifurcación espacial)

| Naturaleza | Ubicación canónica | Contenido |
|------------|-------------------|-----------|
| **Motor** (`nature: motor`) | `SddIA/templates/<template-id>/` | Plantillas que procedimentalizan procesos **del Core SddIA** (create-pattern, create-skill, flujos de gobernanza del repo motor, etc.). |
| **Producto / negocio** (`nature: product`) | `.SddIA/templates/<template-id>/` | Plantillas de **fin concreto del cliente** (features de producto, auditorías de dominio, checklists de release del equipo, etc.). |

`template_id` permanece en **kebab-case** en ambos espacios. Cúmulo fusiona `directories.templates` con la clave local (p. ej. `local_templates` en `.SddIA/local.paths.json`).

## Propósito

- Asociar un **proceso** (`process_ref`) con **parámetros e input_sources** reutilizables.
- Declarar `related_actions` y `related_skills` coherentes con el ciclo vigente (V5).

## Estructura por plantilla

Carpeta `.../<template-id>/` con:

1. **`spec.md`** (obligatorio) — es-ES: propósito, proceso, entradas, cierre.
2. **`spec.json`** (obligatorio) — esquema en `templates-contract.json`; incluir **`nature`** (`motor` | `product`) y `contract_ref`.

### Opcional

- **`config.json`** — valores por defecto sobrescribibles al instanciar.

## Orígenes de entrada (input_sources)

- Rutas canónicas del mapa fusionado (p. ej. `paths.featurePath`).
- Rutas relativas al repo cliente.
- Ficheros o patrones de nombre explícitos.

## Referencias

- **Esquema JSON:** `SddIA/templates/templates-contract.json`
- **Proceso de creación:** `SddIA/process/create-template.md` (si existe).
- **Triaje:** `SddIA/norms/triage-nature-protocol.md`

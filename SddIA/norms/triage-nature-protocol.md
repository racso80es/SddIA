---
uuid: "f1e2d3c4-b5a6-4129-a012-bcdef0123456"
name: "triage-nature-protocol"
version: "1.0.0"
contract: "knowledge-contract v1.0.0"
nature: motor
---

# Protocolo de triaje por naturaleza (`nature`)

## Objetivo
Clasificar conocimiento al migrar desde orígenes legacy (`SddIA_1..4/`) hacia el **Core inyectable** (`SddIA/`), el **espacio local** (`.SddIA/`) o el **Limbo** (arquetipos / fósiles).

## Valores de `nature`
- **`motor`**: infraestructura universal SddIA (agentes, procesos del motor, tokens, seguridad del motor, normas de cápsulas, triggers base). Destino: `SddIA/`.
- **`product`**: constitución del repo cliente, DDD/SOLID de producto, plantillas de entrega, reglas OWASP/linters del cliente. Destino: `.SddIA/` (Starter Kit o proyecto consumidor).

## Criterio Limbo (fósil / duplicado)
Solo se considera **fósil** apto para Limbo si hay colisión por:
- **UUID** idéntico en metadatos de entidad, o
- par **`(name, version)`** idéntico en el artefacto semántico relevante.

Si **no** hay colisión, el artefacto se **clasifica** por `nature` y se enruta a Core o `.SddIA/`, no a Limbo.

## Tokens (casing)
La ruta canónica es **`SddIA/tokens/`** (minúsculas). Los árboles legacy `Tokens/` deben dejarse de usar; copias idénticas conservadas como evidencia pueden archivarse bajo `SddIA/scripts/limbo/archetypes/` con sufijo `.{origen}.archetype.{ext}`.

## Fusión y overrides
- Rutas: `cumulo.paths.json` + `.SddIA/local.paths.json` (local wins; RFC 7396 para null).
- Triggers: `SddIA/norms/interaction-triggers.json` + `.SddIA/interaction-triggers.override.json`.

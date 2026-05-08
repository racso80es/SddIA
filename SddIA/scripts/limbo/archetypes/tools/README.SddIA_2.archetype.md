# Herramientas (SddIA/tools) — Definición vs implementación

Este directorio es **paths.toolsDefinitionPath** (Cúmulo, `SddIA/agents/cumulo.json`). Separa la **capa de definición (SddIA)** de la **capa de implementación (scripts)** para desacoplar la arquitectura IA de los artefactos técnicos.

## Estructura

| Ubicación | Contenido | Propósito |
|-----------|-----------|-----------|
| **SddIA/tools/** (este directorio) | Contrato global: `tools-contract.json`, `tools-contract.md`. Por herramienta: subcarpeta **&lt;tool-id&gt;/** con `spec.md` y `spec.json`. | **Definición:** qué hace la herramienta, contrato, entradas/salidas, fases. Consumido por agentes y procesos. |
| **scripts/tools/** | Índice `index.json` y, por herramienta, cápsula **&lt;tool-id&gt;/** (manifest, scripts, config, doc). | **Implementación:** código, configuración y ejecutables. Ruta canónica: **paths.toolCapsules[&lt;tool-id&gt;]** (Cúmulo). |

## Herramientas definidas

| toolId | Descripción | Tipo |
|--------|-------------|------|
| **start-frontend** | Levanta el dev server Next.js (npm run dev, puerto 3001). | Ejecutable |
| **run-tests-frontend** | Ejecuta tests del frontend (unit, e2e, build, lint). | Ejecutable |
| **prepare-frontend-env** | Prepara entorno: npm install + verificación .env. | Ejecutable |
| **audit-funcional-frontend** | Proceso documental para auditoría funcional: validar front actuando como usuario. Repetible. | Proceso (sin script) |

## Definición por herramienta (SddIA/tools/&lt;tool-id&gt;/)

Cada herramienta debe tener en este directorio una carpeta con:

- **spec.md** — Especificación legible: objetivo, entradas, salidas, fases, cumplimiento del contrato.
- **spec.json** — Especificación machine-readable con **implementation_path_ref** apuntando a Cúmulo.

## Referencias

- Contrato global: `tools-contract.json`, `tools-contract.md` (en este directorio).
- Cúmulo: `SddIA/agents/cumulo.json` → **paths.toolsDefinitionPath**, **paths.toolsPath**, **paths.toolCapsules**.
- Índice de implementaciones: **paths.toolsIndexPath** (Cúmulo).

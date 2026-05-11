---
uuid: a7e3b2c1-0d01-4f00-9a00-000000000106
name: postman-mcp-validation
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_1
context: quality-assurance
capabilities:
  - postman
  - newman
  - api-contract-validation
implementation_path_ref: scripts/tools/postman-mcp-validation
---

# Tool: postman-mcp-validation (SddIA_1)

## Objetivo

Ejecutar la colección Postman del proyecto (Newman CLI) y devolver resumen de ejecución en envelope estándar.

## Delivery

**`scripts/tools/postman-mcp-validation/`**

## Interfaz

- **`result`**: p. ej. `runSummary` (executed, passed, failed), duración. Sin `internalSecret` en salida.
- **Fases**: `init` → `newman` → `done` | `error`.

## Configuración

Rutas a colección y entorno parametrizables en `request` o JSON de cápsula; `baseUrl` por defecto según entorno local del workspace (SddIA_1: típ. API Admin `5010`).


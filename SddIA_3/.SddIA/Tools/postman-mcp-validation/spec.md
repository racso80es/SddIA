---
uuid: c7e3b2c1-0d03-4f00-9a00-000000000306
name: postman-mcp-validation
toolId: postman-mcp-validation
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_3
context: quality-assurance
capabilities:
  - postman
  - newman
  - api-contract-validation
implementation_path_ref: .SddIA/tool-capsules/postman-mcp-validation
---

# Tool: postman-mcp-validation (SddIA_3)

## Objetivo

Ejecutar colección Postman/Newman contra la API alcanzable en `baseUrl` (en esta línea, default típico **5020** si apunta a Product).

## Delivery

**`.SddIA/tool-capsules/postman-mcp-validation/`**

## Interfaz

**`result`**: `runSummary` (executed/passed/failed). Sin secretos en payload.

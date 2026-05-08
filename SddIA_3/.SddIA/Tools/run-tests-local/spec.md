---
uuid: c7e3b2c1-0d03-4f00-9a00-000000000304
name: run-tests-local
toolId: run-tests-local
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_3
context: quality-assurance
capabilities:
  - dotnet-test
  - orchestration
implementation_path_ref: .SddIA/tool-capsules/run-tests-local
---

# Tool: run-tests-local (SddIA_3)

## Objetivo

Orquestar entorno opcional y ejecutar `dotnet test` por alcance. En esta línea, la URL base E2E por defecto suele apuntar a **API Product** (`5020`), no Admin.

## Delivery

**`.SddIA/tool-capsules/run-tests-local/`**

## Interfaz

**`result`**: `testsSummary`, `durationMs`, etc.

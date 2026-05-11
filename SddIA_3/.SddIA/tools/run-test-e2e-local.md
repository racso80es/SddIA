---
uuid: c7e3b2c1-0d03-4f00-9a00-000000000305
name: run-test-e2e-local
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_3
context: quality-assurance
capabilities:
  - dotnet-test
  - e2e-orchestration
  - product-api
implementation_path_ref: scripts/tools/run-test-e2e-local
---

# Tool: run-test-e2e-local — variante **dotnet Product E2E** (SddIA_3)

## Objetivo

Orquestar entorno (opcional), comprobar `/health` de **Admin** y **Product**, compilar y ejecutar:

`dotnet test …/GesFer.Product.Back.E2ETests.csproj --filter "Category=E2E"`

con `E2E_BASE_URL` = URL Product y variables alineadas con Admin según especificación del proyecto.

## Confusión con SddIA_1

**SddIA_1** define el mismo `toolId` para E2E **HTTP** contra Admin. Aquí `spec.json` incluye `semantic_variant: dotnet-product-e2etests`. Unificar `toolId` en el futuro si se desea una sola herramienta por nombre.

## Delivery

**`scripts/tools/run-test-e2e-local/`**

## Interfaz

**`result`**: `testsSummary`, URLs usadas, duración. Nunca volcar `E2E_INTERNAL_SECRET` en salida.


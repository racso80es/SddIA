---
uuid: a7e3b2c1-0d01-4f00-9a00-000000000105
name: run-test-e2e-local
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_1
context: quality-assurance
capabilities:
  - http-e2e
  - api-admin
implementation_path_ref: scripts/tools/run-test-e2e-local
---

# Tool: run-test-e2e-local — variante **HTTP Admin** (SddIA_1)

## Objetivo

Automatizar pruebas E2E **HTTP** contra la **API Admin** en local: smoke (health, swagger, login), lectura de empresas, CRUD empresa y CRUD usuario. No confundir con la variante **SddIA_3** (`dotnet test` sobre `GesFer.Product.Back.E2ETests`).

## Delivery

**`scripts/tools/run-test-e2e-local/`**

## Interfaz

- Credenciales y secretos **solo** por variables de entorno efímeras; nunca en `result` ni `feedback`.
- **`result`**: booleanos y objetos por fase (`smoke`, `companyRead`, `companyCrud`, `userCrud`).
- Códigos `exitCode` distintos de 0 según primera fase fallida (definir tabla en cápsula).

## Nota de convivencia

Si ambas variantes coexisten en un mismo monorepo, valorar renombrar `toolId` en una de ellas en una iteración posterior; esta definición deja explícito `semantic_variant: http-admin-api` en `spec.json`.


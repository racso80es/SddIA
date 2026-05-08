---
index_version: "1.0.0"
workspace_origin: "SddIA_3"
definition_root: ".SddIA/Tools"
delivery_root: ".SddIA/tool-capsules"
contract_ref: "SddIA/tools/tools-contract.md"
---

# Índice de tools locales (SddIA_3)

| toolId | Descripción breve |
|--------|-------------------|
| `prepare-full-env` | Infra Docker; opción de levantar Admin API (`StartApi`). |
| `invoke-mysql-seeds` | Migraciones EF + seeds Admin. |
| `start-api` | Arranque API Admin; parámetros de arranque vía JSON de cápsula. |
| `run-tests-local` | Tests locales; E2E base URL típ. producto `5020`. |
| `run-test-e2e-local` | Orquestación + `dotnet test` E2E **GesFer.Product.Back.E2ETests**. |
| `postman-mcp-validation` | Validación Postman/Newman (base URL configurable). |

Contrato de interfaz (Core): `SddIA/tools/tools-contract.md`.

---
index_version: "1.0.0"
workspace_origin: "SddIA_1"
definition_root: ".SddIA/tools"
delivery_root: "scripts/tools"
contract_ref: "SddIA/tools/tools-contract.md"
---

# Índice de tools locales (SddIA_1)

Definiciones en este árbol; la **implementación (delivery)** debe residir bajo `scripts/tools/<name>/`, resuelto vía `implementation_path_ref` en cada `spec.json` embebido en la cápsula.

| name | Descripción breve |
|--------|-------------------|
| `prepare-full-env` | Infra Docker / MySQL / clientes (entorno backend). |
| `invoke-mysql-seeds` | Migraciones EF + seeds Admin. |
| `start-api` | Arranque API Admin (.NET) + healthcheck. |
| `run-tests-local` | Tests unit/integration/e2e locales (dotnet test). |
| `run-test-e2e-local` | E2E **HTTP** contra API Admin (smoke, lectura, CRUD). |
| `postman-mcp-validation` | Validación de endpoints vía colección Postman/Newman. |

Contrato de interfaz (Core): `SddIA/tools/tools-contract.md`. E/S envelope: `SddIA/norms/capsule-json-io.md`.


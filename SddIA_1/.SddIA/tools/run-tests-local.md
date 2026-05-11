---
uuid: a7e3b2c1-0d01-4f00-9a00-000000000104
name: run-tests-local
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_1
context: quality-assurance
capabilities:
  - dotnet-test
  - orchestration
implementation_path_ref: scripts/tools/run-tests-local
---

# Tool: run-tests-local (SddIA_1)

## Objetivo

Ejecutar tests (unit / integration / e2e / all) invocando `dotnet test`, con orquestación opcional de entorno (`prepare-full-env`, `invoke-mysql-seeds`).

## Delivery

**`scripts/tools/run-tests-local/`**

## Interfaz

- **`result`**: p. ej. `testsSummary`, `durationMs`, alcance ejecutado.
- URL base E2E por defecto alineada con API Admin local (p. ej. `http://localhost:5010`) — parametrizable en `request`.

## Fases sugeridas

`init` → `prepare` → `seeds` → `build` → `tests` → `done` | `error`.


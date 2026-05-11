---
uuid: d7e3b2c1-0d04-4f00-9a00-000000000403
name: run-tests-frontend
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_4
context: quality-assurance
capabilities:
  - npm-scripts
  - lint
  - e2e
  - frontend-tests
implementation_path_ref: scripts/tools/run-tests-frontend
---

# Tool: run-tests-frontend (SddIA_4)

## Objetivo

Ejecutar lint, build, unit y e2e del frontend Product vía scripts npm en `src/`.

## Delivery

**`scripts/tools/run-tests-frontend/`**

## Interfaz

**`result`**: resumen por fase. E2E base URL típica **http://localhost:3000**.


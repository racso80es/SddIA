---
uuid: b7e3b2c1-0d02-4f00-9a00-000000000203
name: run-tests-frontend
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_2
context: quality-assurance
capabilities:
  - npm-scripts
  - lint
  - e2e
  - frontend-tests
implementation_path_ref: scripts/tools/run-tests-frontend
---

# Tool: run-tests-frontend (SddIA_2)

## Objetivo

Ejecutar en `src/` los scripts npm de lint, build, unit y e2e según `testScope`, sin invocar npm “a mano” desde el agente.

## Delivery

**`scripts/tools/run-tests-frontend/`**

## Interfaz

**`result`**: resumen por fase (exit codes o agregado). E2E base URL típica **http://localhost:3001**.


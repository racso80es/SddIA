---
uuid: d7e3b2c1-0d04-4f00-9a00-000000000401
name: prepare-frontend-env
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_4
context: quality-assurance
capabilities:
  - nodejs
  - npm
  - frontend-setup
implementation_path_ref: scripts/tools/prepare-frontend-env
---

# Tool: prepare-frontend-env (SddIA_4 — Product)

## Objetivo

Preparar entorno del frontend **Product** (GesFer.Product.Front): `npm install` en `src/`, verificación `.env.local`.

## Delivery

**`scripts/tools/prepare-frontend-env/`**

## Interfaz

**`result`**: flags de entorno; fases `init` → `install` → `env-check` → `done` | `error`.


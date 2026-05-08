---
uuid: b7e3b2c1-0d02-4f00-9a00-000000000201
name: prepare-frontend-env
toolId: prepare-frontend-env
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_2
context: quality-assurance
capabilities:
  - nodejs
  - npm
  - frontend-setup
implementation_path_ref: .SddIA/tool-capsules/prepare-frontend-env
---

# Tool: prepare-frontend-env (SddIA_2)

## Objetivo

Preparar entorno del frontend **Admin** (GesFer.Admin.Front): dependencias npm bajo `src/` y comprobación de `.env.local`.

## Delivery

**`.SddIA/tool-capsules/prepare-frontend-env/`**

## Interfaz

**`result`**: p. ej. flags `envLocalExists`, `envCreated`. Fases: `init` → `install` → `env-check` → `done` | `error`.

---
uuid: d7e3b2c1-0d04-4f00-9a00-000000000402
name: start-frontend
version: 1.2.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_4
context: quality-assurance
capabilities:
  - nextjs
  - dev-server
  - healthcheck
  - port-policy
implementation_path_ref: scripts/tools/start-frontend
---

# Tool: start-frontend (SddIA_4 — Product)

## Objetivo

Levantar **GesFer.Product.Front** con `npm run dev`. Puerto por defecto **3000**. Health recomendado **`/api/health`** (evita rewrites i18n). Política **`portBlocked`**: `false` (fallar si ocupado) o `kill` (solo si la implementación identifica el proceso como dev server seguro).

## Delivery

**`scripts/tools/start-frontend/`** + `start-frontend-config.json`.

## Interfaz

**`result`**: `urlBase`, `port`, `pid`, etc. Fases: `init` → `port-check` → `port-kill` (opcional) → `next-cache` (opcional) → `launch` → `healthcheck` → `done` | `error`.


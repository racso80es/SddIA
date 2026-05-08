---
uuid: b7e3b2c1-0d02-4f00-9a00-000000000202
name: start-frontend
toolId: start-frontend
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_2
context: quality-assurance
capabilities:
  - nextjs
  - dev-server
  - healthcheck
implementation_path_ref: .SddIA/tool-capsules/start-frontend
---

# Tool: start-frontend (SddIA_2 — Admin)

## Objetivo

Levantar `npm run dev` para **GesFer.Admin.Front** (Next.js). Puerto por defecto típico **3001**. Éxito: HTTP **200** en la URL de comprobación configurada en la cápsula.

## Delivery

**`.SddIA/tool-capsules/start-frontend/`** — incluir `start-frontend-config.json` según necesidad del workspace.

## Interfaz

**`result`**: p. ej. `urlBase`, `port`, `pid`. Fases: `init` → `port-check` → `launch` → `healthcheck` → `done` | `error`.

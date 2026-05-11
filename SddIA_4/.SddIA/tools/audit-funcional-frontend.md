---
uuid: d7e3b2c1-0d04-4f00-9a00-000000000404
name: audit-funcional-frontend
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_4
context: quality-assurance
capabilities:
  - manual-audit
  - playwright-headed
  - process
implementation_path_ref: scripts/tools/audit-funcional-frontend
implementation_type: process
---

# Tool: audit-funcional-frontend (SddIA_4 — Product)

## Tipo

**Proceso documental** (`implementation_type: process`, sin `implementation_path_ref` obligatorio).

## Objetivo

Auditoría funcional del frontend **Product** como usuario: login, dashboard, organizaciones, logout, rutas protegidas. Comandos típicos: `npm run audit:visual`, Playwright UI.

## Requisitos

- API alcanzable según **`NEXT_PUBLIC_API_URL`** en `.env.local`.
- Front en **http://localhost:3000** cuando se usa puerto por defecto del dev server Product.

## Salida

Checklist y hallazgos en **`docs/audits/`** (nombre de fichero acordado por el workspace).


---
uuid: b7e3b2c1-0d02-4f00-9a00-000000000204
name: audit-funcional-frontend
toolId: audit-funcional-frontend
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_2
context: quality-assurance
capabilities:
  - manual-audit
  - playwright-headed
  - process
implementation_path_ref: null
implementation_type: process
---

# Tool: audit-funcional-frontend (SddIA_2 — Admin)

## Tipo

**Proceso documental** — no requiere cápsula ejecutable obligatoria; la “salida” es el informe bajo `docs/audits/` (o ruta que defina el workspace).

## Objetivo

Auditoría funcional del frontend **Admin** actuando como usuario: login, dashboard, organizaciones, logout, rutas protegidas. Opción E2E visual (`npm run audit:visual`, Playwright UI).

## Requisitos

- API Admin en ejecución (especificación legacy: puerto **5011** — validar en el proyecto).
- Herramienta `start-frontend` o equivalente; front en **http://localhost:3001** (típico).

## Interfaz

No aplica envelope JSON de ejecución única salvo que el workspace defina un wrapper; el contrato Core sigue aplicando a tools ejecutables. Este ítem se cataloga como **`implementation_type: process`**.

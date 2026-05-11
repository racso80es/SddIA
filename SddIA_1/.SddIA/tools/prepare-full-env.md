---
uuid: a7e3b2c1-0d01-4f00-9a00-000000000102
name: prepare-full-env
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_1
context: ecosystem-evolution
capabilities:
  - docker
  - local-infra
  - dev-environment
implementation_path_ref: scripts/tools/prepare-full-env
---

# Tool: prepare-full-env (SddIA_1)

## Objetivo

Preparar entorno de desarrollo: servicios Docker (MySQL, caché, herramientas auxiliares), espera a BD y fases opcionales de clientes según configuración local.

## Delivery

**`scripts/tools/prepare-full-env/`**

## Interfaz

Salida con **`result`** describiendo estado por fase (p. ej. `docker`, `clients`). Coherencia `success` / `exitCode`.

## Fases sugeridas

`init` → `docker` → `mysql` → `clients` → `done` | `error`.


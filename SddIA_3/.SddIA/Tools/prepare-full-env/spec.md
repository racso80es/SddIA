---
uuid: c7e3b2c1-0d03-4f00-9a00-000000000302
name: prepare-full-env
toolId: prepare-full-env
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_3
context: ecosystem-evolution
capabilities:
  - docker
  - local-infra
  - optional-start-api
implementation_path_ref: .SddIA/tool-capsules/prepare-full-env
---

# Tool: prepare-full-env (SddIA_3)

## Objetivo

Preparar entorno: Docker (MySQL, servicios auxiliares), espera a BD, fase **`api`** opcional (levantar Admin API en local) y **`clients`** según config.

## Delivery

**`.SddIA/tool-capsules/prepare-full-env/`**

## Interfaz

**`result`**: estado por fase (`docker`, `api`, `clients`, …). Fases: `init` → `docker` → `mysql` → `api` → `clients` → `done` | `error`.

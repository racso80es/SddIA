---
uuid: a7e3b2c1-0d01-4f00-9a00-000000000101
name: invoke-mysql-seeds
toolId: invoke-mysql-seeds
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_1
context: quality-assurance
capabilities:
  - mysql
  - ef-migrations
  - database-seeds
implementation_path_ref: .SddIA/tool-capsules/invoke-mysql-seeds
---

# Tool: invoke-mysql-seeds (SddIA_1)

## Objetivo

Comprobar MySQL, aplicar migraciones EF Core y ejecutar seeds Admin (p. ej. `RUN_SEEDS_ONLY=1`), según política del proyecto.

## Delivery

Implementación local: **`.SddIA/tool-capsules/invoke-mysql-seeds/`** (binario, script o orquestador acordado por el workspace). Sin rutas normativas de plataforma en el Core.

## Interfaz (envelope)

- Entrada: `request` con flags equivalentes a CLI (camelCase recomendado).
- Salida: envelope con **`result`** (no `data`) — p. ej. resumen por fases `mysql`, `migrations`, `seeds`. Sin secretos en `message`, `feedback`, `result`, `error`.

## Fases sugeridas (`feedback[].phase`)

`init` → `mysql` → `db_drop_create` (opcional) → `migrations` → `seeds` → `done` | `error`.

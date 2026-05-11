---
uuid: c7e3b2c1-0d03-4f00-9a00-000000000301
name: invoke-mysql-seeds
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_3
context: quality-assurance
capabilities:
  - mysql
  - ef-migrations
  - database-seeds
implementation_path_ref: scripts/tools/invoke-mysql-seeds
---

# Tool: invoke-mysql-seeds (SddIA_3)

## Objetivo

Igual que línea SddIA_1: MySQL + migraciones EF + seeds Admin; parámetros concretos en la cápsula.

## Delivery

**`scripts/tools/invoke-mysql-seeds/`**

## Interfaz

Envelope con **`result`** por fases `mysql`, `migrations`, `seeds`. Sin secretos en salida.


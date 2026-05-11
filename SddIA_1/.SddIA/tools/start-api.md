---
uuid: a7e3b2c1-0d01-4f00-9a00-000000000103
name: start-api
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_1
context: quality-assurance
capabilities:
  - dotnet
  - http-server
  - healthcheck
implementation_path_ref: scripts/tools/start-api
---

# Tool: start-api (SddIA_1)

## Objetivo

Levantar la API del backend Admin del workspace: build opcional, `dotnet run` / Kestrel, comprobación de **health** HTTP 200.

## Dependencias lógicas

Recomendado: `prepare-full-env` + `invoke-mysql-seeds` cuando la API requiera MySQL listo.

## Delivery

**`scripts/tools/start-api/`** — configuración machine-readable en cápsula (p. ej. JSON con `apiWorkingDir`, `healthUrl`, `portBlocked`, timeouts).

## Interfaz

- **`result`**: p. ej. `urlBase`, `port`, `pid`, `healthy` (sin datos sensibles).
- Códigos de error documentados en la cápsula; coherencia con `exitCode`.

## Fases sugeridas

`init` → `port-check` → `port-kill` (opcional) → `build` → `launch` → `healthcheck` → `done` | `error`.


---
uuid: c7e3b2c1-0d03-4f00-9a00-000000000303
name: start-api
toolId: start-api
version: 1.0.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA_3
context: quality-assurance
capabilities:
  - dotnet
  - http-server
  - healthcheck
implementation_path_ref: .SddIA/tool-capsules/start-api
---

# Tool: start-api (SddIA_3)

## Objetivo

Arrancar API **Admin** con política de configuración **centralizada en JSON de cápsula** (`start-api-config.json` o nombre acordado): directorio de trabajo, puerto, health URL, `portBlocked` fail|kill, timeouts, perfil ASP.NET.

## Variante respecto SddIA_1

En SddIA_3 los parámetros de arranque **no** se sobrescriben por CLI salvo lo explícito en la cápsula; reduce duplicación y “defaults mágicos” en binario.

## Delivery

**`.SddIA/tool-capsules/start-api/`**

## Interfaz

**`result`**: `urlBase`, `port`, `healthy`, etc. Códigos de salida documentados en cápsula (incl. fallo MySQL → sugerir prepare + seeds).

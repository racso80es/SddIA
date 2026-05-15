# Norma: Contrato REST del backend (OpenAPI) frente al frontend Product

**Ámbito:** GesFer.Product.Front (`src/`).  
**Relación:** complementa `commands-via-skills-or-tools.md` y la constitución en `SddIA/CONSTITUTION.md`.

## Principio

La **fuente de verdad** de rutas HTTP, parámetros y modelos expuestos al cliente es el **OpenAPI** servido por el **API backend** (p. ej. `{origen}/swagger/v1/swagger.json`). El frontend **se adecúa** a ese contrato; no al revés.

## Obligaciones para cambios en `src/`

1. Ante una nueva versión o cambio de rutas en el backend, **revalidar** el swagger y actualizar clientes en `src/lib/api/`, tipos y pruebas.
2. `NEXT_PUBLIC_API_URL` (y `API_URL` en tests/servidor) debe apuntar al **origen** del servicio API (esquema + host + puerto). Los segmentos de ruta deben coincidir con los `paths` del OpenAPI.
3. Los perfiles **HTTP** y **HTTPS** en local son válidos según cómo se levante el backend; debe quedar documentado en `src/.env.example` y `src/CONFIGURACION-API.md`.

## Referencias

- Clarify D-09: `docs/features/product-front-objetivos-pendientes/clarify.md`
- Configuración de URL: `src/CONFIGURACION-API.md`

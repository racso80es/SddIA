---
uuid: "c93387f0-d4c8-41ae-8d1a-12f2239d2710"
name: "openapi-contract-rest-admin-frontend"
version: "1.0.0"
nature: "tactical-norm"
author: "norm-creator"
scope: "frontend"
category: "workflow"
dependencies: []
---

## Directriz Core

La fuente de verdad de rutas HTTP, parámetros, códigos de respuesta y modelos del **Frontend Admin GesFer** es el **OpenAPI** publicado por el **API backend Admin** (p. ej. `{origen}/swagger/v1/swagger.json` o endpoint equivalente del servicio administrativo). El frontend admin **se adecúa** al contrato del backend admin; no al revés.

### Obligaciones ante cambios de API Admin

1. Ante nueva versión o cambio de rutas en el backend admin, **revalidar** el Swagger/OpenAPI admin y actualizar clientes HTTP, tipos y pruebas del panel administrativo.
2. **`ADMIN_API_URL`** (o equivalente documentado en `.env.example`) debe apuntar al **origen** del servicio API Admin (esquema + host + puerto). Los segmentos de ruta en el cliente deben coincidir con los `paths` del OpenAPI admin, sin prefijos ni sufijos inventados.
3. Separar explícitamente el origen admin del origen producto (`NEXT_PUBLIC_API_URL` u otra variable de producto); no mezclar clientes ni tipos entre ambos contratos.
4. Los clientes bajo el árbol del frontend admin (p. ej. `lib/api/admin/` o convención del proyecto) derivan URLs y modelos del OpenAPI admin vigente.

### Flujo de alineación

- Descubrir o refrescar OpenAPI del backend admin desplegado o de desarrollo.
- Comparar `paths`, `components.schemas` y métodos con la implementación actual del frontend admin.
- Actualizar clientes, tipos, mocks y documentación de configuración en el mismo cambio o PR que consume la nueva API admin.

## Restricciones Duras (Aduana de Fricción)

- Prohibido definir rutas HTTP, métodos o modelos en el frontend admin que contradigan el OpenAPI vigente del backend admin.
- Prohibido usar `NEXT_PUBLIC_API_URL` (u origen de producto) como base de llamadas al API administrativo cuando el contrato exige `ADMIN_API_URL`.
- Prohibido mergear cambios del frontend admin que consuman endpoints admin no desplegados o no reflejados en el Swagger de referencia del entorno objetivo.
- Prohibido hardcodear URLs completas del API admin con paths ausentes en el OpenAPI publicado.
- Prohibido omitir actualización de tipos y clientes HTTP admin cuando el diff introduce o modifica consumo de endpoints admin.
- Prohibido modificar el contrato OpenAPI del backend admin desde el frontend para “adaptar” el cliente sin despliegue coordinado del API.

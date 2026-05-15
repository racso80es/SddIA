---
uuid: "e6ae3df7-9d47-4dd1-8051-025f9fd171c7"
name: "openapi-contract-rest-frontend"
version: "1.0.0"
nature: "tactical-norm"
author: "norm-creator"
scope: "frontend"
category: "workflow"
dependencies: []
---

## Directriz Core

La fuente de verdad de rutas HTTP, parámetros, códigos de respuesta y modelos expuestos al cliente es el **documento OpenAPI** publicado por el API backend (p. ej. `{origen}/swagger/v1/swagger.json` o endpoint equivalente del stack). El frontend **se adecúa** al contrato del backend; no al revés.

### Obligaciones ante cambios de API

1. Ante nueva versión o cambio de rutas en el backend, **revalidar** el OpenAPI y actualizar clientes HTTP, tipos TypeScript (u otro lenguaje del front) y pruebas afectadas.
2. La variable de entorno de origen API (p. ej. `NEXT_PUBLIC_API_URL` en Next.js, o equivalente del framework) debe apuntar al **origen** del servicio (esquema + host + puerto). Los segmentos de ruta en el cliente deben coincidir con los `paths` del OpenAPI, sin prefijos o sufijos inventados.
3. Los perfiles HTTP y HTTPS en local son válidos según cómo se levante el backend; documentar el origen canónico en `.env.example` y documentación de configuración del frontend.
4. Los clientes API del frontend (p. ej. módulos bajo `lib/api/` o equivalente) derivan URLs y contratos del OpenAPI vigente; no duplicar rutas hardcodeadas que diverjan del spec.

### Flujo de alineación

- Descubrir o refrescar OpenAPI del backend desplegado o de desarrollo.
- Comparar `paths`, `components.schemas` y métodos con implementación actual del frontend.
- Actualizar clientes, tipos, mocks de test y documentación de configuración en el mismo cambio o PR que consume la nueva API.

## Restricciones Duras (Aduana de Fricción)

- Prohibido definir rutas HTTP, métodos o modelos de request/response en el frontend que contradigan el OpenAPI vigente del backend.
- Prohibido modificar el contrato OpenAPI del backend desde el frontend para “adaptar” el cliente sin acuerdo y despliegue coordinado en el API.
- Prohibido hardcodear URLs de API completas con paths que no existan en el OpenAPI publicado.
- Prohibido omitir actualización de tipos y clientes HTTP cuando el diff del frontend consume endpoints nuevos o modificados del backend.
- Prohibido apuntar `NEXT_PUBLIC_API_URL` (o equivalente) a un origen distinto del documentado en `.env.example` y guía de configuración del proyecto sin actualizar ambos.
- Prohibido mergear cambios de frontend que dependan de endpoints backend no desplegados o no reflejados en el OpenAPI de referencia del entorno objetivo.

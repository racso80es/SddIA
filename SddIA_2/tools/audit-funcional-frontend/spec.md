# Especificación: audit-funcional-frontend

**toolId:** `audit-funcional-frontend`
**Definición (SddIA):** Este directorio.
**Tipo:** Proceso documental (sin implementación ejecutable).

## Objetivo

Herramienta de **proceso** que define el procedimiento para realizar la auditoría funcional del frontend GesFer.Admin.Front, actuando como usuario administrativo. Permite repetir la validación en el futuro siguiendo los pasos documentados.

**Objetivo máximo:** Reproducir las acciones del usuario lo máximo posible (simulación real de interacción en cliente visual).

**No requiere script ni ejecutable:** la ejecución es manual, guiada por este spec y por el reporte en paths.auditsPath.

### Ejecución en cliente visual (headed)

Para ver las acciones del ratón, teclado y navegación en tiempo real:
- `npm run audit:visual` — navegador visible (desde `src/`)
- `npm run test:e2e:ui -- tests/audit-funcional.spec.ts` — modo UI interactivo

## Entradas (contexto)

| Parámetro | Tipo | Descripción |
|-----------|------|-------------|
| API Admin activa | requisito | Backend en puerto 5011 |
| Credenciales admin | requisito | Usuario con acceso a la API Admin |
| Variables de entorno | requisito | ADMIN_API_URL, AUTH_SECRET |

## Fases del proceso

### Fase 1 — Preparación
- Verificar que la API Admin está activa (`https://localhost:5011` o `http://localhost:5011`).
- Usar tool **start-frontend** (paths.toolCapsules.start-frontend) o `npm run dev` desde `src/`.
- Verificar que el front responde en `http://localhost:3001`.

### Fase 2 — Validación (manual o E2E automatizada)
- **E2E automatizada (visual):** `npm run audit:visual` desde `src/`. Navegador visible, acciones en pantalla. Requisito: `npx playwright install` y API Admin activa.
- **Manual:** Seguir la tabla de validación en el reporte de auditoría (paths.auditsPath):
  1. Login → Dashboard
  2. Dashboard → Resumen cargado
  3. Organizaciones → Listar, crear, editar
  4. Cerrar sesión → Redirect a login
  5. Protección de rutas (sin sesión → login)

### Fase 3 — Registro de hallazgos
Documentar desviaciones en el reporte de auditoría (AUDITORIA_FUNCIONAL_FRONTEND_YYYY_MM_DD.md).

### Fase 4 — Kaizen
Tras la auditoría, aplicar mejoras:
- Corregir hallazgos de severidad alta/crítica.
- Registrar acciones de mejora derivadas de los hallazgos.
- Actualizar este proceso si se identifican nuevos flujos o criterios.

## Salida esperada

- Reporte en **paths.auditsPath** con resultado global (Aprobado / Con observaciones / No aprobado).
- Checklist de validación completada (✓/✗).
- Hallazgos documentados con severidad.

## Posibles resultados (E2E automatizada)

| Escenario | Tests que pasan | Tests que fallan | Causa |
|-----------|-----------------|------------------|-------|
| **API Admin activa + credenciales válidas** | 8/8 | 0 | Auditoría completa OK |
| **API Admin no conectada o credenciales incorrectas** | 2/8 (tests 1 y 8) | 6/8 (tests 2–7) | Login no redirige a dashboard; tests que requieren sesión fallan |
| **Frontend no responde** | 0 | 8 | Puerto 3001 ocupado o dev server no iniciado |

**Tests que pasan sin API:** 1 (formulario login visible), 8 (protección /dashboard → redirect a login).
**Tests que requieren API:** 2, 3, 4, 5, 6, 7 (login exitoso, dashboard, companies, logout).

## Referencias

- paths.auditsPath: docs/audits/
- paths.toolCapsules.start-frontend: scripts/tools/start-frontend/
- Contrato: SddIA/tools/tools-contract.json (referencia para formato de reporte).
- Rol Auditor: SddIA/agents/auditor/auditor.json.

## Soporte para implementaciones futuras (proveedores)

Cuando futuras features incluyan integración con billeteras o proveedores externos, consultar las especificaciones de proveedores documentados:

| providerId | Descripción | Ruta |
|------------|-------------|------|
| **iota-wallet** | IOTA Wallet — extensión Chrome, self-custody, dApps, Ledger | docs/architecture/providers/iota-wallet/ |

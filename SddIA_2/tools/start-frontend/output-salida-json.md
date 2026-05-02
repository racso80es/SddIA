---
toolId: start-frontend
contract_ref: SddIA/tools/tools-contract.json
spec_ref: SddIA/tools/start-frontend/spec.md
---

# Salida JSON — start-frontend: Análisis, especificación y clarificación

## Tabla codificada de salidas (tools-contract.output.output_codes_table)

| exitCode | success | message_resumen | data_presente | descripción |
|----------|---------|-----------------|---------------|-------------|
| 0 | true | "Frontend levantado; health OK" | Sí: url_base, port, pid, healthy | Éxito: dev server levantado y respondiendo en la URL. |
| 1 | false | "Config no encontrado o inválido" | No | Config no encontrado o JSON inválido; no se intentó arrancar. |
| 2 | false | "Puerto ocupado" | Sí: port, port_in_use | Puerto en uso; usar --port-blocked kill o --port. |
| 3 | false | "No se pudo liberar el puerto" / "Puerto aún ocupado" | Sí o No | Fallo al liberar puerto (kill) o puerto sigue ocupado tras kill. |
| 4 | false | "Directorio frontend no encontrado" | No | frontendWorkingDir no existe; precondición fallida. |
| 6 | false | "Error al lanzar frontend" | No | npm run dev no ejecutó (PATH, permisos, etc.). |
| 7 | false | "Health no respondió a tiempo" | Sí: url_base, port, pid, healthy: false | Proceso arrancado pero health check timeout; comprobar URL manualmente. |

---

## 1. Responsabilidad de la herramienta

La herramienta **start-frontend** tiene una única responsabilidad:

> **Levantar el dev server del frontend (Next.js)** y **certificar** que responde correctamente en el puerto configurado.

Consecuencias para la salida JSON:

- **Éxito:** El JSON debe indicar que el servidor está levantado, accesible y saludable.
- **Fallo:** El JSON debe indicar en qué fase falló y por qué.
- **Consumidores:** Humanos, CI/CD, otras tools (e.g. audit-funcional-frontend) deben poder decidir el siguiente paso sin leer logs adicionales.

---

## 2. Análisis de la salida JSON

### 2.1 Campos del contrato (tools-contract.json)

| Campo | Tipo | Responsabilidad que cubre |
|-------|------|---------------------------|
| **toolId** | string | Identificador para que consumidores sepan qué herramienta produjo el resultado. |
| **exitCode** | number | Código de salida del proceso: 0 = éxito, distinto = fallo. Permite integración en scripts y CI. |
| **success** | boolean | Resumen semántico: ¿se cumplió la responsabilidad? (frontend levantado y respondiendo). |
| **timestamp** | string (ISO 8601) | Momento de finalización. Trazabilidad y auditoría. |
| **message** | string | Resumen breve para humanos y máquinas. Debe ser suficiente para entender el resultado sin leer feedback. |
| **feedback** | array | Trazabilidad de fases (init → port-check → launch → healthcheck → done/error). Permite diagnóstico. |
| **data** | object (opcional) | Datos específicos del fin de la herramienta. |
| **duration_ms** | number (opcional) | Duración total. Útil para métricas y timeouts. |

### 2.2 Campo `data` según responsabilidad

El campo `data` es el que **especifica** el resultado de la responsabilidad de start-frontend:

| Situación | `data` presente | Campos en `data` | Significado |
|-----------|-----------------|------------------|-------------|
| **Éxito** (exitCode 0) | Sí | `url_base`, `port`, `pid`, `healthy` | Frontend levantado; URL accesible; PID para gestión; health OK. |
| **Health timeout** (exitCode 7) | Sí | `url_base`, `port`, `pid`, `healthy: false` | Frontend arrancado pero no respondió a tiempo. Permite al consumidor intentar acceder manualmente. |
| **Puerto ocupado** (exitCode 2, 3) | Sí | `port`, `port_in_use` | Puerto bloqueado; consumidor puede cambiar puerto o intentar `--port-blocked kill`. |
| **Config/dir/launch** (exitCode 1, 4, 6) | No | — | Fallo previo al arranque; no hay datos de proceso. |

---

## 3. Especificación formal

### 3.1 Esquema JSON (salida)

```json
{
  "toolId": "start-frontend",
  "exitCode": 0,
  "success": true,
  "timestamp": "2026-03-14T17:21:41.712962600+00:00",
  "message": "string",
  "feedback": [
    {
      "phase": "init|port-check|launch|healthcheck|done|port-kill",
      "level": "info|warning|error",
      "message": "string",
      "timestamp": "ISO 8601",
      "detail": "string | null",
      "duration_ms": "number | null"
    }
  ],
  "data": { "..." } | null,
  "duration_ms": 0
}
```

### 3.2 Matriz exitCode → responsabilidad

| exitCode | success | message (resumen) | data | Responsabilidad cumplida |
|----------|---------|-------------------|------|---------------------------|
| 0 | true | "Frontend levantado; health OK" | `{ url_base, port, pid, healthy: true }` | Sí |
| 1 | false | "Config no encontrado o inválido" | null | No — no se intentó arrancar |
| 2 | false | "Puerto ocupado" | `{ port, port_in_use: true }` | No — puerto bloqueado |
| 3 | false | "No se pudo liberar el puerto" / "Puerto aún ocupado" | `{ port, port_in_use: true }` o null | No — puerto bloqueado |
| 4 | false | "Directorio frontend no encontrado" | null | No — precondición fallida |
| 6 | false | "Error al lanzar frontend" | null | No — npm no ejecutó |
| 7 | false | "Health no respondió a tiempo" | `{ url_base, port, pid, healthy: false }` | Parcial — arrancó pero no certificado |

### 3.3 Fases en `feedback` (orden cronológico)

| phase | Cuándo aparece | Nivel típico |
|-------|----------------|--------------|
| init | Siempre al inicio | info |
| port-check | Tras init | info, warning o error |
| port-kill | Solo si `--port-blocked kill` y puerto ocupado | info o error |
| launch | Tras port-check | info o error |
| healthcheck | Tras launch | info o warning |
| done | Solo en éxito | info |

---

## 4. Clarificaciones para consumidores

### 4.1 ¿Cuándo considerar éxito?

- `success === true` **y** `exitCode === 0`.
- En `data`: `healthy === true`, `url_base` y `port` presentes.
- El consumidor puede usar `data.url_base` para acceder al frontend.

### 4.2 ¿Cuándo reintentar o cambiar estrategia?

- **exitCode 2 o 3:** Reintentar con `--port-blocked kill` o `--port <otro_puerto>`.
- **exitCode 7:** El proceso está en ejecución (ver `data.pid`). El consumidor puede esperar y hacer un healthcheck manual a `data.url_base`, o asumir fallo parcial.

### 4.3 ¿Cómo parsear la salida?

- La salida es **una sola línea** JSON (sin pretty-print).
- Si hay texto previo (p. ej. del .bat), buscar el primer `{` y el último `}` para extraer el objeto.
- Validar `toolId === "start-frontend"` antes de interpretar.

### 4.4 ¿Qué no incluye la salida?

- **No** incluye stdout/stderr de `npm run dev` (el proceso hijo se ejecuta con stdout/stderr nulos).
- **No** incluye el contenido de la respuesta HTTP del healthcheck, solo que fue 200.
- **No** incluye rutas absolutas del sistema en `data` (solo `url_base`, `port`, `pid`, `healthy`).

### 4.5 Consumidores típicos

| Consumidor | Uso principal |
|------------|---------------|
| **Humano (bat)** | Ver `message` y `success`; en error, revisar `feedback`. |
| **CI/CD** | Comprobar `exitCode === 0` y `success === true`; opcionalmente `data.url_base` para tests E2E. |
| **audit-funcional-frontend** | Usar `data.url_base` como base para pruebas; fallar si `success === false`. |
| **Scripts de orquestación** | Usar `data.pid` para gestionar el proceso (p. ej. kill al finalizar). |

---

## 5. Resumen

La salida JSON de **start-frontend** está diseñada para:

1. **Certificar** si el dev server está levantado y respondiendo (responsabilidad principal).
2. **Informar** en qué fase falló y con qué datos (puerto, PID, URL) cuando aplica.
3. **Permitir** decisión automática (CI) o manual (humano) sin depender de logs externos.

El contrato base (tools-contract.json) se cumple; este documento **especifica** y **aclara** el significado de cada campo en el contexto de la responsabilidad de start-frontend.

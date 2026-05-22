---
status: implemented
feature_ref: docs/features/refactor-topologia-eventos-ola-c-v3
branch: feat/refactor-topologia-eventos-ola-c-v3
started: "2026-05-22"
---

# PBI Kaizen — Gestión de eventos emitidos (topología Ola C V3+)

> **Seguimiento activo:** `docs/features/refactor-topologia-eventos-ola-c-v3/`  
> Proceso iniciado vía `execute-process.py --process refactorization` (fase 1 ejecutada).

## Manifiesto original

### 1. Estructura de carpetas

`subscribers`: procesos suscritos al evento.

Refactorizar la estructura de eventos:

```
./.events/pending                          (entrada de evento)
./.events/processing                       (evento en inicio de procesamiento)
./.events/processing/subscribers           (fichero por cada proceso suscriptor pendiente)
./.events/processed                        (evento con uno o más subscribers OK)
./.events/processed/subscribers
./.events/dead-letter                      (evento con uno o más subscribers KO)
./.events/dead-letter/subscribers
```

En cada apartado: cabecera del evento; en `subscribers/`: los suscriptores.

### 2. Nueva entidad SddIA como controlador de eventos

Convertir acción `route-domain-event` en **proceso** con las mismas responsabilidades base, más:

- Llamada asíncrona a cada suscriptor; gestión del fichero de evento y creación de testigos en `processing/subscribers/`.
- Al recibir respuesta: copiar testigo suscriptor al destino según resultado; decorar con metadata de resultado.
- Si en destino no hay copia del evento, crearla.
- Si era el último suscriptor, eliminar evento de `processing/`.
- Permite convivencia del mismo evento en varias carpetas según estado por suscriptor.

### 3. Adecuar `event-watcher.py` al nuevo procedimiento

### 4. Adecuar documentación necesaria

### 5. Fan-out asíncrono y contrato testigo (spec §5)

- Llamada **asíncrona** a cada suscriptor (sin bloqueo secuencial entre delegaciones).
- Promoción del testigo al recibir respuesta; decoración con `result_status`, `delegation`, `error_trace` según `spec.md` §5.
- Idempotencia: no re-dispatch si testigo ya terminal en `processed/subscribers/` o `dead-letter/subscribers/`.
- Plan Tekton: **Hito K5** en `plan.md`.

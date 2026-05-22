---
document_id: PBI-REFACTOR-TOPOLOGIA-EVENTOS-OLA-C-V3
title: "Kaizen — Gestión de eventos emitidos (topología Ola C V3+)"
format: markdown
version: "1.0.0"
created: "2026-05-22"
status: "completado"
closed: "2026-05-22"
priority: ola-c-v3
feature_ref: docs/features/refactor-topologia-eventos-ola-c-v3
validacion_ref: docs/features/refactor-topologia-eventos-ola-c-v3/validacion.md
branch: feat/refactor-topologia-eventos-ola-c-v3
process: refactorization
commit_ref: "98a33d3"
---

# PBI Kaizen — Gestión de eventos emitidos (topología Ola C V3+)

**Estado:** ✅ Completado — Hitos K1–K6 implementados; Argos **APTO** (`validacion.md`, 2026-05-22). Forja EDA `entity-manager` → `Domain_Entity_Created` (`c172ee3c-cf4d-4ddd-a9c5-b75a89651098`).

**Feature:** `docs/features/refactor-topologia-eventos-ola-c-v3/`  
**Rama:** `feat/refactor-topologia-eventos-ola-c-v3`  
**Commit principal:** `98a33d3`

---

## Manifiesto original (alcance)

### 1. Estructura de carpetas simétrica

Topología V3+ en `cumulo.paths.json` y `eda_bus_utils.py`:

```
./.events/pending
./.events/processing
./.events/processing/subscribers
./.events/processed
./.events/processed/subscribers
./.events/dead-letter
./.events/dead-letter/subscribers
```

Cabecera del evento en cada estado; testigos por suscriptor bajo `subscribers/`.

### 2. Proceso `route-domain-event`

Acción `route-domain-event` **deprecada** → proceso SddIA con orquestador `route_domain_event_core.py`:

- Fan-out **asíncrono** a suscriptores; testigos en `processing/subscribers/`.
- Promoción y decoración de testigos según resultado (`result_status`, `delegation`, `error_trace`).
- Idempotencia ante testigos terminales en `processed/subscribers/` o `dead-letter/subscribers/`.
- Purga de cabecera `processing/` al cerrar todos los suscriptores.

### 3. `event-watcher.py` y `event-sweeper.py`

- Watcher delgado: invoca `execute-process --process route-domain-event`.
- Sweeper adaptado a rutas V3+, guard in-flight y purga de cabeceras.

### 4. Documentación

- Feature completo (`objectives`, `clarify`, `spec`, `plan`, `implementation`, `validacion`).
- Plantilla `eda-instance-events/README.md` actualizada.

### 5. Fan-out async + contrato testigo (spec §5)

- Default `dispatch_mode: async`; regresión sync con `SDDIA_LAB_ROUTE_SYNC=1`.
- Tests unitarios `test_eda_bus_v3plus.py` + E2E `run-eda-e2e-lab.py`.

---

## Entregables (K1–K6)

| Hito | Entregable | Estado |
|------|------------|--------|
| K1 | SSOT topología simétrica | ✅ |
| K2 | `route_domain_event_core` + proceso + handler cápsula | ✅ |
| K3 | Deprecación acción + shim `execute-action` | ✅ |
| K4 | Watcher/sweeper + consumidores | ✅ |
| K5 | Fan-out async + tests | ✅ |
| K6 | Docs + E2E lab | ✅ |

## Forja EDA

- Proceso: `SddIA/process/route-domain-event.md` (`uuid: c8e91f2a-4b6d-4e1a-9f03-2d7e5a684b10`)
- Hash: `sha256:de74f3575f10117320729fd7c63e8567801867c5ce63918204c986157c3331cb`
- Sello: `Domain_Entity_Created` vía `entity-manager` (idempotente)

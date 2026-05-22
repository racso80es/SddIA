---
feature_name: refactor-topologia-eventos-ola-c-v3
created: "2026-05-22"
process: refactorization
base: main
scope: eda-bus-symmetric-topology
supersedes_partial: docs/features/ola-c-v3-coreografia
manifest: docs/todos/pending/TODO_Refactor_Topologia_Eventos_Ola_C_V3.md
---

# Especificación técnica — Topología simétrica del bus EDA (Ola C V3+)

## 1. Contexto

**Ola C V3** (`ola-c-v3-coreografia`) entregó padre inmutable en `.events/pending/` y testigos de suscriptor bajo `.events/subscribers/{processing,processed,dead-letter}/` planos.

**Ola C V3+ (este refactor)** introduce:

1. **Topología simétrica por estado** — cada fase del ciclo (salvo `pending`) aloja **cabecera del evento** + subcarpeta **`subscribers/`**.
2. **Proceso orquestador** — `route-domain-event` deja de ser acción y pasa a **proceso** SddIA invocable vía `execute-process`.
3. **Watcher delgado** — `event-watcher.py` delega en el proceso; la lógica de enrutamiento vive en módulo compartido + handler lab.

Principios V3 **conservados**: inmutabilidad del padre en `pending/`, registro `event-subscriptions.json`, filtro topológico fractal, ECST gate, alerta Kaizen en dead-letter, sweeper como recolector del padre.

## 2. Delta topológico

### 2.1 Antes (V3)

```
.events/
  pending/[UUID].json
  subscribers/
    processing/[UUID].[subscriber].json
    processed/[UUID].[subscriber].json
    dead-letter/[UUID].[subscriber].json
```

### 2.2 Después (V3+)

```
.events/
  pending/
    [UUID].json                         ← entrada canónica (inmutable)
  processing/
    [UUID].json                         ← réplica cabecera (copia byte-identical del pending)
    subscribers/
      [UUID].[subscriber_id].json
  processed/
    [UUID].json                         ← cabecera si no existía en este estado
    subscribers/
      [UUID].[subscriber_id].json
  dead-letter/
    [UUID].json
    subscribers/
      [UUID].[subscriber_id].json
```

### 2.3 Reglas de convivencia

| Regla | Descripción |
|-------|-------------|
| R1 | `pending/[UUID].json` **nunca** se muta durante enrutamiento |
| R2 | Réplicas de cabecera en `processing/`, `processed/`, `dead-letter/` son **copias** del JSON ECST original (sin `delivery_state` embebido) |
| R3 | Un mismo `[UUID]` puede tener presencia simultánea en varias carpetas según avance **por suscriptor** |
| R4 | Testigos viven **solo** bajo `{estado}/subscribers/` |
| R5 | Al cerrar el **último** suscriptor requerido (terminal en `processed/` o `dead-letter/`), eliminar `processing/[UUID].json` |
| R6 | Sweeper sigue siendo único actor autorizado a purgar `pending/[UUID].json` |

## 3. SSOT Cúmulo (`cumulo.paths.json`)

### 3.1 Esquema objetivo

```json
{
  "event_bus": "./.events",
  "eda_bus": {
    "pending": "./.events/pending",
    "processing": "./.events/processing",
    "processed": "./.events/processed",
    "dead_letter": "./.events/dead-letter",
    "subscriptions": "SddIA/core/event-subscriptions.json"
  }
}
```

> **Migración:** Eliminar clave anidada `eda_bus.subscribers.{processing,processed,dead_letter}`. Consumidores deben resolver `{estado}/subscribers/` vía helper central.

### 3.2 Claves planas en `load_eda_bus()`

| Clave | Ruta |
|-------|------|
| `pending` | `.events/pending` |
| `processing` | `.events/processing` |
| `processing_subscribers` | `.events/processing/subscribers` |
| `processed` | `.events/processed` |
| `processed_subscribers` | `.events/processed/subscribers` |
| `dead_letter` | `.events/dead-letter` |
| `dead_letter_subscribers` | `.events/dead-letter/subscribers` |

Alias legacy `subscriber_*` → redirigen a `*_subscribers` durante transición (deprecación documentada en `eda_bus_utils`).

## 4. Contrato de cabecera por estado

### 4.1 Creación de cabecera en `processing/`

Al iniciar enrutamiento de `[UUID]`:

1. Leer `pending/[UUID].json`.
2. Si **no** existe `processing/[UUID].json`, escribir copia atómica idéntica al pending.
3. No modificar `pending/`.

### 4.2 Réplica en destino al promover suscriptor

Al promover testigo a `processed/subscribers/` o `dead-letter/subscribers/`:

1. Mover (o copiar+unlink) testigo desde `processing/subscribers/`.
2. Si **no** existe cabecera en la carpeta destino (`processed/` o `dead-letter/`), crear `destino/[UUID].json` copiando desde `pending/[UUID].json`.
3. Decorar testigo con resultado (§5).

### 4.3 Purga de `processing/`

Tras cada promoción terminal de suscriptor, evaluar:

```
required ← ids de event-subscriptions.json para event_type
terminals ← suscriptores con testigo en processed/subscribers O dead-letter/subscribers
pending_processing ← required \ terminals

SI pending_processing vacío Y existe processing/[UUID].json:
  ELIMINAR processing/[UUID].json
  (opcional) limpiar processing/subscribers/ vacíos
```

## 5. Contrato de testigo suscriptor (V3+)

Extiende esquema V3; campos nuevos en **promoción**.

### 5.1 Processing (`processing/subscribers/`)

```json
{
  "event_uuid": "[UUID]",
  "subscriber": "[subscriber_id]",
  "state": "processing",
  "started_at": "ISO-8601 UTC",
  "event_type": "Domain_Entity_Created",
  "dispatch_mode": "sync|async"
}
```

### 5.2 Processed (`processed/subscribers/`)

Campos V3 + decoración obligatoria:

```json
{
  "event_uuid": "[UUID]",
  "subscriber": "[subscriber_id]",
  "state": "processed",
  "started_at": "ISO-8601 UTC",
  "completed_at": "ISO-8601 UTC",
  "event_type": "Domain_Entity_Created",
  "result_status": "success|skipped-topology|skipped-dlt-threshold|skipped-backfill",
  "delegation": {
    "kind": "process|action|tool",
    "target": "sync-entity-index",
    "exit_code": 0
  }
}
```

### 5.3 Dead-letter (`dead-letter/subscribers/`)

```json
{
  "event_uuid": "[UUID]",
  "subscriber": "[subscriber_id]",
  "state": "dead-letter",
  "started_at": "ISO-8601 UTC",
  "failed_at": "ISO-8601 UTC",
  "event_type": "Domain_Entity_Created",
  "error_trace": "mensaje S+ Grade",
  "delegation": {
    "kind": "process|action|tool",
    "target": "pull-request-review",
    "exit_code": 1
  }
}
```

### 5.4 Idempotencia (cierre D7)

| Situación | Comportamiento |
|-----------|----------------|
| Testigo ya en `processed/subscribers/` o `dead-letter/subscribers/` | **Skip** re-dispatch; no sobrescribir salvo `--force-retry` (solo lab) |
| Testigo en `processing/subscribers/` sin terminal | Permitir reintento (watcher `MAX_ROUTE_ATTEMPTS`) |
| ECST gate fallido | Testigo `ecst-gate` directo a `dead-letter/subscribers/`; cabecera en `dead-letter/` si aplica |

## 6. Proceso `route-domain-event`

### 6.1 Promoción acción → proceso

| Aspecto | Acción legacy | Proceso objetivo |
|---------|---------------|------------------|
| Artefacto | `SddIA/actions/route-domain-event.md` | `SddIA/process/route-domain-event.md` |
| Contrato | `actions-contract v1.2.0` | `process-contract v1.3.0` |
| Context | `event-routing` | `event-routing` |
| Forja | — | **`entity-manager`** (`entity_class: process`) + `process-creator` |
| UUID | `9b314f74-44d3-43c4-b916-871a9fa43f45` | **Nuevo UUID v4** (proceso distinto; acción queda evolution stub) |

### 6.2 Inputs / outputs

**Inputs:**

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `event_file_path` | string | Ruta relativa a `pending/[UUID].json` |
| `cumulo_topology` | object | Inyectado en runtime IDE; opcional en lab |

**Outputs:**

| Campo | Tipo |
|-------|------|
| `success` | boolean |
| `delivery_status` | object `subscriber_id → status` |
| `parent_path` | string |
| `processing_header_path` | string |

### 6.3 Fases del proceso

| Fase | Intent | Delegates_to |
|------|--------|--------------|
| Lectura y validación ECST | READ pending; gate Clase ECST | `skill:filesystem-manager`, validación catálogo |
| Resolución suscripciones | READ `event-subscriptions.json`; filtro topológico | `agent:cumulo` (lógica) |
| Materialización processing | Copiar cabecera; testigos processing/subscribers | `skill:filesystem-manager` |
| Fan-out suscriptores | Dispatch process/action/tool por suscriptor | `action:execute-process`, `action:execute-action`, tools |
| Promoción testigos | Mover a processed/dead-letter; decorar; réplica cabecera; purge processing | middleware en handler físico |
| Cierre | stdout envelope ECST | — |

### 6.4 Fan-out asíncrono

| Perfil | Comportamiento |
|--------|----------------|
| **Runtime objetivo** | Lanzar delegaciones sin bloquear fase; actualizar testigo al callback |
| **Laboratorio v1 (K2)** | Secuencial vía subprocess; `dispatch_mode: sync` bajo flag `SDDIA_LAB_ROUTE_SYNC=1` |
| **Laboratorio v2 (K5)** | Fan-out async real; `dispatch_mode: async` por defecto |
| **Plan Tekton** | Hito **K5** en `plan.md` (CA11 multi-suscriptor concurrente) |

La lógica física se extrae a **`SddIA/scripts/qa/route_domain_event_core.py`** (nombre canónico) consumida por:

- Handler `execute_process_capsules.py` → `route-domain-event`
- Shim temporal `execute-action.py` → delega en `execute-process`
- Tests E2E lab

### 6.5 Acción legacy (deprecación)

Tras forja del proceso:

1. `SddIA/actions/route-domain-event.md` → cabecera `status: deprecated`; puntero a proceso.
2. `execute-action.py` mantiene handler que invoca `execute-process --process route-domain-event`.
3. Retirada del índice `actions/index.md` en hito posterior (post-Argos).

## 7. `event-watcher.py`

### 7.1 Contrato nuevo

```
PARA cada *.json NUEVO en pending/:
  INVOCAR execute-process.py --process route-domain-event \
    --inputs '{"event_file_path":"<rel-path>"}'
  NO invocar route_domain_event inline
```

### 7.2 Responsabilidades que permanecen en watcher

- Bucle de sondeo / `--once` / `--event-file-path`
- Bootstrap `ensure_event_bus_topology`
- Reintentos (`MAX_ROUTE_ATTEMPTS`) ante fallo transitorio
- Log stderr humano mínimo

### 7.3 Código a extraer del watcher

Funciones migradas a `route_domain_event_core.py`:

- `_dispatch_subscriber`
- `_validate_ecst_instance` / `_load_event_class_schemas`
- Orquestación principal (actual `route_domain_event`)

Watcher conserva solo CLI + poll + invocación proceso.

## 8. `event-sweeper.py` (cierre D7)

**Decisión:** conservar rol independiente; **no** fusionar en proceso orquestador.

### 8.1 Cambios requeridos

| Paso | Cambio |
|------|--------|
| Resolución rutas | Leer testigos en `processed/subscribers/` y `dead-letter/subscribers/` |
| Consenso | `required ⊆ processed_subscribers` (mismos ids) |
| Dead-letter | Detectar en `dead-letter/subscribers/` → alerta Kaizen |
| Purga padre | Borrar `pending/[UUID].json` |
| Archivo | Eliminar testigos `processed/subscribers/`; eliminar cabeceras `processed/[UUID].json` y `processing/[UUID].json` residuales |
| Dead-letter persistente | **No** purgar cabecera dead-letter ni testigos (auditoría) |

## 9. Consumidores a actualizar

| Consumidor | Cambio |
|------------|--------|
| `eda_bus_utils.py` | Topología simétrica; helpers cabecera + testigos |
| `emit-domain-mutation.md` | Referencia rutas `eda_bus.pending` (sin cambio semántico) |
| `event-watcher.py` | Delegación proceso |
| `event-sweeper.py` | Rutas subscribers anidadas |
| `run-eda-e2e-lab.py` | Asserts topología V3+ |
| `bus-operator` skill / micro-tools | Rutas si referencian `subscribers/` plano |
| `README.md` | Mapa bus runtime (si diverge de V3) |
| `SddIA/process/index.md` | Fila `route-domain-event` |
| `SddIA/actions/index.md` | Deprecar acción |

## 10. Criterios de aceptación (Argos)

| ID | Criterio |
|----|----------|
| CA1 | Bootstrap crea 7 rutas (4 estados + 3 subscribers/) |
| CA2 | Emit → pending; route → processing header + testigos; padre pending intacto |
| CA3 | Éxito suscriptor → testigo en `processed/subscribers/` + cabecera processed si ausente |
| CA4 | Fallo → testigo en `dead-letter/subscribers/` + alerta sweeper |
| CA5 | Último suscriptor terminal → `processing/[UUID].json` eliminado |
| CA6 | Sweeper purga pending solo con consenso processed/subscribers |
| CA7 | Watcher invoca `execute-process --process route-domain-event` |
| CA8 | Proceso forjado vía `entity-manager`; acción deprecated documentada |
| CA9 | E2E lab verde; sin regresión suscripciones actuales |
| CA10 | Idempotencia: segundo route no duplica testigos terminales |
| CA11 | Fan-out async: ≥2 suscriptores con estados divergentes simultáneos; `dispatch_mode: async` |

## 11. Fuera de alcance

- Cola async real multi-proceso (worker pool) — deuda opcional post-v1.
- Migración de ficheros históricos en `.events/` (cola volátil).
- Cambios en payload ECST de Clases de Evento.

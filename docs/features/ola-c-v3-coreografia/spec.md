---
feature_name: ola-c-v3-coreografia
created: "2026-05-22"
process: feature
base: main
scope: eda-bus-subscriber-topology
supersedes_topology: docs/events/
priority: P4
manifest: docs/todos/done/[ARQUITECTURA] Especificación Técnica Avanzada_ El Genoma de Eventos y Coreografía Asíncrona (Ola C) V3.md
---

# Especificación técnica — Ola C V3: Coreografía Asíncrona (Estado de Suscriptores)

## 1. Contexto

Ola C entregó genoma de Evento (`SddIA/events/`) y bus runtime monolítico bajo `docs/events/` con `delivery_state` embebido en el JSON padre (ledger simplificado Ola A).

**Ola C V3** refactoriza la topología física del bus a `./.events/` (raíz del workspace), descarta el **Evento Padre Mutante** y externaliza la trazabilidad a una topología de **Estado de Suscriptores** con estados industriales: `processing`, `processed`, `dead-letter`.

El JSON padre en `pending/` es **inmutable** durante todo el ciclo de vida coreografiado.

## 2. Principios

| Principio | Regla |
|-----------|-------|
| Inmutabilidad del padre | El evento ECST en `pending/[UUID].json` no se modifica tras la emisión |
| Trazabilidad por suscriptor | Cada suscriptor escribe un testigo atómico `[UUID].[NOMBRE_SUSCRIPTOR].json` |
| Desacoplamiento | El middleware mueve testigos entre carpetas de estado; el enrutador no muta el padre |
| Recolección diferida | `event-sweeper.py` purga el padre solo cuando todos los suscriptores requeridos están en `processed/` |
| Fallo asimétrico | Testigos en `dead-letter/` emiten alerta Kaizen; el padre permanece en `pending/` |

## 3. Topología SSOT (`cumulo.paths.json`)

### 3.1 Clave canónica del bus

```json
{
  "event_bus": "./.events",
  "eda_bus": {
    "pending": "./.events/pending",
    "subscribers": {
      "processing": "./.events/subscribers/processing",
      "processed": "./.events/subscribers/processed",
      "dead_letter": "./.events/subscribers/dead-letter"
    },
    "subscriptions": "SddIA/core/event-subscriptions.json"
  }
}
```

> **Hito C3.1:** Registrar `"event_bus": "./.events"` como ruta raíz canónica. Las claves `eda_bus.*` resuelven relativas a `event_bus` o como rutas absolutas relativas al workspace.

### 3.2 Árbol físico (Hito C3.2)

```
.events/                              ← gitignored (/.events/)
  pending/                            ← Eventos ECST crudos (inmutables)
    [UUID].json
  subscribers/
    processing/                       ← Bloqueos en vuelo
      [UUID].[NOMBRE_SUSCRIPTOR].json
    processed/                        ← Éxitos
      [UUID].[NOMBRE_SUSCRIPTOR].json
    dead-letter/                      ← Fallos S+ Grade
      [UUID].[NOMBRE_SUSCRIPTOR].json
```

### 3.3 Descarte explícito (Ola C → V3)

| Elemento obsoleto | Sustituto V3 |
|-------------------|--------------|
| `docs/events/processing/` (evento padre) | `./.events/subscribers/processing/` (testigo suscriptor) |
| `docs/events/processed/` (evento padre) | `./.events/subscribers/processed/` (testigo suscriptor) |
| `docs/events/dead-letter/` (evento padre) | `./.events/subscribers/dead-letter/` (testigo suscriptor) |
| **Evento Padre Mutante** (`delivery_state` en JSON padre) | Testigos atómicos por suscriptor |
| Recibos `[UUID].[PURPOSE].notificado` | Testigos `[UUID].[NOMBRE_SUSCRIPTOR].json` |
| Subcarpetas `receipts/` anidadas en fase padre | Descartadas |

## 4. Contrato de notificación (Hito C3.3)

### 4.1 Inicio de procesamiento

Cuando un suscriptor (ej. `cumulo`, `argos`) inicia el procesamiento del evento `[UUID]`:

1. Escribe testigo en `./.events/subscribers/processing/[UUID].[NOMBRE_SUSCRIPTOR].json`
2. El padre en `pending/[UUID].json` **no se toca**

**Esquema mínimo del testigo (processing):**

```json
{
  "event_uuid": "[UUID]",
  "subscriber": "[NOMBRE_SUSCRIPTOR]",
  "state": "processing",
  "started_at": "ISO-8601",
  "event_type": "PullRequest_Merged"
}
```

### 4.2 Finalización con éxito

El middleware mueve el testigo:

`subscribers/processing/[UUID].[NOMBRE_SUSCRIPTOR].json` → `subscribers/processed/[UUID].[NOMBRE_SUSCRIPTOR].json`

El testigo en `processed/` añade `completed_at` (ISO-8601). El padre permanece inmutable.

### 4.3 Fallo del suscriptor

El middleware mueve el testigo:

`subscribers/processing/[UUID].[NOMBRE_SUSCRIPTOR].json` → `subscribers/dead-letter/[UUID].[NOMBRE_SUSCRIPTOR].json`

**Esquema mínimo del testigo (dead-letter):**

```json
{
  "event_uuid": "[UUID]",
  "subscriber": "[NOMBRE_SUSCRIPTOR]",
  "state": "dead-letter",
  "started_at": "ISO-8601",
  "failed_at": "ISO-8601",
  "error_trace": "stack trace o mensaje S+ Grade",
  "event_type": "PullRequest_Merged"
}
```

El padre en `pending/` **nunca** se modifica ni se mueve por fallo de suscriptor.

## 5. `event-sweeper.py` — El Recolector (Hito C3.4)

Demonio inerte de limpieza. Ubicación destino: `SddIA/scripts/daemons/event-sweeper.py`.

### 5.1 Algoritmo

```
PARA cada [UUID].json EN ./.events/pending/:
  required ← suscriptores de event-subscriptions.json para event_type del padre
  done     ← archivos en subscribers/processed/ con prefijo [UUID].
  failed   ← archivos en subscribers/dead-letter/ con prefijo [UUID].

  SI failed NO vacío:
    EMITIR alerta Kaizen (sin borrar padre ni testigos)
    CONTINUAR

  SI required ⊆ done (todos los suscriptores en processed/):
    PURGAR pending/[UUID].json
    ARCHIVAR testigos processed/[UUID].* (destino TBD: purge o archive/)
```

### 5.2 Reglas

| Condición | Acción |
|-----------|--------|
| Todos los suscriptores requeridos en `processed/` | Purgar padre + archivar testigos |
| Algún testigo en `dead-letter/` | Alerta Kaizen; **no** borrar padre |
| Suscriptores pendientes (sin testigo en ningún estado terminal) | No-op; esperar |
| Padre huérfano (sin suscriptores requeridos) | Log warning; no purgar automáticamente |

### 5.3 Entrada de suscriptores requeridos

Fuente: `SddIA/core/event-subscriptions.json` — mapa `event_type` → lista de suscriptores con `agent`/`action`/`intent`. El nombre del suscriptor en el testigo coincide con el identificador canónico del agente suscriptor (ej. `cumulo`, `argos`).

## 6. Orquestador central — garantías de arranque (Hito C3.2)

Al iniciar (`event-watcher.py`, `execute-process.py` o módulo bootstrap dedicado), el orquestador **debe** crear idempotentemente:

- `./.events/pending/`
- `./.events/subscribers/processing/`
- `./.events/subscribers/processed/`
- `./.events/subscribers/dead-letter/`

Resolución de rutas exclusivamente vía `cumulo_topology` → `event_bus` + `eda_bus.*`.

## 7. `.gitignore` (Hito C3.1)

Añadir obligatoriamente en la raíz:

```
/.events/
```

Mantener `docs/events/` en `.gitignore` durante la transición; retirar tras migración completa de consumidores.

## 8. Consumidores afectados

| Componente | Cambio |
|------------|--------|
| `cumulo.paths.json` | `event_bus`, reestructuración `eda_bus` |
| `eda_bus_utils.py` | Resolver `./.events/`; defaults actualizados |
| `event-watcher.py` | Bootstrap topología; escribir testigos en lugar de mover padre |
| `route-domain-event` / middleware | Mover testigos processing → processed/dead-letter |
| `execute-process.py` | `_write_pending_event` → `./.events/pending/` |
| `emit-domain-mutation`, `emit-pr-*` | Emisión a `./.events/pending/` |
| `event-sweeper.py` | **Nuevo** — recolector |
| `README.md` | Mapa de rutas actualizado |
| Hooks / gates que leen bus | Actualizar resolución SSOT |

## 9. Migración desde `docs/events/`

| Escenario | Acción |
|-----------|--------|
| Eventos pendientes en `docs/events/pending/` | Migración manual o script one-shot a `./.events/pending/` |
| Consumidores con fallback `docs/events/` | Actualizar a `./.events/` vía Cúmulo |
| `delivery_state` en JSON legacy | Ignorar; no portar a V3 |
| Laboratorios `SddIA_1…4` | Heredan topología vía sync Core |

## 10. Verificación (Argos)

- [ ] `cumulo.paths.json`: clave `event_bus` = `"./.events"`
- [ ] `.gitignore`: entrada `/.events/`
- [ ] Orquestador crea 4 carpetas al arranque
- [ ] Suscriptor escribe testigo `processing/` sin mutar padre
- [ ] Middleware promueve testigo a `processed/` o `dead-letter/` con `error_trace`
- [ ] `event-sweeper.py`: purga padre cuando todos en `processed/`
- [ ] `event-sweeper.py`: alerta Kaizen ante `dead-letter/` sin borrar padre
- [ ] Grep: cero literales `docs/events/` como fallback operativo en consumidores activos

## 11. Trazabilidad

| Artefacto | Ref |
|-----------|-----|
| Backlog P4 | `docs/todos/pending/[OPERATIVO] Backlog pendiente post-PR11…` § Prioridad 4 |
| Manifiesto V3 | `docs/todos/done/[ARQUITECTURA] Especificación Técnica Avanzada… (Ola C) V3.md` |
| Feature predecesora | `docs/features/ola-c-event-entity/` (genoma + Ola C entregada) |
| SSOT rutas | `SddIA/core/cumulo.paths.json` |
| Suscripciones | `SddIA/core/event-subscriptions.json` |

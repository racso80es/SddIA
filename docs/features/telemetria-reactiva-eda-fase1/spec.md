---
feature_name: telemetria-reactiva-eda-fase1
created: "2026-05-27"
process: feature
base: main
scope: SddIA/events, event-creator, events-contract, qa-templates
master_pbi_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
---

# Especificación técnica — Fase 1 · Trinidad de Estímulos

## 1. Contexto

Estado actual (post Fase 0):

- **7 Clases ECST** en `SddIA/events/` raíz + `events-contract.md` + `index.md` plano.
- Sin campo `event_family`; sin subcarpetas `telemetry/`, `orchestration/`, `domain/`.
- `event-creator` deposita siempre bajo `directories.events` (raíz).
- Instancias runtime siguen pipeline **V3+** (`eda_bus.pending`) — sin cambio en esta fase.

Objetivo: genoma fractal alineado al bus futuro (Fase 3) sin big-bang de runtime (D0.2).

## 2. Topología objetivo del genoma

```text
SddIA/events/
├── events-contract.md          # único contrato en raíz
├── index.md                    # índice de familias (no catálogo plano de 7 filas)
├── telemetry/
│   ├── index.md                # Códice de Familia
│   └── raw-execution-finished.md
├── orchestration/
│   └── index.md                # Códice vacío (jurisdicción declarada)
└── domain/
    ├── index.md                # Códice + catálogo 7 ECST migradas
    ├── domain-entity-created.md
    ├── domain-entity-updated.md
    ├── domain-entity-deleted.md
    ├── pull-request-presented.md
    ├── pull-request-merged.md
    ├── system-fracture-detected.md
    └── kaizen-alert-required.md
```

### 2.1 `index.md` por familia (Códice)

Cada subcarpeta incluye frontmatter mínimo:

```yaml
---
family: telemetry | orchestration | domain
maintained_by_agent: cumulo
indexed_at: "<ISO date>"
---
```

Cuerpo obligatorio:

- **Propósito** de la familia (tabla PBI § Fase 1).
- **Emisores autorizados** (lista cerrada).
- **Consumidor runtime previsto** (referencia Fase 3; sin implementar).
- **Catálogo ECST** — tabla `Archivo | uuid | event_type | version` sincronizada con cabeceras YAML.

### 2.2 Índice raíz `SddIA/events/index.md`

Reemplazar catálogo plano por:

- Enlace a contrato `events-contract.md`.
- Tabla de **familias** → path del Códice.
- Nota de integridad: conteo ECST por familia.

## 3. Contrato `events-contract.md` v1.1.0

### 3.1 Campo nuevo en cabecera de Clase

| Campo | Obligatorio | Valores |
|-------|:-----------:|---------|
| `event_family` | Sí | `telemetry` \| `orchestration` \| `domain` |

Reglas:

- Debe coincidir con la subcarpeta física del archivo `{name}.md`.
- Prohibido `event_family: domain` bajo `telemetry/`, etc.
- **Argos / auditoría genoma:** Clase sin campo o valor inválido → incumplimiento contractual.

### 3.2 Sección nueva en contrato (cuerpo)

- **§ Trinidad de Estímulos** — tabla emisor/consumidor/destino (PBI).
- **§ Simetría fractal** — genoma `{family}/` ↔ runtime `./.events/{family}/` (declarativo; runtime Fase 3).
- Actualizar `contract_version` a `1.1.0` en frontmatter del contrato.

### 3.3 Instancia ECST (sin cambio de envelope en Fase 1)

El envelope JSON (`event_id`, `event_type`, `payload`, …) **no** añade `event_family` en instancia en esta fase; la familia se infiere de la Clase al enrutar (Fase 3). Documentar en contrato como deuda explícita hacia Fase 3.C.

## 4. Migración de las 7 Clases existentes

| Archivo actual | Destino | `event_family` |
|----------------|---------|----------------|
| `domain-entity-created.md` | `domain/` | `domain` |
| `domain-entity-updated.md` | `domain/` | `domain` |
| `domain-entity-deleted.md` | `domain/` | `domain` |
| `pull-request-presented.md` | `domain/` | `domain` |
| `pull-request-merged.md` | `domain/` | `domain` |
| `system-fracture-detected.md` | `domain/` | `domain` |
| `kaizen-alert-required.md` | `domain/` | `domain` |

Por cada archivo:

1. Añadir `event_family: domain` en cabecera YAML.
2. Actualizar `contract: events-contract v1.1.0`.
3. Mover físicamente; **no** cambiar `uuid`, `event_type`, `hash_signature` salvo recomputación obligatoria por cambio canónico de path (si el sello incluye path — verificar `crypto-broker`; si el sello es solo contenido semántico, mantener).
4. Actualizar referencias internas a rutas relativas en cuerpo (si existen).
5. Sincronizar fila en `domain/index.md`.

**Referencias cruzadas a actualizar (muestra):**

- `SddIA/core/eda-coverage.json` — `artifact_path` si indexa path de Clase.
- Índices de procesos/agentes que citen `SddIA/events/{name}.md`.
- `rg` post-migración: cero referencias a rutas planas obsoletas en Core.

## 5. Clase nueva `Raw_Execution_Finished`

| Atributo | Valor |
|----------|-------|
| `name` | `raw-execution-finished` |
| `event_type` | `Raw_Execution_Finished` |
| `event_family` | `telemetry` |
| Ruta | `SddIA/events/telemetry/raw-execution-finished.md` |
| Proceso de forja | `event-creator` con inputs § `clarify.md` D1.4 payload |

Emisores autorizados (documentación en Clase):

- Cápsulas / procesos CLI que implementen Peaje Termodinámico (Fase 3) — indexar `execute-process`, `execute-action`, extensiones `execute_process_capsules` cuando existan.

Capabilities sugeridas: `raw_execution_finished`, `thermodynamic_toll`.

## 6. Proceso `event-creator` (create-event)

### 6.1 Input `event_family` obligatorio (D1.9 — cerrado por Kaizen)

> **Estado:** default `domain` **retirado** en `event-creator` v1.2.0 (`kaizen-event-creator-event-family-explicit`). El bloque siguiente documenta el comportamiento histórico de Fase 1.

```yaml
- "event_family":
    description: "Familia Trinidad: telemetry | orchestration | domain"
    required: true
```

**Normalización en runtime (v1.2.0+):**

```text
effective_event_family = process_inputs.event_family.strip().lower()
  if present and non-empty after trim
  else ERROR (input obligatorio)
```

| Invocación | Comportamiento (v1.2.0+) |
|------------|--------------------------|
| Sin `event_family` | **Error** de validación |
| `"event_family": "domain"` | Forja bajo `domain/` |
| `"event_family": "telemetry"` | Forja bajo `telemetry/` |
| `"event_family": "orchestration"` | Forja bajo `orchestration/` |

**Histórico Fase 1 (v1.1.0):** fallback `domain` para retrocompat — eliminado por Kaizen `PBI-KAIZEN-EVENT-CREATOR-EVENT-FAMILY-EXPLICIT`.

Validación fase «Validación de Arquitectura» (sobre `effective_event_family`):

- Enum estricto `{ telemetry, orchestration, domain }`.
- Coherencia con `event_context` / RBAC (Cerbero).
- Para `telemetry`: rechazar si `emitter_agents` incluye agentes ED no-CLI.

> **Contrato de Clase vs. proceso:** en el **artefacto** `{name}.md`, `event_family` sigue siendo **obligatorio** en cabecera (§3). Desde v1.2.0 también es **obligatorio** en el input del proceso `event-creator`.

### 6.2 Resolución de ruta de forja

```text
artifact_event_md = {directories.events}/{effective_event_family}/{event_name}.md
```

- Workspace dinámico del proceso (cuando exista Fase 2) apunta a subcarpeta; en Fase 1 basta resolución Cúmulo + `effective_event_family`.
- Tras forja: actualizar `index.md` de la familia, no el índice plano legacy.

### 6.3 Outputs sin cambio semántico

`handoff_entity_uuid`, `handoff_hash_signature_*` — consumidos por `entity-manager` / `emit-domain-mutation` como hoy.

## 7. Regresión y plantillas (§1.E)

| Artefacto | Acción |
|-----------|--------|
| `SddIA/scripts/qa/test_eda_bus_v3plus.py` | Ajustar fixtures/paths si referencian `SddIA/events/*.md` en raíz; mantener tests V3+ verdes |
| `SddIA/templates/eda-instance-events/README.md` | Añadir nota: genoma fractal en Core; instancia V3+ sigue válida (D0.2) |
| `SddIA/process/event-creator.md` | Inputs + fases alineados a §6 |

## 8. Fuera de alcance (explícito)

- Crear `./.events/telemetry/` ni scripts `route-telemetry`.
- Modificar `event-subscriptions.json` o `cumulo.paths.json` (Fases 2–3).
- Emitir instancias `Raw_Execution_Finished` en runtime.
- Eventos `Tool_Degraded`, orquestación real entre ED.

## 9. Criterios de aceptación (trazabilidad)

| AC PBI | Verificación |
|--------|--------------|
| AC1.1 | Árbol §2 sin esquemas sueltos en raíz |
| AC1.2 | Tres `index.md` de familia con jurisdicción y emisores |
| AC1.3 | `events-contract.md` v1.1.0 + cabeceras Clase |
| AC1.4 | `event-creator` enruta por `effective_event_family` (default `domain` + caso explícito `telemetry` en `execution.md`) |

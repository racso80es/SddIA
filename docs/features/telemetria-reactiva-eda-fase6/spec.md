---
feature_name: telemetria-reactiva-eda-fase6
created: "2026-05-28"
process: feature
base: main
scope: README.md, PBI maestro archive, validacion.md
master_pbi_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
---

# Especificación técnica — Fase 6 · Actualización README.md

## 1. Contexto

Estado actual (post Fases 1–5 en `main`):

| Área | Implementado en Core | README raíz actual |
|------|---------------------|-------------------|
| Genoma eventos | `SddIA/events/{telemetry,orchestration,domain}/` + `events-contract.md` | Cita topología plana / `events/index.md` |
| Runtime bus | `./.events/telemetry|orchestration|domain/` + V3+ `pending/` | Solo V3+ (`eda_bus`) |
| Suscripciones | Tres JSON `event-*-subscriptions.json` | Solo `event-domain-subscriptions` implícito |
| Enrutadores | `route-telemetry`, `route-orchestration`, `route-domain` | Solo `route-domain-event` |
| Workspaces | `paths.workspacesRoot`, `workspace_template` en procesos | `persist_ref` como carpeta de tarea |
| Peaje | `Raw_Execution_Finished` en telemetría | No documentado |
| Radamanto | Agente + batch + Self-Healing | Ausente en catálogo |
| Compliance tokens | `telemetry_receipt`, `Telemetry_Compliance_Breached` | Ausente |

Objetivo: alinear la carta de navegación pública con el ecosistema implementado **sin** duplicar la profundidad de los Códices de Familia ni de las features de fase.

## 2. Arquitectura documental objetivo

```text
README.md
├── Ontología de Activos (tabla — filas Event, Process actualizadas)
├── Eventos: genoma, runtime e instancia
│   ├── Trinidad de Estímulos + event_family
│   ├── Genoma fractal (SddIA/events/)
│   ├── Bus fractal runtime (eda_fractal)
│   ├── Pipeline dominio legacy V3+ (eda_bus — coexistencia D0.2)
│   └── Enrutadores + suscripciones split
├── Agentes del Core (+ Radamanto; Argos vs Radamanto)
├── Orquestación multi-agente (workspaces + persist_ref ortogonal)
├── Aduana Universal (CLI) — NUEVA
│   ├── Peaje Termodinámico
│   ├── telemetry_receipt (fail-soft)
│   └── Telemetry_Compliance_Breached (auditoría async)
├── [secciones existentes sin cambio material: Bóvedas, Cicatriz, Core/instancia, Cápsulas]
└── Enlaces validados (§6.F)
```

## 3. Sección §6.A — Eventos: genoma, runtime e instancia

### 3.1 Trinidad de Estímulos

Tabla canónica (paridad PBI § Fase 1):

| Familia | Naturaleza | Emisor autorizado | Destino runtime |
|---------|------------|-------------------|-----------------|
| `telemetry` | Ruido físico (Nivel 1) | **Solo CLI** | `./.events/telemetry/` |
| `orchestration` | Línea de montaje táctica | CLI (éxito) / auditores | `./.events/orchestration/` |
| `domain` | Verdad ontológica (Nivel 3) | Agentes Core (Cúmulo, Cerbero, Radamanto, …) | `./.events/domain/` |

Campo obligatorio en Clase ECST: `event_family` (enum estricto). Contrato: [`events-contract.md`](../../../SddIA/events/events-contract.md).

### 3.2 Genoma fractal

Estructura a documentar:

```text
SddIA/events/
├── events-contract.md          ← único contrato en raíz
├── telemetry/index.md          ← Códice de Familia
├── orchestration/index.md
└── domain/index.md
```

**Eliminar o corregir** referencia obsoleta a `SddIA/events/index.md` como índice único si contradice el estado real (verificar existencia; si persiste como índice agregador, clarificar rol secundario).

### 3.3 Bus fractal runtime

Rutas SSOT `cumulo.paths.json` → `eda_fractal`:

| Ruta | Propósito | Consumidor |
|------|-----------|------------|
| `./.events/telemetry/` | Alta frecuencia; purga post-consenso suscriptores | `route-telemetry` → Radamanto, telemetry-compliance-audit |
| `./.events/orchestration/` | Latencia mínima | `route-orchestration` |
| `./.events/domain/` | Gobernanza | `route-domain` → Cerbero, Cúmulo, procesos reactivos |

Suscripciones:

- `SddIA/core/event-telemetry-subscriptions.json`
- `SddIA/core/event-orchestration-subscriptions.json`
- `SddIA/core/event-domain-subscriptions.json`

### 3.4 Coexistencia V3+ (D0.2)

Subsección **Pipeline dominio legacy (V3+)** — reubicar contenido actual de topología `pending/processing/processed/dead-letter` sin suprimirlo:

- Aplica a eventos dominio legacy y flujo `PullRequest_Presented` → `pull-request-review`.
- `event-watcher` + `route-domain-event` + `event-sweeper` sobre `eda_bus.*`.
- **No** confundir con rutas fractal: son buses paralelos durante transición gradual.

Diagrama mermaid existente se conserva bajo esta subsección con nota de coexistencia.

## 4. Sección §6.B — Agentes del Core

### 4.1 Fila Radamanto

| Agente | Rol (una línea) |
|--------|------------------|
| **Radamanto** | Actuario de confianza: batching de telemetría CLI, umbrales deterministas, sellado DLT de estatus de herramientas (`Tool_Degraded`, `Status_Restored`, `Tool_Deprecated`). |

Enlace: [`radamanto.md`](../../../SddIA/agents/radamanto.md). Actualizar nota de integridad en [`agents/index.md`](../../../SddIA/agents/index.md) si README declara conteo desactualizado (solo si auditoría detecta contradicción — preferir README autocontenido).

### 4.2 Delimitación Argos vs Radamanto

| Agente | Jurisdicción |
|--------|--------------|
| **Argos** | Materia: calidad estructural, artefactos, verificación por evidencia |
| **Radamanto** | Confianza macroscópica: estadística agregada, DLT estatus herramientas |

### 4.3 Self-Healing (alto nivel)

Lista o diagrama secuencia simplificado:

`Tool_Degraded` → Cerbero revoca → sandbox Tekton/Dédalo → Argos valida estructura → telemetría exitosa → Radamanto `Status_Restored` → Cerbero rehabilita. Límite `max_recovery_attempts` → `Tool_Deprecated`.

Referencia feature: [`telemetria-reactiva-eda-fase4`](../../telemetria-reactiva-eda-fase4/).

## 5. Sección §6.C — Orquestación multi-agente

Reescritura del bloque actual:

1. **Process** fija agente titular por fase.
2. **Workspace dinámico** instanciado por CLI: `.SddIA/workspaces/{process_name}/{execution_id}/` (SSOT `paths.workspacesRoot` + `workspace_template` del proceso).
3. Coordenada inyectada en payload táctico (`workspace_path`); ED operan con **Ceguera Espacial**.
4. **`persist_ref`** (`docs/features/…`) permanece documentación de la tarea — **ortogonal** al workspace operativo (D2.5).
5. Persistencia encapsulada: ED invocan `filesystem-manager` vía `capsule-json-io`; **prohibida** escritura directa a disco desde agentes.

Deprecar narrativa que presente `featurePath`/`fixPath` como destino operativo de ejecución.

## 6. Sección §6.D — Aduana Universal (CLI)

Nueva sección (o integrada tras Orquestación):

### 6.1 Peaje Termodinámico

- Cronómetro antes de ejecutar cápsula.
- Al finalizar: `exit_code`, `duration_ms`, `asset_id`.
- Emisión `Raw_Execution_Finished` (familia `telemetry`) en `./.events/telemetry/`.
- Fail-soft D3.13: fallo E/S telemetría no detiene hilo de negocio.

### 6.2 Recibos termodinámicos (Fase 5)

- Extracción opcional `telemetry_receipt` desde stdout cápsula.
- Omisión **no** falla ejecución de negocio.
- Contratos ED: `telemetry_provided` / `telemetry_schema` en skills/actions.

### 6.3 Auditoría compliance

- Proceso `telemetry-compliance-audit` suscrito en paralelo a telemetría.
- Incumplimiento → `Telemetry_Compliance_Breached` en `./.events/domain/`.
- Gobernanza reactiva post-breach: **pendiente** (enlace a PBI §5.D / Kaizen).

Referencia feature: [`telemetria-reactiva-eda-fase5`](../../telemetria-reactiva-eda-fase5/).

## 7. Sección §6.E — Tabla ontología y SSOT

### 7.1 Fila Event (propuesta)

| Entidad | Finalidad | Ubicación Core | Relación operativa |
|---------|-----------|----------------|-------------------|
| **Event** | Contrato inmutable ECST; clasificado por `event_family`. | `paths.directories.events` — genoma fractal `{telemetry,orchestration,domain}/` | Instancia en bus fractal (`eda_fractal`) o pipeline V3+ (`eda_bus`) según familia y transición |

### 7.2 Fila Process (propuesta)

Añadir a descripción existente: cada proceso declara `workspace_template` obligatorio (process-contract v1.4.0); CLI materializa workspace bajo `paths.workspacesRoot`.

### 7.3 Referencias cumulo.paths.json

- `paths.workspacesRoot`: `.SddIA/workspaces/`
- `eda_fractal.*`: rutas bus fractal
- `eda_bus.*`: pipeline V3+
- `featurePath` / `fixPath`: aliases documentales deprecated → `directories.documentation`

## 8. Sección §6.F — Enlaces y coherencia

Checklist auditoría Tekton/Argos:

| Verificación | Método |
|--------------|--------|
| Enlaces relativos README | Manual + grep paths citados |
| Coherencia vs `cumulo.paths.json` v1.4.0 | Diff semántico tablas rutas |
| Coherencia vs Códices familia | Spot-check 3 `index.md` |
| Sin bus monolítico como único modelo | Lectura § Eventos completa |
| Features referencia existentes | Glob `docs/features/telemetria-reactiva-eda-fase*` |

Enlaces recomendados a añadir:

- [`events-contract.md`](../../../SddIA/events/events-contract.md)
- [`process-contract.md`](../../../SddIA/process/process-contract.md)
- [`telemetria-reactiva-eda-fase0/impact-analysis.md`](../../telemetria-reactiva-eda-fase0/impact-analysis.md) (opcional, pie de § Eventos)

## 9. Cierre documental PBI maestro (Done global)

Única fase con archivo del PBI:

| Paso | Acción |
|------|--------|
| 1 | Mover `docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md` → `docs/todos/done/` |
| 2 | Actualizar frontmatter PBI: `status: done`, tabla fases 6 ✅ |
| 3 | `validacion.md`: `global: APTO`, `pbi_archived: true`, `branch: feat/telemetria-reactiva-eda-fase6` |

## 10. Tests / verificación (sin código nuevo)

| Check | Tipo |
|-------|------|
| AC6.1–AC6.5 | Checklist manual en `validacion.md` |
| Enlaces rotos | Grep + lectura diff |
| Regresión QA | **No** requerida (doc-only); opcional smoke si se tocara script |

## 11. Touchpoints resumidos

| Archivo | Tipo cambio |
|---------|-------------|
| `README.md` | Reescritura/ampliación §§ Eventos, Agentes, Orquestación; nueva § Aduana; ontología |
| `docs/todos/pending/…PBI….md` | Mover a `done/` + frontmatter |
| `docs/features/telemetria-reactiva-eda-fase6/*` | Cascada documental feature |
| Otros | **Prohibido** salvo enlace roto (T6.1) |

## 12. Criterios de aceptación (PBI)

| AC | Verificación spec |
|----|-------------------|
| **AC6.1** | §3 Trinidad + rutas fractal |
| **AC6.2** | §4 Radamanto + delimitación Argos |
| **AC6.3** | §5 workspaces |
| **AC6.4** | §6 Peaje + Raw_Execution_Finished |
| **AC6.5** | §8 auditoría coherencia SSOT |

## 13. Fuera de alcance

- Profundizar implementación Radamanto/compliance (ya en features fase 4–5).
- Actualizar plantillas starter-kit EDA (Kaizen separado).
- Modificar Códices `index.md` de familia (salvo enlace desde README).

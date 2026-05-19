---
pbi_type: arquitectura
pbi_version: V3
ola: C
source_pdf: "[ARQUITECTURA] Especificación Técnica Avanzada_ El Genoma de Eventos y Coreografía Asíncrona (Ola C) V3.pdf"
feature_ref: docs/features/ola-c-event-entity/
merged_pr: "https://github.com/racso80es/SddIA/pull/5"
merge_commit: "35cd940"
closed_at: "2026-05-19"
status: entregado
---

# PBI — Ola C V3: Genoma de Eventos y Coreografía Asíncrona

Documento de seguimiento operativo del PBI de arquitectura. Complementa el PDF homónimo en esta carpeta con el estado **real** de implementación tras el merge en `main`.

**Trazabilidad feature:** [docs/features/ola-c-event-entity/](../features/ola-c-event-entity/)  
**PR cerrado:** [#5](https://github.com/racso80es/SddIA/pull/5) → merge `35cd940`

---

## Resumen ejecutivo

| Ámbito PBI | Estado | Notas |
|------------|:------:|-------|
| §1 Principios fundacionales | ✅ Parcial | Watcher ciego + acciones desacopladas operativos; suscripción por finalidades vía `event-subscriptions.json` |
| §2 Ontología Evento (Clase vs Instancia) | ✅ | Genoma `SddIA/events/`; instancias ECST en bus runtime |
| §3 Impacto sistémico (Core) | ✅ | Constitución, README, entity-manager, índice |
| §4 Topología fractal `.SddIA/events/` + receipts | ⚠️ Desviación | Runtime en `docs/events/` (laudo D10); sin subcarpetas `receipts/` |
| §5 Ciclo coreografiado (receipts + sweeper) | ⏳ Pendiente | Fan-out síncrono actual; sin `event-sweeper.py` ni recibos atómicos |
| §6 Hoja de ruta / cierre | ✅ | Feature completa Fases 1–6; CI verde; merge en `main` |

---

## §1 — Principios fundacionales

- [x] **Ceguera del watcher:** `event-watcher.py` promueve `pending/` → `processing/` y delega enrutamiento sin lógica de negocio embebida.
- [x] **Agnosticismo de acciones:** `emit-domain-mutation`, `emit-pr-merged-event` persisten ECST sin conocer suscriptores.
- [x] **Suscripción por finalidades:** `SddIA/core/event-subscriptions.json` — mapa `event_type` → agente + action/tool + intent.
- [x] **Validación ECST pre-fan-out (extensión Ola C):** instancia vs Clase catalogada antes de despachar suscriptores (`route-domain-event` Paso 2b).

---

## §2 — Ontología del Evento como entidad nativa

- [x] Clases de Evento en `SddIA/events/{name}.md` (kebab-case; sin carpetas anidadas artificiales).
- [x] Identidad inmutable por Clase: `uuid`, `version` (SemVer), `hash_signature`, `event_type` (ECST PascalCase_Snake).
- [x] Contrato de esquema de payload: tablas **REQUIRED / OPTIONAL / FORBIDDEN** en cada Clase + norma en `events-contract.md`.
- [x] Validación runtime de payload contra Clase en el enrutador (violación → `dead-letter/`).
- [ ] Validación en emisor (`emit-domain-mutation`) antes de escribir en disco — deuda; emisores Ola A no abortan aún por esquema.

### Clases ECST forjadas (5)

| Archivo | `event_type` | Estado |
|---------|--------------|:------:|
| `pull-request-merged.md` | `PullRequest_Merged` | ✅ |
| `pull-request-presented.md` | `PullRequest_Presented` | ✅ |
| `domain-entity-created.md` | `Domain_Entity_Created` | ✅ |
| `domain-entity-updated.md` | `Domain_Entity_Updated` | ✅ |
| `domain-entity-deleted.md` | `Domain_Entity_Deleted` | ✅ |

---

## §3 — Impacto sistémico e inserción holística

| Componente Core (PBI §3) | Tarea | Estado | Artefacto / evidencia |
|--------------------------|-------|:------:|------------------------|
| **Constitución Core** | Incluir Evento de Dominio como entidad elemental; prohibir comunicación directa invisible entre agentes | ✅ | `SddIA/CONSTITUTION_CORE.md` §3.1 |
| **README.md** | Mapa de rutas: genoma, runtime, personalización Vía C | ✅ | `README.md` — ontología Event + tres rutas |
| **entity-manager** | Admitir `entity_class: "event"`; delegar en `event-creator` | ✅ | `SddIA/process/entity-manager.md`; piloto en `execute-process.py` |
| **SddIA/events/index.md** | Índice tabular de Clases | ✅ | 5 filas sincronizadas; columnas: uuid, name, event_type, version, contract, context, Capabilities |
| **Proceso event-creator** | Forja automatizada de Clases + índice | ✅ | `SddIA/process/event-creator.md`; catálogo en `process/index.md` |
| **Contrato de familia** | Norma ECST transversal | ✅ | `SddIA/events/events-contract.md` v1.0.0 |
| **Cúmulo SSOT** | Rutas `directories.events`, `eda_bus`, `eda_instance`, `contracts.events` | ✅ | `SddIA/core/cumulo.paths.json` |
| **emit-domain-mutation** | Enum `entity_class` incluye `event` | ✅ | `SddIA/actions/emit-domain-mutation.md` |
| **Consumidores bus** | Alineación a `docs/events/` | ✅ | `event-watcher.py`, `execute-process.py`, `accept-pr.md`, `delivery-close-cycle.md`, `.gitignore` |
| **interaction-triggers** | Intención `intent.create_event` → `event-creator` | ✅ | `SddIA/norms/interaction-triggers.json` |
| **Plantilla Vía C** | Overrides de instancia local | ✅ | `SddIA/templates/eda-instance-events/README.md` → `.SddIA/events/` |
| **CI / QA** | Integridad índices y procesos | ✅ | `.github/workflows/sddia-index-qa.yml` + PyYAML; `verify-process-integrity.py` |

### Desviación respecto al PDF §3 (README)

El PBI V3 original cita runtime oculto en **`.SddIA/events/`**. La implementación cerrada adopta **`docs/events/`** como bus de instancias (laudo `clarify.md` D10, alineado a Cúmulo y consumidores Ola A). **`.SddIA/events/`** queda reservado a **personalización Vía C** (`eda_instance.customization`), no como cola del bus.

---

## §4 — Topología fractal del sistema de archivos

| Elemento PBI | Estado | Implementación actual |
|--------------|:------:|----------------------|
| Cola `pending/` | ✅ | `docs/events/pending/` |
| Cola `processing/` | ✅ | `docs/events/processing/` |
| Terminal `processed/` | ✅ | `docs/events/processed/` |
| Terminal `dead_letter/` | ✅ | `docs/events/dead-letter/` |
| Subcarpetas `receipts/` por estado | ⏳ | No implementado — fuera de alcance Ola C entregada |
| Anidación recibos hijos en fase padre | ⏳ | Pendiente (visión V3 §4–§5) |

---

## §5 — Ciclo de vida coreografiado y gestión asimétrica de fallos

| Paso PBI V3 | Estado | Notas |
|-------------|:------:|-------|
| 1. Captura y aislamiento (watcher → processing) | ✅ | Operativo |
| 2. Despliegue de finalidades + recibos `[UUID].[PURPOSE].notificado` | ⏳ | Fan-out directo; sin recibos atómicos |
| 3. Sello del recibo (middleware `.procesado` / `.error`) | ⏳ | `delivery_state` en JSON del evento (ledger simplificado Ola A) |
| 4. Recolección (`event-sweeper.py`, duplicación asimétrica) | ⏳ | No existe `event-sweeper.py` en repo |

**Entregado en Ola C:** enrutador monolítico (`route-domain-event` / `event-watcher.py`) con `delivery_state` por agente suscriptor e idempotencia parcial en reintentos.

---

## §6 — Conclusión y cierre de planificación

- [x] Modificaciones Constitución, README, índices tabulares y extensión `entity-manager` materializadas.
- [x] Documentación feature completa: `objectives`, `clarify`, `spec`, `plan`, `implementation`, `execution`, `validacion`.
- [x] Veredicto Argos **APTO** (`validacion.md`).
- [x] PR [#5](https://github.com/racso80es/SddIA/pull/5) mergeado en `main` (`35cd940`).
- [x] CI `sddia-index-qa` en verde post-merge.

---

## Commits de entrega (referencia)

| Commit | Descripción |
|--------|-------------|
| `291aa25` | Topología Ola C — README, consumidores, `.gitignore` |
| `430c0a1` | Bus runtime `docs/events/` |
| `9ebd37d` | Hito 1 — Constitución §3.1, `events-contract`, índice |
| `9492d84` | `event-creator` |
| `c530a83` | `entity-manager` piloto `event` |
| `5e8c0ad` | 5 Clases ECST forense |
| `ad78616` | Validación cruzada ECST + plantilla Vía C |
| `57be3f3` | Cierre Argos — `execution.md`, `validacion.md` |
| `1a5b8ee` | CI — PyYAML para verify-process-integrity |
| `77d3b3e` | Hash `delivery-close-cycle` recalculado |

*(Merge squash en `main`: `35cd940`)*

---

## Backlog derivado del PBI (post-Ola C)

| ID | Tarea | Prioridad |
|----|-------|-----------|
| C+1 | `event-sweeper.py` + subcarpetas `receipts/` según §4–§5 V3 | Media |
| C+2 | Middleware de recibos `.procesado` / `.error` por finalidad | Media |
| C+3 | Validación de payload en emisores antes de `WRITE_FILE` | Media |
| C+4 | `payload_schema_hash` REQUIRED en emisores genómicos | Baja |
| C+5 | Fusión runtime de `event-subscriptions.local.json` (Vía C) | Baja |
| C+6 | Indexar acción `emit-pr-presented-event` | Baja |

---

*Última actualización: 2026-05-19 — post-merge PR #5.*

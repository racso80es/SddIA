---
feature_name: telemetria-reactiva-eda-fase0
created: "2026-05-27"
process: feature
phase: 0
master_pbi_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
barrido_date: "2026-05-27"
scope: SddIA/ + SddIA/scripts/ + acoplamientos docs/norms
---

# Análisis de impacto — Telemetría Reactiva EDA (Fase 0)

## Resumen ejecutivo

El Core SddIA opera hoy con un **bus EDA V3+ monolítico** (`eda_bus.pending` → `route-domain-event` → testigos → `event-sweeper`), un **genoma de eventos plano** (7 clases en `SddIA/events/` raíz, sin `event_family`) y **persistencia de tareas acoplada a software** vía literales `docs/features` / `docs/fixes` en scripts QA, pese a que las normas exigen `paths.featurePath` / `paths.fixPath` vía Cúmulo.

La forja propuesta (Fases 1–6) es **viable** si se adopta **coexistencia gradual**: mantener el pipeline V3+ para eventos de dominio actuales mientras se introduce el bus fractal (`./.events/{telemetry,orchestration,domain}/`) y enrutadores dedicados. Los bloqueantes principales son: (1) ausencia de taxonomía y rutas fractales, (2) CLI sin Peaje Termodinámico ni `Raw_Execution_Finished`, (3) DLT hoy exclusivo de **Cúmulo** vía `iota-immutable-publisher` en suscripciones, (4) `workspace_template` inexistente en procesos, (5) `paths.featurePath` referenciado en normas pero **no declarado** en `cumulo.paths.json` universal.

**Radamanto** no existe en el genoma; la transición DLT debe planificarse en Fase 4 sin romper CI/IOTA actual (Cúmulo sigue anclando hasta handoff).

---

## Metodología (0.A)

Barrido con `rg` sobre el workspace (ámbito principal `SddIA/`, `SddIA/scripts/`, normas y plantillas), lectura de SSOT (`cumulo.paths.json`, `event-subscriptions.json`, `events/index.md`, `route_domain_event_core.py`, `execute_process_capsules.py`, `event-watcher.py`) y contraste con tareas PBI Fases 1–6.

---

## Inventario de acoplamientos

| ID | Ubicación | Hallazgo | Fase | Severidad | Gap |
|----|-----------|----------|------|-----------|-----|
| H01 | `SddIA/events/` (7× `.md` + `index.md`) | Topología plana; sin `telemetry/` / `orchestration/` / `domain/`; sin campo `event_family` | 1 | **Bloqueante** | (c) |
| H02 | `SddIA/events/events-contract.md` | No exige `event_family`; instancias solo en `eda_bus.*` | 1 | **Bloqueante** | (b) |
| H03 | `SddIA/process/event-creator.md` | `create-event` sin input `event_family` ni enrutado a subcarpeta | 1 | Alto | (b) |
| H04 | `SddIA/core/cumulo.paths.json` | Solo `eda_bus` V3+ (`pending/processing/processed/dead-letter`); sin rutas `telemetry/orchestration/domain` | 1, 3 | **Bloqueante** | (c) |
| H05 | `SddIA/core/event-subscriptions.json` | Un solo registro; mezcla PR, dominio, Kaizen; todo vía `route-domain-event` / watcher | 3 | **Bloqueante** | (b) |
| H06 | `SddIA/scripts/daemons/event-watcher.py` | Solo observa `pending/`; delega en `route-domain-event` | 3 | Alto | (b) |
| H07 | `SddIA/scripts/daemons/event-sweeper.py` | Recolector V3+; sin ruta `telemetry/` ni purga post-batch | 3, 4 | Alto | (b) |
| H08 | `SddIA/process/route-domain-event.md` | Único enrutador; no existe `route-telemetry` / `route-orchestration` | 3 | **Bloqueante** | (c) |
| H09 | `SddIA/scripts/qa/route_domain_event_core.py` | Dispatch físico: `tool:iota-immutable-publisher`, `action:sync-entity-index`, procesos; fallback `persist_ref` hardcodeado `docs/features/remove-cli-legacy-compat` | 3, 4 | Alto | (b) |
| H10 | `event-subscriptions.json` | `cumulo` + `iota-immutable-publisher` en `PullRequest_*`, `Domain_Entity_*` | 4 | **Bloqueante** (AC0.4) | (c) |
| H11 | `SddIA/agents/cumulo.md` | Jurisdicción SSOT y DLT vía herramienta indexada; sin Radamanto | 4 | Alto | (c) |
| H12 | Genoma `SddIA/agents/` | **Sin** `radamanto.md` / contrato | 4 | Alto | (a) cubierto por PBI |
| H13 | `execute_process_capsules.py` | `default_docs = docs/features|fixes`; `write_pending_event` → único bus; sin cronómetro ni telemetría | 2, 3, 5 | **Bloqueante** | (c) |
| H14 | `eda_bus_utils.infer_persist_ref_from_branch` | Inferencia `docs/features/{slug}` y `docs/fixes/{slug}` | 2 | Alto | (b) |
| H15 | `SddIA/norms/paths-via-cumulo.md`, `process/feature.md` | Documentan `paths.featurePath` / `fixPath` | 2, 6 | Medio | (b) |
| H16 | `SddIA/core/cumulo.paths.json` | **No declara** claves `paths.featurePath` / `paths.fixPath` (solo `directories.documentation: docs`) | 2 | **Bloqueante** | (c) |
| H17 | Procesos `SddIA/process/*.md` | **Ninguno** declara `workspace_template` (solo en PBI/tmp) | 2 | **Bloqueante** | (c) |
| H18 | `skills/actions-contract` (Core) | Sin `telemetry_provided` / `telemetry_receipt` | 5 | Medio | (a) |
| H19 | `SddIA/skills/filesystem-manager.md` | Existe; ED deben delegar — patrón en creators, no universal en agentes obreros | 3 | Medio | (b) |
| H20 | `README.md` (raíz) | Describe bus V3+ / `route-domain-event`; no Trinidad ni Radamanto | 6 | Medio | (a) |
| H21 | `SddIA/templates/eda-instance-events/README.md` | Plantilla instancia alinea V3+; riesgo colisión si instancia override sin fractal | 1, 3 | Alto | (b) |
| H22 | `.SddIA/` (instancia repo) | `eda-local-topology-test`; sin `workspaces/` ni bus fractal | 2, 3 | Informativo | (d) |
| H23 | `SddIA/scripts/qa/test_eda_bus_v3plus.py` | Tests acoplados a topología V3+ | 1, 3 | Alto | (b) |
| H24 | Flags `SDDIA_LAB_*` | Lab simula IOTA/sync; telemetría real no ejercitada en CI salvo `e1-iota-ci` | 3, 5 | Medio | (d) |
| H25 | `SddIA_1`…`SddIA_4` (legado) | Duplican referencias `paths.featurePath` en README/contratos | 6 | Informativo | (d) fuera Core unificado |
| H26 | `docs/todos/tmp/*.md` | PBIs superseded; no ejecutar | — | Informativo | (d) |

**Leyenda gap:** (a) ya cubierto por PBI · (b) ampliar tarea existente · (c) nueva subtarea/decisión · (d) fuera de alcance / residual

---

## Matriz `featurePath` / `fixPath` (AC0.3)

| Categoría | Ubicaciones | Clasificación | Fase |
|-----------|-------------|---------------|------|
| **Normativa Core** | `paths-via-cumulo.md`, `entidades-dominio-ecosistema-sddia.md`, `touchpoints-ia.md`, starter-kit `features-documentation-pattern.md` | Mantener hasta Fase 2; migrar narrativa a `directories.documentation` + workspaces | 2, 6 |
| **Runtime Python (crítico)** | `execute_process_capsules.py`, `eda_bus_utils.py`, `verify-task-closure.py`, `route_domain_event_core.py` | Sustituir literales por resolución Cúmulo + `workspace_template` | 2, 3 |
| **SSOT ausente** | `cumulo.paths.json` sin bloque `paths.featurePath` | **Añadir** o deprecar claves en favor de `.SddIA/workspaces/` (PBI §2.D) | 2 |
| **Histórico / legado** | `SddIA_1`…`SddIA_4/process/README.md` | No bloqueante si Core unificado es SSOT | (d) |
| **Documentación viva** | `docs/features/*`, `docs/fixes/*` (persist_ref real hoy) | Conviven con workspaces hasta migración; features en curso usan rutas actuales | 2 |

---

## Jurisdicción DLT: Cúmulo vs. Radamanto (AC0.4)

| Hoy | Propuesta PBI | Transición recomendada |
|-----|---------------|------------------------|
| `event-subscriptions.json`: **solo** `agent: cumulo` + `tool: iota-immutable-publisher` para PR y mutaciones de dominio | Radamanto: exclusividad DLT para sellado de estatus (`Tool_Degraded`, `Status_Restored`) | **Fase 4:** introducir Radamanto; **mantener** Cúmulo en `PullRequest_Merged` / `Domain_Entity_*` hasta PR de handoff y actualización `e1-iota-ci` |
| `route_domain_event_core.py` invoca binario IOTA directamente | Radamanto batch desde `./.events/telemetry/` | **Fase 3:** cablear suscripción telemetría sin consumidor; **Fase 4:** mover sellado estatus a Radamanto; Cúmulo conserva anclaje PR/entidad hasta acta de migración |
| CI `e1-iota-ci` valida witness `cumulo.iota-immutable-publisher` | Mismo contrato hasta cambio explícito de suscriptor | Ampliar **Fase 4.E** en PBI: acta CI + smoke dual durante ventana |

**Decisión D0.1 (propuesta):** coexistencia dual en Fase 4 — Radamanto sella eventos de **gobernanza de herramientas**; Cúmulo mantiene **anclaje PR/ECST** hasta feature `telemetria-reactiva-eda-fase4` cierre CI.

---

## Decisiones de diseño (0.C)

| ID | Tema | Decisión | Fases afectadas |
|----|------|----------|-----------------|
| D0.1 | DLT Cúmulo / Radamanto | Coexistencia gradual; handoff documentado en Fase 4 | 4, 6 |
| D0.2 | V3+ vs. bus fractal | **Coexistencia** — dominio actual sigue `eda_bus.pending`; telemetría/orquestación nuevas rutas paralelas | 1, 3 |
| D0.3 | `paths.featurePath` en SSOT | Fase 2: añadir `paths.workspacesRoot: .SddIA/workspaces/` y deprecar feature/fix en `cumulo.paths.json` + refactor scripts | 2 |
| D0.4 | `event-watcher` | Fase 3: watcher multi-ruta o watchers por familia; no big-bang apagado de V3+ | 3 |
| D0.5 | Peaje Termodinámico | Fase 3: extensión `execute_process_capsules` / cápsulas shell — cronómetro + emisión CLI-only | 3 |
| D0.6 | PBI maestro | Permanece `pending/`; subtareas 1.B–3.C refinadas inline (ver § Refinamiento) | 0–6 |

---

## Refinamiento sugerido al PBI maestro (inline)

Añadir o explicitar en el PBI unificado:

1. **Fase 1.1** — Migración de tests `test_eda_bus_v3plus.py` y plantilla `eda-instance-events` tras crear subcarpetas.
2. **Fase 2.B** — Declarar `paths.workspacesRoot` en `cumulo.paths.json` (hoy ausente `featurePath` efectivo en universal).
3. **Fase 3.C** — Plan de migración de `event-watcher` a multi-bus sin romper `PullRequest_Presented` → `pull-request-review`.
4. **Fase 4.0** — Acta de handoff DLT Cúmulo → Radamanto + actualización `e1-iota-ci` / `event-subscriptions.json`.
5. **Fase 3 (nuevo evento)** — Definir esquema `Raw_Execution_Finished` en `create-event` **antes** de cablear CLI (depende Fase 1).

---

## Backlog residual (fuera de Fase 0)

- Limpiar / archivar `SddIA_1`…`SddIA_4` referencias obsoletas (no bloquea Core).
- Consolidar `docs/todos/tmp/` en commit aparte si se desea trazabilidad git de superseded.
- Gate pre-commit `eda-coverage` y deuda orphan — ya tratados en features Kaizen; vigilar tras split de suscripciones.

---

## Criterios de aceptación Fase 0 — autodiagnóstico

| AC | Estado | Nota |
|----|--------|------|
| AC0.1 | ✅ | Este documento |
| AC0.2 | ✅ | H01–H13 bloqueantes con decisión D0.x o refinamiento |
| AC0.3 | ✅ | Matriz `featurePath`/`fixPath` § dedicada |
| AC0.4 | ✅ | § Jurisdicción DLT + D0.1 |
| AC0.5 | ⏳ | Requiere revisión Mayeuta / `clarify.md` tras feedback del Vértice |

---

## Referencias de barrido (muestra)

- Genoma: `SddIA/events/index.md` — 7 ECST en raíz.
- Suscripciones: `SddIA/core/event-subscriptions.json` — 6 tipos, DLT en Cúmulo.
- CLI: `execute_process_capsules.py` L1183 (`docs/features`), L1624 (`write_pending_event`).
- Watcher: `event-watcher.py` — solo `pending/`.
- Contratos skills/actions: sin `telemetry_provided` en Core.

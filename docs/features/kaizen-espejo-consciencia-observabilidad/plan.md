---
feature_name: kaizen-espejo-consciencia-observabilidad
created: "2026-08-29"
process: feature
branch_name: feat/kaizen-espejo-consciencia-observabilidad
persist_ref: docs/features/kaizen-espejo-consciencia-observabilidad
phases: "T0-topologia,T1-core-merge,T2-procesos-eda,T3-bridge,T4-wui,T5-smokes-docs"
uuid: 97d96117-49cf-4db7-b860-acd65bee216a
status: dedalo_locked
agent: dedalo
document_id: PBI-KAIZEN-ESPEJO-CONSCIENCIA-001
execution_id: "a15ad28b-27a3-491c-902e-f78c100ffd43"
---

# Blueprint — kaizen-espejo-consciencia-observabilidad

## Estrategia

Mapa precompilado (Cúmulo, índices) × territorio vivo (Argos/Radamanto/Cerbero) × GET pull en el puente, mismo patrón que el Pulso cognitivo. Batches críticos intangibles.

```text
T0  cumulo.paths.json observability.* + evolution uuid
T1  ecosystem_health_core + tests de color L8
T2  entity-manager: dos procesos + suscripciones Domain_Entity_* + handlers nativos
T3  GET /api/system-health + test dispatch
T4  panel WUI
T5  smokes OBS-CA* + implementation/execution → Argos
```

Laudos **L1–L15** en `spec.md`. Genoma: `entity-manager`. Engine/bridge/WUI/paths: Tekton.

**Parada de este commit:** T0–T5 documentados; **sin** materializar código. Siguiente estímulo = ejecución Tekton.

## Fases

### T0 — Topología

- [ ] `observability.map_snapshot` = `.SddIA/observability/map-snapshot.json`
- [ ] `observability.ecosystem_health` = `.SddIA/observability/ecosystem-health.json`
- [ ] Bump `version` de `cumulo.paths.json`
- [ ] Evolution: hito ↔ uuid `97d96117-49cf-4db7-b860-acd65bee216a`
- Gate: paths resolubles; `.SddIA/observability/` gitignored si el padre `.SddIA` ya lo está

### T1 — Merge Core

- [ ] `ecosystem_health_core.rs`: load fail-soft; compile map (índices MD) **solo** si `compile_map`; fuse L8/L9/L10
- [ ] Tests: rojo missed_cycles; gris sin samples; amarillo degraded; rojo revoked; adapters ausentes; map absent
- Gate: `cargo test -p execute-process ecosystem_health`

**L2:** si `kalma2-bridge` no puede compartir el módulo sin acoplar el CLI, extraer crate mínimo en el mismo PR **antes** de duplicar.

### T2 — Procesos y EDA

```text
./sddia-run.sh --process entity-manager --inputs-file .tmp/entity-manager-<uuid>.json
```

- [ ] `compile-ecosystem-map-snapshot` (Cúmulo, `compile_map` implícito)
- [ ] `query-ecosystem-health` (fusión; `compile_map` default false)
- [ ] Handlers en `mod.rs` / `route` nativo (paridad `daemon-heartbeat-audit`)
- [ ] `event-domain-subscriptions.json`: Created/Updated/Deleted → `compile-ecosystem-map-snapshot`
- [ ] Prefijo RAW Kernel. DA-4: `objectives.md` de esta feature presente
- Gate: `sddia-qa audit-eda-coverage --scan` si toca genoma; grep §5 spec

### T3 — Bridge

- [ ] Paths desde Cúmulo `observability.*`
- [ ] `GET /api/system-health` antes de `serve_static`
- [ ] Test `system_health_route_exists_in_dispatch`
- Gate: `cargo test -p kalma2-bridge`

### T4 — WUI

- [ ] `index.html` sección `aria-label="Espejo de consciencia"` tras pulso cognitivo
- [ ] `app.js`: fetch al boot; pintar filas por `color`; no EventSource
- [ ] `style.css`: clases `.health-green|yellow|red|gray` mínimas
- Gate: revisión estática; browser si el puente corre

### T5 — Cierre documental de ejecución

- [ ] `implementation.md` + `execution.md`
- [ ] `validacion.md` Argos; PBI → `done/` en esta rama
- Gate: patrón v1.2.1; **no** en este commit

## Orden y riesgos

| Riesgo | Mitigación |
|--------|------------|
| Bridge parsea genoma | L4/L9; review grep `SddIA/tools` en `kalma2-bridge` / `app.js` |
| Anidamiento batches | L6; grep T2 |
| Path-dep execute-process hincha el puente | Extraer crate (L2) |
| CA7 vs SSOT adapters | Snapshot omite familia (L4/L15) |
| Tabla MD frágil | Reutilizar parser de índices si existe en engine; no regex ad hoc suelta |

## Fuera de este commit

Código T0–T5, PR, merge.

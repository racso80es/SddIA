---
feature_name: telemetria-reactiva-eda-fase0
created: "2026-05-27"
process: feature
branch_name: feat/telemetria-reactiva-eda-fase0
persist_ref: docs/features/telemetria-reactiva-eda-fase0
master_pbi_ref: docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md
master_pbi_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
phase: 0
pbi_archived_at_close: false
status: en_curso
---

# Objetivos — Telemetría Reactiva EDA · Fase 0 (análisis de afectaciones)

## Misión

Ejecutar la **Fase 0** del PBI maestro `PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO` como **feature independiente**: recorrer `SddIA/` y acoplamientos (scripts, daemons, normas, plantillas, `.SddIA/`) para detectar impactos, deudas y puntos ciegos **antes** de modificar código o contratos de las Fases 1–6.

El PBI unificado permanece en `docs/todos/pending/` como **plan de ruta y control** hasta que todas las fases (0–6) cumplan sus criterios globales. Esta feature **no** archiva el PBI maestro al cerrar (`pbi_archived: false` en `validacion.md`).

## Relación con el programa multi-fase

| Fase PBI | Feature prevista (convención) | Entregable clave |
|----------|-------------------------------|------------------|
| **0** | `telemetria-reactiva-eda-fase0` (esta) | `impact-analysis.md` |
| 1 | `telemetria-reactiva-eda-fase1` (futura) | Genoma fractal + `event_family` |
| 2 | `telemetria-reactiva-eda-fase2` (futura) | Workspaces dinámicos |
| 3–6 | features independientes por fase | Según PBI § Fases 3–6 |

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| Cinco PBI consolidados en `docs/todos/tmp/` | No ejecutar ítems tmp; solo este PBI maestro |
| Fase 0 es **gate de arranque** | Bloquea apertura formal de Fase 1 hasta AC0.x |
| Pipeline EDA V3+ (`pending`/`processing`/…) vigente | Análisis debe explicitar coexistencia vs. bus fractal `./.events/{telemetry,orchestration,domain}/` |
| `featurePath` / `fixPath` aún en normas y procesos | Clasificar en matriz de gaps (AC0.3) |
| Jurisdicción DLT Cúmulo vs. Radamanto | Explicitar propuesta de transición (AC0.4) |

## Objetivos medibles (Fase 0)

| ID | Objetivo | Criterio (AC PBI) |
|----|----------|-------------------|
| **F0-O1** | **Inventario de acoplamientos** | Barrido sobre `.events/`, suscripciones, `persist_ref`, `featurePath`/`fixPath`, emisores, CLI; tabla ubicación × fase × severidad |
| **F0-O2** | **Matriz de gaps** | Hallazgos clasificados: cubierto / ampliar tarea / nueva subtarea / fuera de alcance |
| **F0-O3** | **Decisiones bloqueantes** | Ítems bloqueantes con decisión o subtarea asignada a Fase 1–6 antes de abrir Fase 1 |
| **F0-O4** | **Entregable `impact-analysis.md`** | Resumen ejecutivo + hallazgos + decisiones + backlog residual en `persist_ref` |
| **F0-O5** | **Refinamiento PBI maestro** | Subtareas aprobadas incorporadas al PBI pending (commit en rama de esta feature) |
| **F0-O6** | **Validación ejecutabilidad** | Mayeuta / clarificación: Fases 1–6 refinadas sin ambigüedad bloqueante (AC0.5) |

## Ámbito de exploración (checklist PBI § Fase 0)

- Genoma de eventos (`SddIA/events/`)
- Runtime EDA (`event-watcher`, `event-sweeper`, `route-domain-event`, topología V3+)
- CLI / orquestación (`execute-process`, `execute-action`, cápsulas)
- SSOT rutas (`cumulo.paths.json`, `persist_ref`)
- Suscripciones (`event-subscriptions.json` y consumidores)
- Agentes y RBAC (Cerbero, Cúmulo, futuro Radamanto)
- Contratos ED, cápsulas, normas, tests QA, instancia `.SddIA/`

## No objetivos (esta feature)

- Implementar Trinidad de Estímulos, workspaces, Aduana Universal, Radamanto ni tokens (Fases 1–5).
- Actualizar `README.md` raíz (Fase 6).
- Mover el PBI maestro a `docs/todos/done/` ni declarar Done global del programa.
- Cambios de código productivo salvo documentación de análisis y refinamiento del PBI maestro.

## Ley aplicada

- `features-documentation-pattern` v1.2.1
- Proceso `feature` v1.3.0
- PBI maestro: `docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md` § Fase 0

## Artefactos previstos

| Artefacto | Propósito |
|-----------|-----------|
| `clarify.md` | Decisiones y escalados (0.C) |
| `spec.md` | Metodología de barrido y plantillas de hallazgo |
| `plan.md` | Secuencia 0.A → 0.D |
| `impact-analysis.md` | Entregable principal (AC0.1–AC0.4) |
| `implementation.md` / `execution.md` | Trazabilidad del barrido |
| `validacion.md` | Argos; `pbi_archived: false` |

## Estado

| Fase feature | Estado |
|--------------|--------|
| Objetivos | ✅ Este documento |
| Inicialización Git (`workspace-init`) | ✅ 2026-05-27 |
| Clarificación (Mayeuta) | ⏳ |
| Especificación (Dedalo) | ⏳ |
| Planificación | ⏳ |
| Análisis / `impact-analysis.md` | ✅ 2026-05-27 |
| Validación Argos | ⏳ |

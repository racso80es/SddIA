---
feature_name: eda-coverage-ssot-bus-isolation
created: "2026-05-25"
process: feature
branch_name: feat/eda-coverage-ssot-bus-isolation
persist_ref: docs/features/eda-coverage-ssot-bus-isolation
pbi_ref: docs/todos/pending/[Kaizen] EDA cobertura durable, aislamiento bus y smoke e2e — SSOT eda-coverage.md
document_id: PBI-KAIZEN-EDA-COVERAGE-SSOT-BUS-ISOLATION
---

# Objetivos — SSOT eda-coverage y desacople bus EDA

## Misión

Desacoplar la **Aduana genómica** (`pre-commit`, `delivery-close-cycle`) del ciclo de vida efímero del bus de archivos, introduciendo `SddIA/core/eda-coverage.json` como SSOT de correlación entidad ↔ sello EDA; restaurar el job CI **`eda-bus-e2e-smoke`** en verde manteniendo forja lab con `scope: local` y barrido absoluto (sweep vacío).

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| Mitigación `eda-orphan-debt-precommit` cerrada | Workaround retención cabeceras activo; deuda arquitectónica explícita |
| 46 entidades indexadas en genoma Core | SSOT debe cubrir cada `entity_uuid` en catálogos |
| Lab E2E usa `origin_topology: local` | Suscriptores core-only no aplican; sweep incompleto |
| CI ejecuta lab sobre bus compartido `.events/` | Riesgo contaminación; requiere `EVENT_BUS_PATH` aislado |
| `load_eda_bus()` fija default `.events` | Parametrización env sin romper cumulo SSOT |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | **SSOT correlación durable** | `eda-coverage.json` versionado; ref en `cumulo.paths.json` |
| **O2** | **Aduana desacoplada del bus** | `--scan` usa solo `coverage_matrix`; no gate sobre `iter_bus_event_files()` |
| **O3** | **Emisión doble fase** | `emit-domain-mutation`: upsert SSOT antes de pending |
| **O4** | **Sweep vacío** | Sin retención cabeceras; V2 PASS post-watcher |
| **O5** | **Aislamiento bus test** | Lab/CI usan `.tmp/events_test` vía `.env.test` |
| **O6** | **Smoke E2E verde** | `run-eda-e2e-lab.py --json` → exit 0; CI job SUCCESS |
| **O7** | **Router/sweep topológico** | Evento local sin suscriptores aplicables → padre purgado |
| **O8** | **Cierre documental** | `validacion.md` APTO + PBI en `done/` (un PR) |

## No objetivos

- Sustituir anclaje Merkle / DLT de Fase C como prueba de auditoría externa.
- Pre-commit incremental por diff staged (futuro).
- Añadir suscriptor lab permanente en `event-subscriptions.json` prod.
- Revertir `scope: local` en forja lab.
- Cerrar PBIs `Argos_Eda_Emision` ni `NuevoAgenteCertificador`.

## Ley aplicada

- `features-documentation-pattern` v1.2.0
- Proceso `feature` v1.3.0
- Cierre documental en rama (`.cursor/rules/task-closure-documental.mdc`)
- Jerarquía bóvedas: `docs/features/ampliacion-configuracion-entornos/`

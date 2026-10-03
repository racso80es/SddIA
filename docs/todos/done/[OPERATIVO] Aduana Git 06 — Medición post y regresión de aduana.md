---
document_id: PBI-MERGE-THERMO-06-E2E-MEASURE
uuid: "bdcc77c7-93db-4cde-a8c9-580db5d31fcc"
title: "[OPERATIVO] Aduana Git 06 — Medición post y regresión de aduana"
format: markdown
version: "1.0.0"
status: done
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 6
hu_order_total: 6
historia_ref: "docs/todos/historias/[OPERATIVO] Optimización Termodinámica de Aduana Git: Merge de Alta Eficiencia.md"
historia_document_id: HU-MERGE-THERMODYNAMICS
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-MERGE-THERMO-01-OBSERVABILITY
  - PBI-MERGE-THERMO-02-NET-PRUNE
  - PBI-MERGE-THERMO-03-DELTA-PROFILE
  - PBI-MERGE-THERMO-04-ATTESTATION
  - PBI-MERGE-THERMO-05-DOC-PARITY
baseline_decisiones:
  - "AC-7 compara contra el baseline del PBI 01. No se re-muestrea la cadena vieja."
  - "Este PBI no añade comportamiento. Mide y cierra la HU."
---

# Medición post y regresión de aduana

HU `HU-MERGE-THERMODYNAMICS`, orden **06/06**. AC-7. Se forja cuando 01–05 están mergeados.

## 1. Intención

Demostrar, con las mismas sondas del PBI 01, que la aduana nueva es más corta y que los gates que ya bloqueaban siguen bloqueando.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | ≥10 muestras post en `hook-timings` para rama nueva con delta pasivo y ≥10 para rama nueva con delta activo y atestación `full` válida. |
| R-2 | Mediana `total_ms` post ≤ 10 % de la mediana baseline en delta pasivo (reducción ≥ 90 %). |
| R-3 | Mediana `total_ms` post ≤ 50 % de la mediana baseline en delta activo con atestación (reducción ≥ 50 %). |
| R-4 | Regresión única: pre-commit bloquea integridad rota y huérfanos con genoma en staged; `gate-evolution` bloquea material en rango; veto de push a `main`; atestación inválida no abre atajo; `PullRequest_Merged` borra la atestación. |
| R-5 | `validacion.md` con `global: APTO`, tablas baseline (PBI 01) y post, y `pbi_archived: true` al mover este PBI a `docs/todos/done/` en la misma rama. |

## 3. Plan

1. Campaña de 20 pushes de laboratorio (10 pasivos, 10 activos atestados).
2. Tabla comparada contra el baseline del PBI 01.
3. Pasada de regresión R-4.
4. Cierre documental de este PBI. Los PBI 01–05 cierran en sus propias ramas.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-7 | Tablas baseline y post en `validacion.md`. Reducción de mediana ≥ 90 % (pasivo) y ≥ 50 % (activo atestado). | `hook-timings` JSONL |
| AC-5 | Los cuatro gates citados en R-4 bloquean en la pasada de regresión. | Suite `sddia-qa` + tests shell de la HU |

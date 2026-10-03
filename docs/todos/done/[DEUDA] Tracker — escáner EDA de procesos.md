---
document_id: PBI-DEUDA-EDA-SCAN-PROCESOS
uuid: "4393da04-37b3-44fe-9f95-688774a1b643"
title: "[DEUDA] Tracker — escáner EDA ciego a procesos sin backticks"
format: markdown
version: "1.1.0"
status: pending
priority: media
type: deuda
process: bug-fix
dispatch: true
feature_name: eda-scan-process-index
historia_ref: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
historia_document_id: HU-SDDIA-TRACKER-LINEAR-001
historia_uuid: "26209dff-e413-4c6d-8838-5b785251356c"
tracker_ref: OSC-9
created: "2026-10-02"
author: tekton
updated: "2026-10-02"
refined: "2026-10-02"
especificacion_cerrada: "2026-10-02"
execution_mode: autonomo
cola_ejecucion: docs/todos/pending/
derived_from_pr: "https://github.com/racso80es/SddIA/pull/316"
blocked_by:
  - PBI-DEUDA-TRACKER-GENOMA-FORJA
unblocks: []
---

# Escáner EDA y el índice de procesos

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001`.

## 0. Filtro A

| Afirmación de la semilla | Realidad | Corrección |
|--------------------------|----------|------------|
| `scan_orphans` solo indexa filas con backticks | Cierto: `parse_index_uuids` descarta la línea si no contiene `` ` `` (`eda_coverage.rs`). | Se mantiene. |
| Solo los tres `tracker-*` quedan fuera | En `SddIA/process/index.md` hay 29 filas sin backticks. Al quitar el filtro, cinco uuid sin cobertura pasan a huérfanos: `tracker-backlog-query`, `tracker-sync-replay`, `tracker-stamp`, `phagocyte-recovered-fracture-pbis`, `sync-client-assets`. | Los dos ajenos al tracker son colateral de este arreglo. Hay que cubrirlos en el mismo cambio o el gate queda rojo. |
| `tracker-linear-markdown-sync` está en el mismo hueco | Su fila contiene `` `tracker_ref` ``, el escáner ya la ve, y `eda-coverage.json` la tiene con hash no placeholder. | No entra en el backfill de cobertura. |
| `orphan_count: 0` también absuelve eventos y la norma | El escáner lee `SddIA/events/index.md` (familias, sin uuid de clase). No lee `events/domain/index.md`. `work-initiated` y `tracker-sync-failed` no están en la matriz y tampoco salen como huérfanos. `execution-contexts.md` vive en `SddIA/norms/`; el escáner de normas mira `SddIA/library/norms`. | Ese agujero no se cierra aquí. Lo sella el PBI de forja en la matriz, sin cambiar el recorrido de directorios. |

## 1. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Una fila `\| nombre \| uuid \|` del índice de procesos entra al inventario aunque no haya backticks. El uuid sigue teniendo que cumplir el regex de versión que ya usa el escáner. |
| R-2 | Los tres procesos tracker quedan cubiertos por el sello del PBI de forja (evento de forja, no un `sha256:deadbeef` nuevo). Este PBI no re-ancla. |
| R-3 | `phagocyte-recovered-fracture-pbis` y `sync-client-assets` quedan `is_covered: true` en el mismo cambio, sin mutar su comportamiento. Ancla mínima válida para el gate, no un rediseño. |
| R-4 | Test: una fila sintética sin backticks y sin cobertura incrementa `orphan_count`. Con cobertura, no. |

## 2. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-1 | Tras el parser nuevo, quitar la cobertura de `tracker-stamp` hace que `--scan` reporte `orphan_count >= 1`. Restaurarla deja el recuento en el mismo valor que antes de quitarla. |
| AC-2 | Con los cinco uuid cubiertos, `--scan` vuelve a `orphan_count: 0`. |

## 3. Fuera de alcance

- Recorrer `SddIA/events/{domain,orchestration,telemetry}/index.md`.
- Indexar `SddIA/norms/execution-contexts.md` desde el escáner.
- Reescribir la lógica de los dos procesos colaterales.

---
document_id: PBI-DEUDA-TRACKER-GENOMA-FORJA
uuid: "1f389045-ef96-469c-858c-cca1f9d1b259"
title: "[DEUDA] Tracker — backfill forja entity-manager del genoma Linear"
format: markdown
version: "1.1.0"
status: pending
priority: alta
type: deuda
process: refactorization
dispatch: true
feature_name: tracker-genoma-forja-backfill
historia_ref: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
historia_document_id: HU-SDDIA-TRACKER-LINEAR-001
historia_uuid: "26209dff-e413-4c6d-8838-5b785251356c"
tracker_ref: OSC-6
created: "2026-10-02"
author: tekton
updated: "2026-10-02"
refined: "2026-10-02"
especificacion_cerrada: "2026-10-02"
execution_mode: autonomo
cola_ejecucion: docs/todos/pending/
derived_from_pr: "https://github.com/racso80es/SddIA/pull/316"
blocked_by: []
unblocks:
  - PBI-DEUDA-EDA-SCAN-PROCESOS
---

# Backfill de forja del genoma Tracker Linear

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001`. Deuda de la rama `feat/tracker-operations-context` (PR #316).

## 0. Filtro A

| Afirmación de la semilla | Realidad | Corrección |
|--------------------------|----------|------------|
| Todo el genoma de la HU se escribió a mano y la matriz está en `deadbeef` | Estado mixto (matriz a 2026-10-02). Varias fichas ni siquiera están en la matriz; el escáner no las ve, así que `orphan_count: 0` no las absuelve. | Inventario cerrado abajo. Solo se re-sella lo ausente o con `deadbeef`. |
| Re-forjar deja el gate en `orphan_count: 0` | Cierto mientras el escáner siga ciego. El PBI del escáner depende de este: si el parser se arregla antes, los tres procesos tracker rompen el gate. | Este PBI sella primero. No toca `parse_index_uuids`. |
| Hay que recorrer `SddIA/norms/` como el resto de familias | `execution-contexts.md` está en `SddIA/norms/`. El escáner de normas lee `SddIA/library/norms`. Sellar la matriz no lo vuelve visible. | Se sella igual. No se mueve el archivo de sitio. |

### Inventario

| Entidad | uuid | Matriz hoy | Acción |
|---------|------|------------|--------|
| `execution-contexts` | `d8e9f0a1-b2c3-4d5e-6f7a-8b9c0d1e2f3a` | ausente | sellar |
| `linear-tracker-adapter` | `8f3c2a1b-9d4e-4f5a-b6c7-1234567890ab` | `deadbeef` | la posee `PBI-DEUDA-TRACKER-ADAPTER-HASH` si ya cerró; si no, este PBI la sella con hash canónico y no deja `deadbeef` |
| `tracker-backlog-query` | `d4e5f6a7-b8c9-4d0e-a12b-3b4c5d6e7f9a` | ausente | sellar |
| `tracker-sync-replay` | `e1f2a3b4-c5d6-4789-a012-3456789abc01` | ausente | sellar |
| `tracker-stamp` | `f2a3b4c5-d6e7-4890-a123-456789abcdef` | ausente | sellar |
| `tracker-linear-markdown-sync` | `a3b4c5d6-e7f8-4890-a123-456789abcd01` | `Domain_Entity_Created` + hash no placeholder | no re-forjar si el hash canónico coincide |
| `work-initiated` | `b7c8d9e0-f1a2-4b3c-8d7e-6f5a4b3c2d1e` | ausente (índice en `events/domain/`, el escáner no lo lee) | sellar |
| `tracker-sync-failed` | `a9b8c7d6-e5f4-4321-b987-6543210fedcc` | ausente (igual) | sellar |
| `emit-work-initiated-event` | `c8d9e0f1-a2b3-4c5d-8e7f-6a5b4c3d2e1f` | `deadbeef` | sellar |
| `emit-tracker-sync-failed` | `b0c1d2e3-f4a5-4678-9012-3456789abcde` | `deadbeef` | sellar |
| Argos (política `tracker-operations`) | `bd3b1d76-3734-4fbb-b447-ad5e4a5e4907` | `Domain_Entity_Created` de 2026-05-25 | `Domain_Entity_Updated` de la política nueva |
| Tekton | `b3a4c5d6-7e8f-9a0b-1c2d-3e4f5a6b7c8d` | el regex del escáner no acepta el nibble de versión `9` | no regenerar el uuid; sellar la política solo si el creator admite ese uuid |

## 1. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Cada fila «sellar» pasa por `entity-manager` o el creator de su tipo. `last_emitted_event` queda en `Domain_Entity_Created` o `Domain_Entity_Updated`. `last_hash` es el canónico de la ficha, nunca `sha256:deadbeef` ni el placeholder de ceros. |
| R-2 | `tracker-linear-markdown-sync` se deja como está si su `last_hash` ya es el canónico. |
| R-3 | Evolution nueva con los uuid sellados en este cambio. |
| R-4 | Cero cambio de comportamiento en `tracker-stamp`, Kalma2 y `feature-pbi-archive`. |

## 2. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-1 | Las filas «sellar» están en `coverage_matrix` con hash canónico. `--scan` sigue en `orphan_count: 0` (el escáner aún no ve procesos sin backticks ni el índice de `events/domain/`). |
| AC-2 | Ningún secreto en el diff. |
| AC-3 | La ficha de la tool no vuelve a `deadbeef` si el PBI de hash ya la selló. |

## 3. Fuera de alcance

- Arreglar `parse_index_uuids` (PBI del escáner, bloqueado por este).
- Cubrir `phagocyte-recovered-fracture-pbis` y `sync-client-assets` (colateral de ese otro PBI).
- Cambiar el uuid de Tekton.

---
document_id: PBI-MERGE-THERMO-05-DOC-PARITY
uuid: "97836b6f-ba20-4f99-93fa-6dfdbe6ae498"
title: "[OPERATIVO] Aduana Git 05 — Paridad documental de la aduana"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-03"
author: tekton
priority: media
type: operativo
process: feature
dispatch: true
hu_order: 5
hu_order_total: 6
historia_ref: "docs/todos/historias/[OPERATIVO] Optimización Termodinámica de Aduana Git: Merge de Alta Eficiencia.md"
historia_document_id: HU-MERGE-THERMODYNAMICS
cola_ejecucion: docs/todos/pending/
unblocks:
  - PBI-MERGE-THERMO-06-E2E-MEASURE
baseline_decisiones:
  - "HU §2.5-A y §2.5-B. Sin dependencia de los PBI 01–04: se puede forjar en paralelo."
  - "D-2 deja PullRequest_Merged sin suscriptor nuevo; este PBI solo iguala la tabla a los 3 suscriptores vigentes."
---

# Paridad documental de la aduana

HU `HU-MERGE-THERMODYNAMICS`, orden **05/06**. AC-10. No toca hooks ni el motor de revisión.

## 0. Filtro A

| Afirmación a evitar | Corrección |
|---|---|
| `SddIA/process/accept-pr.md` es el SSOT | Ruta pre-packing. El proceso se resuelve por `process_domain_roots`. Ubicación actual: `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md`. Igual `pull-request-review`. |
| Añadir aquí el suscriptor que borra la atestación | D-2 lo pone en la Fase 4 de `accept-pr` (PBI 04). Este PBI no crea suscriptores. |

## 1. Intención

La norma y el contrato del evento dicen lo mismo que el JSON de suscripciones, y un test impide que vuelvan a separarse.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | `pull-request-orchestration.md` §4 y §6 dejan de citar `SddIA/process/accept-pr.md`. Pasan a resolución multi-root, con la ubicación física actual entre paréntesis. Mismo trato para `pull-request-review`. Bump 1.1.0 → 1.2.0 vía `entity-manager`. |
| R-2 | `pull-request-merged.md` lista los tres suscriptores de `event-domain-subscriptions.json["PullRequest_Merged"]`: `tracker-stamp` (tekton), `iota-immutable-publisher` (cumulo), `notify-humanized-pr-merged` (argos). Bump 1.0.0 → 1.1.0; `hash_signature` recalculada. |
| R-3 | Test `sddia-qa`: para cada evento domain, la tabla «Suscripciones» del `.md` y la clave del JSON tienen el mismo conjunto de pares agente+proceso/acción/tool. Cubre todos los eventos domain, no solo `PullRequest_Merged`. |
| R-4 | Registro en `SddIA/evolution/` con los uuid de la norma y del evento. |

## 3. Plan

1. Bumps por `entity-manager`.
2. Test de paridad sobre `SddIA/events/domain/*.md` y `event-domain-subscriptions.json`.
3. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-10 | La norma no contiene `SddIA/process/accept-pr.md`. El `.md` del evento lista exactamente la clave JSON. El test de paridad está en verde para todos los eventos domain. | `rg` + test `sddia-qa` |

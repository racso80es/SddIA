---
document_id: PBI-DEUDA-TRACKER-STAMP-PARIDAD
uuid: "54c2ac42-93e8-490d-8aa9-c914a60cd7c2"
title: "[DEUDA] Tracker — paridad tracker-stamp (D2, AC-8, AC-9, AC-18)"
format: markdown
version: "1.1.0"
status: done
priority: alta
type: deuda
process: feature
dispatch: true
feature_name: tracker-stamp-paridad
historia_ref: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
historia_document_id: HU-SDDIA-TRACKER-LINEAR-001
historia_uuid: "26209dff-e413-4c6d-8838-5b785251356c"
tracker_ref: OSC-8
created: "2026-10-02"
author: tekton
updated: "2026-10-02"
refined: "2026-10-02"
especificacion_cerrada: "2026-10-02"
execution_mode: autonomo
cola_ejecucion: docs/todos/done/
derived_from_pr: "https://github.com/racso80es/SddIA/pull/316"
blocked_by: []
unblocks: []
closed: "2026-10-02"
execution_branch: fix/linear-tracker-adapter-hash
---

# Paridad de `tracker-stamp` con la HU

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001` §4.D (D2, D7(a), AC-8, AC-9, AC-18, AC-21).

## 0. Filtro A

| Afirmación de la semilla | Realidad | Corrección |
|--------------------------|----------|------------|
| No hay comentarios de rama, `pr_url` ni merge SHA | El handler ya comenta: `Work_Initiated` (rama y `persist_ref`), `PullRequest_Presented` (`pr_url`), `PullRequest_Merged` (merge SHA). | Esos tres textos se conservan. Lo que falta es la suite que los afirma y las transiciones de la HU. |
| D7(a) ya está en los emits de PR y basta corregir el PBI archivado | `actions.rs` copia `tracker_ref` solo si el input ya lo trae. `delivery_close.rs` no lo pasa. El contrato ECST de `pull-request-presented.md` no declara el campo (igual de ausente hay que comprobarlo en audited y merged antes de bumpear). `workspace_init.rs` sí lo copia en `Work_Initiated` desde el frontmatter del PBI. | El hueco es el cableado de los tres emits de PR y el bump ECST vía `event-creator`. No se reescribe R-D7-1 del PBI en `done/`: ese texto decía «D7 pendiente» en el momento de la forja. |
| Falta el sello de la HU en `in_progress` | Cierto. No hay `fetch_issue` ni uso de `parent`. `label_pbi` solo se rellena en el `TrackerConfig` de laboratorio y no se consulta. | R-1 y R-2. |
| AC-9 no estaba en la semilla | §4.D: la HU pasa a `done` solo si todos los hijos están `done`. El handler no mira `children`. | Entra como R-5. |
| AC-18 ya tiene test | El único test del handler es el no-op sin `tracker_ref` (AC-21, parcial: no afirma que la cápsula no se invoque más allá de salir antes). | La suite de R-3 cubre AC-18 y AC-21. |

## 1. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Antes de `update_issue_state` o `create_comment`, `fetch_issue`. Si falta la label `tracker.labels.pbi` o `parent` no es el issue de la HU esperada: `warn` y cero escrituras (AC-18). |
| R-2 | En `Work_Initiated`, si el PBI queda en `in_progress` y la HU (`parent`) está en `backlog`, la HU pasa a `in_progress`. |
| R-3 | Suite con mock de laboratorio: ciclo `in_progress` → `in_review` → `done` afirmando los comentarios ya existentes (rama, `pr_url`, merge SHA). Sin `tracker_ref`: no-op y la cápsula no se invoca (AC-21). Label o `parent` incorrectos: AC-18. |
| R-4 | Los emisores de `PullRequest_Presented`, `PullRequest_Audited` y `PullRequest_Merged` copian `tracker_ref` desde el mismo sitio que `Work_Initiated` (frontmatter del PBI de la rama). Los tres contratos ECST ganan el campo OPTIONAL vía `event-creator`. Sin `tracker_ref` en el PBI, el evento sale igual y el sello sigue en no-op. |
| R-5 | En `PullRequest_Merged`, la HU pasa a `done` solo cuando `fetch_issue` muestra todos los hijos PBI en `done` (AC-9). |
| R-6 | El PBI archivado de stamp no se reescribe. Un puntero de una línea hacia este `document_id` es el único añadido permitido. |

## 2. Fuera de alcance

- Migrar el gate local de Done (laudo del PBI de gate: no migrar).
- Añadir un SHA de cabeza al contrato de `PullRequest_Presented` si el payload actual no lo trae.
- Marca `sddia-sync-id` (vive en `tracker-sync-replay`).

## 3. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-1 | R-1, R-2 y R-5 cubiertos por la suite mock. |
| AC-2 | Un ciclo con `tracker_ref` en el PBI deja ese valor en los tres eventos de PR. |
| AC-3 | R-D7-1 del PBI en `done/` permanece como texto histórico. |

Entrega: `docs/features/tracker-stamp-paridad/`.

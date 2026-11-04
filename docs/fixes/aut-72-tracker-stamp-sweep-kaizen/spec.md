---
feature_name: aut-72-tracker-stamp-sweep-kaizen
created: "2026-11-04"
process: bug-fix
phase: design
agents: dedalo
branch_name: Fix_Events_Multiple_03
sighting_branch: Fix_Events_Multiple_03
persist_ref: docs/fixes/aut-72-tracker-stamp-sweep-kaizen
pbi_ref: AUT-72
document_id: PBI-c66c2160
uuid: b42cf4d8-9801-4b44-8f1f-e8b9c9647aa2
parent: AUT-71
execution_id: "523a4752-c526-488d-aaa0-d9a12b1275c6"
status: dedalo_locked
---

# Spec — aut-72-tracker-stamp-sweep-kaizen

## 1. Misión

Kaizen **AUT-72** (historia **AUT-71**, `PBI-c66c2160`): evitar que un `PullRequest_Merged` (o `Delivery_Committed` trunk) con `tekton.tracker-stamp` en **dead-letter** se purgue como `kaizen-finalized` antes de reconciliar el sello **Done** en tracker y prueba local.

## 2. Problema verificado

| Hecho | Evidencia |
| --- | --- |
| `terminal_subscriber_names` cuenta **processed** y **dead_letter** | `eda_bus_topology.rs` — dead-letter cuenta como terminal |
| Con dead no vacío y todos los aplicables ⊆ terminales, el sweep emite `kaizen-finalized` y purga pending | `try_sweep_event` rama `!dead.is_empty()` → `finalize_kaizen_terminal` |
| `PullRequest_Merged` suscribe `tekton.tracker-stamp` (+ IOTA + notify) | `event-domain-subscriptions.json` |
| Fallo de stamp emite `Tracker_Sync_Failed`; replay es suscriptor aparte | `tracker_stamp.rs` / `Tracker_Sync_Failed` → `tracker-sync-replay` |
| Watcher prioriza `kaizen-finalized` sobre testigos dead-letter | `event-watcher` — log «Kaizen terminalizado» antes del guard dead-letter |
| Prueba local Done | `.SddIA/proofs/tracker-done/{tracker_ref}.json` vía `done_gate::write_tracker_done_proof` |

**Síntoma:** tras merge (#433), stamp falla → dead-letter → sweep purga el evento padre → se pierde trazabilidad de reconciliación aunque `Tracker_Sync_Failed` siga vivo.

## 3. Laudos

| Ref | Decisión |
| --- | --- |
| **L-SCOPE** | Rust: `eda_bus_topology` (+ tests). Opcional mínimo en `tracker_stamp` solo para idempotencia/prueba local acordada en T2. Sin cambios de catálogo de suscripciones. |
| **L-SWEEP** | Si `event_type` ∈ {`PullRequest_Merged`, `Delivery_Committed`} y dead-letter incluye `tekton.tracker-stamp` y **no** existe prueba Done local para `tracker_ref` del payload → `status: tracker-stamp-unreconciled`, `purged: false`, pending intacto. |
| **L-REPLAY** | Si existe evento hijo `Tracker_Sync_Failed` pendiente o en vuelo para el mismo `tracker_ref` / correlación, el sweep del padre **no** puede `kaizen-finalized`. (Comprobar testigo o pending correlacionado; fail-closed si ambiguo.) |
| **L-PROOF** | Tras stamp Done exitoso o fetch que confirme estado Done en Linear, escribir/retener `.SddIA/proofs/tracker-done/{tracker_ref}.json` (idempotente). |
| **L-PARITY-AUT70** | Mismo estilo de guard que `fracture-unmaterialized` para fractura sin materializar: status estable, test dedicado, sin inventar purge. |
| **L-KAIZEN** | Prohibido materializar semillas en `docs/todos/`. Cierre issue Linear post-merge del PR de este fix. |
| **L-GIT** | Mutaciones git vía `git-manager` / commits Tekton en `Fix_Events_Multiple_03`. |

## 4. Criterios de aceptación

1. Sweep con `PullRequest_Merged`, stamp en dead-letter, sin proof → **no** `kaizen-finalized`; pending permanece.
2. Mismo escenario con proof local válida → sweep puede completar purge según política existente (otros suscriptores terminales).
3. Test unitario `aut72_tracker_stamp_dead_letter_not_kaizen_finalized` (nombre estable) en módulo de topology.
4. Tras implementación Tekton: `cargo test -p execute-process aut72` verde.
5. `validacion.md` APTO en rama con `pbi_ref: AUT-72`; issue Linear en Done tras merge (provider linear).

## 5. Fuera de alcance

- Reprocesar históricos del bus en producción.
- Cambiar orden de suscriptores de `PullRequest_Merged`.
- AUT-73 (esquema MCP workspace server).

---
feature_name: emit-pr-audited-revoked-registry-rehab-ppr202
created: "2026-08-27"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-emit-pr-audited-event-cerbero
branch_name: refactor/emit-pr-audited-revoked-registry-rehab-ppr202
persist_ref: docs/features/emit-pr-audited-revoked-registry-rehab-ppr202
pbi_ref: docs/todos/pending/[ARQUITECTURA] emit-pr-audited-event — rehabilitación revoked_entities (PPR #202).md
document_id: PBI-PPR-202-EMIT-PR-AUDITED-REVOKED-REGISTRY
uuid: c2e8f4a1-7b3d-4e9c-a5f6-8d1e2f3a4b5c
version_spec: "1.0.0"
status: dedalo_locked
olas:
  - A1
source_correlation_id: "1498e461-3235-483a-b210-907cca744cdd"
source_pr_url: https://github.com/racso80es/SddIA/pull/202
feature_ref: docs/features/accept-pr-revoked-registry-rehab-ppr200
incident_ref: "REVOKED_ENTITY_ALERT_EMIT_PR_AUDITED — abrupt_success_rate_drop since 2026-06-12T10:10:06+00:00"
---

# Spec — emit-pr-audited-revoked-registry-rehab-ppr202

## 1. Misión técnica

Rehabilitar `emit-pr-audited-event` en Cerbero/Radamanto tras revocación lateral `abrupt_success_rate_drop` (PPR #202). Solo A1 Yunque Rúnico — acción atómica sin lifecycle de proceso.

## 2. Diagnóstico

| Vector | Hecho |
|--------|--------|
| Cerbero instancia | `revoked.emit-pr-audited-event` · `entity_type: tool` (fósil) · `reason: abrupt_success_rate_drop` · `since: 2026-06-12T10:10:06+00:00` |
| Radamanto bucket raíz | **ausente** (sin stats previos) |
| Entidad Core | `SddIA/actions/emit-pr-audited-event.md` · handler nativo `engine::actions::emit_pr_audited` |
| Emisor ECST | `pull-request-review` fase Veredicto · `emitter_agent: argos` |
| Laterales | `revoked.refactorization` — fuera |

## 3. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un PR. |
| **L-WAVES** | Solo A1 instancia. Sin A2 motor. |
| **L-REHAB-INST** | A1 = instancia Cúmulo. Evidencia `execution.md`. Prohibido versionar instancia en diff PR. |
| **L-CERBERO** | Eliminar `revoked.emit-pr-audited-event`. Assert `permanent` ausente. |
| **L-STATS** | Crear bucket raíz `emit-pr-audited-event`. |
| **L-RESET-ABS** | `healthy`; `recovery_attempts: 0`; `consecutive_success_count: 0`; `degraded_at: null`; `rehab_laudo: PBI-PPR-202-EMIT-PR-AUDITED-REVOKED-REGISTRY`; `rehabilitated_at` ISO; `samples: []`. |
| **L-ONTOLOGY** | Conservar `entity_type: tool` (fósil). |
| **L-DOC** | Cascada + PBI `done/` + `validacion.md` APTO. |

## 4. Touchpoints

| Locus | Mutación |
|-------|----------|
| `SddIA/engine/execute-process/` | **Intacto** (handler nativo ya operativo). |
| `radamanto.revoked_entities` / `radamanto.stats` (instancia) | A1 solo; evidencia `execution.md`. |
| `directories.evolution` | Entrada UUID `c2e8f4a1-7b3d-4e9c-a5f6-8d1e2f3a4b5c`. |
| `persist_ref` | Cascada + archive PBI + `validacion.md`. |

## 5. Contratos de comportamiento

Post-rehab, `emit-pr-audited-event` invocable vía `try_run_native` sin gate Cerbero. Emisión `PullRequest_Audited` en `eda_bus.pending` conforme contrato ECST.

---
feature_name: radamanto-process-threshold-rehab
created: "2026-08-16"
process: refactorization
items:
  - thresholds-1-1-0-by-entity-type
  - resolve-entity-type-process-catalog
  - fail-soft-ola1-ppr
  - fail-soft-ola2-dcc
  - instance-rehab-dcc
branch_name: refactor/radamanto-process-threshold-rehab
persist_ref: docs/features/radamanto-process-threshold-rehab
pbi_ref: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
document_id: PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS
uuid: ba900e95-1a47-4185-b86c-bc7a251b4fe6
olas:
  - ola-1
  - ola-2
---

# Implementation — radamanto-process-threshold-rehab

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/agents/radamanto.thresholds.json` | v1.1.0 + `success_rate_min_by_entity_type` (process=0.70, tool=0.85, agent=0.75) |
| `SddIA/agents/radamanto.instructions.json` | v1.2.1 · R4.1 lookup por tipo · R4.2 skip latency si `process` |
| `SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs` | `resolve_entity_type` (prefijo → catálogo process → tool); `success_rate_min_for`; latency exempt por tipo |
| `SddIA/engine/execute-process/src/engine/fractal_bus.rs` | default `success_rate_min_by_entity_type: {}` |
| `SddIA/engine/execute-process/src/engine/delivery_close.rs` | fail-soft ola 2: higiene/impacto post `pr_url` |
| `SddIA/engine/execute-process/src/engine/residual_runner.rs` | fail-soft DCC Err + PPR fricción |
| `SddIA/engine/execute-process/src/engine/pull_request_review.rs` | git status post-checkout fail-soft; `is_ppr_fail_soft_friction` |
| `SddIA/engine/execute-process/src/engine/agent_runtime.rs` | PPR `Triaje documental` / `Cosecha Kaizen` failed → `fail_soft` |
| `.SddIA/cerbero/revoked_entities.json` | instancia: DCC ausente de `revoked` (no PR) |
| `.SddIA/radamanto/stats.json` | instancia: DCC `healthy` + `rehab_laudo` canónico (no PR) |

## Fuera de esta entrega

- Faros Kaizen (troceo EDA PPR; centinela `RBAC_EMITTER_NOT_REVOKED`).
- Rehab `feature` / `bug-fix` / `emit-pr-audited-event`.
- Mutación YAML process PPR/DCC.

---
feature_name: radamanto-process-threshold-rehab
created: "2026-08-16"
process: refactorization
branch: refactor/radamanto-process-threshold-rehab
branch_name: refactor/radamanto-process-threshold-rehab
persist_ref: docs/features/radamanto-process-threshold-rehab
pbi_ref: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
document_id: PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS
document_ids:
  - PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS
  - PBI-PPR-174-REVOKED-REGISTRY
  - PBI-PPR-177-DCC-REVOKED-REGISTRY
uuid: ba900e95-1a47-4185-b86c-bc7a251b4fe6
global: APTO
pbi_archived: true
checks:
  AC-OLA1: APTO
  AC-OLA2: APTO
  AC-THRESH: APTO
  AC-TYPE: APTO
  AC-FAILSOFT: APTO
  AC-SCOPE: APTO
  AC-DOC: APTO
  UNIT_TEST: APTO
  INSTANCE_REVOKED_ABSENT: APTO
  RBAC_PROCESS_REGISTRY: APTO
  RBAC_EMITTER_NOT_REVOKED: APTO
git_changes:
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/agents/radamanto.instructions.json
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/engine/execute-process/src/engine/fractal_bus.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/residual_runner.rs
  - SddIA/engine/execute-process/src/engine/pull_request_review.rs
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/evolution/ef2b0ef2-b792-4cb7-ac1b-bfea203f4bde.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/radamanto-process-threshold-rehab/
  - docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #174).md
  - docs/todos/done/[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #177).md
---

# Validación — radamanto-process-threshold-rehab

**APTO** — umbrales por tipo, tipología `process`, fail-soft olas, instancia DCC/PPR fuera de `revoked`, PBI canónico + satélites en `done/`.

| AC | Evidencia |
|----|-----------|
| AC-OLA1 | PPR ∉ `revoked`; `resolve_entity_type(…, pull-request-review)=process`; tests |
| AC-OLA2 | DCC ∉ `revoked`; stats `healthy` + `rehab_laudo` canónico; tests |
| AC-THRESH | JSON 1.1.0; lookup process 0.70 / tool 0.85 |
| AC-TYPE | `bare_process_names_resolve_as_process` |
| AC-FAILSOFT | 8 tests: higiene+pr_url; PPR fricción; agente triaje documental; F2/F4 intactos |
| AC-SCOPE | `permanent.feature` + `revoked.bug-fix` + `emit-pr-audited-event` intactos |
| AC-DOC | cascada + 3 PBI en `docs/todos/done/` |
| UNIT_TEST | `cargo test -p execute-process --lib` filtros → 8 passed |

Aduana remota PPR posterior confirma los checks RBAC en el siguiente ciclo; corte local instancia **APTO**.

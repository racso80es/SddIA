---
feature_name: sddia-codex-software-engineering
created: "2026-08-09"
process: feature
branch_name: feat/sddia-codex-software-engineering
persist_ref: docs/features/sddia-codex-software-engineering
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
execution_id: c76c5d95-b066-49ca-834b-78a4f9443a62
phases: "T0 T1-codex T2-gate T3-tests T4-docs T5-argos"
agents: dedalo
---

# Plan — sddia-codex-software-engineering

| ID | Fase | Touchpoints | Done |
|----|------|-------------|------|
| **T0** | Baseline | ABSTRACT-01 profile API | Notas implementation |
| **T1** | Forja códice | `entity-manager` + `library/codexes/` + index | AC-CODEX |
| **T2** | Gate autoridad | `domain_authority.rs`, `executor.rs` | AC-GATE/ALLOW |
| **T3** | Tests | unit deny/allow | verde |
| **T4** | Docs | implementation, execution, evolution; kitchen ABSTRACT-03 seed | AC-MOVE defer |
| **T5** | Argos + cierre | validacion, PBI done, PR | AC-DOC |

## Orden Tekton

1. T2 gate (independiente de forja física).
2. T1 códice (+ index).
3. T3 tests.
4. T4 docs + seed ABSTRACT-03.
5. T5 Argos.

## Delegaciones

| Necesidad | Vía |
|-----------|-----|
| Alta códice | `./sddia-run.sh --process entity-manager` (`entity_class: codex`) |
| Motor Rust | Tekton directo `SddIA/engine/execute-process` |
| Process move | **Prohibido** este PR |

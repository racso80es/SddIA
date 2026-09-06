---
feature_name: kaizen-git-diff-llm-synthesis
created: "2026-09-06"
process: feature
branch_name: feat/kaizen-git-diff-llm-synthesis
persist_ref: docs/features/kaizen-git-diff-llm-synthesis
pbi_ref: docs/todos/pending/PBI-KAIZEN-GIT-DIFF-LLM-SYNTHESIS.md
execution_id: "00214f28-2a66-4597-8222-6fdc31250d16"
document_id: PBI-KAIZEN-GIT-DIFF-LLM-SYNTHESIS
pbi_uuid: "1540ab52-4354-49a6-9d4e-63135aaccde2"
pbi_version: "1.2.0"
status: in-progress
---

# Objetivos — kaizen-git-diff-llm-synthesis

## Misión

Enriquecer la síntesis Telegram post-`PullRequest_Merged` con hechos Git acotados (`subject` + lista first-parent) vía `git-manager commit_summary`, fail-soft a ECST-only. Blindar fidelidad del prompt con golden set determinista. Cero mutación ECST. Cero entrenamiento de pesos.

## Alcance (manifiesto)

- Ciclo `feature` inicializado (`execution_id` `00214f28-…`).
- Frozen `skill-io-git-manager-frozen` v1.2.0 + cápsula + `git-manager.md` v1.2.0 (uuid skill intacto).
- Handler `notify_humanized_pr_merged.rs` + contrato action (EM update).
- Tests `cargo test -p git-manager` y `cargo test -p execute-process --lib -- notify_humanized`.
- Evolution `modificacion`. Cierre documental en rama + DCC + PR. `accept-pr` condicionado a CI verde.

## Ley aplicada

- Git vía `skill:git-manager`. Troncal `main`.
- DA-2/DA-4: topología `objectives.md` en rama antes de mutar `SddIA/norms/` y `SddIA/skills/` / `SddIA/actions/`.
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CA-CI verde (`run_id`).
- `CONSTITUTION_CORE` Filtro A: no declarar Done sobre diffs locales.

## Criterios (PBI v1.2.0)

K-DIFF-CA1…CA5, K-LLM-CA1…CA3, K-EDA-CA1.

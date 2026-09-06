---
feature_name: kaizen-git-diff-llm-synthesis
created: "2026-09-06"
process: feature
items:
  - capsule-commit-summary
  - handler-golden
  - genome-action
  - evolution-register
document_id: PBI-KAIZEN-GIT-DIFF-LLM-SYNTHESIS
execution_id: "00214f28-2a66-4597-8222-6fdc31250d16"
---

# Implementación — kaizen-git-diff-llm-synthesis

| Item | Path | Nota |
| :--- | :--- | :--- |
| Frozen v1.2.0 | `SddIA/norms/skill-io-git-manager-frozen.md` | uuid `b2c3d4e5-…` intacto. Op `commit_summary`. Sin `cumulo.paths.json`. |
| Cápsula | `SddIA/skills/git-manager/src/main.rs` + `tests/commit_summary.rs` | first-parent names-only; `show -s --format=%s <ref>` (sin `--` path). |
| Skill | `SddIA/skills/git-manager.md` + `index.md` | v1.2.0; uuid `4dac18fc-…` intacto. Sin EM skill. |
| Handler | `notify_humanized_pr_merged.rs` | `invoke_git_manager` fail-soft; SUBJECT/FILES. |
| Action | `notify-humanized-pr-merged.md` | EM update `1fded58f-…` uuid `1cd7bd40-…` v1.1.0 hash `sha256:0a0490d9…`. |

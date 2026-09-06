---
feature_name: kaizen-git-diff-llm-synthesis
created: "2026-09-06"
process: feature
purpose: Estabilización Filtro A del PBI v1.2.0 tras init lab
version_clarify: "1.0.0"
execution_id: "00214f28-2a66-4597-8222-6fdc31250d16"
pbi_ref: docs/todos/pending/PBI-KAIZEN-GIT-DIFF-LLM-SYNTHESIS.md
document_id: PBI-KAIZEN-GIT-DIFF-LLM-SYNTHESIS
pbi_uuid: "1540ab52-4354-49a6-9d4e-63135aaccde2"
pbi_version: "1.2.0"
---

# Clarificación — kaizen-git-diff-llm-synthesis

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `00214f28-2a66-4597-8222-6fdc31250d16`. Rama `feat/kaizen-git-diff-llm-synthesis`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

WIP ajeno en `main` aparcado en stash `aside-pre-feature-kaizen-git-diff-llm-synthesis`. PBI v1.2.0 restaurado en esta rama.

## Decisiones

| ID | Laudo |
|----|-------|
| L-OP | Op nueva `commit_summary` en frozen v1.2.0. Prohibido abusar de `diff_name_only` (working-tree vs HASH). |
| L-GIT | `git diff --name-only -M <ref>^ <ref>` (first-parent, dos argv). `show -s --format=%s`. Si `<ref>^` no resuelve → fail-soft handler. |
| L-SUBJECT | `subject` = `%s` del commit Git. Nunca título de PR. `gh` prohibido. |
| L-INVOKE | Handler: `invoke_git_manager`. No `invoke_capsule_json` suelto. Sin timeout nuevo en `capsules.rs`. |
| L-FORGE-FROZEN | Frozen + `git-manager.md` + cápsula `.rs`: parche DA-4 + evolution. Prohibido `norm-creator` y EM `entity_class: norm`/`skill` (uuid). |
| L-FORGE-ACTION | Action: EM `entity_class: action` `update` (`patch_action_content_update`). uuid `1cd7bd40-…` inmutable. Capability `delegate-git-manager`. |
| L-PROMPT | Git = enriquecimiento interno. Líneas opcionales `SUBJECT:` / `FILES:` (+ truncated). Fail-soft Git → prompt ECST-only ola 2. |
| L-EVAL | Golden aserta prompt/ensamblaje/truncado/fail-soft. Cero oracle LLM vivo. Cero sanitizer post-LLM. Tests lib `notify_humanized_*` + crate `git-manager`. |
| L-EDA | Topología intacta. Test existente `pull_request_merged_subscription_is_humanized_action_not_telegram_tool`. |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo tras checks verdes del PR. |

## Fuera

ECST v1.1.0; `cumulo.paths.json`; timeout Git 3s; matriz HTTP Gemini en execute-process; fine-tuning; reabrir #262.

---
feature_name: kaizen-git-diff-llm-synthesis
created: "2026-09-06"
process: feature
phases:
  - design-commit
  - capsule-commit-summary
  - handler-golden
  - genome-action-evolution
  - dcc-pr-ci-accept
branch_name: feat/kaizen-git-diff-llm-synthesis
persist_ref: docs/features/kaizen-git-diff-llm-synthesis
pbi_ref: docs/todos/pending/PBI-KAIZEN-GIT-DIFF-LLM-SYNTHESIS.md
document_id: PBI-KAIZEN-GIT-DIFF-LLM-SYNTHESIS
uuid: "1540ab52-4354-49a6-9d4e-63135aaccde2"
execution_id: "00214f28-2a66-4597-8222-6fdc31250d16"
---

# Plan — kaizen-git-diff-llm-synthesis

Corte Diseño: **clarify + objectives + spec + plan + commit**. Ejecución (L1–L5) en el mismo ciclo hasta PR verde y `accept-pr`.

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. Semilla `.tmp/feature-kaizen-git-diff-llm-synthesis.json`. `execution_id` `00214f28-2a66-4597-8222-6fdc31250d16`.

## Fase L0 — Diseño (esta parada)

Artefactos bajo `persist_ref` + PBI v1.2.0 en `pending/`.

## Fase L1 — Cápsula `commit_summary` (CA2)

`SddIA/skills/git-manager/src/main.rs` + tests crate. Frozen v1.2.0 y `git-manager.md` v1.2.0 (uuid intactos; DA-4, sin EM skill/norm-creator). Índice `skills/index.md` versión 1.2.0.

```text
cd SddIA && cargo test -p git-manager
```

## Fase L2 — Handler + golden (CA5, K-LLM-CA1/CA2)

`notify_humanized_pr_merged.rs`: `invoke_git_manager` fail-soft; `build_synthesis_prompt` con summary opcional. Tests lib.

```text
cd SddIA && cargo test -p execute-process --lib -- notify_humanized pull_request_merged_subscription
```

## Fase L3 — Action EM (CA4)

Prefijo RAW. Topología `objectives.md` ya en rama.

```text
./sddia-run.sh --process entity-manager --inputs-file .tmp/em-notify-humanized-pr-merged-update.json
```

`entity_class: action` `update`. Prohibido Write sobre `SddIA/actions/`.

## Fase L4 — Evolution + docs de ejecución

`sddia-qa evolution-register` (`modificacion`). `implementation.md` / `execution.md`. Si toca `directories.evolution`: `sddia-qa gate-evolution --json --range` exit 0 antes de push.

## Fase L5 — Cierre, DCC, CI, accept-pr

PBI → `docs/todos/done/`. `validacion.md` con CA-CI `PENDIENTE-CI` hasta `run_id` verde; entonces `global: APTO` + `pbi_archived: true`. `delivery-close-cycle`. Tras checks verdes del PR: `accept-pr`.

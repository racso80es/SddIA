---
feature_name: kalma2-ppr-runtime-gaps-ppr-125
created: "2026-07-22"
process: bug-fix
branch_name: fix/kalma2-ppr-runtime-gaps-ppr-125
persist_ref: docs/fixes/kalma2-ppr-runtime-gaps-ppr-125
agents: tekton
phase: Ejecución
uuid: 0a24332e-e120-480a-87eb-ec9854d27aaa
---

# Ejecución — Kalma2 PPR runtime gaps

## Tests

```bash
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib pull_request_review
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib branch_name_coalesces
```

Resultado: 3 + 1 passed.

## Smoke PPR (lab, agentes unset fallidos por SSL residual)

```bash
env -u SDDIA_AGENT_RUNTIME_COMMAND -u SDDIA_AGENT_RUNTIME_REQUIRE_CLI \
  SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF=1 \
  ./sddia-run.sh --process pull-request-review --inputs '{...pr_branch, persist_ref...}'
```

| Fase | status | evidencia |
|------|--------|-----------|
| Preparación de rama | executed | `handler=ppr-prep-branch` · `git_manager_invoked=true` |
| Triaje técnico | executed | `handler=ppr-tech-triage` · `formal_execute_process=true` · `TECH_FORMAL_EXECUTE_PROCESS=APTO` |
| Handoff | executed/skipped | `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF` |
| Fases agent | failed | EPROTO Kalma2-bridge (fuera de alcance; soft-dep) |

## git-manager CLI

```bash
echo '{"operation_type":"status","repository_path":"...","operation_payload_json":{}}' \
  | ./sddia-run.sh --tool git-manager
```

## Veredicto

**ok** — G1/G2/G4 código + G3 runtime prompt; agentes SSL residual no bloquean peaje nativo.

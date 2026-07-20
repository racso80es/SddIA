---
feature_name: iota-dlt-pr-presented-persistence-7c7bba8c
created: "2026-07-20"
process: bug-fix
branch_name: fix/iota-dlt-pr-presented-persistence-7c7bba8c
persist_ref: docs/fixes/iota-dlt-pr-presented-persistence-7c7bba8c
---

# Ejecución — iota DLT opaco PullRequest_Presented

## Inicio de proceso

```bash
./sddia-run.sh --process bug-fix --inputs '{
  "branch_name": "fix/iota-dlt-pr-presented-persistence-7c7bba8c",
  "persist_ref": "docs/fixes/iota-dlt-pr-presented-persistence-7c7bba8c",
  "bug_summary": "DLT iota-immutable-publisher opaco en PullRequest_Presented 7c7bba8c…",
  "related_todo": "docs/todos/pending/[FIX] iota-immutable-publisher — DLT opaco PullRequest_Presented (7c7bba8c).md"
}'
```

`execution_id`: `a08508e5-abb9-43d8-9c8e-b3f81e254a1e` — workspace-init **executed**.

## Fases

| Fase | Estado |
|------|--------|
| Inicialización | executed |
| Diseño (Dedalo) | simulated → `spec.md` |
| Ejecución (Tekton) | simulated → código + `implementation.md` |
| Verificación (Argos) | `validacion.md` |
| Cierre documental / delivery | pendiente operador |

## Comandos de verificación

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo test -p execute-process capsule_error_trace
CARGO_TARGET_DIR=target cargo test -p execute-process emit_presented
CARGO_TARGET_DIR=target cargo test -p execute-process blocking_tests
CARGO_TARGET_DIR=target cargo build --release -p iota-immutable-publisher -p execute-process
```

## Probe cápsula (post-rebuild)

- `SDDIA_LAB_SIMULATE_IOTA=1` → `transaction_digest` `lab-sim-*`
- Sin mock/relay + wallet dummy → envelope con `error` **y** `feedback` = `iota-publish-unavailable:…`

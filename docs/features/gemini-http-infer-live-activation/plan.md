---
feature_name: gemini-http-infer-live-activation
created: "2026-09-04"
process: feature
branch_name: feat/gemini-http-infer-live-activation
persist_ref: docs/features/gemini-http-infer-live-activation
pbi_ref: docs/todos/pending/[OPERATIVO] Activación live gemini-http-infer — catálogo de modelos y humo de instancia.md
execution_id: "0926e45d-db83-42ea-8a5b-3bafcdb00b57"
phases:
  - A-starter-kit
  - B-crates
  - C-build-test
  - D-smoke-http
  - E-smoke-skill-failsoft
  - F-evolution
---

# Plan — gemini-http-infer-live-activation

## B (crates)

Tekton sobre `SddIA/tools/gemini-http-infer/src/main.rs` y `SddIA/skills/antigravity-cli-executor/src/main.rs`. ED ya forjadas (padre). `entity-manager update` completo reescribe UUID: **no**. `{name}.md` solo si hace falta sello `hash_refresh_only`.

## A

`SddIA/scripts/starter-kit/.dev/.env.example` y `.SddIA/.dev/.env.example`.

## C

`cd SddIA && cargo test -p gemini-http-infer -p antigravity-cli-executor`

## D–E

Humo `--tool` live; skill ELF fail-soft. No CI Google.

## F

Registro evolution si el gate lo exige (crates bajo `directories.tools` / `skills`).

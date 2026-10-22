---
feature_name: tracker-lifecycle-provider
created: "2026-10-22"
process: feature
base: main
scope: provides del adaptador y anti-eco
execution_id: "7a0b47aa-3afa-424e-afc9-a9cb3c67a038"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
---

# Spec — tracker-lifecycle-provider

## Tool

`SddIA/tools/linear-tracker-adapter.md` sigue en `1.1.0`. Frontmatter:

| Campo | Valor |
|-------|--------|
| provides.id | `tracker:lifecycle` |
| provides.contract | `tracker.lifecycle` |
| provides.version | `1.0.0` |
| outputs | `success`, `exitCode`, `message`, `result` |

El gate, con `delegates_to: tool:linear-tracker-adapter` y `requires_capability: tracker:lifecycle`, deja de responder `CAPABILITY_PROVIDER_MISMATCH`.

## Anti-eco

`tracker_outbound.rs` no tiene `const TOOL`. `RECORDED_OPS` es `create_issue`, `update_issue_state`, `create_comment`, `update_issue_description`.

`should_record_linear_tool(repo, tool_name, payload)` compara `tool_name` con el artefacto de la fila. `invoke_tool_for_process` le pasa `repo`.

Un `update_issue_description` con `issue_ref` escribe una línea JSONL con `operation`, `issue_ref`, `event_id`, `occurred_at` y `source_process`.

## Crate

El `match` de `linear-tracker-adapter/src/main.rs` sigue en las seis operaciones. Este PBI no lo edita.

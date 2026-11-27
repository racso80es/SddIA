---
feature_name: aut-134-ppr-handoff-qa-worktree
created: "2026-11-27"
process: bug-fix
branch_name: fix/aut-134-ppr-handoff-qa-worktree
persist_ref: docs/fixes/aut-134-ppr-handoff-qa-worktree
tracker_ref: AUT-134
execution_id: "b6b262ed-c04e-429e-b23e-cb908eb3d70d"
---

# Plan — AUT-134

## T1 — D1 author

Inyectar `author` en child de `handoff_accept_pr` vía `resolve_accept_pr_author`.

## T2 — D2 atestación

Tras fase «Veredicto y bloqueo», llamar `maybe_emit_qa_attestation_before_handoff`; fallback terminal solo si no escrita.

## T3 — D3 worktree

- No reconciliar `execution_id` en disco durante PPR (`agent_runtime`).
- Al cierre PPR exitoso: `git restore` bajo `persist_ref` y eliminar `_agent_handoff.md` no rastreado.

## T4 — Evolución y pruebas

Entrada evolution + `cargo test -p execute-process --lib` + `cargo build -p execute-process`.

## T5 — Cierre

`delivery-close-cycle` en rama `fix/aut-134-ppr-handoff-qa-worktree`.
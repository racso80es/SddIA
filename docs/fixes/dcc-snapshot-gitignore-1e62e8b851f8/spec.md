---
feature_name: dcc-snapshot-gitignore-1e62e8b851f8
created: "2026-09-05"
process: bug-fix
document_id: PBI-FIX-FRACTURE-1e62e8b851f8
---

# Spec — 1e62e8b851f8

## Snapshot

`should_omit_from_snapshot` = todos ajenos **o** vault `.dev` (salvo `.env.example` / `.env.test.example`) **o** `…/starter-kit/.SddIA`. Mismo predicado en porcelain residual.

## git-manager

Tras `git add -A -- path`, si stderr indica ignore (ES/EN), continuar. `added==0` → exit 1. Sin cambio de `operation_type`.

## HEAD

`delivery_close::run` post-bootstrap: si `branch_name` ≈ `HEAD`, `branch_list` → `current_branch_from_list`. Detached → `Err`. `capsule_delivery_gh_pr` blocked `BRANCH_SYMBOLIC_HEAD`.

Hook: `ref_to_branch` resuelve `HEAD`.

## Mayeuta

`is_snapshot_gitignore_trace` precede al cubo Head sha. Excluir esta traza del cubo genérico `failed`/Kintsugi.

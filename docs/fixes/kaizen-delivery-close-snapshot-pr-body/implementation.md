---
feature_name: kaizen-delivery-close-snapshot-pr-body
created: "2026-07-22"
process: bug-fix
branch_name: fix/kaizen-delivery-close-snapshot-pr-body
persist_ref: docs/fixes/kaizen-delivery-close-snapshot-pr-body
pbi_ref: docs/todos/done/[Kaizen] delivery-close — snapshot vacío y pr_body newlines en shell-executor.md
uuid: 09c707bb-03fd-445a-9aa6-bf165b94b7e5
---

# Implementación — snapshot WIP real + `pr_body` vía `--body-file`

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/engine/execute-process/src/engine/phase_capsules.rs` | K1–K3: helpers, `unescape_git_cquoted_path`, snapshot/gh-pr, 7 tests |
| `SddIA/skills/git-manager/src/main.rs` | `rel_path_under_repo` admite paths ausentes (deletes porcelain → `git add`) |
| `docs/fixes/kaizen-delivery-close-snapshot-pr-body/_smoke-close-cycle.json` | Payload K4 (body multilínea) |

**Sin mutación:** `shell-executor` allowlist, `delivery-close-cycle.md`.

## K1 — Snapshot final

Flujo en `capsule_delivery_snapshot_final_with_repo`:

1. `get_last_commit` → `hash_before`
2. `git-manager status` → porcelain vacío ⇒ `executed`, `consolidated: false`
3. Si sucio: `parse_porcelain_paths` (C-quoted/octal UTF-8) → `git-manager commit`
4. Postcondición: segundo `status` limpio + `hash_after ≠ hash_before`
5. Fallo ⇒ `status: failed`, `error_code: SNAPSHOT_DIRTY_SKIPPED`; **no** inserta `snapshot_commit_hash` en state

## K2 — Apertura en forja

Con `pr_body` presente:

1. `resolve_pr_body_file_dir` — `persist_ref/.tmp` o workspace `execution_id`
2. `write_pr_body_file` → `{dir}/pr-body.md` (UTF-8, admite `\n`)
3. Preflight `is_shell_token_safe` en path absoluto (paridad `shell-executor`)
4. `gh pr create … --body-file <abs_path>` — **prohibido** `--body` con markdown en argv

## K3 — Diagnóstico tipado

| `error_code` | Condición |
|--------------|-----------|
| `SNAPSHOT_DIRTY_SKIPPED` | commit fallido, tree sucio post-commit, `hash_after == hash_before`, parser sin paths |
| `PR_BODY_METACHAR` | error shell con `forbidden shell metacharacters` o path body-file inseguro |

Formato `error`: `[ERROR_CODE] mensaje`.

## K4 — Verificación

```bash
cargo test -p execute-process delivery_close_kaizen

export SDDIA_LAB_SKIP_GIT_PUSH=1
export SDDIA_LAB_SIMULATE_GH_PR=1
# WIP sintético en worktree + payload smoke:
# python3 SddIA/scripts/qa/execute-process.py --process delivery-close-cycle \
#   --inputs "$(cat docs/fixes/kaizen-delivery-close-snapshot-pr-body/_smoke-close-cycle.json)"
```

Tests: 7 en `delivery_close_kaizen_tests` (incl. `parse_porcelain_paths_cquoted_unicode_and_delete`).

## Estado verificación local

`cargo test -p execute-process delivery_close_kaizen` → **7 passed**. Smoke = `delivery-close-cycle` real.

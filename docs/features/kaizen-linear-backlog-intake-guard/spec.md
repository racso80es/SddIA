---
feature_name: kaizen-linear-backlog-intake-guard
created: "2026-10-25"
process: feature
---

# Spec — intake guard linear

## T2 — Aduana Git

- Funciones en `hook_common.sh`: lectura de `tracker.provider` y prefijos `docs_layout.todos_*`.
- `pre_commit_gate.sh`: antes de VPI, si `provider=linear` y `--diff-filter=A` bajo buckets todos → `exit 1` con mensaje canónico.

## T4 — Hook Cursor

- `preToolUse` con matcher `Write|StrReplace`.
- Script `SddIA/scripts/qa/cursor-guard-todos.sh`: JSON stdin → `permission: deny` si path bajo bucket todos.

## Fuera de alcance

- Cambiar `path_is_passive` global (docs siguen pasivos para delta_class).
- Forzar `refine.rs` teardown RAII (solo higiene `.gitignore` + purge local).

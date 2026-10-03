---
feature_name: linear-issue-markdown-body
process: feature
created: "2026-10-02"
---

# Plan — linear-issue-markdown-body

1. Extender `linear-tracker-adapter` con `update_issue_description` + tests mock.
2. Handler Rust `tracker_linear_markdown_sync` + registro en `engine/mod.rs` y `handlers/mod.rs`.
3. Entidad proceso `SddIA/process/tracker-linear-markdown-sync.md` + fila en `process/index.md` + hash `sddia-qa recalc-process-hash-signatures`.
4. Documentar op en `SddIA/tools/linear-tracker-adapter.md`.
5. Ejecutar `./sddia-run.sh --process tracker-linear-markdown-sync --inputs '{"sync_all":true}'` con token en bóveda.
6. Cierre documental: PBI en `docs/todos/done/`, `validacion.md` APTO en `persist_ref`.

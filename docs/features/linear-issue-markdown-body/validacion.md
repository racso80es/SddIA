---
feature_name: linear-issue-markdown-body
process: feature
branch: feat/tracker-operations-context
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[DEUDA] Tracker — cuerpo Markdown en Linear.md
document_id: PBI-DEUDA-LINEAR-CUERPO-MARKDOWN
uuid: "b7c85f38-e6b7-4724-9cba-ea47b306c8c7"
created: "2026-10-02"
---

# Validación — linear-issue-markdown-body

- Proceso `tracker-linear-markdown-sync` registrado (hash `sha256:8b1aa7e9…`).
- Operación `update_issue_description` en `linear-tracker-adapter`.
- Test unitario `scrub_redacts_env_lines` en handler.
- Volcado `sync_all` vía `./sddia-run.sh --process tracker-linear-markdown-sync` — producción con bóveda `.SddIA/.dev/.env`: **9/9** (OSC-5…OSC-13), `skipped_count: 0`.
- PBI `PBI-DEUDA-LINEAR-CUERPO-MARKDOWN` en `docs/todos/done/`.

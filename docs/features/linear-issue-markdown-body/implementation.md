---
feature_name: linear-issue-markdown-body
process: feature
created: "2026-10-02"
---

# Implementación — linear-issue-markdown-body

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/tools/linear-tracker-adapter/src/main.rs` | Op `update_issue_description` → `issueUpdate` |
| `SddIA/engine/execute-process/.../tracker_linear_markdown_sync.rs` | Escaneo dirs, scrub, invoke tool |
| `SddIA/process/tracker-linear-markdown-sync.md` | uuid `a3b4c5d6-e7f8-4890-a123-456789abcd01` |
| `SddIA/process/index.md` | Fila proceso |
| `SddIA/tools/linear-tracker-adapter.md` | Lista ops |

Deuda conocida: forja manual (pendiente PBI entity-manager backfill OSC-6).

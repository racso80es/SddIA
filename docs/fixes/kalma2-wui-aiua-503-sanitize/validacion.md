---
feature_name: kalma2-wui-aiua-503-sanitize
created: "2026-09-09"
process: bug-fix
branch: fix/kalma2-wui-aiua-503-sanitize
persist_ref: docs/fixes/kalma2-wui-aiua-503-sanitize
pbi_ref: docs/todos/done/[OPERATIVO] Kalma2 WUI — 503 Gemini sanitizado en epidermis (sin retry).md
document_id: PBI-OPERATIVO-KALMA2-AIUA-503-SANITIZE
uuid: "40510b90-f1f9-4bab-82be-052de6a080e4"
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/282
ci_run_id: "34380881579"
checks:
  KALMA-503-CA1: APTO
  KALMA-503-CA2: APTO
  KALMA-503-CA3: APTO
  KALMA-503-CA4: APTO
  KALMA-503-CA5: APTO
  KALMA-503-CA6: APTO
  KALMA-503-CA-CI: APTO
git_changes:
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - docs/fixes/kalma2-wui-aiua-503-sanitize/
  - docs/todos/done/[OPERATIVO] Kalma2 WUI — 503 Gemini sanitizado en epidermis (sin retry).md
  - SddIA/evolution/24061402-d129-49b1-8a35-ee2388ee4816.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — kalma2-wui-aiua-503-sanitize

**Veredicto global: APTO.** CA-CI sellado con run `34380881579` (PR #282, `headSha` `dae9949`).

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| KALMA-503-CA1 | Envelope 503 → `message` canónico | APTO | `flatten_aiua_wui_sanitizes_incident_503_blob` |
| KALMA-503-CA2 | Texto exacto, 91 chars | APTO | `assert_eq!(err, AIUA_PROVIDER_UNAVAILABLE_MSG)`; `chars().count()==91` |
| KALMA-503-CA3 | Cero retry/sleep/backoff nuevos | APTO | Diff: solo clasificador síncrono. `thread::sleep` preexistente (lock/SSE) |
| KALMA-503-CA4 | No-enmascaramiento | APTO | `"gemini 503"`, `gemini-model-unavailable`, literales timeout/join |
| KALMA-503-CA5 | Tests herméticos | APTO | `cargo test --bin kalma2-bridge`: 41 passed |
| KALMA-503-CA6 | PBI en `done/`; cero diffs app.js / gemini-http-infer | APTO | Este archivo + PBI archivado |
| KALMA-503-CA-CI | Checks GitHub Actions verdes | APTO | [run 34380881579](https://github.com/racso80es/SddIA/actions/runs/34380881579) `success`. Duplicado push `34380878572`: skip e2e/physical del evento push no son fallo |

## Cierre documental

| Paso | Estado |
|------|--------|
| PBI `pending/` → `done/` | Hecho (`document_id` conservado) |
| `pbi_archived: true` | Hecho |
| `global: APTO` | Hecho (`run_id` 34380881579) |
| PR | https://github.com/racso80es/SddIA/pull/282 |

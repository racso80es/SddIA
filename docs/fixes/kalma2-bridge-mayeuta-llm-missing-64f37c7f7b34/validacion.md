---
feature_name: kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
created: "2026-09-09"
process: bug-fix
branch: fix/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
persist_ref: docs/fixes/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
pbi_ref: docs/todos/done/[FIX] kalma2-bridge — fractura sistémica (64f37c7f7b34).md
document_id: PBI-FIX-FRACTURE-64f37c7f7b34
uuid: "64f37c7f-7b34-4000-8000-000000000001"
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/281
ci_run_id: "34365771432"
checks:
  KALMA-LLM-CA1: APTO
  KALMA-LLM-CA2: APTO
  KALMA-LLM-CA3: APTO
  KALMA-LLM-CA4: APTO
  KALMA-LLM-CA6: APTO
  KALMA-LLM-CA-CI: APTO
git_changes:
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/scripts/daemons/kalma2-bridge.sh
  - docs/fixes/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34/
  - docs/todos/done/[FIX] kalma2-bridge — fractura sistémica (64f37c7f7b34).md
  - SddIA/evolution/0e4e3e14-c6e2-417f-8331-332310766679.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34

**Veredicto global: APTO.** CA-CI sellado con run `34365771432` (PR #281, `headSha` `42095f5`).

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| KALMA-LLM-CA1 | ELF release `mayeuta-llm` | APTO | `SddIA/target/release/mayeuta-llm` ELF x86-64 (gitignored) |
| KALMA-LLM-CA2 | Orden release-first + override | APTO | `for rel` release→debug; test `prefers_release_over_debug` + override |
| KALMA-LLM-CA3 | Test hermético | APTO | 4 passed `resolve_mayeuta_llm*` |
| KALMA-LLM-CA4 | WARN lanzador sin `exit 1` | APTO | stub `/bin/true` + override no-ELF: WARN + compile hint; exit 0 |
| KALMA-LLM-CA6 | `POST /api/chat` ≠ traza de este sello | APTO | curl 28 / 0 bytes / no HTTP 500 / no traza literal |
| KALMA-LLM-CA-CI | Checks GitHub Actions verdes | APTO | [run 34365771432](https://github.com/racso80es/SddIA/actions/runs/34365771432): `sddia-index-integrity`, `wasi-runtime-smoke`, `eda-iota-smoke-simulate`, `eda-bus-e2e-smoke`, `eda-iota-physical` pass. Evento `push` `34365765446`: pass + skip e2e/physical (no fallo). |

## Cierre documental

| Paso | Estado |
|------|--------|
| PBI → `docs/todos/done/` | ✅ |
| `pbi_archived: true` | ✅ |
| PR único pre-merge | ✅ https://github.com/racso80es/SddIA/pull/281 |
| CA-CI / `accept-pr` | ✅ run `34365771432`; `accept-pr` a continuación |

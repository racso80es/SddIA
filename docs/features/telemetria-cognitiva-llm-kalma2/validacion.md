---
feature_name: telemetria-cognitiva-llm-kalma2
created: "2026-08-29"
updated: "2026-08-29T13:20:00+02:00"
process: feature
phase: validate
agents: argos
branch: feat/telemetria-cognitiva-llm-kalma2
branch_name: feat/telemetria-cognitiva-llm-kalma2
persist_ref: docs/features/telemetria-cognitiva-llm-kalma2
pbi_ref: docs/todos/done/PBI-TELEMETRY-LLM-COGNITIVE-METRICS-KALMA2.md
document_id: PBI-TELEMETRY-LLM-COGNITIVE-METRICS-KALMA2
uuid: a1535038-8db5-4351-8a81-cfa5586b8c5b
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/222
checks:
  AC-DD1-SSOT: APTO
  AC-DD1-MAPEO-CLI: APTO
  AC-DD2-DEGRADED: APTO
  AC-REF-TRANSPORT: APTO
  AC-RADAMANTO-CONSOL: APTO
  AC-DD4-UMBRALES: APTO
  AC-DD3-SSE: APTO
  AC-WUI-PULSO: APTO
  AC-EVOLUTION-GATE: APTO
  DOC_CASCADE: APTO
git_changes:
  - docs/features/telemetria-cognitiva-llm-kalma2/validacion.md
  - docs/todos/done/PBI-TELEMETRY-LLM-COGNITIVE-METRICS-KALMA2.md
  - SddIA/evolution/a1535038-8db5-4351-8a81-cfa5586b8c5b.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — telemetria-cognitiva-llm-kalma2

**Veredicto:** `global: APTO` · `pbi_archived: true`

- **DD-1:** `raw-execution-finished.md` v1.1.0 + `DEFAULT_TELEMETRY_SCHEMA` en `fractal_bus.rs`; módulo `telemetry_receipt.rs` y mapeo en `thermodynamic.rs` / `phase_capsules.rs`.
- **DD-2:** `mayeuta-llm` emite receipt degradado (tokens cero + `cognitive-degraded`) sin hard fail; tests 7/7.
- **DD-3:** `kalma2-bridge` expone `GET /api/telemetry/stream` (SSE broadcast) y `GET /api/telemetry/cognitive` (snapshot); tests telemetry 2/2.
- **DD-4:** `radamanto.thresholds.json` v1.2.0 bloque `cognitive` con N1 visual / N2 gobernanza; `radamanto_batch_core.rs` acumula y drena inbox STREAM.
- **WUI:** widget pulso cognitivo en `interfaces/kalma2/` (SSE + alerta cuota N1).
- **Evolution:** registro canónico v1.1.1 `a1535038-8db5-4351-8a81-cfa5586b8c5b` (corrige `EVOL_RECORD_INVALID` en `wasi-runtime-smoke`).

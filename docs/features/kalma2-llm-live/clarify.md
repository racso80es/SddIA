---
feature_name: kalma2-llm-live
created: "2026-07-20"
process: feature
purpose: Estabilización Mayeuta del PBI kalma2-llm-live v2.2 (SSE + Foso Python + aduana dual)
---

# Clarificación — kalma2-llm-live

Transcript Mayeuta (2026-07-20). Semilla PBI v2.2.0 (`PBI-KALMA2-LLM-LIVE-V2` / `f0f1b1ec-4b79-47c6-85e2-a0ac2ca3164b`).

## D0 — Apertura

| Campo | Valor |
|-------|--------|
| Proceso | `feature` v1.3.0 |
| `feature_name` | `kalma2-llm-live` |
| Rama | `feat/kalma2-llm-live` |
| `persist_ref` | `docs/features/kalma2-llm-live` |
| `document_id` | `PBI-KALMA2-LLM-LIVE-V2` |
| Init | `execution_id` `7c200ac9-7713-4352-8463-886391b81540` · skips archive/delivery · agentes `awaiting_agents` (CLI Cursor ausente) |
| Depends | `docs/features/kalma2-full-cycle` (A+B+C APTO) |

## D1 — Decisiones vinculantes (laudos PBI)

| ID | Decisión |
|----|----------|
| L-EP | `/api/chat` SSE + `/api/execute`; `/api/interact` compat/deprecado sin LLM classifier |
| L-CI | Derogar `CLASSIFY_INTENT` como aduana |
| L-SK | Evolucionar `mayeuta-llm` existente |
| L-FILE | Dual-mode en `kalma2-agent-runtime-cursor.py` (chat-stream + AGENT_PHASE) |
| L-SF | `System_Fracture_Detected` con `source=kalma2-bridge` |
| L-STOP | UI no mezcla Chat y Forjar Proceso |

## D2 — Fuera de alcance (absorbido en PBI §11 v2.3.3)

| Ítem | Resolución |
|------|------------|
| Disparo solo por insert SQLite | Oráculo CLI + reject `IDE_WATCH_ONLY` + wake opcional |
| Re-forjar ECST/TQM | No; dependencia `kalma2-full-cycle` APTO |
| Versionar secretos | No; gitignore + `.env.example` |

## D3 — Runtime agentes

Bóveda tiene `SDDIA_AGENT_RUNTIME_*` pero `cursor-agent` ausente → fases agent `awaiting_agents`. Forja Tekton en sesión IDE bajo topología documental activa (DA-4).

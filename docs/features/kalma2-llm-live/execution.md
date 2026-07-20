---
feature_name: kalma2-llm-live
created: "2026-07-20"
process: feature
---

# Execution — kalma2-llm-live

## Init

| Campo | Valor |
|-------|--------|
| `execution_id` | `7c200ac9-7713-4352-8463-886391b81540` |
| Rama | `feat/kalma2-llm-live` |
| Skips | `SDDIA_LAB_SKIP_PBI_ARCHIVE` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE` |
| Agentes runtime | `awaiting_agents` (cursor-agent ausente) → forja Tekton en IDE |

## Verificación

| Comando / smoke | Resultado |
|-----------------|-----------|
| `cargo test -p mayeuta-llm` | OK |
| `cargo test -p kalma2-bridge` | OK |
| `cargo test -p execute-process kalma2` | OK |
| CHAT_STREAM mock Python | tokens por stdout |
| CHAT_STREAM SQLite (DB temp) | `composerData` + 2 bubbles + headers; sin recursión CLI |
| S1/S2 smoke `kalma2-chat-infer-smoke.sh` | lab infer `backend=cli` + REQUIRE_INFER fail OK |
| AGENT_PHASE MOCK=1 | envelope JSON `executed` |

## Pendiente live host

| Ítem | Nota |
|------|------|
| S1 live | Instalar `cursor-agent` (`runbook-infer.md`); bóveda `SDDIA_LLM_INFER_COMMAND` |
| S3–S6 | Agent phases, SQLite live, AC2, cierre |
| mayeuta-llm STREAM → Python | tokens OK |
| `POST /api/chat` SSE | frames `data:` |
| `POST /api/execute` | `emitted` + `event_id` |

## Pendiente Argos

- AC2 kill -9 E2E formal
- AC4 `cargo build --release` sin `.py` (ya estructural)
- validacion.md APTO + cierre documental en rama

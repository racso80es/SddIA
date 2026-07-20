---
feature_name: kalma2-full-cycle
created: "2026-07-20"
process: feature
---

# Execution — kalma2-full-cycle

## Init

| Campo | Valor |
|-------|--------|
| `execution_id` | `956100c7-c03f-488b-af1e-2624f84bd0b0` |
| Rama | `feat/kalma2-full-cycle` |
| Skips lab | `SDDIA_LAB_SKIP_PBI_ARCHIVE` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE` |

## Verificación Slice A

| Comando | Resultado |
|---------|-----------|
| `cargo test -p execute-process derive_` | OK |
| `cargo test -p kalma2-bridge project_status` | OK |
| `cargo build -p execute-process -p kalma2-bridge` | OK |

## Verificación Slice B

| Comando | Resultado |
|---------|-----------|
| `cargo test -p execute-process agent_runtime` | 3/3 OK |
| `cargo build -p execute-process` | OK |
| Smoke `kalma2-agent-runtime-lab.sh` | `awaiting_agents` + `_agent_handoff.md` |
| Smoke `kalma2-agent-runtime-cursor.sh` MOCK=1 | `executed` |
| Smoke cursor CLI ausente | `awaiting_agents` (soft) |

## Verificación Slice C

| Comando | Resultado |
|---------|-----------|
| `cargo test -p execute-process load_pbi_body` | 1/1 OK |
| `cargo test -p execute-process build_bug_fix` | 1/1 OK |

## Deudas abiertas

| ID | Nota |
|----|------|
| — | Wrapper Cursor CLI/SDK entregado (`kalma2-agent-runtime-cursor`) |
| — | Argos `validacion.md` **APTO** (A+B+C + B-prod) |
| B2 | Evento dominio handoff opcional |
| Live | PBI `kalma2-llm-live` (f0f1b1ec) — CLI host / SDK / timeout / smoke E2E |

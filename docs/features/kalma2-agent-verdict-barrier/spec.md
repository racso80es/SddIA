---
feature_name: kalma2-agent-verdict-barrier
created: "2026-08-31"
process: feature
base: main
scope: kalma2-agent-verdict-barrier
version_spec: "1.0.0"
status: dedalo_locked
laudo: L-NORM
branch_name: feat/kalma2-agent-verdict-barrier
persist_ref: docs/features/kalma2-agent-verdict-barrier
pbi_ref: docs/todos/pending/[KAIZEN] Kalma2 agent-runtime — veredicto blocked, DNS y halt-after-phase (a9fe100f).md
document_id: PBI-KAIZEN-KALMA2-AGENT-VERDICT-BARRIER
execution_id: "c56f0a70-c2e9-468f-8c98-9c0d044bbd4c"
---

# Especificación — kalma2-agent-verdict-barrier

## 1. Misión técnica

Cerrar F2/F5/F1/F6/F7/F8/F9 del PBI `a9fe100f`: veredicto `blocked` sobrevive prótesis → normalizador → barrera; DNS Node → `awaiting_agents`; `stop_after=design`; acuse L2 explícito; objectives destilado; un canal PBI; `phase_reports.json` en workspace.

## 2. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-NORM** | Allowlist Rust: `executed` \| `awaiting_agents` \| `failed` \| `blocked`. Prótesis: última ocurrencia `(?i)veredicto\s*:\s*(ok\|blocked)`. CLI 0 + blocked → `data.status=blocked`, `success=true`, exit 0. |
| **L-DNS** | Predicado `is_transient_network_error` (Node): `enotfound`, `getaddrinfo`, `eai_again`; opcional `could not resolve host`. Evaluar **antes** de `REQUIRE_CLI`. Timeout (`timeout` en traza) sigue `failed`. No copiar `dcc_transient_network_trace` (`connection timed out` colisiona). |
| **L-HALT** | `SDDIA_TQM_STOP_AFTER=design` (env hijo TQM y/o input). Tras fase Dedalo exacta + `executed` → barrera. Skip `reason: stop_after`, `prior_status: executed`. `derive_cycle_phase`: reason `stop_after` → `awaiting_agents`. |
| **L-L2** | TQM `data.delivery_close = "skipped_l2"` iff `child_env_for_kalma2` inyectó skip. Cero heurística NL. |
| **L-OBJ** | Misión: título H1 + `document_id` (si FM) + ≤400 chars del primer párrafo; no dump YAML. Conservar `pbi_ref` en FM. |
| **L-PROMPT** | `build_prompt`: si hay `pbi_ref`, puntero al fichero. No concatenar `seed[:8000]` + `pbi_body[:12000]`. Si seed contiene `## PBI adjunto`, usar solo el bloque `## Prompt operador`. |
| **L-DISK** | `executor.rs` escribe `{workspace_path}/phase_reports.json` al cerrar el bucle (éxito, barrera, fail). |
| **L-GENOME** | No mutar process YAML ni `delivery_close.rs`. |

## 3. Touchpoints

| Módulo | Cambio |
|--------|--------|
| `kalma2-agent-runtime-cursor.py` | Parser veredicto; red transitoria; `build_prompt` un canal |
| `test_kalma2_runtime_timeout.py` | Fixtures CA-A1/CA-A2 |
| `agent_runtime.rs` | Allowlist `blocked` + test |
| `executor.rs` | `stop_after=design`; persist `phase_reports.json` |
| `thermodynamic.rs` | `derive_cycle_phase` reason `stop_after` |
| `task_queue_manager.rs` | Propagar env; `skipped_l2` |
| `workspace_init.rs` | Destilar Misión |

## 4. Contratos de comportamiento

### 4.1 Veredicto

```text
CLI ok
  → parse transcript (última Veredicto: ok|blocked)
  → blocked: status=blocked, success=true
  → else: executed
CLI fail
  → network (no timeout) → awaiting_agents (ignora REQUIRE_CLI)
  → soft config + REQUIRE_CLI → failed
  → soft config → awaiting_agents
  → else → failed
```

### 4.2 Halt

```text
after Dedalo phase executed:
  if SDDIA_TQM_STOP_AFTER==design:
    barrier_reason = stop_after
    remaining agent+verify+close → skipped (reason stop_after)
PEC cycle_phase = awaiting_agents
aggregate_execution_terminal success = true
```

Fases Dedalo: `Diseño del fix` · `Diseño de Blueprint` · `Diseño de refactor`.

### 4.3 L2

`correlation_id` ∧ ¬`SDDIA_TQM_FULL_CYCLE` → env skip archive+DCC **y** `data.delivery_close: "skipped_l2"`. Si full-cycle, omitir el campo o no usar ese valor.

## 5. Criterios (PBI)

CA-A1, CA-A2, CA-A3, CA-B1, CA-B2, CA-C1, CA-C2. Ver PBI §4.

## 6. Fuera

`stop_after=commit`. NLP. DCC. Keepalive. Genoma process.

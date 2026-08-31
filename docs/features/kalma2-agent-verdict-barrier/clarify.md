---
feature_name: kalma2-agent-verdict-barrier
created: "2026-08-31"
purpose: Estabilización PBI-KAIZEN-KALMA2-AGENT-VERDICT-BARRIER — veredicto blocked, DNS Node, halt-after-design, L2 honesto
branch_name: feat/kalma2-agent-verdict-barrier
persist_ref: docs/features/kalma2-agent-verdict-barrier
pbi_ref: docs/todos/pending/[KAIZEN] Kalma2 agent-runtime — veredicto blocked, DNS y halt-after-phase (a9fe100f).md
document_id: PBI-KAIZEN-KALMA2-AGENT-VERDICT-BARRIER
phase: mayeuta-stabilization
agents: mayeuta
source_audit: docs/audits/kalma2-forge-email-watcher-keepalive-halt-20260830.md
execution_id: "c56f0a70-c2e9-468f-8c98-9c0d044bbd4c"
---

# Clarificación — kalma2-agent-verdict-barrier

PBI v1.1.0 ya es semilla termodinámica. Este artefacto fija laudos; no blueprint.

## D0 — Semilla

Ciclo víctima `9dbcfea6` (`bug-fix` v1.4.2): Dedalo `Veredicto: blocked`, prótesis `executed`, Tekton DNS `getaddrinfo ENOTFOUND api2.cursor.sh`, PEC `failed`. Keepalive `6c0db1296181` fuera.

Vehículo: relevo IDE (`SDDIA_AGENT_RELAY_IDE=1`). Prohibido Kalma2 live hasta Ola A en `main`.

## D1 — Misión

Un despacho Kalma2 no miente `executed` si el agente dijo `blocked`. DNS Node no es colapso. Halt tras Dedalo es contrato (`stop_after=design`), no NLP. L2 se declara en el acuse TQM; no se infiere full-cycle del texto «PR».

## D2 — Laudos

| Ref | Pregunta | Laudo |
|-----|----------|-------|
| **L-NORM** | ¿Basta el `.py`? | **No.** `agent_runtime.rs` debe aceptar `blocked` en allowlist. |
| **L-DNS** | ¿Copiar F4c? | **No.** Tokens Node (`enotfound`, `getaddrinfo`, `eai_again`) + opcional `could not resolve host`. Status `awaiting_agents`. `SDDIA_AGENT_RUNTIME_REQUIRE_CLI` no reclasifica DNS. Timeout sigue terminal. |
| **L-HALT** | ¿`stop_after`? | Env `SDDIA_TQM_STOP_AFTER=design` y/o input TQM. Tras `Diseño del fix` / `Diseño de Blueprint` / `Diseño de refactor` **aunque** `executed`. Skip reason `stop_after`. `cycle_phase=awaiting_agents`. No reutilizar `blocked`. `commit` fuera (MVP). |
| **L-L2** | ¿NL «PR» → full-cycle? | **Prohibido.** Envelope TQM `delivery_close: skipped_l2` cuando L2 inyectó skip. Flag explícito `SDDIA_TQM_FULL_CYCLE` único encendido. |
| **L-PEC** | ¿Halt = failed o completed? | Ninguno. `derive_cycle_phase` trata reason `stop_after` como `awaiting_agents`. `phase_terminal`: skipped neutro. |
| **L-PROMPT** | ¿Canales PBI? | Un canal en `build_prompt`. Misión destilada en `objectives.md`. `phase_reports.json` lo escribe `executor.rs` en `{workspace_path}/`. |
| **L-GENOME** | ¿YAML de proceso? | **No.** Env/TQM. `delivery_close.rs` solo lectura. |

## D3 — Fuera

Keepalive email-watcher. Shell IDE (F3). NLP de fases. Unificar status DNS con F4c `blocked`. Full-cycle sobre ciclo vacío (527007fa).

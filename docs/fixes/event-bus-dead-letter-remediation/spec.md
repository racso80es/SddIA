---
feature_name: event-bus-dead-letter-remediation
created: "2026-07-11"
process: bug-fix
branch_name: fix/event-bus-dead-letter-remediation
persist_ref: docs/fixes/event-bus-dead-letter-remediation
pbi_ref: docs/todos/pending/[FIX] Remediación dead-letters bus de eventos.md
---

# Especificación — Remediación dead-letters bus de eventos

## Problema

Auditoría `event-bus-audit` (2026-07-11T07:07:12Z) en `.SddIA/workspaces/event-bus-audit/7cc4574c-2f6f-447c-88b0-ff9593bf421a/audit-report.md`:

| Anomalía | Cantidad | Causa raíz |
|----------|----------|------------|
| Dead-letter Telegram | 42 | `telegram_notify_core.py` apunta a `scripts/tools/.../main.py` inexistente |
| Dead-letter IOTA/npx | 52 | Fallback TS sin `node_modules`; Rust nativo no priorizado al fallar |
| Dead-letter ECST resolution | 16 | Parser Python extrae `resolution` de prosa FORBIDDEN |
| Dead-letter PR lifecycle | 28 | Precheck falla con URLs `lab-simulated` |
| Pending estancados | 9 | 5× ecst-gate bloqueado + 4× watcher caído |

## Cambio

| Área | Artefacto | Acción |
|------|-----------|--------|
| Telegram | `SddIA/scripts/qa/telegram_notify_core.py` | `invoke_tool_capsule_json` + fallback limbo |
| ECST | `SddIA/scripts/qa/ecst_validation.py` | FORBIDDEN solo líneas `- \`campo\`` (paridad Rust) |
| PR review | `route_domain_event_core.py`, `route_domain_core.rs` | Skip `skipped-lab-simulated` para URLs lab |
| IOTA | `SddIA/scripts/qa/iota_tool_invoke.py` | No fallback TS si Rust falla; `npm install` en limbo |
| Ops | `.events/` | Retirar testigos ecst-gate; re-enrutar pending |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| EB-CA1 | `invoke_send_telegram_notification` usa cápsula Rust SSOT |
| EB-CA2 | `PullRequest_Audited` con `resolution: PASS` pasa ECST gate |
| EB-CA3 | `PullRequest_Presented` lab → `skipped-lab-simulated` (no DL) |
| EB-CA4 | IOTA invoca Rust nativo; sin error npx si binario disponible |
| EB-CA5 | Re-auditoría: 0 pending estancados |
| EB-CA6 | PBI archivado + `validacion.md` APTO |

## Fuera de alcance

- Purga histórica de dead-letters terminales (registro Kaizen).
- Mutación genoma (`SddIA/events/`, `SddIA/tools/`).

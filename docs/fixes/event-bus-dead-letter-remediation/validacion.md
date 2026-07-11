---
feature_name: event-bus-dead-letter-remediation
created: "2026-07-11"
process: bug-fix
branch_name: fix/event-bus-dead-letter-remediation
persist_ref: docs/fixes/event-bus-dead-letter-remediation
global: APTO
pbi_archived: true
branch: fix/event-bus-dead-letter-remediation
---

# Validación — Remediación dead-letters bus de eventos

## Checks

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| EB-CA1 | Telegram usa cápsula Rust SSOT | ✅ | `telegram_notify_core.py` → `invoke_tool_capsule_json`; router Rust `prefer_wasm=false` |
| EB-CA2 | `PullRequest_Audited` pasa ECST gate | ✅ | `forbidden: []`; test Rust `pull_request_audited_forbidden_*`; 5 eventos purged |
| EB-CA3 | URLs lab → `skipped-lab-simulated` | ✅ | `argos.pull-request-review: success` en 3× `PullRequest_Presented` lab |
| EB-CA4 | IOTA prioriza Rust nativo | ✅ | `iota_tool_invoke.py` + `route_domain_core.rs`; sin fallback npx sin Node |
| EB-CA5 | Re-auditoría 0 pending estancados | ✅ | `stale_pending_count: 0` — `.SddIA/workspaces/event-bus-audit/72af29f1-.../audit-report.md` |
| EB-CA6 | PBI archivado + documentación fix | ✅ | `docs/todos/done/` + cascada `spec/implementation/execution/validacion` |

## Cierre documental

PBI `PBI-EVENT-BUS-DL-REMEDIATION` movido a `docs/todos/done/` en rama `fix/event-bus-dead-letter-remediation`.

Auditoría base: `7cc4574c-2f6f-447c-88b0-ff9593bf421a` (9 stale → 0 tras fix).

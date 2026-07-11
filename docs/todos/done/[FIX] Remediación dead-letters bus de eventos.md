---
document_id: PBI-EVENT-BUS-DL-REMEDIATION
title: "[FIX] Remediación dead-letters bus de eventos"
format: markdown
version: "1.0.0"
created: "2026-07-11"
status: done
priority: alta
process: bug-fix
branch_name: fix/event-bus-dead-letter-remediation
fix_ref: docs/fixes/event-bus-dead-letter-remediation
validacion_ref: docs/fixes/event-bus-dead-letter-remediation/validacion.md
closed: "2026-07-11"
---

# PBI-EVENT-BUS-DL-REMEDIATION: Remediación dead-letters bus de eventos

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-EVENT-BUS-DL-REMEDIATION` |
| **Estatus** | ✅ Done |
| **Auditoría** | `.SddIA/workspaces/event-bus-audit/7cc4574c-2f6f-447c-88b0-ff9593bf421a/audit-report.md` |
| **Rama** | `fix/event-bus-dead-letter-remediation` |
| **Persist** | `docs/fixes/event-bus-dead-letter-remediation` |

## Problema

Auditoría empírica del bus (2026-07-11) detecta:

- 86 dead-letter (cabeceras) + 164 testigos
- 9 pending estancados (>24h)
- Top causas: ruta Telegram legacy inexistente (42), npx/IOTA sin deps (52), bug ECST `resolution` (16), precheck PR lab (28)

## Entregables

| # | Entregable | Estado |
|---|------------|--------|
| 1 | Fix `telegram_notify_core.py` — resolución SSOT cápsula Rust | ✅ |
| 2 | Fix parser ECST FORBIDDEN en `ecst_validation.py` | ✅ |
| 3 | Skip controlado URLs lab en precheck `pull-request-review` | ✅ |
| 4 | Prioridad Rust nativo en `iota_tool_invoke.py` + deps limbo | ✅ |
| 5 | Limpieza operativa pending estancados + re-auditoría | ✅ |
| 6 | Cierre documental (`validacion.md` APTO) | ✅ |

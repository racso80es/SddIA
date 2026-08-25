---
feature_name: kaizen-paciente0-redeploy-fricciones
created: "2026-08-25"
process: feature
items_applied: "T1-T7"
branch_name: feat/kaizen-paciente0-redeploy-fricciones
persist_ref: docs/features/kaizen-paciente0-redeploy-fricciones
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
execution_id: "c95fa63f-be71-481b-a927-475e7c885fd0"
---

# Execution — kaizen-paciente0-redeploy-fricciones

## Laboratorio T1–T5

| ID | Resultado |
|----|-----------|
| `cargo test -p execute-process -- email_triage` | 14 passed (T-INFER ×3) |
| `--skip-build` sin testigo | exit 1 |
| `--skip-build` con testigos | exit 0 |
| `entity-manager` UPDATE `instance-creator` | `c1160a5e-…` v1.1.1 (idempotent hash fases) |

## T6 Paciente 0 (2026-08-25)

| Paso | Resultado |
|------|-----------|
| Bundle `dist/sddia-release-consumer-t6` | `20260825T111733Z` 7 bins, 0 `.rs`, `PY_LEAK=no` |
| Overlay `SddIA_AP` | MANIFEST instancia = stamp bundle; `start-sddia.sh` con rama MANIFEST |
| `instance-creator` | `success:true` cid `9528fb5f-…` vault 6; smoke topology OK; `route_domain skipped` (skip_ignition) |
| systemd | `ExecStart=…/SddIA_AP/SddIA/daemons/email-watcher.sh` `active` |
| Ignición | WUI 200; 0 `cargo build` en log; R-07 omite email/telegram script |
| G5 | `verdict=actionable` `subject_elevation=true` proof `413e6edf-…` agenda + WUI inbox |
| Telegram poke | `--tool send-telegram-notification` en instancia: `success:true` `message_id=9` |
| Auditoría | `docs/audits/kaizen-paciente0-redeploy-20260825.md` |

---
feature_name: smokepasarelaasyncpbi-044lab
created: "2026-07-23"
updated: "2026-07-23"
process: feature
phase: Verificación
agent: argos
branch: feat/smokepasarelaasyncpbi-044lab
persist_ref: docs/features/smokepasarelaasyncpbi-044lab
document_id: PBI-044-SMOKE-PASARELA-ASYNC-LAB
pbi_uuid: 8c71b50f-7067-472a-a149-40041920b054
pbi_ref: docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
correlation_id: e92ee44d-9992-4d1b-9384-b5aba5de1acc
global: APTO
pbi_archived: true
approval_status: aprobado
verdict: apto
delivery_state: ready
resolution: PASS_LAB_EVIDENCE
checks:
  DOC_OBJECTIVES: APTO
  DOC_CLARIFY: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_EXECUTION: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_FIXTURE_SMOKE: APTO
  AC_L_S1_TIMING: APTO
  AC_L_S2_CORRELATION: APTO
  AC_L_S3_STATUS: APTO
  AC_L_U_UNITS: APTO
  AC_L_BLIND: APTO
  AC_L_REG: APTO
  AC_L_DOC_EVIDENCE: APTO
  AC_DONE_LAB: APTO
  T_GATE_GIT_MANAGER: APTO
  T_GATE_SHELL_EXECUTOR: NO_APTO
  T_GATE_SHELL_NOTE: "Cerbero allowlist rechaza true/pwd/echo/ls; evidencia T1–T4 vía Shell operador con stdout físico bajo persist_ref"
  PERSIST_REF_RESOLVED: APTO
  PBI_DONE_PRESENT: APTO
  PBI_NOT_REARCHIVED: APTO
  FORGE_ZERO: APTO
git_changes:
  - docs/features/smokepasarelaasyncpbi-044lab/
---

# Validación — smokepasarelaasyncpbi-044lab

**global: APTO** — evidencia física materializada (T1–T4). PBI-044 permanece en `done/` (L8). Forja=0.

| AC | Evidencia |
|----|-----------|
| L-S1 | `_smoke-s1-timing.json` — 12×202, p99≪50 ms |
| L-S2 | `_smoke-s2-domain.json` — `Kalma2_Process_Requested`, cid≡event_id |
| L-S3 | `_smoke-s3-status.json` — `domain.found=true`, status `pending` (techo orch sin PEC) |
| L-U1/U2 | `_smoke-u1-*.txt` / `_smoke-u2-*.txt` — tests verdes |
| L-BLIND | `_smoke-l-blind.txt` — unit no-write-bus ok |
| L-REG | diffs suscripciones vs `main` = 0 |

Residual no bloqueante: `shell-executor` allowlist Cerbero (T_GATE_SHELL_EXECUTOR NO_APTO) — no impide AC-L-* con evidencia en disco.

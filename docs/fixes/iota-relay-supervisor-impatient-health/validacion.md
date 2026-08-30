---
feature_name: iota-relay-supervisor-impatient-health
created: "2026-08-30"
updated: "2026-08-30T11:15:00Z"
process: bug-fix
branch_name: fix/iota-relay-supervisor-impatient-health
persist_ref: docs/fixes/iota-relay-supervisor-impatient-health
pbi_ref: docs/todos/done/[FIX] route-domain-event — fractura sistémica (701c77ebeab8).md
document_id: PBI-FIX-FRACTURE-701c77ebeab8
global: APTO
pbi_archived: true
branch: fix/iota-relay-supervisor-impatient-health
checks:
  RELAY-CA1: APTO
  RELAY-CA4: APTO
  T-A: APTO
  T-B: APTO
  T-C: APTO
  CASCADE_SPEC: APTO
  CASCADE_PLAN: APTO
  CASCADE_IMPLEMENTATION: APTO
  CASCADE_EXECUTION: APTO
  CASCADE_VALIDACION: APTO
  OLA1_DEGRADED: DIFERIDO
  RELAY-CA5_CA7: DIFERIDO
git_changes:
  - SddIA/daemons/iota-publish-relay/src/main.rs
  - docs/fixes/iota-relay-supervisor-impatient-health/
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (701c77ebeab8).md
---

# Validación — fractura `701c77ebeab8` (Argos)

## Veredicto

**APTO** — Ola 0 cerrada en rama `fix/iota-relay-supervisor-impatient-health`. Gracia post-spawn (RELAY-CA1) y omisión de latido post-gracia (RELAY-CA4) implementadas solo en `main.rs`. Tests unitarios 6/6.

## Checks

| Check | Estado | Evidencia |
|-------|--------|-----------|
| RELAY-CA1 | APTO | `grace_refused_does_not_kill` |
| RELAY-CA4 | APTO | `post_grace_refused_kills_and_omits_tick` |
| T-A / T-B / T-C | APTO | tests + `grace_boundary_eq_is_outside` |
| OLA1_DEGRADED | DIFERIDO | deuda runtime/audit/espejo |
| RELAY-CA5_CA7 | DIFERIDO | logs, cola DLT, taxonomía |

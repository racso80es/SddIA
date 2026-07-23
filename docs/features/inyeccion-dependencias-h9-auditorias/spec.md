---
feature_name: inyeccion-dependencias-h9-auditorias
created: "2026-07-22"
updated: "2026-07-23"
process: feature
branch_name: feat/inyeccion-dependencias-h9-auditorias
persist_ref: docs/features/inyeccion-dependencias-h9-auditorias
document_id: PBI-043-H9-AUDITORIAS
execution_id: c9e4b17a-6f2d-4a8e-9c3b-1d5e8f0a7b42
agent: dedalo
scope: "Hito 3 (H9) — Auditorías DI qa:probe + audit:compliance"
q1_laudo: "H9-A+B+C OK; H9-D audit:compliance (sin reuso qa:probe)"
racso_countersign: "2026-07-23T06:53:00Z"
ac_h9_branch: A
---

# Especificación — H9 Auditorías (PBI-043)

## Laudos Racso

| ID | Veredicto |
|----|-----------|
| H9-A | Alta `qa:probe` |
| H9-B | Extensión DI `tool:` en gate/resolver |
| H9-C | `provides: qa:probe` en tools caos/audit |
| H9-D | **RECHAZADO** reuso `qa:probe` en telemetry-compliance-audit → alta `audit:compliance` |

## Modelo R6

- **Caos/sonda:** `qa:probe` → tools (`sandbox-breacher`, `schema-corruptor`, `io-choke`, `event-bus-audit`); binding canónico `tool:event-bus-audit`; preferencia `delegates_to` tool.
- **Gobernanza:** `audit:compliance` → `skill:compliance-auditor` (exclusiva `telemetry-compliance-audit`).
- Runtime: `skill:`\|`action:`\|`tool:` en provider_fs_rel / scan / gate.

## Ola R7 N_ola=5

| ED | Capacidad |
|----|-----------|
| audit-sandbox-isolation-rbac | qa:probe |
| audit-telemetry-compliance-breach | qa:probe |
| audit-thermodynamic-toll-failsoft | qa:probe |
| event-bus-audit | qa:probe |
| telemetry-compliance-audit | audit:compliance |

## Fuera

H10-B §3.4 defer; R10; archivo PBI-043. `llm:interact` catalogada (laudo H10-A) — consumidores en ciclo H10-A.

---
feature_name: ppr-revoked-registry-rehab-ppr190
created: "2026-08-26"
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/ppr-revoked-registry-rehab-ppr190
persist_ref: docs/features/ppr-revoked-registry-rehab-ppr190
pbi_ref: docs/todos/pending/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
document_id: PBI-PPR-190-REVOKED-REGISTRY
uuid: e2b9a4f1-7c83-4d5e-9a16-0f8b3c5d7e21
source_correlation_id: "5a4683c0-db46-4e8e-b5f4-b865ba417e0d"
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
olas:
  - A1
  - A2
---

# Clarificación — ppr-revoked-registry-rehab-ppr190

## D0 — Semilla

| Vector | Hecho |
|--------|--------|
| PBI | `PBI-PPR-190-REVOKED-REGISTRY` · entidad `pull-request-review` |
| Re-revocación | `permanent` @ `2026-08-25T16:25:55Z` (`max_recovery_attempts_exceeded`) + `revoked` @ `2026-08-25T17:24:18Z` (`abrupt_success_rate_drop`) |
| Causalidad | 9 samples KO (~2.2M ms) de hijos foreground post-CLI-detach contaminaron ventana tras rehab #174 |
| Padre | #174+#177 — fail-soft ola 1 PPR + umbrales `process: 0.70`; no cubre supervivencia post-detach |
| Emisor ECST | `github-bridge-watcher` ∉ revoked |

## D1 — Laudo operativo

Un ciclo `refactorization`, un PR. A1 instancia + A2 motor obligatorios. Rehab Cerbero sola = reabrir vector.

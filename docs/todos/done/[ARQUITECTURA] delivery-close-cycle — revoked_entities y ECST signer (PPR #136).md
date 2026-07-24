---
document_id: PBI-PPR-136-DCC-REVOKED-SIGNER
title: "[ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136)"
format: markdown
version: "1.0.0"
created: "2026-07-22"
updated: "2026-07-24"
status: done
priority: media
process: feature
uuid: 67dc94fd-4a6c-4aed-b335-421c021dcfc5
source_feature: docs/features/inyeccion-dependencias-envelope-homologacion
feature_ref: docs/features/delivery-close-cycle-revoked-signer
branch_name: feat/delivery-close-cycle-revoked-signer
execution_id: 00b9e53d-d231-45f5-9685-4d2b86b7ab63
source_correlation_id: e3079c94-2a40-4f60-b9c4-b4ade1ca031b
source_audit: docs/features/inyeccion-dependencias-envelope-homologacion/validacion.md
pr_url: https://github.com/racso80es/SddIA/pull/136
related:
  - .SddIA/cerbero/revoked_entities.json
  - SddIA/process/delivery-close-cycle.md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #125).md
incident_ref: "PPR #136 — RBAC_EMITTER_NOT_REVOKED + RBAC_SIGNER_PRESENT (emitter delivery-close-cycle)"
---

# [ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136)

## Mandato

Liquidar deuda RBAC no bloqueante detectada en aduana PPR #136 sobre el emisor ECST `PullRequest_Presented`.

| ID | Check origen | Evidencia empírica |
|----|--------------|--------------------|
| **E1** | `RBAC_EMITTER_NOT_REVOKED` | `.SddIA/cerbero/revoked_entities.json` → clave `delivery-close-cycle` (`abrupt_success_rate_drop`, since 2026-07-13) |
| **E2** | `RBAC_SIGNER_PRESENT` | ECST `e3079c94-…` sin `signer_identity_rbac` (payload solo `branch`/`pr_url`/`status`) |

## Criterio de cierre

- [x] Laudo Cerbero/Radamanto: rehabilitar o justificar retención de `delivery-close-cycle` en `revoked`.
- [x] `delivery-close-cycle` emite `PullRequest_Presented` con `signer_identity_rbac` no nulo (contrato ECST).
- [ ] Checks `RBAC_EMITTER_NOT_REVOKED` y `RBAC_SIGNER_PRESENT` → APTO en aduana PPR posterior.

## Dedup explícito

| Finding Argos | Tratamiento Cúmulo |
|---------------|-------------------|
| `RBAC_PROCESS_REGISTRY` («PPR revoked») | **Dedup cerrado** — seeds PPR #124/#125 done; empírico: `pull-request-review` **ausente** de `revoked_entities.json` |

## Fuera de alcance

- Rehabilitación de `feature` / `bug-fix` en el mismo registry (salvo que un laudo los agrupe).
- Residual Kalma2-agent-runtime-cursor (seed OPERATIVO PPR #136).

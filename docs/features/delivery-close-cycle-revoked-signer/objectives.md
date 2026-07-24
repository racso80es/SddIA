---
feature_name: delivery-close-cycle-revoked-signer
created: "2026-07-24"
process: feature
branch_name: feat/delivery-close-cycle-revoked-signer
persist_ref: docs/features/delivery-close-cycle-revoked-signer
pbi_ref: docs/todos/done/[ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136).md
document_id: PBI-PPR-136-DCC-REVOKED-SIGNER
execution_id: 00b9e53d-d231-45f5-9685-4d2b86b7ab63
phase: delivery-close
agents: mayeuta,dedalo,tekton,argos
---

# Objetivos — delivery-close-cycle-revoked-signer

## Misión

Liquidar la deuda RBAC no bloqueante de PPR #136 sobre el emisor ECST `PullRequest_Presented` (`delivery-close-cycle`):

1. **E1** — Laudo Cerbero/Radamanto: rehabilitar o justificar retención de `delivery-close-cycle` en `.SddIA/cerbero/revoked_entities.json`.
2. **E2** — Emitir `PullRequest_Presented` con `signer_identity_rbac` no nulo (contrato ECST).
3. **Aduana** — `RBAC_EMITTER_NOT_REVOKED` y `RBAC_SIGNER_PRESENT` → APTO en PPR posterior.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Clave `delivery-close-cycle` en revoked + path de firma ECST | Rehabilitación `feature` / `bug-fix` (salvo laudo agrupado) |
| `emit-pr-presented-event` / contrato `pull-request-presented` | Residual Kalma2-agent-runtime-cursor (PBI OPERATIVO) |
| Cascada documental feature + cierre single-PR | Reescritura amplia de delivery-close-cycle fuera de E1/E2 |

## Criterios de aceptación

- **AC-E1:** laudo + estado revoked coherente.
- **AC-E2:** `signer_identity_rbac` presente en ECST emitido.
- **AC-ADUANA:** ambos checks APTO en aduana PPR posterior.
- **AC-DOC:** PBI en `done/` + `validacion.md` `pbi_archived: true` en la rama del PR.

## Ley aplicada

- Git vía `skill:git-manager`; genoma vía `entity-manager` / `./sddia-run.sh`.
- `features-documentation-pattern` v1.2.x; cierre documental en rama (un PR).
- Jerarquía: Acción → Agente → Skill → Tools.

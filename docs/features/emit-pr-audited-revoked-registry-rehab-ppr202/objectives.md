---
feature_name: emit-pr-audited-revoked-registry-rehab-ppr202
created: "2026-08-27"
process: refactorization
branch_name: refactor/emit-pr-audited-revoked-registry-rehab-ppr202
persist_ref: docs/features/emit-pr-audited-revoked-registry-rehab-ppr202
pbi_ref: docs/todos/pending/[ARQUITECTURA] emit-pr-audited-event — rehabilitación revoked_entities (PPR #202).md
document_id: PBI-PPR-202-EMIT-PR-AUDITED-REVOKED-REGISTRY
uuid: c2e8f4a1-7b3d-4e9c-a5f6-8d1e2f3a4b5c
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: "1498e461-3235-483a-b210-907cca744cdd"
source_pr_url: https://github.com/racso80es/SddIA/pull/202
feature_ref: docs/features/accept-pr-revoked-registry-rehab-ppr200
incident_ref: "REVOKED_ENTITY_ALERT_EMIT_PR_AUDITED — emit-pr-audited-event ∈ revoked as tool since 2026-06-12T10:10:06+00:00"
olas:
  - A1
---

# Objetivos — emit-pr-audited-revoked-registry-rehab-ppr202

## Objetivo

Rehabilitar la acción `emit-pr-audited-event` tras revocación lateral `abrupt_success_rate_drop` (since `2026-06-12T10:10:06+00:00`, PPR #202). Solo A1 Yunque Rúnico — sin A2 motor.

## Alcance

1. **A1 — Saneamiento de instancia:** eliminar `revoked.emit-pr-audited-event`; verificar `permanent` ausente; crear bucket raíz stats `healthy` con laudo #202. Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff del PR.
2. **Ontología:** conservar `entity_type: tool` (fósil Cerbero; acción Core).
3. **Cierre documental single-PR:** cascada bajo `persist_ref` + PBI en `docs/todos/done/` + `validacion.md` APTO (`pbi_archived: true`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | `emit-pr-audited-event` ∉ `revoked` ni `permanent`; stats raíz `healthy`; laudo `PBI-PPR-202-EMIT-PR-AUDITED-REVOKED-REGISTRY`; evidencia A1 en `execution.md`. |
| AC-GIT-CLEAN | Instancia Cerbero/Radamanto ausente del diff del PR. |
| AC-ONTO | `entity_type: tool` conservado (fósil). |
| AC-SMOKE | Handler nativo `emit_pr_audited` invocable post-rehab (smoke opcional). |
| AC-DOC | Cascada patrón; PBI en `done/`; `validacion.md` APTO. |

## Fuera de alcance

- Rehabilitación de `refactorization`.
- Reabrir alcance A1/A2 de accept-pr #200.
- Mutar `radamanto.thresholds.json` v1.1.0.
- Versionar mutaciones de instancia en el PR.

## Restricciones

- Git solo vía `skill:git-manager`. Rama canónica: `refactor/emit-pr-audited-revoked-registry-rehab-ppr202`.
- Cerbero no tiene estado `healthy`: rehab = borrar clave de `revoked`.

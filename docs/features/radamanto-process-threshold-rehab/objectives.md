---
feature_name: radamanto-process-threshold-rehab
created: "2026-08-16"
process: refactorization
branch_name: refactor/radamanto-process-threshold-rehab
persist_ref: docs/features/radamanto-process-threshold-rehab
pbi_ref: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
document_id: PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS
uuid: ba900e95-1a47-4185-b86c-bc7a251b4fe6
phase: mayeuta-stabilization
agents: mayeuta
olas:
  - ola-1
  - ola-2
---

# Objetivos — radamanto-process-threshold-rehab

## Objetivo

Unificar rehabilitación y anti-recurrencia de `pull-request-review` (Ola 1 / PPR #174) y `delivery-close-cycle` (Ola 2 / PPR #177): ontología `entity_type: process`, umbrales Radamanto diferenciados para procesos multi-fase frente a `tool` atómico, fail-soft por ola, un `persist_ref` y un PR.

## Alcance

1. **Laudo Cerbero:** rehabilitar (no permanente) ambas olas; retirar `delivery-close-cycle` de `.SddIA/cerbero/revoked_entities.json`; verificar que `pull-request-review` permanece ausente (corte 2026-08-16 ya sin clave) y no re-entra por el mismo vector.
2. **Ontología:** alinear tipología a `process` (corregir `tool` residual en instancia para estas dos entidades).
3. **Política Radamanto:** umbrales de `success_rate` diferenciados por tipo; `process` multi-fase con mayor tolerancia que `tool`; no depender solo de `LATENCY_THRESHOLD_EXEMPT`.
4. **Redención stats:** `pending_redemption` → `healthy` sin reabrir `success_rate_below_threshold` por misclasificación tipológica.
5. **Fail-soft ola 1:** fricción de sub-fase en `pull-request-review` no colapsa la ejecución lineal; éxito parcial registrable.
6. **Fail-soft ola 2:** timeout no crítico en `telemetry_receipt` / validación de repo no impide `PullRequest_Presented` firmado si commit/push cruzó umbral físico.
7. **Cierre documental single-PR:** cascada bajo `persist_ref` + PBI canónico y satélites ola en `docs/todos/done/` + `validacion.md` APTO (`pbi_archived: true`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-OLA1 | `pull-request-review` ausente de `revoked`; tipología `process`; `RBAC_PROCESS_REGISTRY: APTO` en aduana PPR posterior. |
| AC-OLA2 | `delivery-close-cycle` ausente de `revoked`; tipología `process`; `RBAC_EMITTER_NOT_REVOKED: APTO` en aduana PPR posterior. |
| AC-THRESH | Umbrales Radamanto versionados por tipo; anti-recurrencia del vector `success_rate_below_threshold`. |
| AC-FAILSOFT | Fail-soft ola 1 (PPR) y ola 2 (DCC handoff) aplicados. |
| AC-DOC | Cascada `features-documentation-pattern` completa; canónico + satélites en `done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## Fuera de alcance

- Residual Kalma2 Shell / `git-manager` (dedup OPERATIVO PPR #136 done).
- Merge / handoff `accept-pr` de PR #174 / #177 históricos.
- Faros Kaizen: trocear PPR en eventos EDA; aislar `RBAC_EMITTER_NOT_REVOKED` en centinela EDA.
- Rehabilitación de `feature`, `bug-fix` o `emit-pr-audited-event`.
- Reabrir incidentes #124/#125/#136 ya cerrados.

## Restricciones

- Git solo vía `skill:git-manager`. Rama canónica: `refactor/radamanto-process-threshold-rehab`.
- Prohibido despachar ciclos `bug-fix` satélite por ola.
- Instancia Cerbero/Radamanto (`.SddIA/`) ≠ genoma indexado; mutaciones de umbrales/core vía cadena autorizada cuando el blueprint lo exija.
- Mayeuta no fija números de umbral; Dedalo los especifica en `spec.md`.
- El cuerpo de este documento es el `refined_requirements` de entrada a Dedalo.

## Ley aplicada

- `features-documentation-pattern` v1.2.x (frontmatter + un `.md` por fase).
- Proceso `refactorization` v1.2.2 — fase Estabilización de alcance → Mayeuta.
- Cierre documental en rama (un PR): `task-closure-documental` / patrón v1.2.0+.
- Jerarquía: Acción → Agente → Skill → Tools.
- `SddIA/norms/external-ai-constraints.md` (soberanía de rutas; forja gobernada).

---
feature_name: dcc-revoked-registry-rehab-ppr187
created: "2026-08-21"
process: refactorization
branch_name: refactor/dcc-revoked-registry-rehab-ppr187
persist_ref: docs/features/dcc-revoked-registry-rehab-ppr187
pbi_ref: docs/todos/pending/[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #187).md
document_id: PBI-PPR-187-DCC-REVOKED-REGISTRY
uuid: c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: 4gKBTRCyZzvEFQcbDWFnBmdC3ZjvqTJmauHiYgTWwj32
source_pr_url: https://github.com/racso80es/SddIA/pull/187
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
olas:
  - A1
  - A2
---

# Objetivos — dcc-revoked-registry-rehab-ppr187

## Objetivo

Rehabilitar el proceso `delivery-close-cycle` tras re-revocación `abrupt_success_rate_drop` (PPR #187 / since `2026-08-20T12:04:10Z`) **y** impedir la re-muerte: A1 absoluto en Yunque Rúnico + A2 adjudicación retroactiva de `fail_soft` sobre Aduana EDA cuando el umbral físico (push/`pr_url`) ya cruzó pese a huérfanos EDA **preexistentes**. Un `persist_ref`, un PR.

## Alcance

1. **A1 — Saneamiento de instancia (Yunque Rúnico):** eliminar `revoked.delivery-close-cycle`; verificar `permanent.delivery-close-cycle` ausente; reset absoluto del bucket **raíz** `delivery-close-cycle` en stats (`healthy`, `recovery_attempts: 0`, `degraded_at: null`, `rehab_laudo: PBI-PPR-187-DCC-REVOKED-REGISTRY`, `rehabilitated_at`, poda de `samples` a vacío o ≤3 OK runtime). Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff del PR.
2. **A2 — Adjudicación retroactiva EDA (extensión L-FAILSOFT-OLA2):** tras el bucle de fases DCC y antes de `aggregate_execution_terminal`, si hay umbral físico y el report `"Aduana EDA genómica"` está blocked/failed por `orphan_count > 0` con `argos_verdict: block`, inyectar `fail_soft: true` en ese report. Gate EDA sigue emitiendo block; agregador intacto; sin umbral físico el fallo permanece causal.
3. **Ontología:** conservar `entity_type: process` (ya correcto post-#174+#177).
4. **Umbrales:** tabla Radamanto 1.1.0 intacta; no reabrir `success_rate` ni `abrupt_drop_min_samples`.
5. **Cierre documental single-PR:** cascada bajo `persist_ref` + PBI en `docs/todos/done/` + `validacion.md` APTO (`pbi_archived: true`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | `delivery-close-cycle` ∉ `revoked` ni `permanent`; stats raíz `healthy`; `recovery_attempts: 0`; laudo y timestamp de rehab; ventana `samples` podada; evidencia A1 en `execution.md`. |
| AC-GIT-CLEAN | Instancia Cerbero/Radamanto ausente del diff del PR. |
| AC-ONTO | `entity_type: process` conservado. |
| AC-A2 | EDA blocked + huérfanos preexistentes + umbral físico → `fail_soft` retroactivo + `exit_code: 0`; sin umbral → causal; señal Argos block preservada; agregador sin mutar. |
| AC-TESTS | Cobertura unit/integración del contrato A2; regresiones fail-soft higiene/snapshot intactas. |
| AC-THRESH | Umbrales 1.1.0 intactos. |
| AC-RBAC | `RBAC_EMITTER_NOT_REVOKED: APTO` con emisor DCC en aduana PPR posterior. |
| AC-DOC | Cascada `features-documentation-pattern`; PBI en `done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## Fuera de alcance

- Rehabilitación de `bug-fix`, `refactorization`, `emit-pr-audited-event` o `feature`.
- Backfill EDA de `github-raw-fetcher` / `download-remote-asset` (deuda preexistente).
- Merge / handoff `accept-pr` de PR #187 (ya MERGED).
- Mutar `radamanto.thresholds.json` v1.1.0 sin laudo.
- Poda `survival_hollow` / A3 de #185 (DCC ∉ `LIFECYCLE_PROCESSES`).
- Mutar `aggregate_execution_terminal` o debilitar `capsule_eda_genomic_audit_gate` a pass silencioso.
- Versionar mutaciones de instancia Cerbero/Radamanto en el PR de motor.
- Inyectar `fail_soft` estático en YAML del proceso.

## Restricciones

- Git solo vía `skill:git-manager`. Rama canónica: `refactor/dcc-revoked-registry-rehab-ppr187`.
- Prohibido despachar ciclos `bug-fix` satélite.
- Cerbero no tiene estado `healthy`: rehab = borrar clave de `revoked` (y de `permanent` si apareciera).
- Reset A1 sin poda de ventana KO reabre `abrupt_success_rate_drop` al primer fallo.
- Ampliar solo `is_dcc_secondary_phase` **no** sustituye la adjudicación retroactiva post-bucle.
- Mayeuta no diseña touchpoints de código; Dedalo los fija en `spec.md`.
- El cuerpo de este documento es el `refined_requirements` de entrada a Dedalo.

## Ley aplicada

- `features-documentation-pattern` v1.2.x (frontmatter + un `.md` por fase).
- Proceso `refactorization` — fase Estabilización de alcance → Mayeuta.
- Cierre documental en rama (un PR): `task-closure-documental` / patrón v1.2.0+.
- Jurisprudencia `L-REHAB-INST`, `L-FAILSOFT-OLA2` (#174+#177); anti-recurrencia #185; anti-alucinación auditoría código 2026-08-21 (PBI).
- Jerarquía: Acción → Agente → Skill → Tools.
- `SddIA/norms/external-ai-constraints.md` (soberanía de rutas; forja gobernada).

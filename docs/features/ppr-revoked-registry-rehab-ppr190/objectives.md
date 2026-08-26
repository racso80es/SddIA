---
feature_name: ppr-revoked-registry-rehab-ppr190
created: "2026-08-26"
process: refactorization
branch_name: refactor/ppr-revoked-registry-rehab-ppr190
persist_ref: docs/features/ppr-revoked-registry-rehab-ppr190
pbi_ref: docs/todos/pending/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
document_id: PBI-PPR-190-REVOKED-REGISTRY
uuid: e2b9a4f1-7c83-4d5e-9a16-0f8b3c5d7e21
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: "5a4683c0-db46-4e8e-b5f4-b865ba417e0d"
source_pr_url: https://github.com/racso80es/SddIA/pull/190
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
olas:
  - A1
  - A2
---

# Objetivos — ppr-revoked-registry-rehab-ppr190

## Objetivo

Rehabilitar `pull-request-review` tras re-revocación post-olas #124/#125/#174 (`permanent.max_recovery_attempts_exceeded` + `revoked.abrupt_success_rate_drop`) **y** cortar re-muerte: A1 Yunque Rúnico + A2 poda supervivencia para hijo PPR post-CLI-detach y `cycle_phase` en telemetría PPR.

## Alcance

1. **A1 — Instancia:** eliminar `pull-request-review` de `permanent` y `revoked`; reset stats raíz (`healthy`, `recovery_attempts: 0`, poda KO, `rehab_laudo: PBI-PPR-190-REVOKED-REGISTRY`). Evidencia en `execution.md`. Sin versionar `.SddIA/` en el PR.
2. **A2 — Motor:** `pull-request-review` ∈ procesos con `cycle_phase` en REF; `detached_child: true` en REF del hijo foreground; `is_survival_hollow` ignora KO `detached_child` y `detach: true`.
3. **Ontología:** conservar `entity_type: process`.
4. **Umbrales:** `radamanto.thresholds.json` 1.1.0 intacto.
5. **Cierre:** cascada + PBI en `done/` + `validacion.md` APTO (`pbi_archived: true`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | PPR ∉ `permanent` ni `revoked`; stats `healthy`; laudo + timestamp; ≤3 samples OK; evidencia `execution.md` |
| AC-A2 | KO hijo detached no alimenta samples; PPR simulated → `cycle_phase: initialized` → hollow |
| AC-THRESH | Umbrales 1.1.0 bit-idénticos |
| AC-DOC | PBI en `done/`; `validacion.md` APTO |

## Fuera de alcance

- Rehab `accept-pr`, `bug-fix`, `refactorization`, `emit-pr-audited-event`.
- Mutar umbrales o agregador.
- Versionar instancia en diff git.

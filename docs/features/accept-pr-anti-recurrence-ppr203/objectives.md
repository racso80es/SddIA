---
feature_name: accept-pr-anti-recurrence-ppr203
created: "2026-08-27"
process: refactorization
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr203
persist_ref: docs/features/accept-pr-anti-recurrence-ppr203
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
document_id: PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY
uuid: b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: "6237015f-0f8d-42ea-97ea-a44afac5318d"
source_pr_url: https://github.com/racso80es/SddIA/pull/203
feature_ref: docs/features/accept-pr-revoked-registry-rehab-ppr200
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — post-physical sync/push gap after A2 seal #200; since 2026-08-27T12:31:30Z"
ola: A2
olas:
  - A2
runtime_execution_id: "d96ec0f4-99ef-44d3-9d93-6fa9009a72cc"
---

# Objetivos — ola A2 accept-pr-anti-recurrence-ppr203

## Objetivo

Cortar re-revocación `abrupt_success_rate_drop` de `accept-pr` cuando la fusión ya materializó `merge_commit_hash` y falla `"Sincronización y Limpieza"` (push). El fail_soft del sello (#200) **no** cubre ese path. Instancia Cerbero = ola A1.

## Alcance

1. Fail_soft runtime en report de sync/push post-umbral físico (simetría DCC #187 secondary).
2. Cableado inline Err + adjudicación post-bucle. Agregador intacto.
3. Tests producto; regresiones `t_a2_seal_*` / `t_a2_canon_*`.
4. Cierre documental del PBI #203 en esta rama (un PR con ambos persist_ref).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A2-SYNC | Hash + sync failed → `fail_soft` + `exit_code: 0`; error push visible; sin hash → `exit_code: 1`. |
| AC-A2-SEAL | Sello #200 sin regresión. |
| AC-TESTS | Cobertura A2-sync + filtros `t_a2_` verdes. |
| AC-THRESH | Umbrales 1.1.0 bit-idénticos. |
| AC-GIT-CLEAN | Sin `.SddIA/cerbero/` ni `.SddIA/radamanto/` en el PR. |
| AC-DOC | Cascada A2; PBI canónico en `done/`; `validacion.md` `global: APTO`, `pbi_archived: true`. |

## Fuera de alcance

- Mutar umbrales / `phase_terminal` / hollow salvo evidencia Dedalo de sample exit 1 con agregador 0.
- Reabrir payload `delete_branch` #194.
- Rehab `refactorization`.
- Silenciar error de push (debe quedar en `error` del report).
- YAML estático `fail_soft` en `accept-pr.md`.

## Restricciones

- Git `skill:git-manager`. Rama compartida con A1.
- `fail_soft` es runtime, no genoma process.
- Cuerpo = `refined_requirements` Dedalo A2.

## Ley aplicada

- `features-documentation-pattern` v1.2.x + cierre documental un PR.
- Simetría fail_soft post-umbral: #187 DCC secondary · #200 accept-pr sello · **#203 accept-pr sync**.
- SSOT proceso: `process_domain_roots` → `accept-pr.md` § Limpieza — intacto.
- `external-ai-constraints.md`.

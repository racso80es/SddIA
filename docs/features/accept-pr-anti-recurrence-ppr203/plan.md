---
feature_name: accept-pr-anti-recurrence-ppr203
created: "2026-08-27"
process: refactorization
phase: planning
agents: dedalo
phases:
  - T0-failsoft-sync
  - T0-residual-sym
  - T0-unit-tests
  - T2-evolution
  - T3-argos
  - T4-doc-archive
  - T5-delivery-close
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr203
persist_ref: docs/features/accept-pr-anti-recurrence-ppr203
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
document_id: PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY
uuid: b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e
ola: A2
olas:
  - A2
---

# Plan — ola A2 accept-pr-anti-recurrence-ppr203

Blueprint Tekton. Contratos: `spec.md`. **Stop planning:** no ejecutar T0–T5 en esta sesión.

A1 instancia: `docs/features/accept-pr-revoked-registry-rehab-ppr203/plan.md` (T1 host **después** de T0).

## T0 — Motor fail_soft sync post-merge (AC-A2-SYNC / AC-TESTS)

1. `accept_pr.rs`:
   - `mark_fail_soft_if_sync_post_merge(entry, phase_name, state)` per **L-FAILSOFT-SYNC**.
   - `adjudicate_sync_fail_soft_post_merge(phase_reports, state)` per **L-FAILSOFT-RETRO**.
   - Reusar `accept_pr_physical_threshold_crossed`. **No** mutar sello #200 ni higiene #194.
2. `residual_runner.rs`:
   - Rama Err accept-pr: mark sync (además de sello).
   - Post-bucle `process_name == "accept-pr"`: adjudicar sync **y** sello antes de `aggregate_execution_terminal`.
3. Unit tests `t_a2_sync_*`. Assert `t_a2_seal_*` / `t_a2_canon_*` intactos.
4. **Prohibido** `phase_terminal.rs`, `radamanto_batch_core.rs`, umbrales, YAML `accept-pr.md`.

## T2 — Documental Tekton + evolution

1. `implementation.md` + `execution.md` (items A2). Referenciar evidencia A1 en persist_ref hermano.
2. Evolution UUID `b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e` (una entrada ciclo).
3. `cargo test -p execute-process --lib` filtro `t_a2_`.
4. Assert diff: no instancia Cerbero/Radamanto; no umbrales.

## T3 — Argos

`validacion.md` en **este** persist_ref (cierre del PBI): `global`, checks AC-A2-SYNC/SEAL/TESTS/THRESH/GIT-CLEAN/DOC + AC-A1 del hermano, `git_changes`, `pbi_archived: true`, `branch: refactor/accept-pr-revoked-registry-rehab-ppr203`.

## T4 — Archive PBI

Mover **solo** canónico `docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md` → `docs/todos/done/`. Alias `PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY.md`: eliminar o no archivar duplicado (mismo `uuid`). Tekton; no Cúmulo.

## T5 — DCC

`delivery-close-cycle` · `source_process: feature` (vehículo) / nota `process_label: refactorization` · `persist_ref` A2 (o el que DCC indexe; ambos dirs en el mismo snapshot) · `branch_name` compartido.

Post-merge: smoke accept-pr **sin** re-revocación inmediata (push fail_soft si hash cruzó).

## Orden innegociable

```text
T0 (A2 motor) → T1 (A1 instancia, persist_ref hermano) → T2 → T3 → T4 → T5
```

## Delegaciones

| Fase | Cápsula |
|------|---------|
| Engine + tests | Tekton filesystem-ops / ecosystem-evolution |
| Git | `skill:git-manager` |
| Archive | Tekton |
| PR | `action:execute-process` → `delivery-close-cycle` |

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Solo sello, no sync | Empiria #203; T0 exige sync |
| A1 antes de T0 | 4s recidiva |
| Samples no podados | A1 **L-SAMPLES** |
| Mutar agregador | Prohibido |
| Versionar instancia | AC-GIT-CLEAN |
| Dos PBI archive | **L-DEDUP** |

## Fuera de este plan

Rehab `refactorization`; umbrales; reabrir #194/#200 sello.

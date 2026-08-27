---
feature_name: accept-pr-revoked-registry-rehab-ppr200
created: "2026-08-27"
process: refactorization
phase: planning
agents: dedalo
phases:
  - T0-failsoft-seal
  - T0-residual-sym
  - T0-unit-tests
  - T1-instance-rehab
  - T2-evolution
  - T3-argos
  - T4-doc-archive
  - T5-delivery-close
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr200
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr200
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
document_id: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY
uuid: a8f3c1e2-9b4d-4e7a-8c5f-1d2e3f4a5b6c
olas:
  - A1
  - A2
---

# Plan — accept-pr-revoked-registry-rehab-ppr200

Blueprint ejecutable para Tekton. Contratos: `spec.md` laudos L-* + AC-*.

No es forja de proceso nuevo: no se instancia `{name}.md` bajo `directories.process`. El ciclo vigente es `refactorization` v1.2.2.

## T0 — Motor A2 fail_soft sello post-merge (AC-A2 / AC-TESTS)

1. `accept_pr.rs`:
   - `accept_pr_physical_threshold_crossed(state)` — non_empty `merge_commit_hash`.
   - `mark_fail_soft_if_seal_post_merge(entry, phase_name, state)` per **L-FAILSOFT-SEAL**.
   - `adjudicate_seal_fail_soft_post_merge(phase_reports, state)` per **L-FAILSOFT-RETRO** (idempotente).
   - **No** mutar `delete_branch_hygiene` / handoff #194 (**L-NO-REOPEN-194**).
2. `residual_runner.rs`:
   - Rama Err de `execute_accept_pr_phase`: tras `status=failed` + `error`, llamar `mark_fail_soft_if_seal_post_merge` (**L-INLINE-ERR** — path DLQ empírico).
   - Tras bucle de fases, si `process_name == "accept-pr"`: `adjudicate_seal_fail_soft_post_merge` **antes** de `aggregate_execution_terminal` (**L-RESIDUAL-SYM**).
3. Unit tests `t_a2_seal_*` (§7 spec). Assert regresiones `t_a2_canon_*` intactas.
4. **Prohibido** tocar `phase_terminal.rs`, `radamanto_batch_core.rs`, umbrales, YAML `accept-pr.md`.

## T1 — A1 instancia Yunque Rúnico (AC-A1 / AC-ONTO / AC-GIT-CLEAN)

Locus Cúmulo: `radamanto.revoked_entities` = `.SddIA/cerbero/revoked_entities.json`; `radamanto.stats` = `.SddIA/radamanto/stats.json`. **Fuera del diff git.**

1. Eliminar `revoked.accept-pr`. Assert `permanent.accept-pr` ausente.
2. Bucket raíz `accept-pr` únicamente (**L-RESET-ABS** + **L-SAMPLES** + laudo #200).
3. Assert laterales intactos (`refactorization`, `emit-pr-audited-event`).
4. Volcar evidencia (campos/timestamp, **no** secretos) en `execution.md`.

Orden: T1 **después** de T0 en el mismo host (anti-recurrencia inmediata).

## T2 — Documental Tekton + evolution

1. `implementation.md` + `execution.md` (frontmatter patrón; `items` / `items_applied`; A1 en `execution.md`).
2. Entrada `directories.evolution` vinculando UUID `a8f3c1e2-9b4d-4e7a-8c5f-1d2e3f4a5b6c` + fila `Evolution_log.md` si norma lo exige.
3. Compilar/tests: `cargo test -p execute-process --lib` filtro `t_a2_`.
4. Assert diff: **no** incluye `.SddIA/cerbero/` ni `.SddIA/radamanto/`; **no** incluye `radamanto.thresholds.json`.

## T3 — Verificación Argos (AC-*)

Argos → `validacion.md`: `global`, `checks` mapeando AC-A1/GIT-CLEAN/ONTO/A2/TESTS/THRESH/DOC, `git_changes`, `pbi_archived: true`, `branch: refactor/accept-pr-revoked-registry-rehab-ppr200`.

## T4 — Cierre documental en rama

1. Mover PBI `PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY` de `docs/todos/pending/` → `docs/todos/done/` **en esta rama** (Tekton; no Cúmulo Kaizen).
2. Confirmar `validacion.md` con `pbi_archived: true` coherente.

## T5 — Cierre de entrega DCC

1. `action:execute-process` → `delivery-close-cycle` con `source_process: refactorization`, `persist_ref`, `branch_name`.
2. Git exclusivamente `skill:git-manager`.
3. Post-A2: sello DLQ post-`merge_commit_hash` no debe re-revocar `accept-pr` (`exit_code: 0` / sample OK).

## Orden innegociable

```text
T0 → T1 → T2 → T3 → T4 → T5
```

T0 es el PR motor. T1 no entra al PR. T3/T4 no adelantan `pbi_archived` si el PBI sigue en `pending/`.

## Delegaciones canónicas (RBAC)

| Fase plan | Cápsulas / agentes |
|-----------|-------------------|
| Mutación engine + tests | Tekton (`filesystem-ops` / `ecosystem-evolution`; engine, no genoma `{name}.md`) |
| Git | `skill:git-manager` (`source-control`) |
| A1 instancia | Tekton FS local (evidencia `execution.md`; no stage) |
| Archive PBI | Tekton en rama (norma cierre documental) |
| Cierre PR | `action:execute-process` → `delivery-close-cycle` |

`target_executor_rbac` esperado: `ecosystem-evolution`, `filesystem-ops`, `source-control`. Cruce: ninguna fase delega fuera de ese conjunto.

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Solo post-pass sin Err inline | T0 exige **ambos**; path empírico DLQ = Err |
| A1 sin A2 | Primer merge con sello DLQ reabre `abrupt_success_rate_drop` a n≥3 |
| Samples no podados | Reset absoluto `[]` (**L-SAMPLES**) |
| Laudo #194 residual | Sobrescribir con #200 |
| Mutar agregador / umbrales | Prohibido; fail_soft antes |
| Versionar instancia en PR | Assert T2; Argos AC-GIT-CLEAN |
| Reabrir payload/handoff #194 | **L-NO-REOPEN-194** |

## Fuera de este plan

Rehab laterales Cerbero; umbrales 1.1.0; mutar YAML accept-pr; faros Kaizen bajo `docs/todos/`.

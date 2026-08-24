---
feature_name: dcc-revoked-registry-rehab-ppr187
created: "2026-08-21"
process: refactorization
phase: blueprint
agents: dedalo
phases: T0-T5
branch_name: refactor/dcc-revoked-registry-rehab-ppr187
persist_ref: docs/features/dcc-revoked-registry-rehab-ppr187
pbi_ref: docs/todos/pending/[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #187).md
document_id: PBI-PPR-187-DCC-REVOKED-REGISTRY
uuid: c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14
olas:
  - A1
  - A2
---

# Plan — dcc-revoked-registry-rehab-ppr187

Blueprint ejecutable para Tekton. Contratos: `spec.md` laudos L-* + AC-*.

No es forja de proceso nuevo: no se instancia `{name}.md` bajo `directories.process`. El ciclo vigente es `refactorization` v1.2.2.

## T0 — Motor A2 adjudicación retroactiva EDA (AC-A2 / AC-TESTS)

1. `delivery_close.rs`:
   - Implementar `pub(crate) fn adjudicate_eda_fail_soft_post_physical(phase_reports: &mut [Value], state: &Value)` per **L-PRED-EDA** / **L-FAILSOFT-RETRO**.
   - En `run`: tras el `for phase in phases` y **antes** de `aggregate_execution_terminal`, invocar el helper; reasignar `state["phase_reports"]` si hace falta coherencia.
   - **No** ampliar `is_dcc_secondary_phase` en este ciclo (**L-SECONDARY-LIST**).
   - **No** mutar semántica de `mark_fail_soft_if_secondary` para higiene/impacto.
2. `residual_runner.rs`:
   - Tras el bucle de fases y antes del agregador: si `process_name == "delivery-close-cycle"`, invocar el mismo helper (**L-RESIDUAL-SYM**).
   - No reimplementar predicado; no tocar path hollow / `LIFECYCLE_PROCESSES`.
3. Unit tests en `delivery_close.rs` `#[cfg(test)]` (casos §7 spec). Regresiones higiene/snapshot intactas.
4. **Prohibido** tocar `phase_terminal.rs`, `capsule_eda_genomic_audit_gate`, `radamanto_batch_core.rs`, umbrales.

## T1 — A1 instancia Yunque Rúnico (AC-A1 / AC-ONTO / AC-GIT-CLEAN)

Locus: `.SddIA/` vía Cúmulo (`radamanto.revoked_entities`, `radamanto.stats`). **Fuera del diff git.**

1. Eliminar `revoked.delivery-close-cycle`. Assert `permanent.delivery-close-cycle` ausente.
2. Bucket raíz `delivery-close-cycle` únicamente:
   - `status: healthy`
   - `recovery_attempts: 0`
   - `consecutive_success_count: 0`
   - `degraded_at: null`
   - `rehab_laudo: PBI-PPR-187-DCC-REVOKED-REGISTRY`
   - `rehabilitated_at` ISO UTC de la intervención
   - `samples`: vacío o ≤3 últimos OK runtime (eliminar KO `d7310496…`, `19391b9f…`)
3. Assert fósil `entities.delivery-close-cycle` **no** mutado.
4. Assert laterales intactos (`bug-fix`, `refactorization`, `emit-pr-audited-event`).
5. Volcar evidencia (campos/timestamp, **no** secretos) en `execution.md`.

Orden: T1 **después** de T0 en el mismo host (anti-recurrencia inmediata). T1 puede solaparse con T2 documental si el motor ya está en disco local.

## T2 — Documental Tekton + evolution

1. `implementation.md` + `execution.md` (frontmatter patrón; `items` / `items_applied`; A1 en `execution.md`).
2. Entrada `directories.evolution` vinculando UUID `c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14`.
3. Compilar/tests: `cargo test -p execute-process --lib` (filtros `delivery_close` / fail_soft).
4. Assert diff: **no** incluye `.SddIA/cerbero/` ni `.SddIA/radamanto/`; **no** incluye `radamanto.thresholds.json`.

## T3 — Verificación Argos (AC-*)

Argos → `validacion.md`: `global`, `checks` mapeando AC-A1/GIT-CLEAN/ONTO/A2/TESTS/THRESH/RBAC/DOC, `git_changes`, `pbi_archived: true`, `branch: refactor/dcc-revoked-registry-rehab-ppr187`.

## T4 — Cierre documental en rama

1. Mover PBI `PBI-PPR-187-DCC-REVOKED-REGISTRY` de `docs/todos/pending/` → `docs/todos/done/` **en esta rama** (Tekton; no Cúmulo Kaizen).
2. Confirmar `validacion.md` con `pbi_archived: true` coherente.

## T5 — Cierre de entrega DCC

1. `action:execute-process` → `delivery-close-cycle` con `source_process: refactorization`, `persist_ref`, `branch_name`.
2. Git exclusivamente `skill:git-manager`.
3. Post-A2: peaje EDA con huérfanos preexistentes + umbral físico no debe re-revocar DCC (`exit_code: 0` / sample OK).

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
| Archive PBI | Tekton en rama (norma cierre documental) |
| Cierre PR | `action:execute-process` → `delivery-close-cycle` |

`target_executor_rbac` esperado (proceso `refactorization`): `ecosystem-evolution`, `filesystem-ops`, `source-control`. QA tests: `quality-assurance` si el runtime lo inyecta; tests en crate no exigen cápsula extra.

Cruce RBAC: ninguna fase de este blueprint delega cápsulas fuera de ese conjunto. Genoma `directories.process` / `agents` / `events` **no** se forja.

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Ampliar solo `is_dcc_secondary_phase` y omitir post-pass | T0 exige helper post-bucle; lista secundaria **no** se toca |
| A1 sin poda de KO | Checklist T1 absoluto; un fallo futuro reabre `abrupt_success_rate_drop` |
| Path residual sin simetría | T0 paso 2 obligatorio; helper compartido |
| Debilitar gate EDA «para pasar» | Prohibido; Argos block + `fail_soft` |
| Mutar agregador | Prohibido; fail_soft antes |
| Versionar instancia en PR | Assert T2; Argos AC-GIT-CLEAN |
| Forja genoma por error | Abortar; engine-only |

## Fuera de este plan

Rehab laterales Cerbero; umbrales 1.1.0; backfill huérfanos EDA; hollow A3; accept-pr históricos; faros Kaizen.

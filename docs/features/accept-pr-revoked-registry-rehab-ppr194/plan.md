---
feature_name: accept-pr-revoked-registry-rehab-ppr194
created: "2026-08-27"
process: refactorization
phase: blueprint
agents: dedalo
phases: T0-T5
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr194
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr194
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md
document_id: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY
uuid: 7f3a9c2e-4b1d-4e8a-9c5f-6d7e8a9b0c1d
olas:
  - A1
  - A2
  - A3
satellite_fix_pbi: docs/todos/pending/[FIX] accept-pr delete_branch payload vs git-manager.md
---

# Plan — accept-pr-revoked-registry-rehab-ppr194

Blueprint ejecutable para Tekton. Contratos: `spec.md` laudos L-* + AC-*.

No es forja de proceso nuevo: ciclo vigente `refactorization` v1.2.2.

## T0 — Motor A2 + A3 (AC-A2 / AC-A3 / tests)

### T0.1 — `delete_branch_hygiene` (A2)

1. `accept_pr.rs`: reescribir helper per **L-DELETE-PAYLOAD** / **L-HYGIENE-SOFT**.
2. Fase `"Sincronización y Limpieza"`: push causal; higiene fail-soft por op; propagar `closed_branch` / `hygiene_failure`.
3. Unit tests §7 T-A2-*.

### T0.2 — Handoff truth (A3)

1. `pull_request_review.rs` `handoff_accept_pr`: emitir `accept_pr_handoff` + `accept_pr_handoff_status` per **L-HANDOFF-STATUS** / **L-HANDOFF-RUNTIME**.
2. Unit tests §7 T-A3-*.
3. **Prohibido** tocar `phase_terminal.rs`, `radamanto_batch_core.rs`, umbrales.

### T0.3 — Frozen I/O + genoma skill/process (L-FROZEN-ALIGN / L-HANDOFF-F5)

1. `./sddia-run.sh --process entity-manager` (o cadena autorizada) para:
   - `skill-io-git-manager-frozen.md` → `1.1.0` (+ `delete_branch`/`merge`/`get_last_commit`/`diff_name_only`).
   - `git-manager.md` inputs enum alineado.
   - `pull-request-review.md` outputs: boolean + `accept_pr_handoff_status`.
2. Prohibido Write IDE directo sobre `directories.norms` / `skills/` / `process` domain roots.
3. `accept-pr.md` preferible intacto.

### T0.4 — Compilar / tests crate

```text
cargo test -p execute-process --lib
# filtros sugeridos: delete_branch / hygiene / handoff / accept_pr / pull_request_review
```

## T1 — A1 instancia Yunque Rúnico (AC-A1 / AC-ONTO / AC-GIT-CLEAN)

Locus: `.SddIA/cerbero/revoked_entities.json` + `.SddIA/radamanto/stats.json`. **Fuera del diff git.**

1. Eliminar `revoked.accept-pr`. Assert `permanent.accept-pr` ausente.
2. Bucket raíz `accept-pr` únicamente:
   - `status: healthy`
   - `recovery_attempts: 0`
   - `consecutive_success_count: 0`
   - `degraded_at: null`
   - `rehab_laudo: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY`
   - `rehabilitated_at` ISO UTC de la intervención
   - `samples`: vacío o ≤3 últimos OK runtime (eliminar KO `53d07f32…`, `f95e8c2f…`)
3. Assert laterales intactos (`bug-fix`, `refactorization`, `emit-pr-audited-event`).
4. Volcar evidencia (campos/timestamp, no secretos) en `execution.md`.

Orden: T1 **después** de T0 en el mismo host (anti-recurrencia inmediata post-payload).

## T2 — Documental Tekton + evolution

1. `implementation.md` + `execution.md` (frontmatter patrón; A1 en `execution.md`).
2. Entrada `directories.evolution` UUID `7f3a9c2e-4b1d-4e8a-9c5f-6d7e8a9b0c1d`.
3. Assert diff: **no** incluye `.SddIA/cerbero/` ni `.SddIA/radamanto/`; **no** `radamanto.thresholds.json`.

## T3 — Verificación Argos (AC-*)

Argos → `validacion.md`: `global`, `checks` mapeando AC-A1/GIT-CLEAN/ONTO/A2/A3/SMOKE/THRESH/DOC, `git_changes`, `pbi_archived: true`, `branch: refactor/accept-pr-revoked-registry-rehab-ppr194`.

Aplicar **L-HANDOFF-F5** en el propio peaje de este ciclo si MERGE ausente.

## T4 — Cierre documental en rama

1. Mover PBI canónico `PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY` → `docs/todos/done/`.
2. Mover FIX satélite `PBI-FIX-ACCEPT-PR-DELETE-BRANCH-PAYLOAD` → `docs/todos/done/` (mismo PR; **L-UNIFY**).
3. Confirmar `validacion.md` con `pbi_archived: true`.

## T5 — Cierre de entrega DCC

1. `action:execute-process` → `delivery-close-cycle` con `source_process: refactorization`, `persist_ref`, `branch_name`.
2. Git exclusivamente `skill:git-manager`.
3. Post-rehab: handoff `accept-pr` operable; higiene payload canónica; sin re-revocación por `hygiene_failure` de contrato.

## Orden innegociable

```text
T0 → T1 → T2 → T3 → T4 → T5
```

T0 = PR motor (A2+A3+frozen). T1 no entra al PR. T3/T4 no adelantan `pbi_archived` si los PBI siguen en `pending/`.

## Delegaciones canónicas (RBAC)

| Fase plan | Cápsulas / agentes |
|-----------|-------------------|
| Mutación engine + tests | Tekton (`filesystem-ops` / `ecosystem-evolution`) |
| Frozen / skill / process YAML | `action:execute-process` → `entity-manager` |
| Git | `skill:git-manager` (`source-control`) |
| Archive PBI | Tekton en rama (norma cierre documental) |
| Cierre PR | `action:execute-process` → `delivery-close-cycle` |

`target_executor_rbac` esperado: `ecosystem-evolution`, `filesystem-ops`, `source-control`.

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| A1 sin poda KO | Checklist T1 absoluto; un fallo futuro reabre abrupto |
| Solo alinear payload sin frozen | T0.3 obligatorio (**L-FROZEN-ALIGN**) |
| F5 sigue escribiendo `accept_pr_handoff: true` con MERGE ausente | **L-HANDOFF-F5** + outputs process + tests T-A3 |
| Recortar cápsula en vez de declarar | Prohibido; declarar 1.1.0 |
| Versionar instancia en PR | Assert T2; Argos AC-GIT-CLEAN |
| Forja genoma por Write IDE | Abortar; solo `entity-manager` |
| Reabrir silencio #37 | Fuera de alcance explícito |

## Fuera de este plan

Rehab laterales; umbrales 1.1.0; bypass git crudo; `bug-fix` satélite aparte; faros Kaizen nuevos (salvo deuda ya sembrada absorbida).

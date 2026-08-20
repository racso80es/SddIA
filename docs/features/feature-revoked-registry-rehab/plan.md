---
feature_name: feature-revoked-registry-rehab
created: "2026-08-20"
process: refactorization
phase: blueprint
agents: dedalo
phases: T0-T5
branch_name: refactor/feature-revoked-registry-rehab
persist_ref: docs/features/feature-revoked-registry-rehab
pbi_ref: docs/todos/pending/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
document_id: PBI-FEATURE-185-REVOKED-REGISTRY
uuid: c8f4e2a1-7b3d-4e59-9f6a-2d1e0c9b8a7f
olas:
  - A1
  - A2
  - A3
---

# Plan — feature-revoked-registry-rehab

Blueprint ejecutable para Tekton. Contratos: `spec.md` laudos L-* + AC-*.

No es forja de proceso nuevo: no se instancia `{name}.md` bajo `directories.process`. El ciclo vigente es `refactorization` v1.2.2.

## T0 — Motor A2 fail-soft padre (AC-A2)

1. `delivery_close.rs`: añadir `delivery_push` al bucle de claves copiadas a `data` del envelope (L-DCC-DATA). Extraer helper puro `feature_parent_dcc_fail_soft_eligible` (o módulo compartido con `mark_fail_soft_if_secondary`) si evita duplicar predicado; **no** cambiar semántica DCC existente.
2. `phase_capsules.rs` · `capsule_feature_invoke_delivery_close`:
   - `invoke_process_full` (L-INVOKE-FULL).
   - Merge `pr_url` / `delivery_push` / snapshot / event_id al state padre siempre que existan en `envelope.data`.
   - `success` → `Ok` executed (hoy).
   - `!success` + predicado L-SOFT-OK → `Ok` `{status: failed, fail_soft: true, handler: feature-delivery-close, ...}`.
   - resto → `Err` causal.
3. `residual_runner.rs` rama `feature`\|`bug-fix`: preservar `fail_soft` del `Ok`; **no** marcar fail_soft en `Err` (L-CAUSAL-ERR, L-SYMMETRY).
4. Unit tests predicado + agregador (casos §7 spec: pr_url+higiene vs snapshot failed). **No** mutar `phase_terminal.rs`.

## T1 — Motor A3 poda hueca (AC-A3)

1. `thermodynamic.rs`: en payload REF, set `cycle_phase` (éxito: `derive_cycle_phase`; fallo lifecycle: `"failed"`). Set `lab_hollow: true` si lab-skip de cierre en el run (env y/o `reason` en phase_reports). PEC sin castrar.
2. `radamanto_batch_core.rs`: gate hueco **antes** de `samples.push` (L-BATCH-SKIP). Stamp `skipped`; `mark_consumed`.
3. Tests: initialized no entra; `lab_hollow`+completed no entra; `failed` real sí entra; PEC no es responsabilidad del batch.

## T2 — A1 instancia (AC-A1 / AC-ONTO)

Locus: `.SddIA/` (Cúmulo `radamanto.revoked_entities`, `radamanto.stats`). **Fuera del diff git.**

1. Eliminar `permanent.feature`. Assert `revoked.feature` ausente.
2. Bucket raíz `feature` únicamente: `status: healthy`, `recovery_attempts: 0`, `consecutive_success_count: 0`, `degraded_at: null`, `rehab_laudo: PBI-FEATURE-185-REVOKED-REGISTRY`, `rehabilitated_at` ISO. Recortar `samples`.
3. Assert `entity_type: process` si reaparece en Cerbero (rehab = borrar clave, no reescribir).
4. Assert laterales intactos (`bug-fix`, `emit-pr-audited-event`; fósiles `entities.feature` / `process:feature`).
5. Volcar evidencia (hashes/campos, **no** volcar secretos) en `execution.md`.

Orden: T2 **después** de T0+T1 en el mismo host (anti-recurrencia inmediata). T2 puede ejecutarse en paralelo de T3 documental si el motor ya está en disco local.

## T3 — Documental Tekton + evolution

1. `implementation.md` + `execution.md` (frontmatter patrón; `items` / `items_applied`).
2. Entrada `directories.evolution` vinculando UUID `c8f4e2a1-7b3d-4e59-9f6a-2d1e0c9b8a7f`.
3. Compilar/tests: `cargo test -p execute-process --lib` (filtros fail_soft / thermodynamic / radamanto_batch).
4. Assert diff: **no** incluye `.SddIA/cerbero/` ni `.SddIA/radamanto/`; **no** incluye `radamanto.thresholds.json`.

## T4 — Verificación Argos (AC-*)

Argos → `validacion.md`: `global`, `checks` mapeando AC-A1/ONTO/A2/A3/THRESH/DOC, `git_changes`, `pbi_archived: true`, `branch: refactor/feature-revoked-registry-rehab`.

## T5 — Cierre documental en rama + DCC

1. Mover PBI `PBI-FEATURE-185-REVOKED-REGISTRY` de `docs/todos/pending/` → `docs/todos/done/` **en esta rama** (Tekton; no Cúmulo Kaizen).
2. `action:execute-process` → `delivery-close-cycle` con `source_process: refactorization`, `persist_ref`, `branch_name`.
3. Git exclusivamente `skill:git-manager`.

## Orden innegociable

```text
T0 → T1 → T2 → T3 → T4 → T5
```

T0 y T1 son el PR motor. T2 no entra al PR. T4/T5 no adelantan `pbi_archived` si el PBI sigue en `pending/`.

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
| `invoke_process_full` spawn anidado en unit test | Testear helper puro + JSON envelope fixture; no exigir binario hijo en cada caso |
| Envelope DCC sin `delivery_push` en `data` | T0 paso 1 primero; fallback fases solo defensa |
| Simetría `bug-fix` interpreta rehab | Código compartido sí; Cerbero/stats `bug-fix` no se tocan (assert T2) |
| A1 incompleto (`attempts` ≥ 3) | Primer fallo re-emite `Domain_Entity_Deprecated` — checklist T2 absoluto |
| Hueco mal clasificado (`completed`+skip periférico) | Solo lab-skip de **cierre** o `cycle_phase` initialized/awaiting; no cualquier `skipped` |
| Forja genoma por error | Abortar; engine-only |

## Fuera de este plan

Faros Kaizen; rehab laterales Cerbero; umbrales 1.1.0; accept-pr históricos; residual Shell/`git-manager`.

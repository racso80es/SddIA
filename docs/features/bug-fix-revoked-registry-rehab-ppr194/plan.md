---
feature_name: bug-fix-revoked-registry-rehab-ppr194
created: "2026-08-27"
process: refactorization
phase: blueprint
agents: dedalo
phases: T0-T5
branch_name: refactor/bug-fix-revoked-registry-rehab-ppr194
persist_ref: docs/features/bug-fix-revoked-registry-rehab-ppr194
pbi_ref: docs/todos/pending/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #194).md
document_id: PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY
uuid: 8a4b0d3f-5c2e-4f9b-8d6a-7e8f9a0b1c2d
olas:
  - A1
---

# Plan — bug-fix-revoked-registry-rehab-ppr194

Blueprint ejecutable para Tekton. Contratos: `spec.md` laudos L-* + AC-*.

No es forja de proceso nuevo: ciclo vigente `refactorization` v1.2.2. Sin mutación motor/genoma (**L-NO-A2** / **L-TYPE-VERIFY PASS**).

## T0 — Assert tipológico (AC-TYPE-VERIFY)

1. Confirmar en disco: `resolve_entity_type` / `resolve_process_path` ⇒ `bug-fix`→`process` (`radamanto_batch_core.rs` + presence bajo `process_domain_roots`).
2. Si el assert **falla** (motor estampa `tool`): **ABORT** y escalar A2 motor — no improvisar. Si pasa: continuar (caso esperado).
3. Prohibido tocar engine en este ciclo salvo aborto/escalado.

## T1 — A1 instancia Yunque Rúnico (AC-A1 / AC-ONTO / AC-GIT-CLEAN)

Locus: Cúmulo `radamanto.revoked_entities` + `radamanto.stats`. **Fuera del diff git.**

1. Eliminar `revoked.bug-fix`. Assert `permanent.bug-fix` ausente.
2. Materializar bucket raíz `bug-fix` únicamente:
   - `entity_type: process`
   - `status: healthy`
   - `recovery_attempts: 0`
   - `consecutive_success_count: 0`
   - `degraded_at: null`
   - `rehab_laudo: PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY`
   - `rehabilitated_at` ISO UTC de la intervención
   - `samples: []` (bucket hoy ausente → no inventar KO)
3. Assert laterales intactos (`accept-pr`, `refactorization`, `emit-pr-audited-event`).
4. Assert cero `tool` residual para `bug-fix` en Cerbero/stats post-A1.
5. Volcar evidencia (campos/timestamp, no secretos) en `execution.md`.

## T2 — Documental Tekton + evolution

1. `implementation.md` + `execution.md` (frontmatter patrón; A1 + L-TYPE-VERIFY en `execution.md`).
2. Entrada `directories.evolution` UUID `8a4b0d3f-5c2e-4f9b-8d6a-7e8f9a0b1c2d`.
3. Assert diff: **no** incluye `.SddIA/cerbero/` ni `.SddIA/radamanto/`; **no** `radamanto.thresholds.json`; **no** engine.

## T3 — Verificación Argos (AC-*)

Argos → `validacion.md`: `global`, `checks` mapeando AC-A1/GIT-CLEAN/ONTO/TYPE-VERIFY/THRESH/DOC, `git_changes`, `pbi_archived: true`, `branch: refactor/bug-fix-revoked-registry-rehab-ppr194`.

## T4 — Cierre documental en rama

1. Mover PBI canónico `PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY` → `docs/todos/done/` (**en esta rama**).
2. Confirmar `validacion.md` con `pbi_archived: true`.
3. Prohibido segundo PR documental; prohibido Tekton/Argos sembrando Kaizen bajo `docs/todos/`.

## T5 — Cierre de entrega DCC

1. `action:execute-process` → `delivery-close-cycle` con `source_process: refactorization`, `persist_ref`, `branch_name`.
2. Git exclusivamente `skill:git-manager`.
3. Post-rehab: despacho soberano `bug-fix` operable; ontología `process`; umbrales 1.1.0 intactos.

## Orden innegociable

```text
T0 → T1 → T2 → T3 → T4 → T5
```

T0 = gate tipológico (sin PR motor). T1 no entra al PR. T3/T4 no adelantan `pbi_archived` si el PBI sigue en `pending/`.

## Delegaciones canónicas (RBAC)

| Fase plan | Cápsulas / agentes |
|-----------|-------------------|
| Assert tipología + A1 instancia + docs | Tekton (`filesystem-ops` / `ecosystem-evolution`) |
| Git | `skill:git-manager` (`source-control`) |
| Archive PBI | Tekton en rama (norma cierre documental) |
| Cierre PR | `action:execute-process` → `delivery-close-cycle` |

`target_executor_rbac` esperado: `ecosystem-evolution`, `filesystem-ops`, `source-control`.

Genoma `directories.process` / `agents` / engine **no** se forja.

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| A1 sin materializar stats | Checklist T1 absoluto; bucket ausente ⇒ CREATE sano |
| Conservar `entity_type: tool` | **L-ONTOLOGY**; assert T-A1-ONTO |
| Versionar instancia en PR | Assert T2; Argos AC-GIT-CLEAN |
| Inventar A2 «por simetría» accept-pr | **L-WAVES** / **L-NO-A2**; tipología PASS |
| Rehab laterales del mismo PPR #194 | Assert T1 laterales; fuera de alcance |
| Forja genoma por Write IDE | Abortar; sin touchpoints genoma |

## Fuera de este plan

Rehab laterales; umbrales 1.1.0; mutación motor/hollow; bypass git crudo; faros Kaizen nuevos.

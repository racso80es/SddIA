---
feature_name: aut-136-ppr-verdict-qa-attestation-pre-handoff
created: "2026-11-27"
process: bug-fix
phase: design
branch_name: fix/aut-136-ppr-verdict-qa-attestation-pre-handoff
persist_ref: docs/fixes/aut-136-ppr-verdict-qa-attestation-pre-handoff
pbi_ref: AUT-136
tracker_ref: AUT-136
historia_ref: AUT-131
execution_id: "f3c98fff-6b95-4122-85b8-7c64463f3f5d"
---
# Spec — AUT-136 PPR promote verdict + qa-attestation pre-Handoff

## Misión

Tras Fase 5 «Veredicto y bloqueo» sin bloqueos previos, `pull-request-review` debe promover `state.verdict=aprobado` **antes** de `maybe_emit_qa_attestation_before_handoff`, de modo que `qa-attestation/1.0` y el handoff a `accept-pr` (F7) vean el veredicto ya propagado.

## Diagnóstico (causa raíz)

AUT-134 añadió emisión de atestación post-F5, pero el promote de `state.verdict` a `aprobado` solo ocurre en el cierre residual terminal (tras **todas** las fases, incluida F7). Orden actual defectuoso:

1. F5 Veredicto ejecuta (Argos/barrier) sin escribir necesariamente `state.verdict`.
2. Hook post-F5 llama `maybe_emit_qa_attestation_before_handoff` → `try_write_from_ppr` falla si no hay `state.verdict`/`argos_verdict`/`validacion.md` suficiente.
3. F7 `handoff_accept_pr` exige `state.verdict==aprobado` → `skipped` / `verdict_not_aprobado`.
4. Solo al final del residual se inserta `verdict=aprobado` (demasiado tarde para F7 y para la atestación pre-handoff).

## AC

| ID | Criterio | Oráculo |
|----|----------|---------|
| AC-1 | Tras F5 limpia, `state.verdict=aprobado` antes de F7 | Unit test `promote_ppr_verdict_after_veredicto` + inspección state |
| AC-2 | `qa-attestation/1.0` escrita post-promote y pre-F7 | `qa_attestation_written` / path bajo proofs tras hook |
| AC-3 | F7 no salta por `verdict_not_aprobado` cuando F5 sin bloqueos | Test handoff con state promovido |
| AC-4 | Sin promote si bloqueo previo (triaje documental / `argos_verdict=block` / F5 blocked) | Tests negativos barrier ADU-CA1 |
| AC-5 | Reutilizar `ppr_verdict_aprobado` como `pub(crate)` | Compilación crate + test existente frontmatter |
| AC-6 | Motor verde | `cargo test -p execute-process --lib` filtros PPR/attestation |

## Superficies

- `SddIA/engine/execute-process/src/engine/pull_request_review.rs`: nueva `promote_ppr_verdict_after_veredicto`; mantiene `maybe_emit_qa_attestation_before_handoff`.
- `SddIA/engine/execute-process/src/engine/residual_runner.rs`: hook post-«Veredicto y bloqueo» — **promote → maybe_emit** (orden estricto).
- `SddIA/engine/execute-process/src/engine/qa_attestation.rs`: `ppr_verdict_aprobado` → `pub(crate)`.

## Diseño estabilizado

1. **`promote_ppr_verdict_after_veredicto(repo, state, inputs)`**
   - No-op si `state.verdict` ya es `aprobado`.
   - No-op / no promover si señales de bloqueo: `documental_triage_blocked`, `argos_verdict==block`, o último report de «Veredicto y bloqueo» con `status=blocked`.
   - Promover solo si `qa_attestation::ppr_verdict_aprobado(repo, state, inputs)` es true **o** el report de F5 es `executed`/`simulated` sin bloqueo y no hay señales de rechazo (paridad con promote terminal actual, pero adelantado).
   - Efecto: `state.verdict = "aprobado"`; si falta, alinear `argos_verdict = "pass"` cuando el promote se basa en APTO/aprobado.
2. **Hook residual** (tras cada fase, solo PPR + nombre F5):
   ```text
   promote_ppr_verdict_after_veredicto(...);
   maybe_emit_qa_attestation_before_handoff(...);
   ```
3. **Fallback terminal** (cierre residual): conservar escritura de atestación si aún no `qa_attestation_written` (idempotencia AUT-134); el promote temprano no elimina ese safety net.
4. **Fuera de alcance**: contrato accept-pr, restore worktree, reconcile agent_runtime (ya cubiertos por AUT-134).

## Riesgos / no-regresión

- No promover en caminos ADU-CA1 (triaje documental blocked → Veredicto barrier).
- No emitir atestación con veredicto falso: el promote debe anclarse en `ppr_verdict_aprobado` o en ausencia explícita de bloqueo + F5 no-blocked.
- Mantener `qa_attestation_written` como candado anti-doble-escritura.

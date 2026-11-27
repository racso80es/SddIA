---
feature_name: aut-132-gate-anti-anemia-reglas-contractuales
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-132-gate-anti-anemia-reglas-contractuales
persist_ref: docs/refactors/aut-132-gate-anti-anemia-reglas-contractuales
tracker_ref: AUT-132
historia_ref: AUT-125
execution_id: "47facb2e-84b4-48b8-ba38-0429be4605fd"
phases:
  - T1-rama
  - T2-ola0-oraculos
  - T3-paridad-codigo
  - T4-evolution
  - T5-validacion
  - T6-cierre
---
# Plan — AUT-132

Blueprint Tekton/Argos. Código canónico del peaje ya heredado; prioridad = **verificar + sellar**, no reimplementar.

1. **T1 — Rama:** `refactor/aut-132-gate-anti-anemia-reglas-contractuales` desde `main` post-laudo AUT-125 (git solo `skill:git-manager`). Worktree limpio antes de Ola 0.
2. **T2 — Ola 0 (oráculos en rojo/verde):** Ejecutar AC-1…AC-7 / AC-TRUNC de `spec.md`. Registrar bajo `persist_ref` evidencia (`oracle-*.log` o extracto en `execution.md`). Si algún oráculo falla → T3 obligatorio; si todos verdes → T3 es auditoría de paridad (sin diff productivo).
3. **T3 — Paridad código (solo si hay gap):** Alinear `entity_no_anemia.rs`, `gate_entity_no_anemia.rs`, `phase_capsules.rs` (`index_anemia_gate`), `pull_request_review.rs` (`anemia_triage`) al contrato de `spec.md`. Prohibido tocar forjas/`entity_manager` (AUT-131) o remediación masiva de genoma (AUT-133). Confirmar ausencia de `remediate_bulk.py` / `rebuild_actions_index.py`.
4. **T4 — Evolución:** Verificar `relacionado` AUT-132 en `SddIA/evolution/a8f3c2e1-9d4b-4f7a-b6e5-1c2d3e4f5a6b.md`. Si el cierre exige traza propia, alta `tipo_operacion: alta` + fila `Evolution_log.md` (sin inventar hash).
5. **T5 — Verificación Argos:**
   - `cargo test -p execute-process entity_no_anemia`
   - `cargo test -p sddia-qa gate_entity_no_anemia`
   - `cargo test -p execute-process index_anemia_gate_ tech_triage_`
   - `sddia-qa gate-entity-no-anemia --if-touched --base origin/main --json` → `violation_count: 0` en delta
   - Rellenar `validacion.md` (checks ↔ AC; CI `PENDIENTE-CI` hasta run verde).
6. **T6 — Cierre:** PR único → `pull-request-review` (anemia in-process) → merge. No KM/`docs/todos/` desde Tekton/Argos.

## Dependencias / exclusiones

| Hermano | Relación |
| --- | --- |
| AUT-125 | Padre / ratchet `--if-touched` |
| AUT-131 | Paralelo (forjas); no solapar superficies |
| AUT-133 | Deuda genoma 322; no bajar línea base aquí |

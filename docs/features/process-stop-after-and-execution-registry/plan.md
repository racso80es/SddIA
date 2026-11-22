---


feature_name: process-stop-after-and-execution-registry
created: "2026-11-21"
updated: "2026-11-21"
process: feature
purpose: Plan ejecutable — stop_after por phase_id + write-ahead + resume (MAN-34)
branch_name: feat/process-stop-after-and-execution-registry
persist_ref: docs/features/process-stop-after-and-execution-registry
pbi_ref: MAN-34
document_id: PBI-PROCESS-STOP-AFTER-AND-EXECUTION-REGISTRY
pbi_uuid: "7f4e9a12-3c8b-4d5e-9a21-6b8c4e109f32"
agent: dedalo
phase: diseno-blueprint
version_plan: "1.0.0"
status: blueprint-ready
norm_ref: features-documentation-pattern@1.3.2
phases:
  - H0
  - H1
  - H2
  - H3
  - H4
  - H5
execution_id: "d7301123-c49f-4627-90f1-ff1fe78bcc66"
---
# Plan de ejecución — process-stop-after-and-execution-registry

Consumir `spec.md` + `clarify.md` en solo lectura. No reabrir L-PHASE-ID…L-WI.
Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`.
Genoma process/contract solo vía **entity-manager** (DA-2 / **D-GENOME**).
Orden: **H0 → H1 → H2 → H3 → H4 → H5**.

## H0 — Preflight

| # | Acción | Criterio |
|---|--------|----------|
| 0.1 | Rama `feat/process-stop-after-and-execution-registry` vía git-manager | Creable / existente desde `main` |
| 0.2 | Snapshot legado: `stop_after_design`, `is_dedalo_design_phase`, inyección TQM env, persist solo al cierre, WI en Fase 1 | Anotar en `execution.md` |
| 0.3 | Inventariar call sites: `executor.rs`, `task_queue_manager.rs`, `thermodynamic.rs`, `workspace_init.rs`, `verify_process_integrity.rs`, `resolver.rs` | Lista en `implementation.md` |
| 0.4 | Inventariar procesos del catálogo multi-root sin `phases[].id` | Lista para H1/entity-manager |

**Gate H0:** inventario + rama listos; sin mutación aún.

## H1 — Contrato + `phase_id` en catálogo

| # | Acción | Paths |
|---|--------|-------|
| 1.1 | Bump `process-contract` (≥1.5.0): exigir `phases[].id`; documentar inputs base `stop_after` lista + `resume` bool | `SddIA/process/process-contract.md` vía entity-manager |
| 1.2 | Añadir `id` canónicos delivery (§2.1 spec) en `feature` / `bug-fix` / `refactorization` | domain roots process |
| 1.3 | Añadir `id` al resto del catálogo (inventario H0) | entity-manager + hash_signature |
| 1.4 | Extender `verify-process-integrity`: `id` presente, único, `snake_case` | `verify_process_integrity.rs` |
| 1.5 | Ajustar texto ignición AUT-65 delivery: WI solo en entry tekton (**D-WI** / **L-WI**) | genoma delivery |
| 1.6 | `verify-process-integrity` verde | **KA-STOP-1** |

**Gate H1:** AC-1 / KA-STOP-1.

## H2 — Erradicación legacy + `stop_after` lista

| # | Acción | Paths |
|---|--------|-------|
| 2.1 | Implementar `normalize_stop_after` (string→lista) | módulo compartido / executor |
| 2.2 | Evaluar barrera por `phase.id ∈ stop_after` tras `executed` | `executor.rs` / **D-STOP-EVAL** |
| 2.3 | Eliminar `SDDIA_TQM_STOP_AFTER`, `stop_after_design`, `is_dedalo_design_phase` | **D-NO-ENV** / **D-ERADICATE** |
| 2.4 | TQM: leer lista; **no** inyectar env en `child_env_for_kalma2` | `task_queue_manager.rs` |
| 2.5 | Tests: parada múltiple; deser legacy string; grep-negativo env/literales | **KA-STOP-2** / **KA-STOP-3** |
| 2.6 | Confirmar `derive_cycle_phase` `stop_after`→`awaiting_agents` | **D-THERMO** |

**Gate H2:** AC-2 + AC-3.

## H3 — Write-ahead + índice active-executions

| # | Acción | Paths |
|---|--------|-------|
| 3.1 | Crear `execution_registry.rs` (o equivalente): persist atómico reports + índice | **D-WRITEAHEAD** / **D-INDEX** |
| 3.2 | Init skeleton al arranque; pre `in_progress`; post status; barrera-cierre | `executor.rs` / `workspace_init` si aplica |
| 3.3 | Upsert `.SddIA/state/active-executions.json` | **L-RUNTIME-LOCUS** |
| 3.4 | Test: ningún JSON runtime bajo `persist_ref`; flushes visibles mid-run | **KA-STOP-4** |
| 3.5 | Registrar `mod` en `engine/mod.rs` | Compila |

**Gate H3:** AC-4.

## H4 — Resume + Work_Initiated

| # | Acción | Paths |
|---|--------|-------|
| 4.1 | Input `resume` + resolución `execution_id` / índice | **D-RESUME** |
| 4.2 | Skip `executed` + artifact check diseño (`spec.md` / `plan.md`) | **D-ARTIFACT-CHECK** / **KA-STOP-5** |
| 4.3 | Relanzar `in_progress`/`failed`/`blocked`; no rehacer 1–2 | **KA-STOP-6** |
| 4.4 | Barrera `stop_after` consumida no bloquea avance | **L-RESUME** |
| 4.5 | Mover emisión `Work_Initiated` a entry tekton; quitar de Fase 1 | `workspace_init.rs` + entry tekton |
| 4.6 | Tests WI: stop design sin evento; tekton emite una vez | **KA-STOP-7** |

**Gate H4:** AC-5…AC-7.

## H5 — Suite, docs de forja y handoff Argos

| # | Acción | Criterio |
|---|--------|----------|
| 5.1 | `cargo build --release` + `cargo test` (filtros stop/resume/registry/integrity) | **KA-STOP-8** |
| 5.2 | Clippy acotado al crate si el régimen lo exige | Verde |
| 5.3 | Materializar `implementation.md` + `execution.md` con evidencia real | **L-TRUTH** |
| 5.4 | Confirmar supersede informe TQM-STOP-AFTER citado en expediente | **D-SUPERSEDE** |
| 5.5 | Handoff Argos / `validacion.md` | Checks AC-1…8 |

**Gate H5:** ACs del `objectives.md` verdes con evidencia.

## Matriz AC ↔ hijos

| AC | Hijos | Verificación |
|----|-------|--------------|
| AC-1 (KA-STOP-1) | H1 | contrato + catálogo + integrity |
| AC-2 (KA-STOP-2) | H2 | grep legacy limpio |
| AC-3 (KA-STOP-3) | H2 | stop por `phase_id` + `awaiting_agents` |
| AC-4 (KA-STOP-4) | H3 | write-ahead + índice + locus |
| AC-5 (KA-STOP-5) | H4 | resume post-design |
| AC-6 (KA-STOP-6) | H4 | resume mid-phase |
| AC-7 (KA-STOP-7) | H4 | WI solo tekton |
| AC-8 (KA-STOP-8) | H5 | build + test verdes |

## Stop conditions (blocked)

- Catálogo no migrable a `id` sin romper integrity → **blocked** con lista de procesos; no skipear gate.
- Imposible persistir atómicamente bajo `.SddIA/state/` en el host → **blocked**; no escribir runtime bajo `docs/`.
- entity-manager / DA-2 no disponible para mutar genoma → **blocked**; no forja directa de process YAML.
- Tentación de conservar env `SDDIA_TQM_STOP_AFTER` como alias → rechazo de diseño (**L-NO-ENV**).
- Artifact check de diseño no materializable de forma estable → fail-closed + friction_id; no inventar artefactos (**L-TRUTH**).

## Handoff Tekton

1. Leer `spec.md` §§1–5 y este plan; no reabrir laudos Mayeuta.
2. Ejecutar H0→H5; materializar `implementation.md` + `execution.md`.
3. Genoma solo vía entity-manager; preservar L-THERMO del antecedente verdict-barrier.
4. No escribir bajo `docs/todos/` ni JSON runtime bajo `persist_ref`.
5. Al verde local, cierre vía Argos + DCC contra ACs KA-STOP-1…8.

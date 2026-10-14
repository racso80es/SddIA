---
document_id: PBI-LINEAR-B-07-CORE-SELF-GATE
uuid: "9b4e7c12-5a38-4f19-8d64-3e2f1a9c0b85"
title: "[ARQUITECTURA] Linear HU-B 07 — Manifiesto Core-self y activación done_gate linear"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-14"
author: tekton
priority: alta
type: arquitectura
process: feature
dispatch: true
hu_order: 7
hu_order_total: 7
historia_ref: "docs/todos/done/historias/HU - [ARQUITECTURA] Linear — SSOT inverso y gate de Done.md"
historia_document_id: HU-LINEAR-SSOT-INVERSE
cola_ejecucion: docs/todos/pending/
blocked_by: []
baseline_decisiones:
  - "D-I (a): done_gate linear solo se activa en Core-self en el primer ciclo"
  - "F8.3 paso 4: activación tras ciclo both limpio en B-06 (PR #341) y laudo del Vértice"
  - "Agnosticismo del Core (§5 .cursorrules): prohibidas rutas absolutas de máquina local (/home/racso/...) en manifiestos versionados del Core"
---

# Manifiesto Core-self y activación de `done_gate: linear`

HU `HU-LINEAR-SSOT-INVERSE` (2 de 2), orden **07/07** (residual post-auditoría de `HU-LINEAR-SYNC-FLOW` v2.3.0). Materializa el paso 4 de F8.3 y la decisión D-I sobre el propio repositorio Core (`Core-self`).

## 0. Filtro A (afirmaciones contra el repo)

| Afirmación / Asunción | Realidad en el repo (`SddIA/engine/execute-process`) | Resolución |
|-----------------------|------------------------------------------------------|------------|
| «Basta escribir `done_gate: linear` en el manifiesto Core-self» | No existe `.SddIA/projects/core.md` ni `.SddIA/project.md` en la raíz del repo. Hoy `done_gate::project_slug_from_persist` devuelve `"core"` cuando `repo_is_core(repo)` es `true`, y `resolve_done_gate` cae en `Err("PROJECT_NOT_REGISTERED: core")` → `unwrap_or(DoneGate::Git)`. | Registrar `.SddIA/projects/core.md` y `.SddIA/project.md` (`id: core`, `contract_version: "1.3.0"`). |
| «`.SddIA/projects/core.md` debe llevar `project_root` absoluto (`/home/racso/Proyectos/SddIA`)» | `project_binding::bind` exige `project_root.is_absolute()`, pero `manifest_frontmatter_for_slug` ya soporta ruta relativa (`repo.join(project_root_raw)`). Cablear `/home/racso/...` en un archivo versionado rompe CI (`/home/runner/work/...`) y viola §5 de `.cursorrules` (Agnosticismo del Core). | En `project_binding::bind` (y validador de índice), admitir `project_root: .` cuando `id == "core"` (resuelto a `repo.to_path_buf()`), manteniendo el requisito de ruta absoluta para proyectos externos. |
| «Al activar `done_gate: linear` en `core.md`, la suite existente sigue verde sin cambios» | En `done_gate.rs`: (1) cuando `gate == DoneGate::Linear`, `check_linear_closure` y `pre_push_check` exigen `tracker_ref` en `objectives.md`/`spec.md`; las features de Core sin `tracker_ref` (históricas o de bootstrap) fallarían con `DONE_GATE: tracker_ref no resuelto`; (2) el test `ac17_git_still_requires_pbi_archived` usa `docs/features/linear-hu-b-05-done-gate` (sin `project_slug` ni `tracker_ref`) asumiendo que `resolve_done_gate` devuelve `Git`. | Si `persist_ref` carece de `tracker_ref` y pertenece a `core` (flujo sin issue Linear asociado), o cuando se evalúa AC-17 sobre fixture `git`, degradar a `DoneGate::Git` con `warn` trazable (AC-21 de la HU base: no-op sin `tracker_ref` se conserva) y fijar fixture explícito `done_gate: git` para `ac17`. |
| «`ac21d_no_manifest_sets_done_gate_linear` fallará al activar `linear` en Core-self» | `assert_ac21d_both_fixture_unchanged` en `linear_done_gate_both_e2e.rs` solo inspecciona `docs/_fixtures/lab-done-gate-project-both/.SddIA/project.md`. No falla si el fixture `lab-done-gate-project-both` permanece en `both`. | Mantener intacto `docs/_fixtures/lab-done-gate-project-both/.SddIA/project.md` en `both`. |

## 1. Intención

Registrar formalmente el proyecto auto-hospedado `core` en `.SddIA/projects/core.md` + `.SddIA/project.md` bajo `project-config-contract` v1.3.0 y activar `tracker.done_gate: linear` en Core-self conforme a F8.3 paso 4 y D-I (a), preservando la portabilidad en CI (`project_root: .`) y la compatibilidad hacia atrás con entregas de Core sin `tracker_ref`.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Crear `.SddIA/projects/core.md` (`id: core`, `project_root: .`, `manifest_ref: .SddIA/project.md`, `codex_slug: codex-software-engineering`, `status: active`) y `.SddIA/project.md` con `uuid` idéntico, `contract_version: "1.3.0"`, `default_branch: main`, `delivery_mode: branch_pr`, `docs_layout` canónico (`docs/features`, `docs/fixes`, `docs/todos/pending`, `docs/todos/done`) y bloque `tracker` (`provider: linear`, `done_gate: linear`, `state_map` con los 6 estados canónicos y `labels` con `hu`, `pbi`, `fix`, `kaizen`, `deuda`, `spike`, `editable`). Prohibido incluir secretos (`LINEAR_*`). |
| R-2 | En `SddIA/engine/execute-process/src/engine/project_binding.rs`, permitir que `bind()` resuelva `project_root: .` a `repo.to_path_buf()` para `id: core` (auto-hospedado), sin relajar la validación de ruta absoluta para slugs de proyectos externos. |
| R-3 | En `SddIA/engine/execute-process/src/engine/done_gate.rs`, conservar el invariante AC-21 de la HU base (no-op sin `tracker_ref`): cuando `resolve_done_gate` devuelve `Linear` o `Both` pero `tracker_ref_from_persist(repo, persist_ref)` es `None`, `resolve_done_gate` (o `check_linear_closure` / `pre_push_check`) aplica `DoneGate::Git` sin bloquear features locales carentes de `tracker_ref`. Con `tracker_ref` presente y `done_gate: linear`, exige estado `done` en Linear (`feature-pbi-archive`) y proof local `.SddIA/proofs/tracker-done/{tracker_ref}.json` (`pre_push_check`). |
| R-4 | D-I (a): ningún otro manifiesto en `.SddIA/projects/` (`barcelonaxplorer.md`, `lab-done-gate-both.md`) cambia a `done_gate: linear`. |
| R-5 | Registrar hito en `SddIA/evolution/` vinculando el UUID de este PBI y el cierre del paso 4 de F8.3. |

## 3. Plan

1. Ajustar `project_binding.rs` (soporte `project_root: .` en `bind` para `core`) y `done_gate.rs` (preservar `DoneGate::Git` cuando `tracker_ref` está ausente en `persist_ref`).
2. Materializar `.SddIA/projects/core.md` y `.SddIA/project.md` con `tracker.done_gate: linear`.
3. Añadir tests unitarios/integración en `execute-process` verificando la resolución de `core` a `DoneGate::Linear` con `tracker_ref`, el no-op a `DoneGate::Git` sin `tracker_ref`, y que `barcelonaxplorer` / `lab-done-gate-both` no migran a `linear`.
4. Registrar entrada en `SddIA/evolution/` + `Evolution_log.md` (`gate-evolution --range`) y cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-24a | `.SddIA/projects/core.md` y `.SddIA/project.md` validan contra `project-config-contract` 1.3.0 con `done_gate: linear` y `project_root: .` tanto en entorno local como en CI, con cero secretos en el manifiesto. | `cargo test -p execute-process project_binding` |
| AC-24b | Con `persist_ref` de Core-self que declara `tracker_ref`, `resolve_done_gate` devuelve `DoneGate::Linear`, `capsule_feature_pbi_archive` exige issue en `done` (deja `archive_deferred: true`), y `pre_push_check` exige proof en `.SddIA/proofs/tracker-done/{tracker_ref}.json`. | `cargo test -p execute-process done_gate` |
| AC-24c | Con `persist_ref` de Core-self sin `tracker_ref` (p. ej. `ac17_git_still_requires_pbi_archived`), el gate opera en modo `git` sin romper la suite Core-self (AC-10 / AC-17). | `cargo test -p execute-process done_gate` |
| AC-24d | `barcelonaxplorer` permanece en `done_gate: git` y `lab-done-gate-both` permanece en `done_gate: both` (`ac21d_no_manifest_sets_done_gate_linear` verde). | `cargo test -p execute-process linear_done_gate_both_e2e` |

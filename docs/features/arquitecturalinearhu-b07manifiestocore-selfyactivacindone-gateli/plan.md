---

feature_name: arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
created: "2026-10-14"
process: feature
phases: [T0, T1, T2, T3, T4, T5]
document_id: PBI-LINEAR-B-07-CORE-SELF-GATE
pbi_uuid: "9b4e7c12-5a38-4f19-8d64-3e2f1a9c0b85"
branch_name: feat/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
persist_ref: docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
pbi_ref: "docs/todos/pending/[ARQUITECTURA] Linear HU-B 07 — Manifiesto Core-self y activación done_gate linear.md"
correlation_id: "fd2db080-dbf3-44ab-b70f-63b90d748455"
phase: blueprint
agents: dedalo
execution_id: "0061cd1b-fe5e-46d6-80f1-cd4db92a8559"
---
# Plan — HU-B 07 Core-self `done_gate: linear`

Blueprint Dedalo → Tekton. Sin forja genómica `entity-manager`. Sin migrar otros proyectos (D-I). Orden anti-rotura: degrade **antes** de registrar Core `linear`.

## Fases

### T0 — Documentación Dedalo
- [x] Consumir `objectives.md` / `clarify.md` (R-1…R-7, C-1…C-9)
- [x] `spec.md` laudos L-CORE…L-EVOL + UUID canónico `7c4e2a91-…`
- [x] este `plan.md` (`execution_id` f351761b-…)

### T1 — Motor `project_binding::bind` (R-2 / L-ROOT / L-CONTRACT)
- [ ] Resolver `project_root: .`/`./` → `repo.to_path_buf()` solo si `id`/`slug` == `core`
- [ ] Mantener error absoluto para slugs externos
- [ ] Prosa L-CONTRACT en `project-config-contract.md` (sin bump SemVer)
- [ ] Tests `bind_core_dot_project_root` + `bind_external_rejects_relative`
- [ ] Si isolation falla con labs relativos: exclusión mínima L-ISOLATION
- **WIP:** código/prosa/tests ya presentes en working tree; Tekton **verifica** y no regenera UUID.

### T2 — Motor `done_gate::resolve_done_gate` (R-3 / L-DEGRADE)
- [ ] Degradar `Linear`/`Both` → `Git` cuando `tracker_ref_from_persist` es `None` + warn
- [ ] No tocar firmas de `pre_push_check` / archive salvo herencia del resolve
- [ ] Tests AC-24b (con `tracker_ref`) y AC-24c / no-reg AC-17 (sin `tracker_ref`)
- **WIP:** implementación presente; falta evidencia `cargo test`.

### T3 — Manifiestos Core-self (R-1 / L-CORE / L-TEAM)
- [ ] Mantener UUID `7c4e2a91-8b3f-4d6e-a1c0-9e5f2b8d0473` en índice + manifiesto
- [ ] `.SddIA/projects/core.md` + `.SddIA/project.md` según `spec.md` §4
- [ ] `team_key: OSC`, `done_gate: linear`, state_map 6, labels completas; cero `LINEAR_*`
- [ ] No mutar `barcelonaxplorer` / lab-*
- **WIP:** manifiestos ya en disco; Tekton confirma paridad con plantilla §4.

### T4 — Suite e2e / D-I (AC-24a…d)
- [ ] `cargo test -p execute-process project_binding`
- [ ] `cargo test -p execute-process done_gate`
- [ ] `cargo test -p execute-process linear_done_gate_both_e2e`
- Residual: sin evidencia previa (BLOCKED_SHELL en ejecuciones Tekton anteriores).

### T5 — Evolution + cierre documental (R-5 / R-6 / L-EVOL / L-SELF)
- [ ] Hito `SddIA/evolution/` + `gate-evolution --range` / `Evolution_log.md` (UUID PBI)
- [ ] `implementation.md` / `execution.md` (Tekton)
- [ ] Argos: `validacion.md` APTO + checks AC-24a…d
- [ ] PBI → `docs/todos/done/` + `pbi_archived: true` (vía **git**: sin `tracker_ref`)
- [ ] `delivery-close-cycle` (mismo PR; no segundo PR docs)

## Orden de forja

```text
T1 (bind) → T2 (degrade) → T3 (manifiestos) → T4 (tests) → T5 (evolution/cierre)
```

T1∥T2 admisible si tests cruzados lo exigen. **T3 después de T2**. Si WIP ya tiene T1–T3: Tekton salta a verificación T4 → T5.

## Delegación / RBAC

| Fase | delegates_to / capacidad | Contexto |
|------|--------------------------|----------|
| T1–T4 | `agent:tekton` | `filesystem-ops`, `ecosystem-evolution` |
| Tests | `skill:shell-executor` / cargo local Tekton | `quality-assurance` (Argos en T5) |
| Git | `skill:git-manager` vía `./sddia-run.sh --tool git-manager` | `source-control`; prohibido bypass raw destructivo |
| Verificación | `agent:argos` | Consume `objectives.md` + `spec.md` → `validacion.md` |
| KM semillas | Solo `agent:cumulo` / `Kaizen_Alert_Required` | Dedalo/Tekton/Argos no escriben TODOs bajo `docs/todos/` |
| Cierre PR | `action:execute-process` → `delivery-close-cycle` | Tras Argos APTO |

## Criterios de salida por fase

| Fase | Done local |
|------|------------|
| T1 | `bind(core)` con `.` verde; externos absolutos intactos |
| T2 | Sin `tracker_ref` → `Git`; con `tracker_ref` → `Linear` |
| T3 | Manifiestos canónicos; `done_gate_for_inputs(core) == Linear` (antes de degrade por persist) |
| T4 | Tres comandos cargo verdes |
| T5 | Cascada documental + PBI archivado en rama + evolution F8.3.4 |

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Activar manifiestos antes del degrade | Orden T2 → T3 |
| Isolation `bind(core)` vs labs | L-ISOLATION |
| Cierre B-07 sin Linear issue | L-SELF — vía git |
| Inventar éxito de T4/T5 | Prohibido; evidencia o `blocked` explícito |

---

feature_name: arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
created: "2026-10-14"
process: feature
base: main
scope: "Manifiesto Core-self + done_gate: linear (F8.3 paso 4 / D-I (a))"
version_spec: "1.1.0"
uuid: "c7e2a91b-4f38-4d6e-a8c1-0b5d9e3f7a24"
status: dedalo_locked
document_id: PBI-LINEAR-B-07-CORE-SELF-GATE
pbi_uuid: "9b4e7c12-5a38-4f19-8d64-3e2f1a9c0b85"
historia_document_id: HU-LINEAR-SSOT-INVERSE
branch_name: feat/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
persist_ref: docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
pbi_ref: "docs/todos/pending/[ARQUITECTURA] Linear HU-B 07 — Manifiesto Core-self y activación done_gate linear.md"
correlation_id: "fd2db080-dbf3-44ab-b70f-63b90d748455"
project_slug: core
impacts_doc: true
phase: blueprint
agents: dedalo
core_manifest_uuid: "7c4e2a91-8b3f-4d6e-a1c0-9e5f2b8d0473"
execution_id: "0061cd1b-fe5e-46d6-80f1-cd4db92a8559"
---
# Spec — HU-B 07 Manifiesto Core-self y `done_gate: linear`

Consumo: `objectives.md` (R-1…R-7, AC-24a…d) + `clarify.md` (C-1…C-9). Norma: `features-documentation-pattern` 1.3.0.

## 1. Problema

`project_slug_from_persist` en repo Core → `"core"`, pero sin registro el motor cae en `PROJECT_NOT_REGISTERED: core` → `DoneGate::Git`. F8.3 paso 4 / D-I (a) exigen `tracker.done_gate: linear` **solo** en Core-self tras B-06 (#341).

Cablear `project_root` absoluto de máquina viola §5 Agnosticismo del Core y rompe CI. Activar `linear` sin degradación rompe features Core sin `tracker_ref` (AC-17 / bootstrap).

## 2. Mapa requisitos → laudos

| Req | Laudo |
|-----|-------|
| R-1 | **L-CORE** + **L-TEAM** |
| R-2 | **L-ROOT** + **L-ISOLATION** |
| R-3 | **L-DEGRADE** + **L-STRICT** |
| R-4 | **L-DI** |
| R-5 | **L-EVOL** |
| R-6 | **L-SELF** |
| R-7 | **L-CONTRACT** |

## 3. Laudos Dedalo

| ID | Laudo |
|----|-------|
| **L-CORE** | Índice `.SddIA/projects/core.md` + manifiesto `.SddIA/project.md` (`id: core`, mismo `uuid`, `contract_version: "1.3.0"`, `done_gate: linear`). UUID canónico **ya materializado**: `7c4e2a91-8b3f-4d6e-a1c0-9e5f2b8d0473`. Sin bump de `contract_version`. |
| **L-ROOT** | Índice: `project_root: .` (único valor versionado admisible). En `bind()`: si `id`/`slug` == `"core"` y raw ∈ {`.`, `./`} → `repo.to_path_buf()`. Slugs no-`core` siguen exigiendo ruta absoluta. |
| **L-CONTRACT** | Ajuste **prosa** en `project-config-contract.md` 1.3.0 (sin SemVer bump): `project_root` absoluto **o**, solo con `id: core`, literal `.`/`./`. Forma/validador alineados a L-ROOT. |
| **L-TEAM** | `tracker.team_key: OSC`. `project_id` omitido. Cero secretos `LINEAR_*`. `env_ref` opcional solo si bóveda ya existe; no inventar vault. |
| **L-DEGRADE** | Degradación central en `resolve_done_gate`: si gate manifiesto ∈ {`Linear`,`Both`} **y** `tracker_ref_from_persist` es `None` → `DoneGate::Git` + `warn` trazable (`done_gate degraded to git: tracker_ref ausente`). `pre_push_check` / archive heredan el no-op AC-21. |
| **L-STRICT** | Con `tracker_ref` y gate efectivo `Linear`: archive exige issue `done` (`archive_deferred`); `pre_push_check` exige proof `.SddIA/proofs/tracker-done/{tracker_ref}.json`. |
| **L-DI** | D-I (a): no mutar `barcelonaxplorer`, `lab-done-gate`, `lab-done-gate-both` ni fixtures `docs/_fixtures/lab-done-gate-project*`. Fixture both permanece `both`. |
| **L-ISOLATION** | Al `bind(core)` con root = repo: raíces relativas de lab en índice no disparan falso `PROJECT_GIT_INVALID`. Exclusión mínima: omitir raíces **relativas** de fixtures en `assert_git_isolation_for_slug` cuando slug enlazado es `core`. |
| **L-SELF** | Esta feature B-07 **no** declara `tracker_ref` → cierre documental por vía **git** (`validacion.md` APTO + `pbi_archived: true` + PBI en `docs/todos/done/`) pese a manifiesto Core `linear`. |
| **L-GENOME** | Sin forja `entity-manager`. Touchpoints: `project_binding.rs`, `done_gate.rs`, manifiestos `.SddIA/`, prosa contrato, evolution, tests. |
| **L-EVOL** | Hito `SddIA/evolution/` + `Evolution_log.md` (`gate-evolution --range`) con UUID PBI `9b4e7c12-5a38-4f19-8d64-3e2f1a9c0b85` y cierre F8.3 paso 4. |

## 4. Artefactos Core-self (canónicos)

### 4.1 Índice — `.SddIA/projects/core.md`

```yaml
---
id: core
uuid: "7c4e2a91-8b3f-4d6e-a1c0-9e5f2b8d0473"
project_root: .
manifest_ref: .SddIA/project.md
codex_slug: codex-software-engineering
status: active
---
```

### 4.2 Manifiesto — `.SddIA/project.md`

```yaml
---
id: core
uuid: "7c4e2a91-8b3f-4d6e-a1c0-9e5f2b8d0473"
git_remote: https://github.com/racso80es/SddIA.git
default_branch: main
delivery_mode: branch_pr
contract_version: "1.3.0"
codex_slug: codex-software-engineering
tracker:
  provider: linear
  team_key: OSC
  done_gate: linear
  state_map:
    backlog: Backlog
    todo: Todo
    in_progress: In Progress
    in_review: In Review
    done: Done
    cancelled: Cancelled
  labels:
    hu: hu
    pbi: pbi
    fix: fix
    kaizen: kaizen
    deuda: deuda
    spike: spike
    editable: sddia-editable
docs_layout:
  features: docs/features
  fixes: docs/fixes
  todos_pending: docs/todos/pending
  todos_done: docs/todos/done
---
```

Prohibido regenerar UUID distinto al canónico si el índice/manifiesto ya existen con ese valor.

## 5. Cambios de motor

### 5.1 `project_binding::bind`

1. Si `id`/`slug` == `"core"` y raw ∈ {`.`, `./`} → `project_root = repo.to_path_buf()`.
2. Else si no absoluto → `Err("PROJECT_CONFIG_INVALID: project_root no absoluto")`.
3. Else absoluto → usar tal cual.
4. Isolation: L-ISOLATION al enlazar `core`.

### 5.2 `done_gate::resolve_done_gate`

```text
gate := done_gate_for_inputs(slug)
si gate ∈ {Linear, Both} ∧ tracker_ref_from_persist(persist).is_none():
    warn trazable
    return Git
return gate
```

### 5.3 Tests (mínimo)

| Test | Aserción |
|------|----------|
| `bind_core_dot_project_root` | `bind(..., core)` OK; `project_root == repo` |
| `bind_external_rejects_relative` | slug no-core + `project_root: .` → error absoluto |
| `resolve_core_linear_with_tracker_ref` | persist con `tracker_ref` → `Linear` |
| `resolve_core_degrades_without_tracker_ref` | sin `tracker_ref` → `Git` |
| `di_other_projects_unchanged` / `ac21d_*` | barcelonaxplorer ≠ linear; lab-both = `both` |

Verificación: `cargo test -p execute-process project_binding`, `done_gate`, `linear_done_gate_both_e2e`.

## 6. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-24a | Índice + manifiesto 1.3.0; `done_gate: linear`; `team_key: OSC`; `project_root: .`; portable; sin secretos. | `cargo test -p execute-process project_binding` |
| AC-24b | Con `tracker_ref`: gate `Linear`; archive `archive_deferred`; pre-push exige proof. | `cargo test -p execute-process done_gate` |
| AC-24c | Sin `tracker_ref`: gate `Git` (AC-10 / AC-17). | `cargo test -p execute-process done_gate` |
| AC-24d | Otros proyectos/fixtures intactos; `ac21d_no_manifest_sets_done_gate_linear` verde. | `cargo test -p execute-process linear_done_gate_both_e2e` |

## 7. Impacto en documentación (DIA)

| Artefacto | Acción |
|-----------|--------|
| `SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md` | Prosa L-CONTRACT (sin bump) |
| `.SddIA/projects/core.md` + `.SddIA/project.md` | Crear/mantener canónicos §4 |
| `SddIA/evolution/` + `Evolution_log.md` | Hito F8.3 paso 4 |
| README raíz / manuales usuario | Ninguno |

## 8. Fuera de alcance

- `done_gate: linear` en proyectos no-core / producción externa.
- Relajar absoluto para slugs externos.
- Webhook productivo; crear issues Linear; bump `contract_version`.
- Segundo PR solo documental; `tracker_ref` en esta feature.
- Mutar fixtures lab / `barcelonaxplorer`.

## 9. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Entrega Core **con** `tracker_ref` sin stamp | L-STRICT (bloqueo esperado) |
| `bind(core)` vs isolation labs | L-ISOLATION |
| Suite rota si manifiestos `linear` antes del degrade | Orden blueprint: T2 → T3 |
| `team_key` OSC incorrecto | Evidencia `OSC-*`; patch manifiesto sin reabrir motor |

## 10. Contexto WIP (informativo, no laudo)

Working tree ya contiene motor (L-ROOT/L-DEGRADE/L-ISOLATION), manifiestos Core-self y prosa contrato de ejecuciones Tekton previas (bloqueadas en T4/T5 por Shell). Contrastado en esta pasada Dedalo: índice/manifiesto §4, `bind` L-ROOT, `resolve_done_gate` L-DEGRADE y prosa L-CONTRACT presentes en disco. Residual operativo: evidenciar T4 (`cargo test`) y T5 (evolution + cierre documental). Dedalo no ejecuta ni inventa éxito de esos pasos.

---
feature_name: arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
created: "2026-10-14"
process: feature
branch_name: feat/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
persist_ref: docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
pbi_ref: "docs/todos/pending/[ARQUITECTURA] Linear HU-B 07 — Manifiesto Core-self y activación done_gate linear.md"
document_id: PBI-LINEAR-B-07-CORE-SELF-GATE
pbi_uuid: "9b4e7c12-5a38-4f19-8d64-3e2f1a9c0b85"
historia_document_id: HU-LINEAR-SSOT-INVERSE
correlation_id: "fd2db080-dbf3-44ab-b70f-63b90d748455"
project_slug: core
phase: estabilizacion-requisitos
agents: mayeuta
execution_id: "f351761b-9b35-4e44-bd81-ae99fd259679"
---
# Objetivos — HU-B 07 Manifiesto Core-self y `done_gate: linear`

## Objetivo

Registrar formalmente el proyecto auto-hospedado `core` (`.SddIA/projects/core.md` + `.SddIA/project.md`, `project-config-contract` 1.3.0) y activar `tracker.done_gate: linear` en Core-self (F8.3 paso 4 / D-I (a)), con `project_root: .` portable en CI y degradación a `git` cuando el `persist_ref` carece de `tracker_ref`.

## Ley aplicada

- `.cursorrules` §5 Agnosticismo del Core — sin rutas absolutas de máquina en manifiestos versionados.
- `project-config-contract` 1.3.0 — `done_gate ∈ {git, linear, both}`; excepción `project_root: .` solo `id: core`; sin secretos en manifiesto.
- `features-documentation-pattern` 1.3.0 / `task-closure-documental` — Done condicional al `done_gate` efectivo; esta feature sin `tracker_ref` → vía git.
- HU `HU-LINEAR-SSOT-INVERSE` F8.3 paso 4; laudo D-I (a).

## Alcance

| Incluye | Excluye |
|---------|---------|
| Índice + manifiesto Core-self con `done_gate: linear`, `team_key: OSC` | `done_gate: linear` en otros proyectos / producción externa |
| `project_binding::bind` admite `project_root: .` para `id: core` + aislamiento fixtures relativos | Relajar absoluto para slugs no-`core` |
| Degradación central en `resolve_done_gate` (`Linear`/`Both` → `Git` sin `tracker_ref`) | Eliminar gate de Done; crear issues desde Linear |
| Prosa contrato 1.3.0 sin SemVer bump | Bump `contract_version`; forja de genoma nueva |
| Tests `project_binding` / `done_gate` / e2e both intacto | Mutar `barcelonaxplorer` / fixtures lab |
| Evolution F8.3 paso 4 + UUID PBI | Segundo PR solo documental; `tracker_ref` en esta feature |

## Requisitos refinados

| ID | Requisito |
|----|-----------|
| R-1 | Crear/mantener `.SddIA/projects/core.md` (`id: core`, `project_root: .`, `manifest_ref: .SddIA/project.md`, `codex_slug: codex-software-engineering`, `status: active`) y `.SddIA/project.md` con el mismo `uuid` (`7c4e2a91-8b3f-4d6e-a1c0-9e5f2b8d0473`), `contract_version: "1.3.0"`, `default_branch: main`, `delivery_mode: branch_pr`, `docs_layout` canónico y `tracker`: `provider: linear`, `team_key: OSC`, `done_gate: linear`, `state_map` (6 claves canónicas), `labels` (`hu`, `pbi`, `fix`, `kaizen`, `deuda`, `spike`, `editable`). Cero secretos `LINEAR_*`. |
| R-2 | En `project_binding.rs`, `bind()` resuelve `project_root: .`/`./` → `repo.to_path_buf()` cuando `id`/`slug` es `core`; slugs externos siguen exigiendo ruta absoluta. Aislamiento: al enlazar `core`, raíces relativas de fixtures no disparan falso `PROJECT_GIT_INVALID`. |
| R-3 | En `done_gate.rs`: degradación en `resolve_done_gate` — si gate es `Linear` o `Both` y `tracker_ref_from_persist` es `None`, devolver `DoneGate::Git` con `warn` (AC-21). Con `tracker_ref` y gate efectivo `linear`: exigir estado `done` en Linear (`feature-pbi-archive`) y proof `.SddIA/proofs/tracker-done/{tracker_ref}.json` (`pre_push_check`). |
| R-4 | D-I (a): `barcelonaxplorer` no pasa a `linear`; `lab-done-gate-both` permanece `both`; fixtures lab intactos. |
| R-5 | Hito en `SddIA/evolution/` + `Evolution_log.md` (`gate-evolution --range`) vinculando UUID PBI `9b4e7c12-5a38-4f19-8d64-3e2f1a9c0b85` y cierre F8.3 paso 4. |
| R-6 | Cierre documental en la misma rama del PR: esta feature **sin** `tracker_ref` → vía git (`validacion.md` APTO + `pbi_archived: true` + PBI en `docs/todos/done/`). |
| R-7 | Prosa `project-config-contract.md` 1.3.0 documenta excepción `project_root: .` para Core-self (sin bump SemVer). |

## Criterios de aceptación (entrada Argos)

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-24a | `.SddIA/projects/core.md` + `.SddIA/project.md` validan contrato 1.3.0 con `done_gate: linear`, `team_key: OSC` y `project_root: .` en local y CI; sin secretos. | `cargo test -p execute-process project_binding` |
| AC-24b | Con `tracker_ref` en persist Core-self: `resolve_done_gate` → `Linear`; archive exige issue `done`; `pre_push_check` exige proof. | `cargo test -p execute-process done_gate` |
| AC-24c | Sin `tracker_ref`: gate opera en `git` (AC-10 / AC-17 verdes). | `cargo test -p execute-process done_gate` |
| AC-24d | `barcelonaxplorer` no en `linear`; `lab-done-gate-both` en `both`; `ac21d_no_manifest_sets_done_gate_linear` verde. | `cargo test -p execute-process linear_done_gate_both_e2e` |

## Plan orientativo (entrada Dedalo)

1. Ajustar `project_binding.rs` + `done_gate.rs` + prosa contrato (R-2, R-3, R-7).
2. Materializar manifiestos Core-self (R-1).
3. Tests unitarios/integración (AC-24a…d).
4. Evolution + cierre documental en rama vía git (R-5, R-6).

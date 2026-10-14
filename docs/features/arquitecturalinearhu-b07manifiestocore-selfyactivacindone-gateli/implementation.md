---
feature_name: arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
created: "2026-10-14"
process: feature
document_id: PBI-LINEAR-B-07-CORE-SELF-GATE
pbi_uuid: "9b4e7c12-5a38-4f19-8d64-3e2f1a9c0b85"
persist_ref: docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
correlation_id: "fd2db080-dbf3-44ab-b70f-63b90d748455"
branch_name: feat/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
phase: execution
agents: tekton
execution_id: "f351761b-9b35-4e44-bd81-ae99fd259679"
---
# Implementación — HU-B 07 Core-self `done_gate: linear`

## Artefactos en disco (verificados esta sesión; sin regenerar UUID)

| Artefacto | Acción |
|-----------|--------|
| `SddIA/engine/execute-process/src/engine/project_binding.rs` | L-ROOT: `bind` resuelve `project_root: .`/`./` → `repo` solo si `slug`/`id` == `core`; L-ISOLATION: omite raíces relativas al enlazar `core`. Tests `bind_core_dot_project_root`, `bind_external_rejects_relative`. |
| `SddIA/engine/execute-process/src/engine/done_gate.rs` | L-DEGRADE: `resolve_done_gate` degrada `Linear`/`Both` → `Git` si `tracker_ref` ausente + warn. Tests AC-24b/c + `di_other_projects_unchanged`. |
| `.SddIA/projects/core.md` | Índice Core-self (`uuid` `7c4e2a91-8b3f-4d6e-a1c0-9e5f2b8d0473`, `project_root: .`). Paridad spec §4. |
| `.SddIA/project.md` | Manifiesto 1.3.0, `team_key: OSC`, `done_gate: linear`, state_map 6 + labels; sin `LINEAR_*`. Paridad spec §4. |
| `SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md` | Prosa L-CONTRACT (sin bump SemVer). |
| `implementation.md` / `execution.md` | Este paquete (`execution_id` f351761b-…). |

## D-I (inspección estática)

- `docs/_fixtures/lab-done-gate-project-both/.SddIA/project.md` permanece `done_gate: both`.
- Índices `.SddIA/projects/{barcelonaxplorer,lab-done-gate,lab-done-gate-both}.md` no migrados a `linear`.
- Único manifiesto con `done_gate: linear` bajo `.SddIA/`: `project.md` (core).

## No materializado (bloqueo de entorno)

| Ítem | Motivo |
|------|--------|
| Suite `cargo test -p execute-process` (`project_binding` / `done_gate` / `linear_done_gate_both_e2e`) | Shell IDE rechazado (también vía subagente). Sin exit codes. |
| `SddIA/evolution/{uuid}.md` + `Evolution_log.md` / `gate-evolution --range` | Requiere `sddia-qa evolution-rehash`; Shell bloqueado. Prohibido inventar `hash_integrity`. |
| Checkout `feat/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli` vía `git-manager` | Shell bloqueado. HEAD leído: `docs/cerrar-pbi-kalma2-branch-before-mutate` (rama incorrecta vs spec). |
| Cierre documental Argos / PBI → `done/` | Fuera de fase Tekton pura; depende de T4 verde + rama correcta. |

## Forja genómica

Sin `entity-manager` (L-GENOME).

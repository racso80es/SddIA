---

feature_name: arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
created: "2026-10-14"
purpose: estabilizacion-requisitos-mayeuta
process: feature
branch_name: feat/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
persist_ref: docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
pbi_ref: "docs/todos/pending/[ARQUITECTURA] Linear HU-B 07 — Manifiesto Core-self y activación done_gate linear.md"
document_id: PBI-LINEAR-B-07-CORE-SELF-GATE
pbi_uuid: "9b4e7c12-5a38-4f19-8d64-3e2f1a9c0b85"
correlation_id: "fd2db080-dbf3-44ab-b70f-63b90d748455"
phase: estabilizacion-requisitos
agents: mayeuta
execution_id: "0061cd1b-fe5e-46d6-80f1-cd4db92a8559"
---
# Clarify — HU-B 07 Core-self `done_gate: linear`

PBI `PBI-LINEAR-B-07-CORE-SELF-GATE` (orden 07/07 de `HU-LINEAR-SSOT-INVERSE`). Cierra F8.3 paso 4 y D-I (a): activar `tracker.done_gate: linear` **solo** en Core-self.

## Realidad contrastada (filtro A)

| Asunción | Hecho en repo | Decisión |
|----------|---------------|---------|
| Basta poner `done_gate: linear` en un manifiesto Core | Sin registro → `PROJECT_NOT_REGISTERED: core` → fallback `Git`. En WIP: `.SddIA/projects/core.md` + `.SddIA/project.md` (uuid `7c4e2a91-8b3f-4d6e-a1c0-9e5f2b8d0473`, `done_gate: linear`, `team_key: OSC`, `project_root: .`). | Mantener índice + manifiesto Core-self (`contract_version: "1.3.0"`) como artefacto canónico. |
| `project_root` absoluto `/home/racso/...` | Viola §5 Agnosticismo; rompe CI. En WIP `bind()` ya admite `.`/`./` solo si `id`/`slug` == `core`. | Índice: `project_root: .`. No cablear rutas de máquina. |
| Suite Core verde al activar `linear` | Consumidores fallan sin `tracker_ref`. En WIP `resolve_done_gate` degrada `Linear`\|`Both` → `Git` si `tracker_ref_from_persist` es `None`. | Degradación **central** en `resolve_done_gate` + `warn` (AC-21). |
| Activar `linear` rompe `ac21d_…` | Assert solo inspecciona fixture `lab-done-gate-project-both`. | Mantener lab en `both`; no tocar `barcelonaxplorer` (`git`). |
| `bind(core)` vs fixtures relativos | Comparación path relativa↔absoluta puede falso `PROJECT_GIT_INVALID`. | L-ISOLATION: al enlazar `core`, no tratar raíces **relativas** de fixtures como anidación falsa. |

## Decisiones estabilizadas

| ID | Laudo Mayeuta |
|----|----------------|
| C-1 | Alcance = registro Core-self + `done_gate: linear` + ajustes mínimos en `project_binding::bind` y `done_gate` (+ prosa contrato sin SemVer bump). Sin forja `entity-manager`. |
| C-2 | `project_root: .` es el único valor admisible en el índice versionado de Core. Prohibido cablear rutas de máquina local. |
| C-3 | Degradación sin `tracker_ref` **en** `resolve_done_gate`: gate ∈ {`Linear`,`Both`} y `tracker_ref_from_persist` = `None` → `DoneGate::Git` + `warn`. Con `tracker_ref` y gate efectivo `linear`: issue `done` (`feature-pbi-archive`) + proof `.SddIA/proofs/tracker-done/{tracker_ref}.json` (`pre_push_check`). |
| C-4 | D-I (a): ningún otro `.SddIA/projects/*.md` de producto migra a `linear`. No mutar fixtures `docs/_fixtures/lab-done-gate-project*`. |
| C-5 | `tracker.team_key: OSC`. `project_id` omitido. Cero secretos `LINEAR_*`. `env_ref` solo si bóveda ya existe. |
| C-6 | Fuera de alcance: expansión `linear` a otros proyectos; webhook productivo; genoma nuevo; declarar APTO sin tests verdes `project_binding` / `done_gate` / `linear_done_gate_both_e2e`. |
| C-7 | Evolution: hito en `SddIA/evolution/` + `Evolution_log.md` (`gate-evolution --range`) vinculando UUID PBI `9b4e7c12-5a38-4f19-8d64-3e2f1a9c0b85` y cierre F8.3 paso 4. |
| C-8 | Esta feature B-07 **no** declara `tracker_ref` → cierre documental por vía **git** (`validacion.md` APTO + `pbi_archived` + PBI en `docs/todos/done/` en la misma rama), coherente con C-3 / AC-24c. |
| C-9 | Prosa `project-config-contract` 1.3.0: excepción `project_root: .` solo para `id: core` (sin bump SemVer). |

## Preguntas resueltas (sin bloqueo)

- ¿Laudo Vértice F8.3 paso 4? **Sí** — PBI `baseline_decisiones` D-I (a) + F8.3 tras B-06 PR #341.
- ¿`both` intermedio en Core-self producto? **No** — directo a `linear`.
- ¿`team_key`? **OSC** (C-5).
- ¿Mutar `barcelonaxplorer` o fixtures lab? **No**.
- ¿`tracker_ref` en esta entrega B-07? **No** → Done por vía git (C-8).
- ¿Reabrir requisitos por WIP parcial? **No** — T1–T3 en disco alineados a C-*; residual T4/T5 es de **ejecución**, no de estabilización.

## Riesgos residuales (informativos)

- Primera entrega Core-self **con** `tracker_ref` quedará sujeta a proof Linear en pre-push.
- `bind()` especial `core`+`.` acotado: no relajar absoluto para slugs externos.
- T4 (`cargo test`) y T5 (evolution/cierre) pendientes de evidencia; no inventar éxito.

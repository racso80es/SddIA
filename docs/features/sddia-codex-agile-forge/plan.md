---
feature_name: sddia-codex-agile-forge
created: "2026-09-25"
process: feature
phases: "T0-contract T1-binding T2-events T3-forge T4-hooks T5-docs"
branch_name: feat/sddia-codex-agile-forge
persist_ref: docs/features/sddia-codex-agile-forge
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-04
agents: dedalo
---

# Plan — sddia-codex-agile-forge

Blueprint de ejecución. Motor editable directo. Genoma de códice solo vía creator.

| ID | Fase | Touchpoints | Done |
|----|------|-------------|------|
| **T0** | Contrato | `contracts/project-config-contract.md` en el códice (alta gobernada). Cúmulo: `events_domain_roots`, `instance.projects`, `paths.resolution` | Contrato legible + Cúmulo parsea |
| **T1** | Binding | `engine/project_binding.rs` + tests: contrato, uuid cruzado, git raíz, anti-anidamiento, escape, precedencia `delivery_mode` triple, rutas bajo `project_root` | AC-A1-*, AC-A2-PRECEDENCE, AC-PATHS, AC-A2-DEFAULT |
| **T2** | Eventos | `resolve_event_contract`; `composed_subscriptions`; ECST `PBI_Forged` y `Delivery_Committed` en `events/` del códice; `subscriptions.json` del códice; dead-letter `no_subscriber` sin autoridad | AC-EVENTS-ROOT, AC-SUBS |
| **T3** | forge-pbi | `process-creator` jurisdiction domain, root software-engineering; membership del códice; handler de sellado; `workspace_init` respeta `trunk_direct` (no rama nueva) | AC-FORGE, AC-TIERS, AC-A2-TRUNK |
| **T4** | Hooks | `main_push_allowed` + rama en `pre_push_gate.sh`; cláusula `delivery_mode` en `pull-request-orchestration` y `pr-acceptance-protocol` (version bump vía norm path gobernado o, si el creator no admite párrafo, evolution + edición de norma con entity-manager) | AC-A2-HOOKS, AC-A2-NORM, AC-CORE-SELF |
| **T5** | Cierre | `implementation.md`, `execution.md`, evolution uuid PBI, `validacion.md` en `PENDIENTE-CI` hasta run verde, PBI a `done/` solo cuando `global: APTO` | AC-DOC, AC-EVO, AC-BUILD |

## Orden

1. Commit de esta planificación (T docs de fase, sin motor).
2. T0→T1 con `cargo test -p execute-process` del módulo nuevo en verde **antes** de T3.
3. T2 y T3. T4 al final del motor para no mover el veto de `main` del Core antes de tener el test doble.
4. `cargo test -p execute-process` y `cargo build -p execute-process --release`.
5. `implementation.md` + `execution.md`. PR vía `delivery-close-cycle`. `validacion.md` no pasa a `APTO` hasta check verde. Entonces `accept-pr` y PBI a `done/` en la misma rama antes del merge (cierre documental). Si el check sale rojo: un parche, un push, sin bucle de `gh pr checks`.

## Delegaciones

| Necesidad | Vía |
|-----------|-----|
| Motor Rust + tests | Tekton directo |
| Alta process `forge-pbi`, eventos, contrato, bump de normas | `./sddia-run.sh --process` creator / `entity-manager` |
| Cúmulo `cumulo.paths.json` | Edición directa documentada (precedente ABSTRACT-03) |
| Git | `skill:git-manager` |
| Commit de planificación | Este paso, antes de T0 |

## Gate

```text
T1 tests rojos → STOP (prohibido T3)
push a main del Core → sigue vetado
global APTO sin run_id verde → prohibido
```

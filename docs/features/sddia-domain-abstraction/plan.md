---
feature_name: sddia-domain-abstraction
created: "2026-08-05"
process: feature
branch_name: feat/sddia-domain-abstraction
persist_ref: docs/features/sddia-domain-abstraction
pbi_ref: docs/todos/done/[ARQUITECTURA] Separación de Dominio SddIA y Abstracción del Contexto de Ejecución.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-01
execution_id: 758d4440-2889-47a9-b412-ffab00ba0c1d
phases: "T0-audit T1-profile T2-wsinit T3-tests T4-docs T5-argos"
agents: dedalo
---

# Plan — sddia-domain-abstraction

## Fases

| ID | Fase | Intent | Touchpoints | Done cuando |
|----|------|--------|-------------|-------------|
| **T0** | Baseline | Confirmar I7 path DI→detector; listar tests DI/Cerbero reutilizables | `capability_di_resolver.rs`, `workspace_init.rs`, `cerbero_di_rbac` | Notas en `implementation.md` |
| **T1** | Perfil de dominio | `resolve_execution_profile(repo, inputs)` + schema documentado | nuevo `domain_profile.rs` (o equiv.), `mod.rs` | Unit: precedence input > instancia > default |
| **T2** | Gate Git + detector | Endurecer `is_workspace_init_phase`; skip git por perfil | `workspace_init.rs`, cableado desde `executor` si hace falta pasar inputs ya resueltos | `git_required:false` sin `SDDIA_LAB_SKIP_GIT` |
| **T3** | Tests / smoke | AC-WSINIT, AC-BOOT, AC-DENY | `#[cfg(test)]` en motor; opcional `_smoke-*.json` en persist_ref | `cargo test -p execute-process` verde en suites tocadas |
| **T4** | Docs runtime | Fixture ejemplo perfil + notas smoke | `persist_ref`, evolution breve | Paths Cúmulo correctos |
| **T5** | Argos + cierre | `validacion.md` APTO; PBI → `done/`; `delivery-close-cycle` | cascada documental | Un PR |

## Orden de ejecución Tekton

1. T1 lector perfil (sin side-effects).
2. T2 integrar en `workspace_init::run` + detector.
3. T3 tests.
4. T0/T4 documentación de evidencia.
5. Compilar release / test package.
6. Argos (T5) — no abrir PR hasta AC verdes.

## Delegaciones (blueprint)

| Necesidad | Cápsula / vía |
|-----------|----------------|
| Lectura/escritura docs feature | IDE relay / `skill:filesystem-manager` si runtime |
| Mutación motor Rust | Tekton directo en `SddIA/engine/execute-process` (fuera de genoma indexado DA-2) |
| Genoma process/norms/codexes | **Prohibido** en este PR salvo laudo nuevo |
| Git ops entrega | `skill:git-manager` vía `delivery-close-cycle` |

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Default `git_required:true` rompe no-software | Documentar; smoke con perfil explícito false |
| Skip DI rompe detector | Detector también mira `requires_capability` |
| `.SddIA/` gitignore | No versionar perfil real; versionar ejemplo en `persist_ref` |

## Fuera del plan

ABSTRACT-02 · GesFer · nuevos ECST · vaciado `SddIA/process/`

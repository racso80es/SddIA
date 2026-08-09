---
feature_name: sddia-domain-abstraction
created: "2026-08-05"
process: feature
branch_name: feat/sddia-domain-abstraction
persist_ref: docs/features/sddia-domain-abstraction
pbi_ref: docs/todos/done/[ARQUITECTURA] Separación de Dominio SddIA y Abstracción del Contexto de Ejecución.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-01
execution_id: 758d4440-2889-47a9-b412-ffab00ba0c1d
phase: tekton-execution
agents: mayeuta,dedalo,tekton
---

# Objetivos — sddia-domain-abstraction

## Misión

Estabilizar la **Separación de Dominio Estricta** del Core SddIA: el motor (`SddIA/core/`, `SddIA/engine/`) arranca y enruta entropía EDA **sin** presuponer repositorio Git, Pull Request ni códice de ingeniería de software. Los propósitos operativos viven en **Códices de Dominio** inyectables (`directories.library_codexes`).

Este ciclo entrega el **MVP de abstracción de contexto de ejecución** (L-SPLIT-A). La poda ontológica que vacía `SddIA/process/` de flujos software-only queda fuera (PBI hijo ABSTRACT-02).

## Alcance

| Dentro | Fuera |
|--------|-------|
| Gate de Git en `workspace_init` / Inicialización por perfil o códice activo | Migrar `feature`/`bug-fix`/`refactorization` fuera de Core process |
| Contrato de activación/enrutado de Códices (sobre `codex-contract` existente) | Forjar catálogo completo de códices PA/domótica |
| Smoke: evento dominio no-PR sin `.git` + denegación Cerbero sin autoridad | Inventar ECST `Email_Received`/`Prompt_Submitted` sin necesidad demostrada |
| Corrección de paths a SSOT Cúmulo en docs/AC | Reescritura amplia de watchers/daemons no tocados por el gate |
| Cascada documental + cierre single-PR | GesFer / Paciente 0 (depende de este cierre) |

## Hitos (MVP)

1. **H1 — Arranque sin Git obligatorio:** Inicialización ejecutable con perfil no-software; Git solo si el códice/perfil lo exige.
2. **H2 — Activación de Códice:** Runtime puede resolver si hay autoridad de dominio (códice activo) antes de ejecutar cargas acopladas.
3. **H3 — Prueba EDA no-software:** Un estímulo de dominio existente (p. ej. familia mensajería/tarea manual) se procesa o deniega limpiamente sin metadatos de repo.

## Criterios de aceptación

- **AC-BOOT:** Core + evento dominio no-PR sin exigir `.git`.
- **AC-WSINIT:** Git opcional por códice/perfil (no solo `SDDIA_LAB_SKIP_GIT`).
- **AC-CODEX:** Activación/enrutado de códice consultable vía Cúmulo + `codex-contract`.
- **AC-DENY:** Sin autoridad → denegación Cerbero sin panic.
- **AC-BUILD:** `cargo build --release` OK.
- **AC-DOC:** PBI en `done/` + `validacion.md` `pbi_archived: true` en la rama del PR.

## Ley aplicada

- Rutas solo vía `SddIA/core/cumulo.paths.json`.
- Git vía `skill:git-manager` / `proc:git-sync` cuando el perfil lo active.
- Genoma vía `entity-manager` / `./sddia-run.sh`.
- `features-documentation-pattern` v1.2.x; cierre documental en rama (un PR).
- Jerarquía: Acción → Agente → Skill → Tools.
- Filtros C/A/B: no expandir a migración process completa en este PR (Filtro C).

## Handoff Dedalo

Consumir este cuerpo como `refined_requirements`. Diseñar `spec.md` + `plan.md` estrictamente bajo L-SPLIT-A; marcar I7 (`is_workspace_init_phase` vs `proc:git-sync`) como riesgo de diseño a cerrar con evidencia de código.

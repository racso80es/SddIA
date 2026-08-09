---
feature_name: sddia-domain-abstract-03-relocalizacion
created: "2026-08-09"
process: refactorization
branch_name: feat/sddia-domain-abstract-03-relocalizacion
persist_ref: docs/features/sddia-domain-abstract-03-relocalizacion
pbi_ref: docs/todos/done/[REFACTOR] PBI-SDDIA-DOMAIN-ABSTRACT-03 — Relocalización física process software.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
phase: tekton-execution-blocked
agents: tekton
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
correlation_id: ""
---

# Objetivos — sddia-domain-abstract-03-relocalizacion

## Misión

Relocalizar **físicamente** los process de ciclo de vida software (`feature`, `bug-fix`, `refactorization`) y, salvo laudo Dedalo en contrario, el ciclo PR (`pull-request-review`, `accept-pr`, `delivery-close-cycle`) fuera de `directories.process` del Core hacia jurisdicción de códice/instancia, con **path de resolución del orquestador demostrado** (overlay Cúmulo / instancia) **sin** romper TQM / Kalma2 / `sddia-run`.

Hereda ABSTRACT-02: autoridad de códice ya existe; este ciclo cierra el **AC-MOVE** diferido.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Resolución process no hardcode-only a `SddIA/process` (Cúmulo ± overlay instancia) | Reabrir diseño de gate ABSTRACT-02 salvo ajuste mínimo post-move |
| Move físico de L-SCOPE-LIFECYCLE (+ PR cycle default) | Migrar `*-creator`, `entity-manager`, routes EDA, daemons no-software |
| Índice / referencias / smokes post-move | `codex-personal-assistant` / GesFer |
| Cascada documental + cierre single-PR | Inventar destino fuera de SSOT Cúmulo |

## Hitos

1. **H1 — Resolución:** diseño + implementación de resolve overlay; evidencia AC-RESOLVE.
2. **H2 — Move:** relocalización física + índice (AC-MOVE / AC-INDEX).
3. **H3 — Compat:** smokes AC-RUN + AC-TQM; build release.

## Criterios de aceptación

- **AC-RESOLVE / AC-MOVE / AC-INDEX / AC-RUN / AC-TQM / AC-BUILD / AC-DOC** (ver `clarify.md` D4).

## Ley aplicada

- Rutas solo vía `SddIA/core/cumulo.paths.json` (y fusión local documentada).
- Git vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`.
- Genoma process vía forja gobernada; motor resolver editable directo.
- `features-documentation-pattern` v1.2.1; cierre documental en rama (un PR).
- Filtro C: **L-RESOLVE-FIRST** — no move ciego.
- Jerarquía: Acción → Agente → Skill → Tools.

## Handoff Dedalo

Consumir este cuerpo como `refined_requirements`. Diseñar `spec.md` + `plan.md` con orden L-RESOLVE-FIRST; dictaminar destino físico y alcance final del ciclo PR.

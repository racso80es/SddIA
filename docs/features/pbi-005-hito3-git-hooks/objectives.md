---
feature_name: pbi-005-hito3-git-hooks
created: "2026-05-20"
process: feature
branch_name: feat/pbi-005-hito3-git-hooks
persist_ref: docs/features/pbi-005-hito3-git-hooks
pbi_ref: PBI-005
hito: 3
fase: 0
related_backlog:
  - docs/todos/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md
  - docs/todos/done/[OPERATIVO] Planificación de Backlog_ Resolución de Pasivos y Automatización Core (Ola A).md
related_todo:
  - docs/todos/TODO-BLINDAJE-IA-OBRERA.md
---

# Objetivos — PBI-005 Hito 3: Git Hooks / Aduana de Fricción

## Misión

Materializar la infraestructura de control **pre-commit** para blindar el repositorio contra la entropía de IAs externas (Cursor/Jules), como subconjunto del **CA-3** del PBI-005. Los hooks de ciclo PR (`pre-push` / `post-merge`) quedan en fases posteriores según el backlog consolidado (H3.1–H3.3).

## Cláusula de restricción (Protocolo de blindaje)

> **Toda mutación en el genoma debe pasar por el triaje de Argos (hooks de Git) antes de ser persistida.**

## Objetivos medibles

| ID | Objetivo | Criterio de hecho | Fase |
|----|----------|-------------------|------|
| **A** | Script `SddIA/scripts/qa/git-hooks/pre-commit` que invoque `verify-process-integrity.py` | El commit aborta con exit ≠ 0 si falla la integridad de procesos; instalación documentada en `implementation.md` | 1 (post-laudo) |
| **B** | Blindaje en la aduana de commit: mutaciones en genoma exigen evento correlacionado en bus EDA (`eda_bus`; véase `clarify.md` D5) | `audit-entity-eda-coverage.py --scan`; hard-fail si `orphan_count > 0`; commits vía `skill:git-manager` en procesos oficiales | 1 (post-laudo) |
| **C** | Registro en backlog consolidado | Enlace a esta feature; Hito 3 **en progreso** — no cerrar H3.1–H3.5 ni CA-3 hasta `validacion.md` | 0 (documental) |
| H3.1 | Contrato de hooks `pre-push` / `post-merge` | Norma táctica o `SddIA/evolution/`, alineada a `pull-request-orchestration.md` | 2+ |
| H3.2 | `pre-push` → `delivery-close-cycle` / `PullRequest_Presented` | JSON en `eda_bus.pending` sin `execute-process` manual | 2+ |
| H3.3 | `post-merge` en `main` → `accept-pr` / `emit-pr-merged-event` | `PullRequest_Merged` sin `--action` suelto; sin `gh pr merge` en hooks | 2+ |
| H3.5 | Smoke reproducible | `validacion.md` con `event_ids` | cierre |

## Alcance Fase 0 (esta entrega)

- Directorio de feature y presente `objectives.md`.
- Sin instalación en `.git/hooks/` ni cableado físico hasta laudo del operador.

## No objetivos (Fase 0)

- Cerrar PBI-005 al 100 % ni marcar **CA-3** como cumplido.
- Retirada de shims CLI Ola C.
- Hooks `pre-push` / `post-merge` (H3.2–H3.3).

## Ley aplicada

- PBI-005 matriz «Automatización Git» y **CA-3** — `docs/todos/done/[OPERATIVO] Planificación de Backlog… (Ola A).md` v1.5.1.
- Backlog post-PR11 § Prioridad 1 (H3.1–H3.5).
- `docs/todos/TODO-BLINDAJE-IA-OBRERA.md` Fase C (aduana física Argos).
- Bus SSOT: `SddIA/core/cumulo.paths.json` → `eda_bus.pending`.
- Proceso `feature` v1.2.0; precedencia `pr-presented-orchestration` para presentación/fusión PR.

## Inicio formal (laboratorio)

| Campo | Valor |
|-------|--------|
| Proceso | `feature` v1.2.0 — fase 1 `workspace-init` **executed** |
| Rama | `feat/pbi-005-hito3-git-hooks` (creada desde `main`) |
| Inputs | `tmp/init-pbi-005-hito3-git-hooks.json` |
| Fases 2–6 | `simulated` (agentes IDE; sin `delivery-close-cycle` en este arranque) |

## Estado

| Fase feature | Estado |
|--------------|--------|
| Objetivos (Fase 0) | ✅ |
| Inicialización Git | ✅ `execute-process.py --process feature` |
| Clarificación | ✅ `clarify.md` (D5 Existencia en Bus) |
| Especificación | ✅ `spec.md` v1.1.0 (ADN D1–D12) |
| Planificación | ✅ `plan.md` |
| Ola A — `git-hooks/pre-commit` | ✅ APTO (`validacion.md`) |
| Instalación `.git/hooks` | ✅ (`install-hooks.ps1`; operador en clones) |
| Cierre PR #12 + DLT | ✅ `main` @ `12119f7` |
| Ola B (H3.1–H3.3) | ✅ `pbi-005-hito3-ola-b` PR #13 |

---
feature_name: pbi-005-hito3-ola-b
created: "2026-05-20"
process: feature
branch_name: feat/pbi-005-hito3-ola-b
persist_ref: docs/features/pbi-005-hito3-ola-b
pbi_ref: PBI-005
hito: 3
ola: B
parent_feature: docs/features/pbi-005-hito3-git-hooks
related_backlog:
  - docs/todos/[OPERATIVO] Planificación de Backlog_ Resolución de Pasivos y Automatización Core (Ola A).md
  - docs/todos/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md
---

# Objetivos — PBI-005 Hito 3 Ola B: Hooks ciclo PR (cierre CA-3)

## Misión

Completar el **CA-3** del PBI-005 y desbloquear el cierre al **100 %** del ítem operativo, materializando la **Ola B** del Hito 3: hooks Git `pre-push` y `post-merge` que depositen eventos en el bus EDA **sin invocaciones CLI manuales** ni runbooks ad hoc, delegando exclusivamente en los procesos canónicos `delivery-close-cycle` y `accept-pr`.

La **Ola A** (`pre-commit` Argos) está cerrada en `main` (PR #12, feature hermana `pbi-005-hito3-git-hooks`). Esta feature continúa el linaje sin reabrir debates ya resueltos en `clarify.md` de la hermana (D1–D12).

## Mapa de pendientes PBI-005 (solo lo que bloquea cierre)

| ID | Pendiente PBI-005 | Feature / evidencia | Estado |
|----|-------------------|---------------------|--------|
| CA-3.1 | Aduana `pre-commit` | `pbi-005-hito3-git-hooks` PR #12 | ✅ |
| CA-3.2 | Git rutinario deposita eventos sin CLI manual | **Esta feature** (H3.2–H3.3) | ⏳ |
| CA-3.3 | Payloads con hashes reales | Heredado PR #11–#12 | ✅ |
| CA-3.4 | Ruta `eda_bus.pending` SSOT | `cumulo.paths.json` | ✅ |
| DoD | «Ausencia de alucinación causal» en hooks PR | H3.1–H3.4 + `validacion.md` | ⏳ |
| D.3 | Reexportar PDF operativo desde `.md` | Higiene P6 — **fuera de alcance** | ⏳ |

> **Fuera de alcance PBI-005:** deuda Ola C shims CLI (OC.1–OC.5), handlers laboratorio `accept-pr` (L.1), coreografía Ola C V3 — backlog consolidado § P2–P5, no condicionan DoD del PBI.

## Objetivos medibles

| ID | Objetivo | Criterio de hecho | Fase |
|----|----------|-------------------|------|
| **H3.1** | Contrato normativo de hooks `pre-push` / `post-merge` | Documento en `SddIA/evolution/` o norma táctica alineada a `pull-request-orchestration.md` y `pr-acceptance-protocol.md` | 1 (spec) |
| **H3.2** | Hook `pre-push` → `delivery-close-cycle` | Tras `git push`, JSON `PullRequest_Presented` en `eda_bus.pending` sin `execute-process` manual ni `emit-pr-presented-event` suelto | 2 (impl) |
| **H3.3** | Hook `post-merge` en `main` → `accept-pr` | Tras merge local a `main`, `PullRequest_Merged` sin `--action` suelto ni `gh pr merge` | 2 (impl) |
| **H3.4** | Prohibición `gh pr merge` en hooks | Revisión Argos; cumplimiento `pull-request-orchestration.md` § 4 | 2 (impl) |
| **H3.5** | Smoke reproducible | `validacion.md` con `event_ids` Presented + Merged vía hooks | 3 (cierre) |
| **C** | Cierre documental PBI-005 | Actualizar manifiesto operativo v1.5.0: CA-3 ✅, DoD ✅, estado «completado» | 3 (cierre) |

## Alcance documental (fases Mayeuta + Dedalo)

- `objectives.md`, `clarify.md` (Mayeuta) y `spec.md` v1.0.0 con Resoluciones de Acero O1–O5.
- Implementación física de hooks, evolution H3.1 e instalador O5 — fase Tekton (`plan.md` + impl).

## No objetivos

- Retocar `pre-commit` / `pre_commit_gate.py` salvo extensión del instalador.
- Retirada de shims CLI Ola C (OC.x).
- Handler laboratorio completo de `accept-pr` (L.1).
- `external-ai-constraints.md` (TODO-BLINDAJE Fase A).
- Reexportación PDF del PBI operativo.

## Ley aplicada

- PBI-005 § CA-3 y DoD — `docs/todos/[OPERATIVO] Planificación de Backlog… (Ola A).md` v1.4.0.
- Backlog post-PR11 § Prioridad 1 Ola B (H3.1–H3.4).
- Precedencia fractal: `docs/features/pr-presented-orchestration/` (presentación) y `SddIA/process/accept-pr.md` (fusión).
- Ola A cerrada: `docs/features/pbi-005-hito3-git-hooks/` (`spec.md` § 7 como borrador heredado).
- Proceso `feature` v1.2.0; bus SSOT: `SddIA/core/cumulo.paths.json` → `eda_bus.pending`.

## Inicio formal (laboratorio)

| Campo | Valor |
|-------|--------|
| Proceso | `feature` v1.2.0 — fase 1 `workspace-init` **executed** |
| Rama | `feat/pbi-005-hito3-ola-b` (desde `main`) |
| Inputs | `tmp/init-pbi-005-hito3-ola-b.json` |
| Fases 2–6 | Fase 2 (Mayeuta) vía IDE; fases 3–6 pendientes |

## Estado

| Fase feature | Estado |
|--------------|--------|
| Objetivos (Fase 0) | ✅ |
| Inicialización Git | ✅ `execute-process.py --process feature` |
| Clarificación (Mayeuta) | ✅ `clarify.md` + O1–O5 cerradas |
| Especificación (Dedalo) | ✅ `spec.md` v1.0.0 |
| Planificación | ✅ `plan.md` |
| Implementación Ola B | ✅ hooks + cápsula accept-pr |
| Validación Argos | ✅ `validacion.md` APTO (lab) |
| Cierre PBI-005 100 % | ⏳ merge PR en `main` |

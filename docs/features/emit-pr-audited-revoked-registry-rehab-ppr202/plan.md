---
feature_name: emit-pr-audited-revoked-registry-rehab-ppr202
created: "2026-08-27"
process: refactorization
phase: planning
agents: dedalo
phases:
  - T1-instance-rehab
  - T2-evolution
  - T3-argos
  - T4-doc-archive
  - T5-delivery-close
branch_name: refactor/emit-pr-audited-revoked-registry-rehab-ppr202
persist_ref: docs/features/emit-pr-audited-revoked-registry-rehab-ppr202
pbi_ref: docs/todos/pending/[ARQUITECTURA] emit-pr-audited-event — rehabilitación revoked_entities (PPR #202).md
document_id: PBI-PPR-202-EMIT-PR-AUDITED-REVOKED-REGISTRY
uuid: c2e8f4a1-7b3d-4e9c-a5f6-8d1e2f3a4b5c
olas:
  - A1
---

# Plan — emit-pr-audited-revoked-registry-rehab-ppr202

Blueprint ejecutable para Tekton. Contratos: `spec.md` laudos L-* + AC-*.

## T1 — A1 instancia Yunque Rúnico (AC-A1 / AC-ONTO / AC-GIT-CLEAN)

Locus Cúmulo: `radamanto.revoked_entities` = `.SddIA/cerbero/revoked_entities.json`; `radamanto.stats` = `.SddIA/radamanto/stats.json`. **Fuera del diff git.**

1. Eliminar `revoked.emit-pr-audited-event`. Assert `permanent.emit-pr-audited-event` ausente.
2. Crear bucket raíz `emit-pr-audited-event` (**L-RESET-ABS** + **L-ONTOLOGY** `entity_type: tool`).
3. Assert laterales intactos (`refactorization`).
4. Volcar evidencia en `execution.md`.

## T2 — Documental Tekton + evolution

1. `implementation.md` + `execution.md`.
2. Entrada `directories.evolution` UUID `c2e8f4a1-7b3d-4e9c-a5f6-8d1e2f3a4b5c`.

## T3 — Argos

`validacion.md` APTO · `pbi_archived: true` · `branch` coherente.

## T4 — Archive PBI

Mover PBI canónico a `docs/todos/done/`.

## T5 — delivery-close-cycle

Apertura PR vía DCC (fase posterior).

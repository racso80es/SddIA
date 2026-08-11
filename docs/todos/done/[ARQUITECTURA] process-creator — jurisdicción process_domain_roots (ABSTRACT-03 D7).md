---
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03-D7-PROCESS-CREATOR
title: "[ARQUITECTURA] process-creator — jurisdicción process_domain_roots (ABSTRACT-03 D7)"
format: markdown
version: "1.0.0"
created: "2026-08-09"
updated: "2026-08-10"
status: cerrado
priority: media
process: feature
uuid: a3c7e91f-2b4d-4f8a-9c1e-7d6b0a5f3211
source_feature: docs/features/sddia-domain-abstract-03-relocalizacion
feature_ref: docs/features/process-creator-process-domain-roots
branch_name: feat/process-creator-process-domain-roots
source_correlation_id: 3211daac-00d2-4833-b37e-979d899e3468
source_audit: docs/features/sddia-domain-abstract-03-relocalizacion/validacion.md
pr_url: https://github.com/racso80es/SddIA/pull/165
related:
  - SddIA/process/process-creator.md
  - SddIA/core/cumulo.paths.json
  - SddIA/library/codexes/codex-software-engineering/process/
  - docs/todos/done/[REFACTOR] PBI-SDDIA-DOMAIN-ABSTRACT-03 — Relocalización física process software.md
incident_ref: "PPR #163 — D7 process-creator escribe solo bajo directories.process tras packing códice"
---

# [ARQUITECTURA] process-creator — jurisdicción process_domain_roots (ABSTRACT-03 D7)

## Mandato

Liquidar deuda **D7** diferida en ABSTRACT-03: tras el move de los 6 process software-lifecycle al packing códice, `process-creator` sigue persistiendo altas bajo `directories.process` (Core) e indexar solo ese root.

| ID | Origen | Evidencia empírica |
|----|--------|--------------------|
| **D7** | `spec.md` Dedalo · laudo `L-PACK-MULTIROOT-SIX-MOVE` | `process-creator.md` fases escriben `{directories.process}/{name}.md` + índice Core |
| **R1** | Cúmulo `process_domain_roots` 1.6.0 | Alta post-move de miembro software → path Core (entropía vs packing) |

## Criterio de cierre

- [x] `process-creator` (o forja sustituta) resuelve destino vía `process_domain_roots` + política de jurisdicción (software-lifecycle → códice; resto → Core).
- [x] Índice del root destino actualizado; sin fila fantasma en Core para altas de dominio.
- [x] AC / smoke: alta de process software no deja artefacto ejecutable bajo `SddIA/process/`.
- [x] Documentar overlay instancia si aplica (`.SddIA/local.paths.json`).

## Fuera de alcance

- Re-mover los 6 ya packing (ABSTRACT-03 cerrado en rama).
- Migrar `entity-manager`, daemons, routes EDA (L-KEEP-CORE).
- Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136 done).

## Dedup

| Finding | Tratamiento |
|---------|-------------|
| `GIT_EVIDENCE_SESSION_SHELL` | **Dedup done** — `docs/todos/done/[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136).md` |
| `MERGE_ALREADY_OBSERVED` | **Sin seed** — peaje handoff `accept-pr` |
| `emit-pr-audited-event` revoked | **Sin seed** — diseño aduana; `audit_event_reference` = CID Presented |

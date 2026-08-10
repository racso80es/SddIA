---
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
uuid: 7ade2a5f-be13-41ef-8b11-deb96fd58be3
title: "[REFACTOR] Relocalización física process software fuera del Core"
format: markdown
version: "0.2.0"
created: "2026-08-09"
status: done
priority: Media (post ABSTRACT-02)
process: refactorization
depends_on:
  - PBI-SDDIA-DOMAIN-ABSTRACT-02
parent_pbi: PBI-SDDIA-DOMAIN-ABSTRACT-02
feature_name: sddia-domain-abstract-03-relocalizacion
branch_name: feat/sddia-domain-abstract-03-relocalizacion
persist_ref: docs/features/sddia-domain-abstract-03-relocalizacion
refined: "2026-08-09"
archived: "2026-08-09"
---

# [REFACTOR] Relocalización física process software (cerrado en rama)

**Prerrequisito ABSTRACT-02:** satisfecho (Done + validacion APTO). Promovido kitchen→pending.

## Objetivo

Mover `feature` / `bug-fix` / `refactorization` (+ ciclo PR si aplica) fuera de `directories.process` Core hacia jurisdicción de códice/instancia, con path de resolución del orquestador demostrado (overlay Cúmulo / instancia).

## Origen

`docs/features/sddia-codex-software-engineering/spec.md` § AC-MOVE diferido.

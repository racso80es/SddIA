---
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
uuid: pending-on-forge
title: "[REFACTOR] Relocalización física process software fuera del Core"
format: markdown
version: "0.1.0"
created: "2026-08-09"
status: pendiente-kitchen
priority: Media (post ABSTRACT-02)
process: refactorization
depends_on:
  - PBI-SDDIA-DOMAIN-ABSTRACT-02
parent_pbi: PBI-SDDIA-DOMAIN-ABSTRACT-02
---

# [REFACTOR] Relocalización física process software (semilla kitchen)

**No iniciar** hasta cierre APTO de ABSTRACT-02.

## Objetivo

Mover `feature` / `bug-fix` / `refactorization` (+ ciclo PR si aplica) fuera de `directories.process` Core hacia jurisdicción de códice/instancia, con path de resolución del orquestador demostrado (overlay Cúmulo / instancia).

## Origen

`docs/features/sddia-codex-software-engineering/spec.md` § AC-MOVE diferido.

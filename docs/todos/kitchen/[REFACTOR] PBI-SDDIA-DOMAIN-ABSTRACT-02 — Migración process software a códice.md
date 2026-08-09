---
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
uuid: pending-on-forge
title: "[REFACTOR] Migración de process software-engineering a Códice de Dominio"
format: markdown
version: "0.1.0"
created: "2026-08-05"
status: pendiente-kitchen
priority: Alta (post ABSTRACT-01)
process: refactorization
depends_on:
  - PBI-SDDIA-DOMAIN-ABSTRACT-01
parent_pbi: PBI-SDDIA-DOMAIN-ABSTRACT-01
---

# [REFACTOR] Migración process software → códice (semilla kitchen)

**Estado:** semilla post-split Mayeuta de ABSTRACT-01. **No iniciar** hasta cierre APTO de `PBI-SDDIA-DOMAIN-ABSTRACT-01`.

## Objetivo

Extraer de `SddIA/process/` los flujos de ingeniería de software (`feature`, `bug-fix`, `refactorization`, y el ciclo PR asociado según laudo Dedalo) hacia un Códice de Dominio `codex-software-engineering` inyectable, dejando el Core process agnóstico o con orquestadores universales.

## Notas

- UUID definitivo: forjar vía `entity-manager` / ciclo `refactorization` al promover a `pending/`.
- AC legado «process/ solo agnóstico» de ABSTRACT v1/v2 vive **aquí**, no en ABSTRACT-01.
- Transcript origen: `docs/features/sddia-domain-abstraction/clarify.md` § L-SPLIT-B.

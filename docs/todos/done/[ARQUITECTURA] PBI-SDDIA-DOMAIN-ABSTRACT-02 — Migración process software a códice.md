---
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
uuid: 1b45bff6-da8a-4e31-879e-3068ed80b213
title: "[ARQUITECTURA] Migración de process software-engineering a Códice de Dominio"
format: markdown
version: "1.0.0"
created: "2026-08-05"
refined: "2026-08-09"
archived: "2026-08-09"
status: done
priority: Alta
process: feature
feature_name: sddia-codex-software-engineering
branch_name: feat/sddia-codex-software-engineering
persist_ref: docs/features/sddia-codex-software-engineering
parent_pbi: PBI-SDDIA-DOMAIN-ABSTRACT-01
depends_on:
  - PBI-SDDIA-DOMAIN-ABSTRACT-01
---

# [ARQUITECTURA] Migración process software → Códice de Dominio

**Versión:** 1.0.0 (Mayeuta 2026-08-09)  
**Ciclo:** `feature` · `feat/sddia-codex-software-engineering` · `docs/features/sddia-codex-software-engineering/`  
**Padre:** ABSTRACT-01 Done (PR #161)

## 1. Objetivo

Formalizar el dominio de ingeniería de software como **Códice inyectable** (`codex-software-engineering`) y exigir su autoridad en runtime para los process de ciclo de vida software, dejando el Core sin supuesto implícito de «siempre desarrollamos código».

## 2. Alcance MVP (este ciclo)

1. Forjar e indexar `codex-software-engineering` (`codex-contract`).
2. Declarar membresía: `feature`, `bug-fix`, `refactorization` + ciclo PR (`pull-request-review`, `accept-pr`, `delivery-close-cycle`) salvo laudo Dedalo en contrario.
3. Gate runtime sobre perfil/códice activo (ABSTRACT-01): sin autoridad → deny sin panic; con autoridad → allow.
4. Dictamen relocalización física de `.md` process (AC-MOVE) o defer ABSTRACT-03.

## 3. Fuera de alcance

- Creators / `entity-manager` / routes EDA / audits de gobernanza Core
- `codex-personal-assistant`
- GesFer

## 4. Criterios de aceptación

- [ ] **AC-CODEX:** códice indexado conforme contrato
- [ ] **AC-MEMBER:** inventario process explícito
- [ ] **AC-GATE:** deny sin códice/perfil software
- [ ] **AC-ALLOW:** allow con perfil software
- [ ] **AC-BUILD:** `cargo build -p execute-process --release` OK
- [ ] **AC-DOC:** PBI en `done/` + `validacion.md` `pbi_archived: true`
- [ ] **AC-MOVE:** move físico APTO **o** defer ABSTRACT-03 documentado

## 5. Referencias

- Clarify/objectives: `docs/features/sddia-codex-software-engineering/`
- Split origen: `docs/features/sddia-domain-abstraction/clarify.md` § L-SPLIT-B

---
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-01
uuid: 7d81a9f2-3c4e-4b1a-8f0e-2d9c1b4e6a3f
title: "[ARQUITECTURA] Separación de Dominio SddIA y Abstracción del Contexto de Ejecución"
format: markdown
version: "3.0.0"
created: "2026-07-20"
refined: "2026-08-05"
archived: "2026-08-09"
status: done
priority: Alta (Prerrequisito Bloqueante)
process: feature
feature_name: sddia-domain-abstraction
branch_name: feat/sddia-domain-abstraction
persist_ref: docs/features/sddia-domain-abstraction
depends_on:
  - docs/features/kalma2-full-cycle
  - docs/features/vanguardia-soberania-local
child_pbis:
  - PBI-SDDIA-DOMAIN-ABSTRACT-02
---

# [ARQUITECTURA] Separación de Dominio SddIA y Abstracción del Contexto de Ejecución

**Versión:** 3.0.0 (Mayeuta 2026-08-05 — colapso v1+v2; split MVP vs refactor hijo)  
**Ciclo activo:** `feature` · `feat/sddia-domain-abstraction` · `docs/features/sddia-domain-abstraction/`  
**Transcript:** `clarify.md` · **Requisito estable:** `objectives.md`

## 1. Clarificación estratégica

SddIA se formaliza como Sistema Nervioso Central Reactivo (EDA) de propósito general. El Motor Core (`SddIA/core/`, `SddIA/engine/`) no debe presuponer que el estímulo proviene de un Pull Request ni que exista un repositorio Git. Arranca, enruta vía Cúmulo/Cerbero e invoca ejecución S+ Grade con independencia del dominio (software, mensajería, asistencia, etc.).

Los propósitos operativos viven en **Códices de Dominio** inyectables (`directories.library_codexes` → `SddIA/library/codexes/`), bajo `codex-contract`.

## 2. Partición de alcance (obligatoria)

| ID | Ciclo | Contenido |
|----|-------|-----------|
| **ABSTRACT-01 (este PBI)** | `feature` | MVP: Git opcional en arranque; activación de códice; smoke dominio no-PR; denegación Cerbero sin panic |
| **ABSTRACT-02 (hijo)** | `refactorization` (futuro) | Migrar `feature`/`bug-fix`/`refactorization` (+ ciclo PR) a `codex-software-engineering`; vaciar process Core de software-only |

## 3. Hitos MVP (ABSTRACT-01)

### H1 — Desacoplamiento de Inicialización (`workspace_init`)

Git/ramas/versionado **solo** si el perfil o Códice de Dominio activo lo exige. El escape lab `SDDIA_LAB_SKIP_GIT` no sustituye el gate de producción.

### H2 — Activación de Códices

Formalizar resolución de autoridad/enrutado semántico sobre el catálogo existente (`codex-contract` + índice). No reabrir códices FE/BE ya catalogados salvo necesidad demostrada.

### H3 — Estímulo de dominio no-software

Procesar o denegar limpiamente un ECST de dominio **existente** (p. ej. `telegram-message-received`, `manual-task-requested`) sin metadatos de repositorio. Paths SSOT:

- Genoma: `SddIA/events/` (`directories.events`)
- Bus: `./.events/domain` (`eda_fractal.domain`)
- Instancia: `.SddIA/events` (`eda_instance.customization`)

## 4. Criterios de aceptación

- [ ] **AC-BOOT:** Arranque + evento dominio no-PR sin exigir `.git`
- [ ] **AC-WSINIT:** Git opcional por códice/perfil
- [ ] **AC-CODEX:** Activación/enrutado de códice vía Cúmulo
- [ ] **AC-DENY:** Sin autoridad → denegación Cerbero sin panic
- [ ] **AC-BUILD:** `cargo build --release` OK
- [ ] **AC-DOC:** Cascada documental; PBI en `done/`; `validacion.md` `pbi_archived: true` en la rama del PR

## 5. Fuera de alcance (explícito)

- Vaciado total de `SddIA/process/` (ABSTRACT-02)
- Alta obligatoria de `codex-personal-assistant` / eventos `Email_Received` / `Prompt_Submitted`
- Inyección GesFer (Paciente 0) — depende del cierre de este PBI

## 6. Inexactitudes corregidas respecto a v1/v2

1. Path `.SddIA/events/domain/` como genoma → **falso**; usar Cúmulo.
2. «process/ solo agnóstico» como AC de este PR → **diferido** a ABSTRACT-02.
3. Códices «por crear» → cantera y contrato **ya existen**; faltan activación runtime + códices software/PA.
4. `Process: refactor` en semilla → **`feature`** por mandato de ciclo.

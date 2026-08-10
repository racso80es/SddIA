---
feature_name: sddia-domain-abstraction
created: "2026-08-05"
purpose: Estabilización PBI-SDDIA-DOMAIN-ABSTRACT-01 — separación de dominio Core vs códices; poda de acoplamiento Git/software en arranque
branch_name: feat/sddia-domain-abstraction
persist_ref: docs/features/sddia-domain-abstraction
pbi_ref: docs/todos/done/[ARQUITECTURA] Separación de Dominio SddIA y Abstracción del Contexto de Ejecución.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-01
execution_id: 758d4440-2889-47a9-b412-ffab00ba0c1d
phase: mayeuta-stabilization
agents: mayeuta
---

# Clarificación — PBI-SDDIA-DOMAIN-ABSTRACT-01

## D0 — Semilla

- **PBI origen (kitchen):** `docs/todos/kitchen/[REFACTOR] Separación de Dominio SddIA y Abstracción del Contexto de Ejecución.md` (`document_id: PBI-SDDIA-DOMAIN-ABSTRACT-01`; `status: pendiente-kitchen`).
- **Ciclo:** feature `sddia-domain-abstraction` · rama `feat/sddia-domain-abstraction`.
- **Init:** `./sddia-run.sh --process feature` + `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` + `SDDIA_LAB_SKIP_GIT=1` → `workspace-init` **executed** (`execution_id: 758d4440-2889-47a9-b412-ffab00ba0c1d`). Fases Mayeuta…Argos agent-runtime **abortadas** (`cursor-agent` colgado / `SDDIA_AGENT_RUNTIME_COMMAND` activo); estabilización materializada en IDE (relay).
- **Mandato operador:** proceso **`feature`** (no `refactorization`); primera acción = **refinar** semilla (contiene refactorizaciones mezcladas) y auditar congruencias/inexactitudes.
- **Dependencias citadas:** `docs/features/kalma2-full-cycle` · `docs/features/vanguardia-soberania-local` (ambas topologías presentes).
- **Bloqueante aguas abajo:** kitchen GesFer («Paciente 0») espera este PBI.
- **Normas / SSOT:** `SddIA/core/cumulo.paths.json`, `codex-contract.md`, `features-documentation-pattern`, `external-ai-constraints`, `capsule-json-io.md`.

## D1 — Entropía documental de la semilla

| Defecto | Evidencia | Corrección |
|---------|-----------|------------|
| **Doble cuerpo** | v1.0.0 + bloque «Refinamiento 1.0» v2.0.0 en el mismo fichero | Colapsar a **v3.0.0** único |
| **Proceso vs mandato** | Frontmatter `Process: refactor` / rama `refactor/…` | Operar como **`feature`** / `feat/sddia-domain-abstraction` (L-PROCESS) |
| **Filename vs título** | `[REFACTOR]` vs títulos ARQUITECTURA/Desacoplamiento | Renombrar al mover a `pending/` |
| **Destination Path fantasma** | `docs/todos/kitchen/PBI_Separacion_Dominio_SddIA.md` ≠ path real | Eliminar; SSOT = path físico del PBI |
| **Mezcla feature∪refactor** | Hitos de capacidad nueva + poda masiva de `SddIA/process/` | Partir: MVP en este ciclo; migración forja → PBI hijo (L-SPLIT) |

## D2 — Auditoría de congruencia (especificación vs realidad)

| ID | Afirmación PBI | Estado empírico | Veredicto |
|----|----------------|-----------------|-----------|
| **I1** | Bus de dominio en `.SddIA/events/domain/` | Cúmulo: genoma `directories.events` → `SddIA/events`; bus fractal `eda_fractal.domain` → `./.events/domain`; instancia `eda_instance.customization` → `.SddIA/events` | **INEXACTO** — corregir a rutas Cúmulo |
| **I2** | Taxonomía códices por crear en `library/codexes/` | Path existe; `codex-contract` + 3 códices FE/BE admin/product; **no** hay `codex-software-engineering` ni `codex-personal-assistant` | **PARCIAL** — formalizar activación/enrutado; alta de códices software/PA es alcance distinto |
| **I3** | `SddIA/process/` debe quedar «solo agnóstico» | Contiene `feature`, `bug-fix`, `refactorization`, `pull-request-review`, `accept-pr`, `delivery-close-cycle`, etc. | **ASPIRACIONAL / fuera de 1 PR** — diferir (L-SPLIT-B) |
| **I4** | `workspace_init` asume Git siempre | `workspace_init.rs` invoca `git-manager` salvo `SDDIA_LAB_SKIP_GIT` (escape lab, no códice) | **CONGRUENTE como deuda** — objetivo = gate por códice/perfil, no solo env lab |
| **I5** | Eventos ejemplo `Email_Received` / `Prompt_Submitted` | No existen; análogos: `telegram-message-received`, `kalma2-process-requested`, `manual-task-requested`, familia `domain-entity-*` | **INEXACTO** — AC sobre eventos reales o nuevos vía `event-creator` |
| **I6** | Core ignora PR/Git en estímulos | Dominio ECST aún cataloga `pull-request-*`; orquestación software vive en process Core | **DEUDA REAL** — no implica borrar ECST PR; implica no exigirlos para ignición genérica |
| **I7** | `is_workspace_init_phase` ↔ `delegates_to: skill:git-manager` | `feature.md` usa `requires_capability: proc:git-sync` (sin `delegates_to` git) | **RIESGO** — posible desalineación detector/handler; Dedalo debe verificar path real de ignición |

## D3 — Laudos Mayeuta (estabilización)

| ID | Decisión |
|----|----------|
| **L-PROCESS** | Ciclo = **`feature`**. Alinear PBI `process: feature`. Rama `feat/sddia-domain-abstraction`. |
| **L-SPLIT-A** | **Este PR (MVP):** (1) desacoplar `workspace_init`/arranque de Git obligatorio vía perfil/códice; (2) contrato de activación de Códice de Dominio consultable por runtime; (3) prueba de evento de dominio no-software sin `.git`; (4) denegación Cerbero sin autoridad (anti-panic). |
| **L-SPLIT-B** | **Fuera / PBI hijo** `PBI-SDDIA-DOMAIN-ABSTRACT-02` (refactorization): migración de `feature`/`bug-fix`/`refactorization` (+ PR cycle) desde Core `process/` hacia `codex-software-engineering`; vaciado ontológico de process software-only. |
| **L-SPLIT-C** | Alta de `codex-personal-assistant` y eventos PA específicos = PBI/feature satélite tras MVP; no bloquea AC de independencia de arranque. |
| **L-PATHS** | Toda especificación usa SSOT Cúmulo: genoma `SddIA/events/…`, bus `./.events/…`, instancia `.SddIA/events`. Prohibido `.SddIA/events/domain/` como path de genoma. |
| **L-AC-PROCESS** | AC «process/ solo agnóstico» **derogado** de este ciclo; sustituido por AC de arranque/bus/códice (ver objectives). |
| **L-EVIDENCE** | Prueba de dominio no-software: fixture/smoke sobre ECST existente (`telegram-message-received` o `manual-task-requested`) o evento mínimo nuevo solo si Dedalo lo exige; no inventar `Email_Received` sin `event-creator`. |
| **L-GENOME** | Mutaciones genoma vía `entity-manager` / `./sddia-run.sh`. Docs bajo `persist_ref` + PBI en `docs/todos/` fuera de gate EDA. |
| **L-RUNTIME** | Agent-runtime Kalma2 inestable en esta sesión; fases Mayeuta+ se materializan en IDE hasta laudo contrario. |

## D4 — Criterios de aceptación (producto, post-refine)

| AC | Enunciado |
|----|-----------|
| **AC-BOOT** | Arranque Core + consumo de un evento de dominio no-PR **sin** exigir árbol `.git` activo (perfil/códice sin software-engineering). |
| **AC-WSINIT** | `workspace_init` / fase de Inicialización: Git **opcional** según perfil/códice activo; no solo `SDDIA_LAB_SKIP_GIT`. |
| **AC-CODEX** | Contrato de activación/enrutado de Códice de Dominio documentado y consultable desde topología Cúmulo (`directories.library_codexes` + `codex-contract`). |
| **AC-DENY** | Estímulo sin códice/autoridad adecuada → Cerbero deniega (Falta de Autoridad / equivalente) **sin panic** del orquestador. |
| **AC-BUILD** | `cargo build --release` (crate Core pertinente) OK tras cambios. |
| **AC-DOC** | Cascada `features-documentation-pattern`; PBI → `docs/todos/done/`; `validacion.md` `global: APTO`, `pbi_archived: true` en la rama del PR. |

## D5 — Handoff Dedalo

1. Localizar path real de ignición `workspace_init` bajo `requires_capability: proc:git-sync` (I7) y diseñar gate por códice/perfil.
2. Definir touchpoints mínimos: `workspace_init.rs`, bindings DI, posible norma/códice activador; **sin** migrar process software en este PR.
3. Diseñar smoke reproducible AC-BOOT/AC-DENY con evento de dominio real.
4. Emitir `spec.md` + `plan.md` acotados a L-SPLIT-A; referenciar PBI-02 para L-SPLIT-B.

---
feature_name: sddia-codex-software-engineering
created: "2026-08-09"
purpose: Estabilización PBI-SDDIA-DOMAIN-ABSTRACT-02 — migración process software-engineering a Códice de Dominio
branch_name: feat/sddia-codex-software-engineering
persist_ref: docs/features/sddia-codex-software-engineering
pbi_ref: docs/todos/done/[ARQUITECTURA] PBI-SDDIA-DOMAIN-ABSTRACT-02 — Migración process software a códice.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
execution_id: c76c5d95-b066-49ca-834b-78a4f9443a62
phase: mayeuta-stabilization
agents: mayeuta
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-01
---

# Clarificación — PBI-SDDIA-DOMAIN-ABSTRACT-02

## D0 — Semilla

- **PBI origen (kitchen):** `docs/todos/kitchen/[REFACTOR] PBI-SDDIA-DOMAIN-ABSTRACT-02 — Migración process software a códice.md` (`document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02`; `status: pendiente-kitchen`; uuid seed `pending-on-forge`).
- **Ciclo:** feature `sddia-codex-software-engineering` · rama `feat/sddia-codex-software-engineering`.
- **Init:** `execute-process --process feature` + skips archive/delivery + `execution_profile.git_required:false` → `workspace-init` **executed** (`execution_id: c76c5d95-b066-49ca-834b-78a4f9443a62`). Mayeuta agent-runtime omitido; estabilización IDE (relay).
- **Mandato operador:** proceso **`feature`** (semilla decía `refactorization`).
- **Padre:** ABSTRACT-01 **Done** — PR [#161](https://github.com/racso80es/SddIA/pull/161) merge `4de6497`; perfil activo + gate Git ya en motor.
- **Transcript origen:** `docs/features/sddia-domain-abstraction/clarify.md` § L-SPLIT-B.
- **Normas / SSOT:** `codex-contract`, `directories.library_codexes`, `directories.process`, `features-documentation-pattern`, `external-ai-constraints`, perfil ABSTRACT-01.

## D1 — Entropía de la semilla

| Defecto | Corrección |
|---------|------------|
| `uuid: pending-on-forge` | UUID v4 definitivo `1b45bff6-da8a-4e31-879e-3068ed80b213` |
| `process: refactorization` vs mandato | **`feature`** (L-PROCESS) |
| Filename `[REFACTOR]` | Renombrar al promover a `pending/` como `[ARQUITECTURA]` |
| Alcance «extraer» sin mecánica | Partir: **autoridad de códice + gate runtime** (MVP) vs **relocalización física** de `.md` (Dedalo / posible ABSTRACT-03) |

## D2 — Congruencia (post-ABSTRACT-01)

| ID | Hecho | Implicación |
|----|-------|-------------|
| **I1** | `feature`/`bug-fix`/`refactorization`/`pull-request-review`/`accept-pr`/`delivery-close-cycle` viven en `SddIA/process/` | Núcleo software-lifecycle aún en Core |
| **I2** | Códices FE/BE existen; **no** hay `codex-software-engineering` | Alta vía `codex-creator` / `entity-manager` |
| **I3** | `codex-contract` = normas + vibe; **no** lista process bindings hoy | Extender composition y/o cuerpo con inventario de process del dominio (Dedalo fija forma sin romper contrato) |
| **I4** | ABSTRACT-01: `execution_profile` / `.SddIA/active-domain-profile.json` | Reutilizar: `codex_slug: codex-software-engineering` + `git_required: true` para ciclos software |
| **I5** | Orquestador resuelve process desde `directories.process` (genoma) | Mover archivos fuera de Core **rompe** resolución salvo overlay Cúmulo/instancia — riesgo alto; MVP no asume move físico ciego |
| **I6** | Creators (`process-creator`, `*-creator`) son forja Core, no «uso software» | **Fuera** del vaciado software-lifecycle (permanecen Core) |

## D3 — Laudos Mayeuta

| ID | Decisión |
|----|----------|
| **L-PROCESS** | Ciclo = **`feature`**. Rama `feat/sddia-codex-software-engineering`. |
| **L-PARENT** | ABSTRACT-01 prerrequisito **satisfecho**; se puede forjar. |
| **L-MVP-A** | **Este PR:** (1) forjar `codex-software-engineering` indexado; (2) declarar membresía de process software-lifecycle; (3) gate runtime: esos process exigen perfil/códice software activo (sobre ABSTRACT-01); (4) smoke: sin códice → denegación controlada; con códice → init software OK. |
| **L-MVP-B** | Relocalización física de `feature.md` etc. fuera de `directories.process` = **solo si** Dedalo demuestra path de resolución (instancia/overlay) sin romper TQM/Kalma2. Si no → defer **ABSTRACT-03** kitchen. |
| **L-SCOPE-PR** | Ciclo PR (`pull-request-review`, `accept-pr`, `delivery-close-cycle`) entra en membresía del códice **salvo** laudo Dedalo que los marque Core-agnósticos. Default: **incluidos** en software-engineering. |
| **L-KEEP-CORE** | `entity-manager`, `*-creator`, `route-*`, daemons/governance, `kalma2-interact`, audits → **no** migrar. |
| **L-GENOME** | Genoma vía `entity-manager` / `./sddia-run.sh`. Motor `execute-process` editable directo (no DA-2). |
| **L-UUID** | `1b45bff6-da8a-4e31-879e-3068ed80b213` |

## D4 — Criterios de aceptación (producto)

| AC | Enunciado |
|----|-----------|
| **AC-CODEX** | `codex-software-engineering` existe, indexado en `library/codexes/index.md`, cumple `codex-contract`. |
| **AC-MEMBER** | Inventario explícito de process del dominio software (mín. `feature`, `bug-fix`, `refactorization`; PR cycle según L-SCOPE-PR). |
| **AC-GATE** | Ejecutar process miembro **sin** códice/perfil software activo → denegación Cerbero/orquestador **sin panic**. |
| **AC-ALLOW** | Con perfil `codex_slug` software (+ `git_required` coherente) → `feature` workspace-init ejecutable. |
| **AC-BUILD** | `cargo build -p execute-process --release` OK. |
| **AC-DOC** | Cascada completa; PBI → `done/`; `validacion.md` `pbi_archived: true` en rama del PR. |
| **AC-MOVE** | (Opcional / Dedalo) Relocalización física APTO **o** explícitamente diferida a ABSTRACT-03 con evidencia. |

## D5 — Handoff Dedalo

1. Diseñar frontmatter/cuerpo de `codex-software-engineering` (composition normas: p.ej. `features-documentation-pattern`, `pr-acceptance-protocol`, git-ops si UUID indexado).
2. Mecánica de gate: ¿dónde enganchar (pre-fase init, DI Cerbero, lector perfil)? Mínimo touchpoints.
3. Dictaminar AC-MOVE: factible en este PR o ABSTRACT-03.
4. `spec.md` + `plan.md` bajo L-MVP-A; no vaciar creators Core.

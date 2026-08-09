---
feature_name: sddia-domain-abstract-03-relocalizacion
created: "2026-08-09"
purpose: Estabilización PBI-SDDIA-DOMAIN-ABSTRACT-03 — relocalización física process software fuera de directories.process Core
process: refactorization
branch_name: feat/sddia-domain-abstract-03-relocalizacion
persist_ref: docs/features/sddia-domain-abstract-03-relocalizacion
pbi_ref: docs/todos/done/[REFACTOR] PBI-SDDIA-DOMAIN-ABSTRACT-03 — Relocalización física process software.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
phase: mayeuta-stabilization
agents: mayeuta
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
correlation_id: ""
---

# Clarificación — PBI-SDDIA-DOMAIN-ABSTRACT-03

## D0 — Semilla

- **PBI origen (kitchen):** `docs/todos/done/[REFACTOR] PBI-SDDIA-DOMAIN-ABSTRACT-03 — Relocalización física process software.md` (`document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03`; `status: pendiente-kitchen`; `uuid: pending-on-forge`).
- **Ciclo:** `refactorization` · rama `feat/sddia-domain-abstract-03-relocalizacion` · `persist_ref` arriba.
- **Origen AC-MOVE:** `docs/features/sddia-codex-software-engineering/spec.md` § D1 / AC-MOVE diferido; `validacion.md` ABSTRACT-02 = **APTO** con **AC-MOVE APTO_DEFERRED**.
- **Padre:** ABSTRACT-02 Done — códice `codex-software-engineering` (`a69d04b0-…`) + gate `DOMAIN_AUTHORITY_DENIED` + membresía process software-lifecycle (incl. ciclo PR).
- **Huecos inyección:** `pbi_ref` y `correlation_id` vacíos en payload runtime; SSOT de PBI = path kitchen arriba hasta promoción a `pending/`.
- **Normas / SSOT:** `SddIA/core/cumulo.paths.json` (`directories.process`), `codex-software-engineering`, `process-contract`, `features-documentation-pattern` v1.2.1, `external-ai-constraints`, motor `execute-process` (`core/resolver.rs`).

## D1 — Entropía de la semilla

| Defecto | Corrección |
|---------|------------|
| `uuid: pending-on-forge` | UUID v4 definitivo al forjar/promover PBI a `pending/` (L-UUID) |
| Filename `[REFACTOR]` + kitchen | Coherente con `process: refactorization`; renombrar a `[ARQUITECTURA]` solo si operador alinea naming ABSTRACT-01/02 |
| «No iniciar hasta APTO ABSTRACT-02» | Prerrequisito **satisfecho** (PBI-02 en `docs/todos/done/` + validacion APTO) |
| Destino físico ambiguo («códice/instancia») | Dedalo fija path canónico (overlay Cúmulo vs `.SddIA/…`); Mayeuta no inventa ruta (L-DEST) |

## D2 — Congruencia empírica (post-ABSTRACT-02)

| ID | Hecho | Implicación |
|----|-------|-------------|
| **I1** | `feature`/`bug-fix`/`refactorization`/`pull-request-review`/`accept-pr`/`delivery-close-cycle` siguen en `SddIA/process/` | Move físico **pendiente**; gate de autoridad ya existe sin move |
| **I2** | `resolve_process_path` (`resolver.rs`) une hardcode `repo.join("SddIA/process")` — **no** lee `directories.process` ni overlay instancia | Mover `.md` sin cambiar resolución = **quiebra** TQM/Kalma2/`sddia-run` |
| **I3** | Cúmulo declara un solo `directories.process` → `SddIA/process`; `eda_instance` solo cubre events | **No hay** overlay process documentado/implementado hoy |
| **I4** | Membresía códice ya lista los 6 process (Dedalo ABSTRACT-02 **D2**) | Default este ciclo: **mover los 6** salvo laudo Dedalo que deje ciclo PR en Core |
| **I5** | Creators / `entity-manager` / routes EDA / daemons permanecen Core (L-KEEP-CORE padre) | Fuera de alcance de move |
| **I6** | Índices `SddIA/process/index.md`, EDA coverage, process-creator asumen genoma bajo `directories.process` | Move exige actualización índice + resolución + posibles creators/tests |
| **I7** | `process-creator` escribe siempre bajo `directories.process` Core | Tras move, alta de process software-lifecycle debe apuntar a jurisdicción destino (Dedalo) |

## D3 — Laudos Mayeuta

| ID | Decisión |
|----|----------|
| **L-PROCESS** | Ciclo = **`refactorization`**. Rama `feat/sddia-domain-abstract-03-relocalizacion`. |
| **L-PARENT** | ABSTRACT-02 prerrequisito **satisfecho**; se puede diseñar/ejecutar move. |
| **L-RESOLVE-FIRST** | **Prohibido** mover `.md` antes de path de resolución demostrado (Cúmulo `directories.*` y/o overlay instancia + cambio en `resolve_process_path`). Orden: diseño resolución → smoke resolve → move → smoke ciclo. |
| **L-SCOPE-LIFECYCLE** | Obligatorio en alcance: `feature`, `bug-fix`, `refactorization`. |
| **L-SCOPE-PR** | Ciclo PR (`pull-request-review`, `accept-pr`, `delivery-close-cycle`): **incluidos por default** (alineado membresía códice). Dedalo puede excluirlos con laudo explícito y evidencia de por qué deben permanecer Core-agnósticos. |
| **L-DEST** | Destino = jurisdicción códice/instancia **decidida por Dedalo** (p. ej. extensión Cúmulo + dir instancia, o path bajo library/codex packing). Prohibido hardcode de cliente fuera de `cumulo.paths.json` / fusión local. |
| **L-KEEP-CORE** | No migrar creators, entity-manager, kalma2-interact, audits, governance/daemons no-software. |
| **L-COMPAT** | Tras move: `./sddia-run.sh --process feature|bug-fix|refactorization` (y PR cycle si L-SCOPE-PR) resuelve y ejecuta al menos init sin panic; gate ABSTRACT-02 intacto. |
| **L-GENOME** | Mutaciones genoma process vía `entity-manager` / forja gobernada; motor `execute-process` editable directo (resolver). Docs/`persist_ref` fuera de gate EDA. |
| **L-UUID** | Sustituir `pending-on-forge` al promover PBI; no bloquear estabilización documental. |
| **L-PBI** | PBI permanece kitchen hasta promoción operatoria a `pending/`; cierre Done = move a `done/` + `validacion.md` en esta rama (un PR). |

## D4 — Criterios de aceptación

| AC | Enunciado |
|----|-----------|
| **AC-RESOLVE** | Orquestador resuelve process software-lifecycle desde path **no-Core** (o overlay documentado) vía topología Cúmulo/instancia; evidencia unitaria o smoke sobre `resolve_process_path` / equivalente. |
| **AC-MOVE** | `.md` de L-SCOPE-LIFECYCLE (+ L-SCOPE-PR según Dedalo) **ausentes** de genoma Core `SddIA/process/` (o stub mínimo prohibido salvo laudo) y presentes en destino canónico. |
| **AC-INDEX** | Índice(s) process y referencias runtime alineados; sin entradas fantasma al path viejo. |
| **AC-RUN** | Smoke: con perfil/códice software, `feature` (u otro miembro) inicia sin panic; sin autoridad → `DOMAIN_AUTHORITY_DENIED` intacto (ABSTRACT-02). |
| **AC-TQM** | TQM / Kalma2 / `sddia-run` no rotos para process Core restantes ni para miembros relocalizados. |
| **AC-BUILD** | `cargo build -p execute-process --release` OK. |
| **AC-DOC** | Cascada `features-documentation-pattern`; PBI → `docs/todos/done/`; `validacion.md` `global: APTO`, `pbi_archived: true` en la rama del PR. |

## D5 — Handoff Dedalo

1. Diseñar mecánicas de **resolución overlay** (leer `directories.process` desde Cúmulo; capa instancia; precedencia Core vs overlay) y tocar `resolver.rs` (+ tests).
2. Fijar **destino físico** canónico y si ciclo PR se mueve (confirmar o derogar L-SCOPE-PR).
3. Plan de migración: move archivos, índice, EDA/coverage si aplica, process-creator / docs que asuman Core path.
4. Emitir `spec.md` + `plan.md` bajo L-RESOLVE-FIRST; no autorizar Tekton a borrar Core process sin AC-RESOLVE verde en diseño.

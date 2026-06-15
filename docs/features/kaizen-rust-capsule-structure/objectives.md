---
feature_name: kaizen-rust-capsule-structure
created: "2026-06-15"
process: refactorization
branch_name: feat/kaizen-rust-capsule-structure
persist_ref: docs/features/kaizen-rust-capsule-structure
pbi_ref: docs/todos/done/kaicen Estructura de Cápsulas Rust.md
priority: kaizen-arquitectura
updated: "2026-06-15"
status: k7_pr_presentado
pause_after: k7-delivery-close-cycle
next_wave: post-merge-debt-backlog
handoff: docs/features/kaizen-rust-capsule-structure/status.md
debt_ref: plan.md#backlog-de-deuda-técnica-post-k6
pr_url: https://github.com/racso80es/SddIA/pull/93
---

# Objetivos — Kaizen Estructura de Cápsulas Rust

## Misión

Erradicar entropía Python en la capa operativa `SddIA/scripts/` (tools, skills, daemons) y consolidar ejecución **S+ Grade** mediante cápsulas Rust nativas: binarios sin estado bajo `SddIA/tools/`, `SddIA/skills/` y `SddIA/daemons/`, con contrato `capsule-json-io` (envelope por stdin, `result`/`feedback`/`exitCode` por stdout).

## Contexto (delta termodinámico — post-Ola 3)

| Aspecto | Estado tras Kaizen | Pendiente (deuda) |
|---------|-------------------|-------------------|
| Tools | SSOT `SddIA/tools/` + runtime Rust | IOTA TS legacy (DEBT-K3) |
| Skills | SSOT `SddIA/skills/` + poda limbo | Fallbacks WASI (DEBT-K7) |
| Daemons | 4 centinelas `native-rust` | DLT delegate Python (DEBT-K2) |
| Runtime QA | `execute-process.py` operativo | Fuera alcance — DEBT-K1 |

## Hitos

| Hito | Contenido | Entregable |
|------|-----------|------------|
| **K1** | Mapeo y auditoría termodinámica | `clarify.md` §D3 + `spec.md` ✅ |
| **K2** | Adecuar contratos | `daemons-contract`, `tools-contract`, `skills-contract` ✅ |
| **Ola 1 — Skills** | `scripts/skills/` → `SddIA/skills/` | ✅ SK-CA* |
| **Ola 2 — Tools** | `scripts/tools/` → `SddIA/tools/` | ✅ TL-CA* |
| **Ola 3 — Daemons** | `scripts/daemons/` → `SddIA/daemons/` | ✅ DM-CA* |
| **K6** | Poda + certificación EDA | ✅ `validacion.md` APTO |
| **K7** | Cierre PR | ✅ PR [#93](https://github.com/racso80es/SddIA/pull/93) |

**Orden de consolidación:** Skills → Tools → Daemons (`clarify.md` §D8) — **completado**.

## Objetivos medibles

| ID | Objetivo | Estado |
|----|----------|--------|
| O1 | Encapsulamiento S+ (V1) | ✅ cápsulas; orquestador Python documentado (DEBT-K1) |
| O2 | Contrato I/O | ✅ skills/tools/daemons migrados |
| O3 | Autonomía centinelas | ✅ 4 binarios Rust |
| O4 | Higiene estructural | ✅ poda limbo; drift docs DEBT-K8 |
| O5 | Telemetría (V2) | ✅ E2E + heartbeat |
| O6 | Matriz Rust (V3) | ✅ `implementation.md` |

## Deuda técnica planificada

Ver [`plan.md` §Backlog de deuda técnica](./plan.md#backlog-de-deuda-técnica-post-k6). No bloquea K7.

| ID | Resumen |
|----|---------|
| DEBT-K1 | Orquestador `scripts/qa/` Python |
| DEBT-K2 | `github_bridge_process_pr.py` |
| DEBT-K3 | IOTA publisher TS limbo |
| DEBT-K4–K9 | Ver plan (runtime legacy, forja daemon-creator, docs, IOTA crate) |

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Forja genoma vía `entity-manager` / `*-creator`.
- Cierre: un PR + `validacion.md` APTO + PBI en `docs/todos/done/` — **K7 ✅ PR #93**.

## Fases (runtime IDE)

| Fase | Agente | Estado |
|------|--------|--------|
| Estabilización de alcance | Mayeuta | ✅ |
| Diseño de refactor | Dedalo | ✅ |
| Ejecución Ola 1–3 + K6 | Tekton | ✅ |
| Verificación | Argos | ✅ `validacion.md` |
| Cierre | delivery-close-cycle | ✅ PR #93 |

Handoff: [`status.md`](./status.md).

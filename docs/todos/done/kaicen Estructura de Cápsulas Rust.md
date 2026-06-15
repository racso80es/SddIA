---
document_id: PBI-KAIZEN-RUST-CAPSULE-STRUCTURE
title: "[Kaizen] Erradicación de Entropía Python y Transición a Estructura de Cápsulas Rust"
format: markdown
version: "1.0.0"
created: "2026-06-15"
status: done
priority: alta
process: refactorization
branch_name: feat/kaizen-rust-capsule-structure
feature_ref: docs/features/kaizen-rust-capsule-structure
validacion_ref: docs/features/kaizen-rust-capsule-structure/validacion.md
pr_url: https://github.com/racso80es/SddIA/pull/93
closed: "2026-06-15"
---

# PBI-KAIZEN: Erradicación de Entropía Python y Transición a Estructura de Cápsulas Rust

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-KAIZEN-RUST-CAPSULE-STRUCTURE` |
| **Estatus** | ✅ K7 — PR [#93](https://github.com/racso80es/SddIA/pull/93) presentado |
| **Validación** | [`validacion.md`](../../features/kaizen-rust-capsule-structure/validacion.md) |
| **Handoff** | [`status.md`](../../features/kaizen-rust-capsule-structure/status.md) |
| **Deuda** | [`plan.md` §Backlog](../../features/kaizen-rust-capsule-structure/plan.md#backlog-de-deuda-técnica-post-k6) |
| **Feature** | [`docs/features/kaizen-rust-capsule-structure/`](../../features/kaizen-rust-capsule-structure/) |
| **Rama** | `feat/kaizen-rust-capsule-structure` |

## Resumen de entrega

| Ola | Entregable |
|-----|------------|
| Ola 1 | Skills Rust en `SddIA/skills/` |
| Ola 2 | Tools Rust en `SddIA/tools/` |
| Ola 3 | 4 centinelas Rust en `SddIA/daemons/` |
| K6 | `validacion.md` global APTO |
| K7 | PR [#93](https://github.com/racso80es/SddIA/pull/93) |

Legacy podado en `SddIA/scripts/limbo/{skills,tools,daemons}/`.

**Post-merge:** backlog DEBT-K1…K9 en `plan.md` (features independientes).

## 1. Clarificación (Clarify)

Actualmente, el ecosistema SddIA depende de múltiples scripts físicos en Python alojados en SddIA/scripts/ (tools, skills, qa, daemons/centinelas como event-watcher o telegram-gateway). Esta arquitectura legacy genera fricción en el despliegue (requiere runtime de Python, pip, entornos aislados) y rompe la simetría de la ejecución encapsulada S+ Grade. Para asegurar la portabilidad absoluta del motor SddIA y la inmutabilidad de su operativa, todos los artefactos de ejecución deben ser migrados a la estructura nativa Rust, operando como ejecutables binarios sin estado que respetan el contrato JSON I/O por stdin/stdout.

## 2. Objetivos (Objectives)

Encapsulamiento S+ Grade: Eliminar la dependencia del intérprete de Python en la ejecución del core.

Estandarización de Interfaz: Asegurar que el 100% de las tools y skills operen bajo la norma SddIA/norms/capsule-json-io.md, recibiendo el sobre (envelope) por entrada estándar y emitiendo result, feedback y exitCode por salida estándar.

Autonomía de Centinelas: Refactorizar los daemons (watcher del sistema de archivos, puente de GitHub, Telegram) en binarios Rust eficientes y concurrentes (usando crates como notify y tokio), garantizando que sigan siendo "despertadores ciegos" sin jurisdicción lógica.

Higiene Estructural (Poda Ontológica): Eliminar el directorio SddIA/scripts/ de la capa operativa principal una vez completada la migración, moviendo el genoma compilado a SddIA/tools/, SddIA/skills/ SddIA/daemons/ con sus respectivos Cargo.toml.

## 3. Plan de Ejecución (Plan)

Ejecutado según `docs/features/kaizen-rust-capsule-structure/plan.md` — Olas 1–3 + K6 certificados.

## 4. Criterios de Aceptación (Validation)

| ID | Estado |
|----|--------|
| V1 | ✅ Binarios Rust skills/tools/daemons (intérprete QA fuera alcance) |
| V2 | ✅ E2E lab + heartbeat telemetría |
| V3 | ✅ Matriz Rust documentada en `implementation.md` |

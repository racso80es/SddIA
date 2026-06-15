---
feature_name: kaizen-rust-capsule-structure
created: "2026-06-15"
updated: "2026-06-15"
process: refactorization
branch_name: feat/kaizen-rust-capsule-structure
persist_ref: docs/features/kaizen-rust-capsule-structure
pause_after: k6-certification
debt_ref: plan.md#backlog-de-deuda-técnica-post-k6
---

# Implementación — Kaizen (Ola 1–3 + K6)

**Handoff operativo:** [`status.md`](./status.md)

## K2 — Contratos

| Artefacto | Versión |
|-----------|---------|
| `skills-contract.md` | v1.4.0 |
| `tools-contract.md` | v1.5.0 |
| `daemons-contract.md` | v1.0.0 §3 `SddIA/daemons/{name}/` |

## Ola 1 — Skills ✅

| Área | Artefactos |
|------|------------|
| Runtime | `capsule_resolve.py` (skills), `execute_process_capsules.py`, `execute-action.py` |
| SSOT | `cumulo.execution_capsules.skills` → `SddIA/skills/` |
| Poda | `scripts/skills/` → `limbo/skills/` |

## Ola 2 — Tools ✅

| Área | Artefactos |
|------|------------|
| Runtime | `resolve_tool_*`, `invoke_tool_capsule_json`, `iota_tool_invoke.py`, `_tool_envelope_field` |
| SSOT | `cumulo.execution_capsules.tools` → `SddIA/tools/`; tool `.md` paths |
| Poda | `scripts/tools/` → `limbo/tools/` |
| Tests | `test_chaos_tools`, `test_chaos_immunity_eda`, `test_execute_suite`, `test_telegram_tool_capsule` |

## Ola 3 — Daemons ✅

### Nuevo (Rust)

| Ruta | Rol |
|------|-----|
| `SddIA/sddia-daemon-runtime/` | Lock, heartbeat, `eda_bus`, `eda_sweep` |
| `SddIA/daemons/event-watcher/` | Poll pending → route delegate |
| `SddIA/daemons/event-sweeper/` | Recolector pending sweep |
| `SddIA/daemons/telegram-watcher/` | Long-poll Telegram → gateway |
| `SddIA/daemons/github-bridge-watcher/` | Poll GitHub → DLT delegate |
| `SddIA/daemons/*.sh` | Launchers release/debug |

### Runtime Python (delegación / lab)

| Ruta | Cambio |
|------|--------|
| `capsule_resolve.py` | `resolve_daemon_capsule` |
| `governance_daemon_manager_core.py` | `native-rust` launch |
| `run-eda-e2e-lab.py`, `run-iota-ci-smoke.py` | watcher Rust |
| `github_bridge_process_pr.py` | **Nuevo** — DLT/IOTA para P3 |
| `test_bucle_fantasma_bus.py` | legacy → `limbo/daemons/` |

### SSOT / genoma

| Entidad | Versión | Runtime |
|---------|---------|---------|
| `event-watcher.md` | 1.1.0 | native-rust |
| `event-sweeper.md` | 1.0.0 | native-rust |
| `telegram-watcher.md` | 1.1.0 | native-rust |
| `github-bridge-watcher.md` | 1.1.0 | native-rust |

### K6 ✅

- E2E lab, heartbeat audit, `validacion.md` APTO, PBI en `done/`.

## Deuda técnica (planificada)

Backlog detallado: [`plan.md` §Backlog de deuda técnica](./plan.md#backlog-de-deuda-técnica-post-k6).

| ID | Artefacto | Nota |
|----|-----------|------|
| DEBT-K1 | `SddIA/scripts/qa/` | Orquestador Python — fuera alcance Kaizen |
| DEBT-K2 | `github_bridge_process_pr.py` | Hot path DLT github-bridge |
| DEBT-K3 | `limbo/tools/iota-immutable-publisher/` | Publisher IOTA TS |
| DEBT-K4 | `daemon_centinel_runtime.py` | Solo `limbo/daemons/` |
| DEBT-K5 | `limbo/daemons/*` | Archivo legacy |
| DEBT-K6 | `daemon-creator` | Forja simulada en lab |
| DEBT-K7 | `limbo/skills/*.py` | Fallback WASI |
| DEBT-K8 | README + docs históricos | Rutas `scripts/daemons` obsoletas |
| DEBT-K9 | IOTA Rust crate | Stub pendiente |

## Siguiente paso

**K7** — `delivery-close-cycle` + PR único. Ver [`status.md`](./status.md) §K7.

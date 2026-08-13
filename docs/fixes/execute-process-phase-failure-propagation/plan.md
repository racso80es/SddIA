---
feature_name: execute-process-phase-failure-propagation
created: "2026-08-11"
process: bug-fix
version_plan: "1.0.0"
branch_name: fix/execute-process-phase-failure-propagation
persist_ref: docs/fixes/execute-process-phase-failure-propagation
pbi_ref: docs/todos/done/[FIX] execute-process — fallo de fase debe fallar ejecución global (EV-AUD-005).md
document_id: 04f8f435-450b-477a-970a-4a05dd0224cb
finding: EV-AUD-005
correlation_id: dcb9efed-2268-4298-8108-7a55cf4db323
phases:
  - diseño
  - helper-terminal
  - cableado-runners
  - regresion-cobertura
  - verificacion-cierre
---

# Plan — EV-AUD-005 phase-failure propagation

Blueprint de ejecución para Tekton. **No** se instancia un proceso nuevo en genoma; se corrige el agregador del orquestador.

## Fase 0 — Diseño (Dedalo)

- [x] `objectives.md` consumido (alias bug_summary + EV-AUD-005)
- [x] `spec.md` emitido (L1–L4, CA1–CA7)
- [x] `plan.md` emitido (este archivo)

## Fase 1 — Helper terminal (Tekton)

| # | Tarea | Detalle | Estado |
|---|-------|---------|--------|
| 1.1 | Extraer agregador puro | `phase_reports` + `state` → `TerminalVerdict` | [x] |
| 1.2 | Semántica status | Hard fail: `failed`/`blocked`; neutros: `executed`/`skipped`/`simulated`/`awaiting`/`awaiting_agents` | [x] |
| 1.3 | Extracción causal | Primera fase fallida → `failed_phase`, código tipado y `error` | [x] |
| 1.4 | Fail-soft de fase | Solo si contrato lo declara; hoy hard-fail por defecto (ver spec §L1.5) | [x] |
| 1.5 | Tests unitarios del helper | Matriz CA2 + códigos tipados + patrón Cerbero | [x] código; cargo pendiente shell |

**Gate:** tests del módulo helper en verde antes de dar por cerrada la Fase 2.

`delegates_to` lógico: `agent:tekton` (mutación motor Rust; sin forja genoma).

## Fase 2 — Cablear residual + executor (Tekton)

| # | Tarea | Detalle | Estado |
|---|-------|---------|--------|
| 2.1 | `residual_runner::run_generic` | Sustituir `success = !blocked` por agregador; `status_code`/`exit_code`/`error` desde verdict | [x] |
| 2.2 | `state["phase_reports"]` | Materializar antes de `thermodynamic::run` | [x] |
| 2.3 | Envelope `data` | Inyectar `failed_phase*` cuando `!success` | [x] |
| 2.4 | `executor::run_generic` | Mismo helper (parity) | [x] |
| 2.5 | Peaje | Pasar `success`/`exit_code` agregados; no alterar fail-soft I/O D3.13 | [x] + telemetría causal |
| 2.6 | Opcional | Unificar `delivery_close` / `capsule_invoke_smoke` si el diff es acotado | [x] |

**Riesgo:** procesos chaos (`audit-thermodynamic-toll-failsoft`) que esperan exit 0 pese a estrés I/O — no confundir con fase `failed`. Mitigación: test CA3.

**WIP:** reconciliar con `spec.md` (normativa). No mezclar `kalma2.rs` / `task_queue_manager.rs` sin causalidad EV-AUD-005.

## Fase 3 — Regresión y cobertura (Tekton)

| # | Tarea | Detalle | Estado |
|---|-------|---------|--------|
| 3.1 | Fixture Cerbero persistencia | Patrón `62b201cf-…` → global fail | [x] `t9_regression_62b201cf_*` |
| 3.2 | DI gate / cápsula / agente | Un vector por CA6 | [x] T4–T7 |
| 3.3 | Neutros | Solo simulated/skipped/awaiting_agents → success si no hay failed | [x] T2 |
| 3.4 | `implementation.md` + `execution.md` | Frontmatter `features-documentation-pattern` | [x] |

## Fase 4 — Verificación y cierre documental (Argos + delivery)

| # | Tarea | Entregable |
|---|-------|------------|
| 4.1 | `validacion.md` | `global: APTO`, checks CA1–CA7, `pbi_archived: true` |
| 4.2 | PBI → `docs/todos/done/` | conservar `document_id` |
| 4.3 | `delivery-close-cycle` | PR único pre-merge |

`delegates_to`: `agent:argos` → luego `action:execute-process` (`delivery-close-cycle`).

## Orden de dependencias

```
Fase 0 ──► Fase 1 ──► Fase 2 ──► Fase 3 ──► Fase 4
```

## Restricciones operativas

- Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`.
- Tekton/Argos **no** escriben semillas Kaizen bajo `docs/todos/` (solo cierre documental PBI en Fase 4).
- No `--no-verify` / force push.
- No inventar éxito de validación sin evidencia.

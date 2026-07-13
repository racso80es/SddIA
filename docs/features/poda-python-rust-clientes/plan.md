---
feature_name: poda-python-rust-clientes
created: "2026-07-11"
updated: "2026-07-11"
process: feature
branch_name: feat/poda-python-rust-clientes
persist_ref: docs/features/poda-python-rust-clientes
phases: 8
agent_planificador: dedalo
consolidation_order: engine-cores, clientes, purga-ssot, cores-qa, capa-qa, docs-genoma, verify-binaries, cierre-cero-py
---

# Plan de implementación — poda-python-rust-clientes

Blueprint Tekton. PBI v2.0.0 — **cero `.py` y cero referencias operativas**; rotura de compatibilidad asumida.

## 0. Estado de la entrega

| Bloque | Estado | Evidencia |
|--------|--------|-----------|
| Inicialización Git | ✅ | `./sddia-run.sh --process feature` |
| Clarificación (Mayeuta) | ✅ | `clarify.md` D1–D8 |
| Objetivos | ✅ | `objectives.md` O1–O16 |
| Especificación (Dedalo) | ✅ | `spec.md` |
| **Ola 1 — Engine cores** | ✅ | `implementation.md` |
| **Ola 2 — Clientes** | ✅ | `implementation.md` |
| **Ola 3 — Purga limbo + Cúmulo** | ✅ | `implementation.md` |
| **Ola 4 — Purga cores QA duplicados** | ✅ | `implementation.md` |
| **Ola 5 — Capa QA/aduana** | ✅ | `sddia-qa`, 0 `.py` en QA |
| **Ola 6 — Docs genómicas** | ✅ | `implementation.md` |
| **Ola 7 — Verificación binarios** | ✅ | `verify-compiled-capsules` 24/24 |
| **Ola 8 — Gate cero-Python + cierre** | ✅ | `validacion.md` APTO |

**Orden de consolidación:**

```text
Ola 1 Engine ──► Ola 2 Clientes ──► Ola 3 Limbo
      ──► Ola 4 Cores QA duplicados
      ──► Ola 5 Capa QA + DEBT-K2
      ──► Ola 6 Docs skills/contratos
      ──► Ola 7 Verificación binarios Rust
      ──► Ola 8 Gate O11/O12 + Argos + delivery-close
```

---

## Ola 1 — Porte cores engine ✅

Ver `implementation.md`. Criterio: CA1–CA3, O1–O5.

---

## Ola 2 — Adecuación clientes ✅

Ver `implementation.md`. Criterio: CA5, O6.

---

## Ola 3 — Purga limbo y SSOT ✅

Ver `implementation.md`. Criterio: CA6–CA7, O7.

---

## Ola 4 — Purga cores QA duplicados

**Intent:** Eliminar fósil Python ya portado a Rust en `execute-process`; migrar golden/tests.

| # | Entregable | Detalle |
|---|------------|---------|
| 4.1 | Inventario paridad | Matriz Python QA ↔ módulo Rust (15 cores) |
| 4.2 | Tests Rust | Cubrir gaps de `test_*.py` que importaban cores |
| 4.3 | Eliminar cores | `route_fractal_event_core.py`, `radamanto_batch_core.py`, `telemetry_compliance_audit_core.py`, `fix_tool_process_core.py`, `cerbero_governance_react_core.py`, `route_domain_event_core.py`, `execute-action.py`, `execute_process_core.py`, `telegram_*_core.py`, `daemon_*_core.py`, `governance_daemon_manager_core.py`, `kalma2_interact_core.py` |
| 4.4 | Helpers QA post-limbo | Consolidar `telegram_gateway_transmute`, `watcher_idempotency` en Rust o retirar si redundantes |

**Criterio de salida:** O13; CA12; cores ausentes; tests Rust verdes para comportamiento antes cubierto por cores Python.

---

## Ola 5 — Capa QA/aduana sin Python

**Intent:** Liquidar `SddIA/scripts/qa/` como árbol Python; aduana vía Rust/shell.

| # | Entregable | Detalle |
|---|------------|---------|
| 5.1 | Git hooks | `pre_commit_gate`, `pre_push_gate`, `post_merge_gate` → Rust o shell + `execute-process` |
| 5.2 | Runners lab | `run-eda-e2e-lab`, `run-iota-ci-smoke`, `run-wasi-ci-smoke` → smokes Rust o procesos SddIA |
| 5.3 | Utilidades | `capsule_resolve.py`, `orchestrator_resolve.py`, `eda_bus_utils.py`, … → crates compartidos o CLI |
| 5.4 | Tests | 17 `test_*.py` → `cargo test` / integration tests; eliminar duplicados |
| 5.5 | **DEBT-K2** | Portar `github_bridge_process_pr` a Rust; `github-bridge-watcher` sin `python_bin()` |
| 5.6 | Wrappers raíz | Eliminar `scripts/qa/verify-process-integrity.py`, `scripts/migrate-local-constitutions-once.py` o portar |
| 5.7 | PoC | Eliminar/migrar `docs/features/poc-interface-comunicacion/_browser-func-test-kalma2.py` |

**Criterio de salida:** O14, O16; CA13, CA14; `SddIA/scripts/qa/` sin `.py`.

---

## Ola 6 — Documentación genómica

**Intent:** SSOT sin referencias a delivery Python.

| # | Entregable | Detalle |
|---|------------|---------|
| 6.1 | `SddIA/skills/*.md` | `implementation_path_ref` → `SddIA/skills/{name}/` + target nativo/WASI |
| 6.2 | Contratos/normas | `tools-contract`, `daemons-contract`, `skills-portability`, etc. |
| 6.3 | README / índices | Eliminar instrucciones `python SddIA/...` |
| 6.4 | Forja | Mutaciones vía `entity-manager` donde aplique |

**Criterio de salida:** O15; CA15; CA11 parcial (genoma operativo limpio).

---

## Ola 7 — Gate cero-Python + cierre

**Intent:** Verificación global O11/O12 y Done documental.

| # | Entregable | Detalle |
|---|------------|---------|
| 7.1 | Gate ficheros | `find . -name '*.py'` excl. `.venv/`, `.tools/` → vacío |
| 7.2 | Gate referencias | `rg` referencias operativas `.py` / `python3` → vacío |
| 7.3 | Suite completa | `cargo test` workspace + smokes EDA |
| 7.4 | `implementation.md` / `execution.md` | Evidencias olas 4–7 |
| 7.5 | `validacion.md` | `global: APTO`, `pbi_archived: true` |
| 7.6 | PBI → `docs/todos/done/` | Mismo PR |
| 7.7 | `SddIA/evolution/` | Registro olas 4–7 |

**Delegates_to:** `agent:argos`, `delivery-close-cycle`

**Criterio de salida:** O10–O12; CA10–CA17; Done PBI v2.

---

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Regresión al eliminar cores QA | Migrar tests antes de borrar; golden Rust |
| Hooks rotos para contribuidores | Documentar nueva aduana en README; rotura asumida |
| DEBT-K2 IOTA complejo | Priorizar simulate path + crate Rust existente |
| CI externa invoca `.py` | Inventario + comunicación rotura; no shim |

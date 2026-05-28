---
feature_name: inmunidad-caos-fase1
created: "2026-05-28"
process: feature
items_applied:
  - "[OPERATIVO] 1.A execution-contexts.md §2.9 chaos-engineering"
  - "[OPERATIVO] 1.B tools-contract.md v1.3.0 §6 termodinámica"
  - "[OPERATIVO] 1.C chaos_workspace_utils.py + touchpoints-ia.md Inocuidad"
  - "[OPERATIVO] 1.D.1 io-choke spec + cápsula"
  - "[OPERATIVO] 1.D.2 schema-corruptor spec + cápsula"
  - "[OPERATIVO] 1.D.3 sandbox-breacher spec + cápsula"
  - "[OPERATIVO] 1.E tools/index.md + test_chaos_tools.py (7 tests OK)"
---

# Ejecución — Fase 1

## [OPERATIVO] 1.A — Contexto RBAC

| Touchpoint | Cambio |
|------------|--------|
| `SddIA/norms/execution-contexts.md` | `version: 1.1.0`; §2.9 `chaos-engineering` |

## [OPERATIVO] 1.B — Contrato tools

| Touchpoint | Cambio |
|------------|--------|
| `SddIA/tools/tools-contract.md` | `contract_version: 1.3.0`; §6 termodinámica; §7 historial |

## [OPERATIVO] 1.C — Inocuidad runtime

| Touchpoint | Cambio |
|------------|--------|
| `SddIA/scripts/qa/chaos_workspace_utils.py` | `assert_workspace_bound` |
| `SddIA/norms/touchpoints-ia.md` | Principio 3 Inocuidad |

## [OPERATIVO] 1.D — Tools ofensivas

| Tool | Spec | Cápsula |
|------|------|---------|
| `io-choke` | `SddIA/tools/io-choke.md` | `scripts/tools/io-choke/io_choke.py` |
| `schema-corruptor` | `SddIA/tools/schema-corruptor.md` | `scripts/tools/schema-corruptor/schema_corruptor.py` |
| `sandbox-breacher` | `SddIA/tools/sandbox-breacher.md` | `scripts/tools/sandbox-breacher/sandbox_breacher.py` |

## [OPERATIVO] 1.E — Catálogo y regresión

| Touchpoint | Cambio |
|------------|--------|
| `SddIA/tools/index.md` | +3 filas `chaos-engineering` |
| `SddIA/scripts/qa/test_chaos_tools.py` | 7 tests OK |

## Pendiente

- Argos → `validacion.md` APTO
- PR / `delivery-close-cycle`

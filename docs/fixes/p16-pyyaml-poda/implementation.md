---
feature_name: p16-pyyaml-poda
created: "2026-07-10"
process: bug-fix
items:
  - id: A1
    artifact: requirements.txt
    nature: dependency-manifest
    operation: audit-only
    outcome: mantener PyYAML
  - id: A2
    artifact: SddIA/scripts/qa/*.py
    nature: qa-scripts
    operation: grep-audit
    outcome: 7 directos + 7 indirectos documentados
  - id: A3
    artifact: docs/fixes/p16-pyyaml-poda/
    nature: fix-documentation
    operation: create
    outcome: spec + implementation + execution + validacion
---

# Implementación — P16 poda PyYAML

## Inventario consumidores directos (`import yaml`)

| Archivo | Rol |
|---------|-----|
| `execute_process_core.py` | Parser frontmatter YAML (SSOT legacy bridge) |
| `execute_process_capsules.py` | Motor fases + bridge residual |
| `execute-action.py` | Resolución aliases acción (red seguridad) |
| `verify-process-integrity.py` | Gate CI integridad genoma |
| `verify-task-closure.py` | Gate cierre documental |
| `recalc-process-hash-signatures.py` | Mantenimiento hash process |
| `audit-doc-parity.py` | Paridad documental features/fixes |

## Consumidores indirectos (`parse_frontmatter` → PyYAML)

| Archivo | Rol |
|---------|-----|
| `_execute_process_route_bridge.py` | Bridge EDA route (P17 pendiente) |
| `_execute_process_capsules_bridge.py` | Bridge procesos residuales (P17 pendiente) |
| `execute_process_forges.py` | Forjas entity-manager legacy |
| `governance_daemon_manager_core.py` | Runtime centinelas |
| `audit-entity-eda-coverage.py` | Gate EDA genómico |
| `daemon_heartbeat_audit_core.py` | Audit heartbeat |
| `daemon_centinel_runtime.py` | Runtime centinela |
| `eda_bus_utils.py` | Utilidades bus EDA |

## Touchpoints productivos (entrypoint)

| Touchpoint | PyYAML |
|------------|--------|
| `SddIA/target/*/execute-process` (Rust) | ❌ No |
| `orchestrator_resolve.py` | ❌ No |
| `sddia-run.sh` | ❌ No (delega binario) |

## Propuesta

**No mutar** `requirements.txt`. Registrar justificación en `execution.md`. La eliminación de PyYAML es gate de los FIX hijos de porte bridges (post-P17).

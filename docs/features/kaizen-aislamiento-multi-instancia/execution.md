---
feature_name: kaizen-aislamiento-multi-instancia
created: "2026-08-26"
process: feature
branch_name: feat/kaizen-aislamiento-multi-instancia
persist_ref: docs/features/kaizen-aislamiento-multi-instancia
items_applied:
  - systemd_percent_f
  - instance_root_resolver
  - lock_pid_stop
agents: tekton
execution_id: "3b40b62c-d048-4896-b8c1-1ee267ca7704"
---

# Execution — kaizen-aislamiento-multi-instancia

## Unit / smoke

| Suite | Resultado |
|-------|-----------|
| `engine::handlers::instance_creator` (3) | OK |
| `SddIA/scripts/qa/test-instance-root-resolver.sh` | OK |

## Motor vs ensayo host

| Criterio | Estado |
|----------|--------|
| Plantillas `%f`, 0 `@@SDDIA_CORE_ROOT@@` | APTO |
| 0 `pkill -x` operativo en wrappers | APTO |
| Resolver env/cwd/fallback | APTO |
| Ensayo forja + `SddIA_AP` | F-SYS-02 user **APTO** (`@AP` ExecStart bajo AP). Disco AP ausente; WUI down → **NO APTO** PBI completo. |

## Residual

Cerrado: `entity-manager` instance-creator v1.3.0; norma Core `sddia-distribution-protocol` v1.2.2 bajo feature activa.

Abierto: `delivery-close-cycle` / PR (no invocado). Residual disco: `/home/racso/Proyectos/SddIA_ISO_B` (topology instance-creator; no enable).

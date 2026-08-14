---
feature_name: kalma2-phase-barrier-timeout-persist
created: "2026-08-14"
process: refactorization
base: main
scope: kalma2-phase-barrier-timeout-persist
branch_name: refactor/kalma2-phase-barrier-timeout-persist
persist_ref: docs/features/kalma2-phase-barrier-timeout-persist
pbi_ref: docs/todos/done/[REFACTOR] Kalma2 — serialización de fases, timeout y rama refactor (KALMA2-AUD-4b9de6).md
document_id: 1de0bdd1-6144-4e45-8efa-92db0f399147
version_spec: "1.0.0"
status: dedalo_locked
laudo: L-BARRIER
agents: dedalo
execution_id: d630a6cf-1767-4751-a2b9-b1f4210a01fb
---

# Especificación — kalma2-phase-barrier-timeout-persist

## 1. Misión técnica

Cerrar KALMA2-AUD-4b9de6-001..005 en el orquestador nativo y en la prótesis Cursor: serializar fases agente, tratar timeout como fallo de fase, conservar `refactor/`, inyectar `persist_ref` al hijo TQM.

## 2. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-BARRIER** | Barrera en `executor::run_generic` (procesos `feature` / `bug-fix` / `refactorization`). Tras fase solo-`agent:` con status ∈ {`failed`,`blocked`,`awaiting_agents`,`awaiting`}, las fases restantes agente + Verificación + Cierre se marcan `skipped` (`reason: prior_agent_phase_not_executed`). `simulated` no dispara. |
| **L-TIMEOUT** | Runtime Python: `"timeout"` **fuera** de allowlist soft. `TimeoutExpired` → `failed`, `success=false`. Soft config intacta (CLI ausente / 401 / auth). |
| **L-TIMEOUT-ENV** | `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` default `600`. `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION` override si `phase_name` empieza por `ejecuc`. |
| **L-PREFIX** | `workspace-init`: si `branch_name` ya tiene prefijo de trabajo (`feat/`/`feature/`/`fix/`/`refactor/`), no reescribir. Default `refactorization` → `refactor/{task}`. Extraer slug de `refactor/` en `workspace_task_name`. TQM: default `refactor/{slug}` (no `feat/{slug}`). |
| **L-PERSIST** | TQM `build_child_inputs` inserta `persist_ref`: FM `persist_ref_suggested` o `persist_ref`; fallback Cúmulo `paths.featurePath`/`fixPath` + slug. `agent_runtime` no emite Null si `inputs` o `state.workspace.persist_ref` tienen valor. Prompt: fallback a `inputs.persist_ref`. |
| **L-SKIP-LAB** | `SDDIA_LAB_SKIP_PBI_ARCHIVE` / `SDDIA_LAB_SKIP_DELIVERY_CLOSE` también aplican a `refactorization`. |
| **L-GENOME** | No mutar `refactorization.md`. `git-operations.md`: alineación de ejemplo de prefijos **solo** vía `entity-manager` (fuera del bisturí engine/scripts). |

## 3. Touchpoints

| Módulo | Cambio |
|--------|--------|
| `SddIA/engine/execute-process/src/engine/executor.rs` | Barrera de fase + skip lab refactorization |
| `SddIA/engine/execute-process/src/engine/workspace_init.rs` | Conservar `refactor/`; default prefix |
| `SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs` | `persist_ref` + default `refactor/` |
| `SddIA/engine/execute-process/src/engine/agent_runtime.rs` | Resolver `persist_ref` no vacío |
| `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs` | `infer_persist_ref_from_branch`: `refactor/` → `featurePath` |
| `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py` | Timeout terminal; env por fase; prompt persist |

`phase_terminal`: `awaiting_agents` permanece **neutral** en agregación global. El corte de Argos es la barrera, no recualificar awaiting como failed (salvo timeout en runtime, que ya emite `failed`).

## 4. Contratos de comportamiento

### 4.1 Barrera

```text
for phase in phases:
  if barrier_armed && (agent_only || verify_or_close):
    report skipped (handler: phase-barrier)
    continue
  execute
  if agent_only && status in {failed, blocked, awaiting_agents, awaiting}:
    barrier_armed = true
```

Hijo TQM: Ejecución `failed` (timeout) → Verificación `skipped` → `aggregate_execution_terminal` → `success=false` (fase failed causal). Argos no se invoca.

### 4.2 Timeout

Allowlist soft **sin** `"timeout"`:

`no encontrado` | `not found` | `no instalado` | `api_key` | `401` | `auth`

Si el mensaje contiene `timeout` → nunca soft, aunque coincida otro marker.

### 4.3 persist_ref

Orden: `persist_ref_suggested` FM → `persist_ref` FM → `{featurePath\|fixPath}/{slug}` vía Cúmulo. Path con `..` se descarta.

## 5. Tests

| Caso | Aserción |
|------|----------|
| TQM refactor PBI con `suggested_branch: refactor/foo` | `branch_name=refactor/foo`; `persist_ref` no vacío |
| TQM refactor sin FM persist | `persist_ref` = `{featurePath}/{slug}` |
| workspace-init `branch_name=refactor/x` | output `refactor/x` (no `feat/x`) |
| executor: Ejecución `failed` | Verificación `skipped`; sin handler agent-runtime en esa fase |
| Python `is_soft_config_error("timeout 600s")` | `False` |
| Python CLI ausente | sigue soft (salvo `REQUIRE_CLI`) |

## 6. Fuera de alcance

Evolution EV-AUD-002/007. Full-cycle TQM. UX WUI. Mutación de process Core.

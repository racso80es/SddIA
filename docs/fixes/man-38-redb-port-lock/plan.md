---

feature_name: man-38-redb-port-lock
created: "2026-11-08"
process: bug-fix
phases:
  - id: T1
    title: Helper scoped + eda_port sin cache productiva
  - id: T2
    title: Quitar bootstrap incondicional CLI y run_process
  - id: T3
    title: event-watcher claim/ACK scopes separados
  - id: T4
    title: Auditoría Arc productivo + excepciones documentadas
  - id: T5
    title: Docs lifetime + telemetría + verificación CA
pbi_ref: MAN-38
document_id: PBI-90397f23
branch_name: fix/man-38-redb-port-lock
persist_ref: docs/fixes/man-38-redb-port-lock
target_executor: tekton
status: implementation-done
execution_id: "be63c70f-633c-452e-aaf1-8ffa645fd54b"
---
# Plan

Descomposición de implementación (no blueprint de proceso). Un commit por tarea vía `skill:git-manager`.
Prohibido: TODOs bajo `docs/todos/` (Cumulo / `Kaizen_Alert_Required` únicamente).

| Id | Tarea | Hecho |
|----|--------|--------|
| T1 | En `sddia-event-manager-host` (+ `eda_port`): `with_productive_port(repo, op_label, f)`; wrappers EDA usan scoped productivo; `install_port` mock intacto; unit CA-6 (productivo no queda en thread-local tras op). Conservar retry `SDDIA_EVENT_MANAGER_OPEN_BUDGET_MS`. | C2 | done |
| T2 | Eliminar `bootstrap_composition_root` incondicional de `main.rs` (`run_main` / tool / action) y de `engine::run_process`. Flags EDA CLI pasan por helper scoped. | C1 | done |
| T3 | `event-watcher`: sin Arc de larga duración; claim scoped → drop → spawn ruta sin Arc → ACK/NACK scoped. Tests integración Redb (CA-3/CA-5). | C3 | done |
| T4 | Inventario callers `resolve_productive_port` / `port_arc` / `bootstrap_composition_root` / `install_productive_port` en daemons/tools; migrar a scoped o documentar excepción con prueba+presupuesto en persist. | C4 | done |
| T5 | `port-lifetime.md` (ownership); telemetría contención (`process`, `operation`, `wait_ms`, `budget_ms`, `outcome`); lab CA-1/CA-2/CA-4; `implementation.md`/`execution.md`; gate CA-7 (sin reabrir MAN-35). | C5 + CA-8 | done (docs + tests unit) |

## Orden

1. T1 (cimiento scoped) → T2 (dejar de instalar al arrancar) → T3 (watcher).
2. T4 (barrido residual) → T5 (docs + evidencia CA).

No subir el presupuesto de open como remedio. No tocar contrato 1.2 ni reintroducir FS/`EventStore` en consumidores.

## Verificación mínima (Tekton)

```bash
cd SddIA && cargo test -p sddia-event-manager-host --lib
cd SddIA && cargo test -p execute-process eda_port --lib -- --test-threads=1
cd SddIA && cargo test -p event-watcher -- --test-threads=1
```

Lab manual / script: PPR largo en vuelo + watcher + `forge-pbi` (CA-4); `lsof`/`fuser` o equivalente sobre `.SddIA/runtime/events.redb` durante fase no-EDA del PPR (CA-1/CA-2).

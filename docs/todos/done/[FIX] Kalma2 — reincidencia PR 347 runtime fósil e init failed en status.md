---
document_id: PBI-FIX-KALMA2-REINCIDENCE-STALE-RUNTIME-AND-FAILED-PROJECTION
uuid: "e3b27269-6736-4735-ba6f-fd94bb87d1c2"
title: "[FIX] Kalma2 — reincidencia PR #347: runtime fósil, init failed en status y stream PTC"
format: markdown
version: "1.0.0"
created: "2026-10-14"
status: done
priority: alta
process: bug-fix
dispatch: true
suggested_branch: fix/kalma2-reincidence-stale-runtime-and-failed-projection
persist_ref: docs/fixes/kalma2-reincidence-stale-runtime-and-failed-projection
source_audit: docs/audits/kalma2-wui-hu-b07-core-self-20261014.md
prior_audit: docs/audits/kalma2-pbi-telegram-pec-silence-20261013.md
reopened_from: PBI-FIX-KALMA2-BRANCH-AND-STATUS
reopened_from_pr: "https://github.com/racso80es/SddIA/pull/347"
incident_correlation_id: "fd2db080-dbf3-44ab-b70f-63b90d748455"
friction_ids:
  - F-STALE-BINARY-POST-MERGE
  - F-DEBUG-FIRST-ORCHESTRATOR-SPLIT
  - F-PHASE-END-IGNORES-FAILED
  - F-DIRTY-CLEAN-NEW-BRANCH-STRANDED
  - F-PROGRESS-SSE-ZERO-BYTES
related:
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/engine/execute-process/src/engine/invoke_orchestrator.rs
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/progress_trace.rs
  - SddIA/engine/execute-process/src/engine/accept_pr.rs
  - interfaces/kalma2/app.js
  - start-sddia.sh
out_of_scope:
  - "No rediseñar la WUI."
  - "No relajar el gate dirty-worktree."
  - "No implementar aquí el PBI HU-B 07 Core-self (PBI-LINEAR-B-07-CORE-SELF-GATE)."
---

# [FIX] Kalma2 — reincidencia PR #347: runtime fósil, init failed en status y stream PTC

## 1. Contexto

El PBI `PBI-FIX-KALMA2-BRANCH-AND-STATUS` se cerró en el PR #347 (18:36 CEST del 2026-10-14). A las 19:43 CEST, al despachar `PBI-LINEAR-B-07-CORE-SELF-GATE` desde Kalma2 WUI (`correlation_id` `fd2db080-dbf3-44ab-b70f-63b90d748455`), se repitieron cinco de las seis fricciones de la auditoría del 13 de octubre: la inicialización falló, la rama `feat/…` no nació, Mayeuta, Dedalo y Tekton mutaron la rama que ya estaba en checkout (`docs/cerrar-pbi-kalma2-branch-before-mutate`), la caja Respuesta quedó congelada en `awaiting_agents` y la consola de progreso quedó en 0 líneas.

Evidencia: `docs/audits/kalma2-wui-hu-b07-core-self-20261014.md`.

## 2. Por qué reincidió un fix ya mergeado

### 2.1. F-STALE-BINARY-POST-MERGE / F-DEBUG-FIRST-ORCHESTRATOR-SPLIT

El código de PR #347 está en el árbol fuente, pero no en los binarios que atendieron el ciclo:

| Binario | mtime (CEST) | Merge PR #347 | Símbolos PR #347 (`strings`) |
|---------|--------------|---------------|------------------------------|
| `SddIA/target/debug/execute-process` | 18:02:09 | 18:36:39 | **Ausentes** (`workspace_init_failed`, `short_verdict`, `delete_new_feature_before_recreate`) |
| `SddIA/target/release/execute-process` | 18:01:32 | 18:36:39 | **Ausentes** |
| `SddIA/target/release/kalma2-bridge` (pid 15196) | 18:00:57 | 18:36:39 | **Ausentes** (`latest_phase_end`, `phase_end_projection`) |

`accept-pr` fusiona a `main` y empuja, pero no recompila `execute-process` ni `kalma2-bridge` ni recicla `sddia-kalma2-bridge@.service`. Además `resolve_orchestrator` en `kalma2-bridge` (`main.rs:250`), `invoke_orchestrator.rs:14`, `telegram-watcher` y `github_bridge.rs` eligen `target/debug/execute-process` antes que `target/release/execute-process` sin comprobar si el ELF está por detrás del código fuente o del testigo `.sha256`.

### 2.2. F-PHASE-END-IGNORES-FAILED (hueco en el código de PR #347)

Aunque `kalma2-bridge` se recompile hoy, `phase_end_from_trace` (`SddIA/interfaces/kalma2-bridge/src/main.rs:868-874`) descarta `phase_status: "failed"`:

```text
if status != "executed" && status != "blocked" {
    return None;
}
```

La inicialización fallida emite `moment: "end"`, `phase_status: "failed"`. `latest_phase_end` la ignora. Si el barrier de PR #347 salta el resto de fases y el PEC temprano (`awaiting_agents` / `success`) ya se escribió antes del hijo, `GET /api/status` sigue respondiendo `awaiting_agents` en vez de `failed` con la causa de `workspace-init`.

### 2.3. F-DIRTY-CLEAN-NEW-BRANCH-STRANDED (hueco en el código de PR #347)

En `workspace_init.rs:509-524`, el código nuevo hace `checkout_work_branch` (creando la rama desde el HEAD sucio actual) y luego evalúa `dirty_paths_outside_scope`. Si hay suciedad ajena y retorna `Err("dirty-worktree: …")`, no vuelve a la rama previa ni borra la rama recién creada cuando `!branch_existed`. El árbol queda cambiado a una rama `feat/…` o `fix/…` nacida del HEAD equivocado (en la auditoría era `docs/cerrar-pbi-…`, 1 commit detrás de `origin/main`).

### 2.4. F-PROGRESS-SSE-ZERO-BYTES

`/api/progress/stream?correlation_id=…` no entregó frames en 2 s pese a existir 7 hojas en `.events/progress/fd2db080-…/`, y la consola de progreso quedó vacía durante 8 minutos.

## 3. Criterios de aceptación

| ID | Criterio |
|----|----------|
| R-CA1 | `phase_end_from_trace` y `phase_end_projection` en `kalma2-bridge` aceptan `phase_status == "failed"` y proyectan `status: "failed"` con el `verdict`/`error` de la fase (incluyendo `Inicialización de Espacio de Trabajo`), pisando cualquier PEC temprano `awaiting_agents`. |
| R-CA2 | En `workspace_init.rs`, si `dirty_paths_outside_scope` falla tras `checkout_work_branch`, restaurar la rama previa (`prior_branch`) y, si `!branch_existed`, borrar la rama recién creada antes de devolver `Err("dirty-worktree: …")`. |
| R-CA3 | `resolve_orchestrator` (`kalma2-bridge`, `invoke_orchestrator`, watchers) no selecciona un `target/debug/execute-process` más antiguo que `target/release/execute-process`, y en `start-sddia.sh` se sellan también los testigos `.sha256` de `execute-process` y `kalma2-bridge` (hoy solo sellados el 28-09). |
| R-CA4 | Diagnosticar y corregir el bloqueo de entrega en `/api/progress/stream` (`ProgressStreamReader` / `tiny_http` worker pool cuando coexiste con `/api/telemetry/stream`) para que el replay inicial de `.events/progress/{cid}/` llegue en `< 1 s`. |
| R-CA5 | Recompilar `execute-process` (release + debug) y `kalma2-bridge` (release), reciclar el proceso vivo del puente y verificar con `strings` / testigo `.sha256` que el runtime activo contiene los símbolos de PR #347 + este fix. |
| R-CA6 | Tests unitarios en `kalma2-bridge` (`phase_status: "failed"` sobre PEC `awaiting_agents`, replay SSE) y en `execute-process` (rollback de rama nueva cuando `dirty-worktree` aborta). |

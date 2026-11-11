---
feature_name: man-43-event-manager-desacople-total
created: "2026-11-11"
updated: "2026-11-11"
process: feature
branch_name: feat/man-43-event-manager-desacople-total
persist_ref: docs/features/man-43-event-manager-desacople-total
pbi_ref: MAN-43
parent_ref: MAN-40
document_id: PBI-0f4f0f5f
agent: tekton
phase: ejecucion
status: partial
verdict: blocked
execution_id: "3278ec5b-21ae-4fe0-af14-62a7e5ba0e07"
seed: continue-tekton
norm_ref: features-documentation-pattern@1.3.1
---
# Execution — man-43-event-manager-desacople-total

## Veredicto

**`blocked`** (parcial materializado): H0 documentado + H1 núcleo `EventUri`/`PublishSeal` v2/`2.0.0` forjado vía MCP; **no** se puede cerrar AC ni declarar verde sin (a) evidencia `cargo`/suite/gates y (b) `forge_key` legítimo / allowlist MCP.

## Qué se hizo

1. Leídos PBI refine **PBI-0f4f0f5f**, canal `MAN-43`, `spec.md`/`plan.md` v1.0.1, puerto vivo 1.2.
2. Inventario H0.1 cerrado (motor/daemons/bridges/tools/lab).
3. Materializado contrato **2.0.0**: `EventFamily`, `EventUri`, `PublishSeal { event_id, event_uri }`, `list_terminal_events`, `list_deliveries` en mock, gate/host/factory a `^2.0.0`.
4. Propagados callers del sello (actions, domain_mutation, event_store_query, daemon_heartbeat, eda_port_legacy, event-bus-audit, iota_ci_smoke).
5. Docs forja: `implementation.md` + este `execution.md`.

## Bloqueos explícitos (no inventar éxito)

| Bloqueo | Detalle |
|---------|---------|
| Cerbero MCP | `.SddIA/project.md` **sin** `mcp.allowed_executables` → `run_check` deniega `cargo`/`./sddia-run.sh` |
| forge_key | Ausente para `tracker_ref: MAN-43`; no se fabricó ledger fuera de `forge-pbi` |
| H1.5 Genoma | Schema/bindings/taxonomía v2 requieren `entity-manager` (L-GENOME) |
| H2–H6 | No iniciados (FS topology, watcher, periféricos, demolición, APTO) |
| Git | Sin commit; git-manager no invocable vía MCP allowlist vacía; `git_status` MCP muestra docs feature untracked |

## Evidencia git (MCP)

`git_status` (aduana): untracked `docs/audits/MAN-43.md` + `docs/features/man-43-event-manager-desacople-total/` (más cambios de código no reflejados aquí si el índice va por detrás — verificar en siguiente pasada con allowlist).

## Siguiente paso recomendado

1. Declarar `mcp.allowed_executables` (p.ej. `cargo`, `./sddia-run.sh`) en manifiesto instancia.
2. Sellado `forge-pbi` / ledger MAN-43 si el runtime lo exige.
3. `cargo test -p sddia-core` + contract suite Redb + corregir fallos.
4. Continuar H2 (absorber testigos FS) sin dualidad productiva.

## Paths relativos materializados

- `docs/features/man-43-event-manager-desacople-total/implementation.md`
- `docs/features/man-43-event-manager-desacople-total/execution.md`
- `SddIA/sddia-core/src/ports/event_manager/*` (contrato v2)
- Callers motor/tools/host listados en `implementation.md`

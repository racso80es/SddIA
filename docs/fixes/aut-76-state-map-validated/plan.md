---
feature_name: aut-76-state-map-validated
created: "2026-11-06"
process: bug-fix
phase: blueprint
agents: dedalo
phases: T0-T7
branch_name: fix/aut-76-state-map-validated
sighting_branch: fix/aut-76-state-map-validated
persist_ref: docs/fixes/aut-76-state-map-validated
pbi_ref: AUT-76
document_id: PBI-222b4def
uuid: 222b4def-325a-4645-80d3-fbb15bf7d34b
execution_id: "6c28414f-8a36-4f66-b9e3-4969dc642978"
---

# Plan — aut-76-state-map-validated

Parada actual (**L0**): `spec.md`, `plan.md`. Commit de planificación antes de mutar código. Rama: `fix/aut-76-state-map-validated`.

`bug-fix` / `workspace-init` no arrancan: Domain Authority es el defecto. T0 es el salto táctico. T1 recompila el motor **antes** de reinyectar orquestador.

## T0 — Sellar planificación (esta entrega)

| Paso | Acción |
| --- | --- |
| T0.1 | Rama `fix/aut-76-state-map-validated` desde `main`. Frontmatter coherente (`pbi_ref`, `document_id`, `uuid`, `execution_id`). |
| T0.2 | `spec.md` + `plan.md` bajo `persist_ref`. Prohibido `implementation.md` en este commit. |
| T0.3 | Commit: `docs(AUT-76): planificación state_map.validated en aduana` |

## T1 — Aduana de manifiesto

1. Añadir `"validated"` a `TRACKER_STATE_KEYS` en `SddIA/engine/execute-process/src/engine/project_binding.rs`, entre `in_validation` e `in_review`.
2. Test: manifiesto con `validated: Validated` pasa `bind`. Clave desconocida sigue fallando.
3. `cargo test -p execute-process --lib` sobre el módulo. Recompilar binario usado por `./sddia-run.sh`.

## T2 — Anti-retroceso

1. `state_rank` en `tracker_sync_replay.rs`: `validated` → 4; `in_review` → 5; `done` → 6.
2. Ajustar `state_rank_orders_cycle` y transiciones: `in_validation`→`validated` avanza; `validated`→`in_validation` se descarta; `validated`→`in_review` avanza; `in_review`→`validated` se descarta.

## T3 — Auditoría §3.4

1. Insertar fila rango 4 `validated` / Validated (Argos APTO).
2. Correr `in_review` a 5 y `done` a 6.
3. Anti-retroceso: si está en `validated`, un `in_validation` rezagado se descarta.

## T4 — Contrato (tras T1)

1. `./sddia-run.sh --process entity-manager` update de `project-config-contract`: conjunto de claves + bump SemVer.
2. Prohibido editar el `.md` del contrato a mano.

## T5 — Evolution

1. Alta canónica en `SddIA/evolution/` + fila en `Evolution_log.md`. Ligar `document_id` / estímulo AUT-76.

## T6 — Verificación de desbloqueo

1. `./sddia-run.sh --process refine-item --inputs '{"item_ref":"AUT-76","project_slug":"core"}'`.
2. CA: el acuse **no** contiene `clave desconocida 'validated'`. Mayeuta / DoR de AUT-76 quedan fuera.

## T7 — Cierre documental

1. `implementation.md` + `execution.md` (un commit por tarea T1–T6).
2. `validacion.md` APTO. Un PR. Sin `docs/todos/`.

---
feature_name: kaizen-ciclo-jurisdiccion-todos
created: "2026-08-29"
process: feature
purpose: Estabilización Mayeuta — siete fricciones de ciclo post-PR #219
version_clarify: "1.0.0"
execution_id: "1550128b-c2ef-4c4d-8cbb-181a15a66940"
pbi_ref: docs/todos/pending/[KAIZEN] Ciclo jurisdicción todos — norm-creator parcial, huérfanos EDA y colapso DCC sin fractura.md
document_id: PBI-KAIZEN-CICLO-JURISDICCION-TODOS
---

# Clarificación — kaizen-ciclo-jurisdiccion-todos

Transcript Mayeuta. Semilla: PBI `PBI-KAIZEN-CICLO-JURISDICCION-TODOS` v1.1.0. Init lab `execution_id` `1550128b-c2ef-4c4d-8cbb-181a15a66940`.

## D0 — Apertura

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 |
| `feature_name` | `kaizen-ciclo-jurisdiccion-todos` |
| Rama | `feat/kaizen-ciclo-jurisdiccion-todos` |
| `persist_ref` | `docs/features/kaizen-ciclo-jurisdiccion-todos` |
| Origen | Fricción de ciclo de `PBI-OPER-DEUDA-TECNICA-KINTSUGI-001` (PR #219). No reabre su alcance. |

## D1 — Forge: dos defectos, un creador

`entity-manager` ya inyecta `tactical_norm_dependencies` (`entity_manager.rs:229`). `run_norm_forge` no lo lee y no emite `## Restricciones Duras`. No hay semilla de restricciones: introducir `tactical_norm_hard_constraints` (string; vacío → bloque presente con lista vacía o «Ninguna.»). `tactical_norm_friction` permanece como Directriz Core.

Contrato destino: `norms-contract` v1.1.0, no v1.0.0 del default EM.

## D2 — Re-forja `todos-jurisdiction`

Solo tras L1. `lifecycle_operation: update`, `tactical_norm_version: 1.1.0`, uuid inmutable `f0b8ce4a-2f79-4516-bee0-acfe0d25bd58`. Dependencia: `4c448c82-de41-460f-b24f-82a84fa5ed69`. Prohibido `Write` sobre `SddIA/library/norms/`.

## D3 — CA3 vs CA3b

Huérfanos EDA **saldados** (2026-08-28). Alcance vivo: quitar `sha256:pending-forge` de `github-raw-fetcher` y `download-remote-asset` vía `entity-manager` update (no ampliar `kalma2-mvp-sync-activos`).

`delivery-close-cycle.md` § Notas ya cita `backfill-manifest.json` + `correlation_id`. Falta el predicado real: `merkle_anchored != true` y ruido `backfill Fase C en curso`. Completar, no inventar segunda vía.

## D4 — Fractura en DCC

Paridad con `emit_workspace_init_fracture`. Toda fase DCC `blocked`/`failed` deposita `System_Fracture_Detected` con `friction_id` propio. Idempotencia: `friction_id` + `process_name` (precedente fan-out). No inundar el bus.

## D5 — Evolution `eda-coverage.json`

Preferir **exención explícita** en `gate-evolution` del path `SddIA/core/eda-coverage.json` cuando el mutador es el motor (`emit-domain-mutation`). Auto-inyectar `relacionado` es más frágil (el registro lo declara el operador). Round-trip `evolution-rehash` queda prohibido como paso obligatorio.

## D6 — `.gitignore`

Sustituir o complementar `/.tmp` con `**/.tmp/`. Conservar `/.tmp` raíz si hay consumidores. No versionar `{persist_ref}/.tmp/pr-body.md`.

## D7 — Colapso mudo

`obediencia-procesos.md` es norma **motor** (`directories.norms`): no hay `entity_class` para ella. Parche bajo topología feature (DA-4) + registro evolution. Cláusula: fallo oficial sin evento → emitir vía canónica (o proceso que lo haga) y **detener**; prohibido `git`/`gh`/`curl` raw.

## D8 — Fuera de alcance

- Reabrir PR #219 / rehacer su merge.
- Ampliar granularidad del gate EDA a «solo diff del PR» (el PBI documenta el dolor; no pide rediseñar el scan).
- Re-forjar las 9 normas ya conformes.

---
feature_name: centinelas-fracture-ola-20260812
created: "2026-08-16"
process: bug-fix
base: main
branch: fix/centinelas-fracture-ola-20260812
uuid: e4b8c2a1-7d3f-4a96-9c5e-2f8b1d0a6e47
scope: laudo-no-regresion-archivo-pbi-ola
consolidated_pbis:
  - PBI-FIX-FRACTURE-d0fb9b49071f
  - PBI-FIX-FRACTURE-28c5228720ea
  - PBI-FIX-FRACTURE-d3fa640e468b
  - PBI-FIX-FRACTURE-655099e956f1
verdict: B-documentary-debt
segregated:
  - EV-AUD-003 → fix/process-creator-full-contract-forge
related_fixes:
  - docs/fixes/centinelas-fracture-ola-20260716
  - docs/fixes/centinelas-kalma2-fracture-ola-20260722
  - docs/fixes/centinelas-fracture-ola-20260723
  - docs/fixes/daemon-heartbeat-ingest-ignition
  - docs/features/heartbeat-circuit-regimen-20260811
  - docs/fixes/kaizen-regex-lookahead-panic
---

# Spec — Ola fracturas centinelas (2026-08-12)

## Decisión

Un solo `bug-fix` consolida 4 PBIs `System_Fracture_Detected` satélites (emisor `argos` / `daemon-heartbeat-audit`):

| document_id | Proceso | Traza (ciclos / last_heartbeat) |
|-------------|---------|----------------------------------|
| PBI-FIX-FRACTURE-d0fb9b49071f | event-sweeper | 1580 @ 2026-08-12T16:19:15Z |
| PBI-FIX-FRACTURE-28c5228720ea | event-watcher | 3070 @ 2026-08-13T06:59:11Z |
| PBI-FIX-FRACTURE-d3fa640e468b | github-bridge-watcher | 790 @ 2026-08-12T16:19:14Z |
| PBI-FIX-FRACTURE-655099e956f1 | telegram-watcher | 1581 @ 2026-08-12T16:18:46Z |

**Laudo: (B) deuda documental** (Nodo de Control 2026-08-16) — fracturas históricas; runtime sano al momento del diseño; **cero mutación genómica**. Prohibido alterar `missed_cycles`, keepalive o abrir ramas por centinela.

**EV-AUD-003** queda fuera de esta ola (rama sugerida `fix/process-creator-full-contract-forge`). Corrección de precisión: el hallazgo no es «fases dentro de `workspace_template`»; `run_process_forge` omite *por separado* `process_phases`, `inputs`, `outputs` y `workspace_template`, y escribe un stub `Fase inicial` cuyo hash no coincide con el artefacto. Sin colisión con el régimen de latidos.

`plan.md` **no** se emite: no hay blueprint de proceso nuevo; la ejecución es archivo PBI + verificación de no-regresión bajo el proceso `bug-fix` vigente.

## Discriminación A vs B (empírica — diseño)

| Hecho | Evidencia | Lectura |
|-------|-----------|---------|
| 4 centinelas vivos | Locks en `daemons_instance.status`; heartbeats `status: alive` (side-channel) @ 2026-08-16T15:58–15:59Z | Runtime sano |
| Heartbeats frescos | `daemons_instance.state/heartbeat-audit.json`: `missed_cycles=0` en los 4 | Sin síntoma activo |
| Mitigaciones ya en main | Circuito A+B+C+D (PR #168) + panic Kaizen look-ahead (PR #175) + olas 20260716/22/23 + ingest ignición PR #155 | Causa genómica de esta familia cerrada |
| `last_heartbeat` diverge por daemon | sweeper/github/telegram cortan a 2026-08-12T16:19Z; watcher sobrevive hasta 2026-08-13T06:59Z | Parada de proceso, no congelación de ingest (distinto a #168) |
| Amplificación watcher | `enrich-fracture-pbi-kaizen` panica look-ahead → `PoisonError` (cerrado PR #175) | Explica el satélite `28c5228720ea` |
| PBIs huérfanos | Ola 20260723 archivó satélites 2026-07-23…25; estos 4 `document_id` (trazas 2026-08-12…14) quedaron en `pending/` | Deuda de cierre documental |
| Segregación | PBI EV-AUD-003 (`process-creator` stub) en `pending/` con `suggested_branch: fix/process-creator-full-contract-forge` | Fuera de alcance de esta ola |

Las trazas satélite son snapshots de downtime/staleness **previos** a la recuperación observada el 2026-08-16. No demuestran causa raíz residual en genoma al laudo B.

## Cambios (Tekton)

1. **Verificar no-regresión** antes de archivar: evidencia viva en `daemons_instance` → `missed_cycles < 3` (esperado `0`) en obligatorios; opcionales coherentes con el entorno. **No** reescribir audit ni heartbeats.
2. **Archivar 4 PBI** `docs/todos/pending/` → `docs/todos/done/`, `status: cerrado`, `fix_ref: docs/fixes/centinelas-fracture-ola-20260812`.
3. Materializar `implementation.md` + `execution.md` bajo este `persist_ref`.
4. Evolution bajo `directories.evolution` vinculando `uuid` de esta spec (`e4b8c2a1-7d3f-4a96-9c5e-2f8b1d0a6e47`).
5. **No mutar** `start-sddia.sh`, daemons, `materialize-fracture-pbi`, keepalive, umbrales de `missed_cycles`, ni genoma protegido. Si el gate empírico tumba el laudo → pivot (A), detener archivo y escalar (fuera de este diseño).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Laudo (B) documentado con evidencia de audit fresco (`missed_cycles=0` o `<3`) en obligatorios al momento de ejecución |
| CA2 | Ignición verificada: 2/2 centinelas obligatorios vivos + heartbeats frescos; opcionales coherentes con el entorno |
| CA3 | Los 4 `document_id` consolidados en `docs/todos/done/` con `fix_ref` de esta ola |
| CA4 | `validacion.md` global `APTO`, `pbi_archived: true`, `branch: fix/centinelas-fracture-ola-20260812` |
| CA5 | Diff sin mutación genómica; EV-AUD-003 no tocado en esta rama |
| CA6 | Sin cambios a keepalive / umbrales / ramas por centinela |

## Límites

- Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`.
- Prohibido bypass raw (`gh`/`git`/`curl`) hasta cierre documentado.
- Tekton/Argos **no** escriben semillas Kaizen bajo `docs/todos/`; solo archivan los 4 PBI de esta ola.
- Cumulo topology: `directories.documentation` → `docs`; `daemons_instance.status|state` → `.SddIA/daemons/{status,state}`.
- Ceguera espacial: sin rutas host fuera de topología inyectada.

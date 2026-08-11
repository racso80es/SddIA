---
feature_name: centinelas-fracture-ola-20260723
created: "2026-08-11"
process: bug-fix
base: main
branch: fix/centinelas-fracture-ola-20260723
uuid: a7c3e91f-2b4d-4e8a-9f01-6d5c8b3a1742
scope: laudo-no-regresion-archivo-pbi-ola
consolidated_pbis:
  - PBI-FIX-FRACTURE-21f55bcdecfb
  - PBI-FIX-FRACTURE-0d65b4775574
  - PBI-FIX-FRACTURE-a69be9535f82
  - PBI-FIX-FRACTURE-131fa2c33271
  - PBI-FIX-FRACTURE-d67f6c0b0195
verdict: B-documentary-debt
related_fixes:
  - docs/fixes/centinelas-fracture-ola-20260716
  - docs/fixes/centinelas-kalma2-fracture-ola-20260722
  - docs/fixes/daemon-heartbeat-ingest-ignition
---

# Spec — Ola fracturas centinelas (2026-07-23)

## Decisión

Un solo `bug-fix` consolida 5 PBIs `System_Fracture_Detected` (emisor `argos` / `daemon-heartbeat-audit`).

**Laudo: (B) deuda documental** — fracturas históricas ya mitigadas en `main`; no hay causa raíz residual demostrable en runtime al 2026-08-11. Prohibido mutar genoma salvo que la verificación empírica de Tekton/Argos tumbe el laudo (entonces pivot a (A)).

`plan.md` no se emite: no hay blueprint de proceso nuevo; la ejecución es archivo PBI + verificación de no-regresión bajo el proceso `bug-fix` vigente.

## Discriminación A vs B (empírica)

| Hecho | Evidencia | Lectura |
|-------|-----------|---------|
| 4 centinelas vivos | Locks en `daemons_instance.status`: event-sweeper/watcher, github-bridge-watcher, telegram-watcher (PIDs desde 2026-08-10T15:18Z) | Runtime sano |
| Heartbeats frescos | `daemons_instance.state/heartbeat-audit.json` @ 2026-08-11T07:20Z: `missed_cycles=0` en los 4 | No hay síntoma activo |
| Mitigaciones ya en main | Keepalive + idempotencia materialize (ola 20260716); vault/cleanup/gate (ola 20260722); `_ingest_telemetry_heartbeats` + gate ignición (PR #155) | Causa operativa remediada |
| PBIs huérfanos | PR #155 archivó `d22645cea40c` / `bb5d18128823` / `da29db92ed52`; **no** estos 5 `document_id` (trazas 2026-07-23…25) | Deuda de cierre documental |
| Ausencia de ola posterior | Sin PBI fracture nuevos post-2026-07-25 hasta arranque de esta rama | No-regresión latente no observada |

Las trazas satélite (469 / 234 / 18 / 17 ciclos; `last_heartbeat` 2026-07-23…25) son snapshots de downtime/staleness **previos**. Tras archivo de PBI hermanos en PR #155, `materialize-fracture-pbi` legítimamente abrió nuevos `document_id` (hash de traza distinta) que quedaron en `pending/` sin ola de cierre.

## Cambios (Tekton)

1. **Verificar no-regresión** antes de archivar: `./start-sddia.sh` (o evidencia equivalente ya viva) → 2/2 obligatorios + heartbeats auditados frescos (`missed_cycles < 3` obligatorios; opcionales vivos si el entorno los tiene).
2. **Archivar 5 PBI** `docs/todos/pending/` → `docs/todos/done/`, `status: cerrado`, `fix_ref: docs/fixes/centinelas-fracture-ola-20260723`.
3. Materializar `implementation.md` + `execution.md` bajo este `persist_ref`.
4. Evolution bajo `directories.evolution` vinculando `uuid` de esta spec.
5. **No mutar** `start-sddia.sh`, daemons, `materialize-fracture-pbi` ni genoma protegido **salvo** fallo del gate (entonces reclasificar a (A) y detener archivo hasta diseño correctivo).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Laudo (B) documentado con evidencia de audit fresco (`missed_cycles=0` o `<3`) en obligatorios al momento de ejecución |
| CA2 | Ignición verificada: 2/2 centinelas obligatorios vivos + heartbeats frescos; opcionales coherentes con el entorno |
| CA3 | Los 5 `document_id` consolidados en `docs/todos/done/` con `fix_ref` de esta ola |
| CA4 | `validacion.md` global `APTO`, `pbi_archived: true`, `branch: fix/centinelas-fracture-ola-20260723` |
| CA5 | Sin mutación genómica en el diff **o**, si hubo pivot (A), causa raíz y remediación explícitas en `implementation.md` |

## Límites

- Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`.
- Prohibido bypass raw (`gh`/`git`/`curl`) hasta cierre documentado.
- Tekton/Argos **no** escriben semillas Kaizen bajo `docs/todos/`; solo archivan los 5 PBI de esta ola.
- Cumulo topology: `directories.documentation` → `docs`; `daemons_instance.status|state` → `.SddIA/daemons/{status,state}`.

---
feature_name: centinelas-fracture-ola-20260901
created: "2026-09-04"
process: bug-fix
base: main
branch: fix/centinelas-fracture-ola-20260901
uuid: 70b29d72-b36e-4055-830b-e2809047f0b2
scope: laudo-no-regresion-archivo-pbi-ola
consolidated_pbis:
  - PBI-FIX-FRACTURE-ace57b065f9b
  - PBI-FIX-FRACTURE-6cc3b954bad3
  - PBI-FIX-FRACTURE-4f209670a96f
  - PBI-FIX-FRACTURE-3d326490b80d
  - PBI-FIX-FRACTURE-19bfe7cf3371
verdict: B-documentary-debt
execution_id: "05697623-d6d8-4c76-81b2-e8a270d4605d"
related_fixes:
  - docs/fixes/centinelas-fracture-ola-20260819
  - docs/fixes/centinelas-fracture-ola-20260812
  - docs/fixes/centinelas-fracture-ola-20260723
  - docs/features/latido-ontologico-vitalidad-organos
excluded:
  - PBI-FIX-FRACTURE-7bc20a6b4dd6
---

# Spec — Ola fracturas centinelas (2026-09-01)

## Decisión

Un solo `bug-fix` documental consolida 5 PBIs `System_Fracture_Detected` satélites (emisor `argos` / `daemon-heartbeat-audit` / `emit_orphan_lock_fracture`):

| document_id | Proceso | Traza |
|-------------|---------|--------|
| PBI-FIX-FRACTURE-ace57b065f9b | email-watcher | lock huérfano PID 638582 @ 2026-09-01T14:30:52Z |
| PBI-FIX-FRACTURE-6cc3b954bad3 | event-sweeper | lock huérfano PID 7007 @ 2026-09-01T14:30:52Z |
| PBI-FIX-FRACTURE-4f209670a96f | github-bridge-watcher | lock huérfano PID 7103 @ 2026-09-01T14:30:15Z |
| PBI-FIX-FRACTURE-3d326490b80d | iota-publish-relay | lock huérfano PID 653392 @ 2026-09-01T14:30:36Z |
| PBI-FIX-FRACTURE-19bfe7cf3371 | telegram-watcher | lock huérfano PID 7079 @ 2026-09-01T14:30:35Z |

**Laudo: (B) deuda documental** — ventana 2026-09-01T14:30Z (evento de host); runtime sin fractura activa al cierre; **cero mutación genómica**. Prohibido fusionar `document_id`. Prohibido alterar keepalive, umbrales o abrir ramas por centinela.

**Fuera de alcance:** `PBI-FIX-FRACTURE-7bc20a6b4dd6` (`system-vitality-probe` / `sddia-qa` ausente). Ciclo aparte.

`plan.md` **no** se emite: no hay blueprint de proceso nuevo; la ejecución es archivo PBI + verificación de no-regresión.

## Discriminación A vs B (empírica)

| Hecho | Evidencia @ 2026-09-04T09:33:54Z | Lectura |
|-------|----------------------------------|---------|
| Sweep Argos | `fractures_emitted: []` | Sin fractura activa |
| Macrófago | 5 candidatos = estos `document_id`; `apply: false` | Traza `last_heartbeat` anterior a `lock.started_at` vigente |
| `heartbeat-audit.json` | `missed_cycles=0`, `classification=healthy` en los 5 + watcher + kalma2 | Sin staleness |
| Obligatorios vivos | `event-watcher` pid 67844, `event-sweeper` pid 67914; `started_at` 2026-09-04T06:30:52Z | Gate ignición OK |
| Opcionales vivos | email 75943, github-bridge 75938, iota-relay 75932, telegram 75937; `started_at` 2026-09-04T06:34:45Z | Entorno coherente |
| Trazas satélite | `last_heartbeat` 2026-09-01T14:30:15–52Z | Snapshot de downtime, no colapso activo |
| Mayeuta | cubo `huérfan` → EDA coverage | Clasificación cruzada; **no** mandato de backfill |

## Cambios (Tekton)

1. Verificar no-regresión: sweep + `missed_cycles < 3` en obligatorios.
2. Archivar 5 PBI `pending/` → `done/`, `status: cerrado`, `fix_ref: docs/fixes/centinelas-fracture-ola-20260901`.
3. Materializar `implementation.md` + `execution.md` + `validacion.md`.
4. Evolution bajo `directories.evolution` vinculando uuid `70b29d72-b36e-4055-830b-e2809047f0b2`.
5. **No mutar** genoma protegido. Si gate tumba laudo → pivot (A), detener archivo.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Laudo (B) con evidencia audit fresco (`missed_cycles=0` o `<3`) |
| CA2 | Obligatorios vivos + heartbeats frescos; opcionales coherentes con entorno |
| CA3 | 5 `document_id` en `docs/todos/done/` con `fix_ref` |
| CA4 | `validacion.md` global `APTO`, `pbi_archived: true` |
| CA5 | Diff sin mutación genómica; `7bc20a6b4dd6` intacto en `pending/` |
| CA6 | Sin cambios a keepalive / umbrales / ramas por centinela |
| CA7 | Identidad de PBI conservada (cinco `document_id`; cero fusión) |

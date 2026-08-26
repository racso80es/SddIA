---
feature_name: centinelas-fracture-ola-20260819
created: "2026-08-26"
process: bug-fix
base: main
branch: fix/centinelas-fracture-ola-20260819
uuid: a1c9e7f3-2b4d-5e6f-8a9b-0c1d2e3f4a5b
scope: laudo-no-regresion-archivo-pbi-ola
consolidated_pbis:
  - PBI-FIX-FRACTURE-fe227c6e32d3
  - PBI-FIX-FRACTURE-432fdf5a94ee
  - PBI-FIX-FRACTURE-1daf40c4dac7
  - PBI-FIX-FRACTURE-f34e42b10828
  - PBI-FIX-FRACTURE-4d9431bc66b3
verdict: B-documentary-debt
antecesor_audit: docs/audits/centinelas-fracturas-eventos-pending-20260826.md
related_fixes:
  - docs/fixes/centinelas-fracture-ola-20260812
  - docs/features/heartbeat-circuit-regimen-20260811
  - docs/audits/kaizen-aislamiento-multi-instancia-20260826.md
excluded:
  - PBI-FIX-EMAIL-WATCHER-IMAP-ACCOUNT-WATERMARK
---

# Spec — Ola fracturas centinelas (2026-08-19)

## Decisión

Un solo `bug-fix` documental consolida 5 PBIs `System_Fracture_Detected` satélites (emisor `argos` / `daemon-heartbeat-audit`):

| document_id | Proceso | Traza (ciclos / last_heartbeat) |
|-------------|---------|----------------------------------|
| PBI-FIX-FRACTURE-f34e42b10828 | github-bridge-watcher | 745 @ 2026-08-16T17:07:07Z |
| PBI-FIX-FRACTURE-4d9431bc66b3 | telegram-watcher | 1492 @ 2026-08-16T17:06:58Z |
| PBI-FIX-FRACTURE-432fdf5a94ee | event-sweeper | 788 @ 2026-08-19T08:40:36Z |
| PBI-FIX-FRACTURE-fe227c6e32d3 | email-watcher | 1532 @ 2026-08-19T16:26:27Z |
| PBI-FIX-FRACTURE-1daf40c4dac7 | event-watcher | 237 @ 2026-08-20T12:06:43Z |

**Laudo: (B) deuda documental** — fracturas históricas (ventana 16–20 ago); runtime sin fractura activa al cierre; **cero mutación genómica**. Prohibido alterar `missed_cycles`, keepalive o abrir ramas por centinela.

**Fuera de alcance:** `PBI-FIX-EMAIL-WATCHER-IMAP-ACCOUNT-WATERMARK` (laudo A — bug funcional watermark; ciclo `bug-fix` separado).

`plan.md` **no** se emite: no hay blueprint de proceso nuevo; la ejecución es archivo PBI + verificación de no-regresión.

## Discriminación A vs B (empírica)

| Hecho | Evidencia @ 2026-08-26T14:12Z | Lectura |
|-------|-------------------------------|---------|
| Sweep Argos | `fractures_emitted: []` | Sin fractura activa |
| `heartbeat-audit.json` | `missed_cycles=0` en los 5 | Sin síntoma de staleness |
| Obligatorios vivos | `event-watcher` pid 57131, `event-sweeper` pid 49944 | Gate ignición OK |
| Trazas satélite | Posteriores a ola 20260812 (16 ago) | Nueva ventana de indisponibilidad, no re-apertura de IDs anteriores |
| Mitigaciones en main | A+B+C+D (PR #168), Kaizen panic (PR #175), aislamiento multi-instancia (`fb12e07`) | Causa estructural de olas jul–ago parcialmente absorbida |
| Causa histórica probable | Downtime host, colisión multi-instancia (F-SYS-02/F-DEP-10), R-07 | Documentado en `docs/audits/centinelas-fracturas-eventos-pending-20260826.md` |
| Residual operativo | Locks huérfanos `email-watcher`, `telegram-watcher` (PID muerto en lock; side-channel desfasado) | No bloquea laudo B; opcionales; reinicio recomendado post-merge |

## Cambios (Tekton)

1. Verificar no-regresión: sweep + `missed_cycles < 3` en obligatorios.
2. Archivar 5 PBI `pending/` → `done/`, `status: cerrado`, `fix_ref: docs/fixes/centinelas-fracture-ola-20260819`.
3. Materializar `implementation.md` + `execution.md` + `validacion.md`.
4. Evolution bajo `directories.evolution` vinculando uuid `a1c9e7f3-2b4d-5e6f-8a9b-0c1d2e3f4a5b`.
5. **No mutar** genoma protegido. Si gate tumba laudo → pivot (A), detener archivo.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Laudo (B) con evidencia audit fresco (`missed_cycles=0` o `<3`) |
| CA2 | Obligatorios vivos + heartbeats frescos; opcionales coherentes con entorno |
| CA3 | 5 `document_id` en `docs/todos/done/` con `fix_ref` |
| CA4 | `validacion.md` global `APTO`, `pbi_archived: true` |
| CA5 | Diff sin mutación genómica; watermark PBI intacto en `pending/` |
| CA6 | Sin cambios a keepalive / umbrales / ramas por centinela |

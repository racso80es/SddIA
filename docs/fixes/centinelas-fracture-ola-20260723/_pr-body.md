## Summary

**Motivo de este fix (laudo B — deuda documental, no bug de runtime):**

Los 5 PBI en `pending/` nacieron de `System_Fracture_Detected` (Argos / `daemon-heartbeat-audit`) entre 2026-07-23 y 2026-07-25: centinelas que omitieron ≥3 ciclos de `Daemon_Heartbeat`. La **causa operativa ya estaba mitigada** en `main` por olas previas (`centinelas-fracture-ola-20260716`, `centinelas-kalma2-fracture-ola-20260722`) y por `daemon-heartbeat-ingest-ignition` (PR #155: ingest térmico + gate de ignición).

Tras archivar PBI hermanos en #155, `materialize-fracture-pbi` generó **nuevos `document_id`** (hash de traza distinto: 469/234/18/17 ciclos) que quedaron huérfanos en `pending/` sin ola de cierre. Este PR **no muta genoma ni daemons**: audita no-regresión empírica y cierra esa deuda documental en un solo `bug-fix`.

| document_id | Centinela | Traza histórica |
|-------------|-----------|-----------------|
| `21f55bcdecfb` | event-sweeper | 469 ciclos @ 2026-07-23 |
| `0d65b4775574` | event-watcher | 469 ciclos @ 2026-07-23 |
| `a69be9535f82` | github-bridge-watcher | 234 ciclos @ 2026-07-23 |
| `131fa2c33271` | telegram-watcher | 18 ciclos @ 2026-07-24 |
| `d67f6c0b0195` | telegram-watcher | 17 ciclos @ 2026-07-25 |

Persist: `docs/fixes/centinelas-fracture-ola-20260723/` · `validacion.md` APTO · `pbi_archived: true`.

## Test plan

- [x] 4 locks vivos (2/2 obligatorios + opcionales)
- [x] `heartbeat-audit.json` → `missed_cycles=0` en los 4
- [x] 5 PBI solo en `docs/todos/done/` (pending limpio)
- [x] Diff sin mutación bajo genoma protegido (solo docs + evolution)

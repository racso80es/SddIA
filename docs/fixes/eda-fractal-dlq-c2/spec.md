---
feature_name: eda-fractal-dlq-c2
created: "2026-07-16"
process: bug-fix
base: main
scope: eda-fractal-dlq
laudo: C2
uuid: 3b42e74f-9b9e-4efa-84ad-c8431ba290b2
---

# Spec — EDA fractal DLQ (laudo C2)

## Decisión

Cuando un evento fractal tiene `delivery_state` **terminal con al menos un `failed`**, el archivo se **mueve** a `eda_fractal.dead_letter` (`./.events/dead-letter`).  
Cuando todos los stamps son terminales OK (`success|skipped*`), se mantiene **unlink** (opción B).

## SSOT

`SddIA/core/cumulo.paths.json` → `eda_fractal.dead_letter` = `./.events/dead-letter` (prevalece sobre `eda_bus.dead_letter` en topología fractal).

## Cambios

1. Alta `eda_fractal.dead_letter` en Cúmulo.
2. `BusTopology.dead_letter` + `ensure_fractal_dirs` crea el directorio.
3. `event-sweeper` / `eda_sweep`: all-ok → unlink; terminal-with-failure → rename a DLQ.
4. `route_domain_fractal_event` (`purge_after`): mismo criterio post-despacho.
5. `event-watcher`: `fractal_fully_terminal` incluye `failed` (no re-despacho infinito).

## CA

| ID | Criterio |
|----|----------|
| CA1 | Domain all-ok → unlink (sin regresión B) |
| CA2 | Domain terminal-with-failure → ausente en `.events/domain/`, presente en `.events/dead-letter/` |
| CA3 | Destino DLQ se crea si no existe (sin panic FS) |
| CA4 | SSOT `eda_fractal.dead_letter` resuelto por runtime |

---
feature_name: eda-fractal-lifecycle-option-b
created: "2026-07-16"
process: bug-fix
base: main
scope: eda-fractal-lifecycle
---

# Spec — EDA fractal lifecycle (opción B)

## Decisión

`route_domain_fractal_event` usa `purge_after=true`: tras consenso de suscriptores el JSON en `.events/domain/` se elimina.

## Cambios

1. Stamp `delivery_state[subscriber_id]` en disco tras cada suscriptor; skip `skipped-already-delivered` si ya terminal.
2. Sweeper barre también `.events/{domain,orchestration,telemetry}` si todos los stamps son terminales OK.
3. Telegram: ACK offset antes de gateway; persistencia en `.SddIA/daemons/state/telegram-watcher.json` + `seen(update_id)`.
4. Watcher: no re-despacha archivos fractal-terminal; limita eco tras side-effect Telegram.

## CA

| ID | Criterio |
|----|----------|
| CA1 | Domain event con todos success → archivo ausente post-route |
| CA2 | Reintento no re-ejecuta telegram-fallback si stamp success |
| CA3 | Offset Telegram avanza aunque gateway falle |
| CA4 | Sweeper purga domain terminal sin depender de pending V3+ |

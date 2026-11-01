---
feature_name: eda-subscriber-event-push
created: "2026-10-31"
process: refactorization
branch_name: Fix_Events_Multiple
tracker_ref: AUT-21
---

# Runbook T7 — Re-enrutado de sellos `success` sin efecto

Cuando `subscriber_delivery` registró `success` en `tracker-stamp` pero Linear no cambió (AUT-20 / AUT-21).

## Precondiciones

- `SDDIA_EVENT_STORE=redb` (default Core-self).
- Binario compilado: `SddIA/target/debug/execute-process`.
- Identificar `event_id` y comprobar `event_type` (`PBI_Refined`, `HU_Refined`, …).

## Pasos

1. Inspección (opcional):

```bash
SddIA/target/debug/execute-process --query-events --inputs \
  '{"family":"domain","event_type":"PBI_Refined","limit":20}'
```

2. Reset del suscriptor `tekton.tracker-stamp` a `Pending`:

```bash
SddIA/target/debug/execute-process --reset-subscriber-delivery --inputs \
  '{"event_id":"<uuid>","subscriber_id":"tekton.tracker-stamp","reason":"AUT-21-T7"}'
```

3. Re-enrutado bloqueante:

```bash
./sddia-run.sh --process route-domain-event --inputs \
  '{"event_file_path":".events/pending/<uuid>.json","blocking":true}'
```

4. Verificar en Linear el `tracker_ref` del payload y la fila `subscriber_delivery` (no debe quedar `success` en no-op remoto).

## Lote

Script: `_lab/replay-false-success-stamps.py` — lista de `event_id` en el cuerpo o por argumentos.

Casos históricos citados en AUT-21: `4581b7f6-cc3b-4486-b2b2-efa9a12491cb` (AUT-6 lineage). Ejecutar solo si el ECST sigue en store.

---
feature_name: eda-redb-03-watcher-vitality
created: "2026-10-21"
process: feature
document_id: PBI-EDA-REDB-03-WATCHER-VITALITY
uuid: "9b4e7c25-8a1d-4e63-b5f0-2d8c6a1e4f97"
version_spec: "1.0.0"
---

# Especificación — eda-redb-03-watcher-vitality

## Store

`EventStore::list_pending_deliveries` recorre `subscriber_delivery` y devuelve pares cuya fila está en `Pending`. No devuelve el ECST.

## Watcher

Cada iteración, incluido `--once`:

1. Si el reloj de auditoría venció (30 s, vencido al arrancar), `execute-process --process daemon-heartbeat-audit --inputs '{"sweep":true}'`.
2. Si el reloj de vitalidad venció (`SDDIA_VITALITY_PROBE_SECONDS`, vacío = 300, menor de 30 o inválido = 30), `execute-process --process system-vitality-probe --inputs '{}'`.
3. Barrido de ficheros igual que v1.1.0.
4. Si `writes_store` y existe `events.redb`, reclamo de ids pending cuya ruta no es un fichero. Delega `route-domain-event` con `event_file_path` sintético.

## Despacho

| Tier | Ruta sin fichero | Ruta con fichero |
| --- | --- | --- |
| `none` o ausente | Handler determinista. Cero spawn de `SDDIA_AGENT_RUNTIME_COMMAND`. | Igual. |
| `high`, `medium`, `low` | `AGENT_PHASE` por stdin: `ecst` y `workspace_path` autorizado. | Proceso o acción ya cableados. |

## Ficha

`event-watcher` pasa a `1.2.0`. UUID `f995cc89-22a7-488d-9b25-ddb1e5e3a4a4` se conserva. La jurisdicción declara el reclamo por id y las dos sondas, y que el watcher no abre el ECST.

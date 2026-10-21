---

feature_name: eda-redb-04-readers
created: "2026-10-21"
purpose: "Decisiones de lectura del store y del gate de heartbeats"
execution_id: "06f21693-9776-4537-a9fe-00cc76dda132"
---
# Clarify — eda-redb-04-readers

## Decisiones

- `filesystem`, valor vacío o desconocido no abre `events.redb` ni barre `.events/**` desde los flags nuevos. Respuesta: `EVENT_STORE_DISABLED`, exit distinto de 0.
- `dual` y `redb` leen el store. No hay fallback a ficheros si la base no existe: el lector devuelve vacío.
- El SSE sigue siendo `event: progress\ndata: {body}\n\n`. Solo cambia el origen de los frames (`progress_traces` ordenado por `seq`).
- Proofs (`.SddIA/proofs`) y el inbox cognitivo (`.SddIA/radamanto/inbox`) siguen en disco.
- El side-channel de heartbeats no entra en `redb`. `audit_telemetry_file` acepta `source: side-channel`, registra el latido y no estampa entrega sobre ese JSON.
- El CLI permanece en flags. `--query-events` exige `family` y/o `event_type`, y `limit`. `--replay-dead-letter` exige `event_id` y llama a `EventStore::replay`.

---

feature_name: eda-redb-01-store
created: "2026-10-21"
process: feature
base: main
scope: eda-redb-01-store
version_spec: "1.0.0"
document_id: PBI-EDA-REDB-01-STORE
uuid: "3f8c1a62-7d4e-4b91-8a27-6c5e0d9f1b34"
execution_id: "82b4551a-3443-457b-8650-b3a0b6b6447a"
---
# Especificación — eda-redb-01-store

## 1. Crate

| Pieza | Valor |
|-------|-------|
| Ruta | `SddIA/sddia-event-store` |
| Nombre | `sddia-event-store` |
| Workspace | miembro de `SddIA/Cargo.toml` |
| Dependencia | `redb = "=2.6.4"` |
| Otras | `serde`, `serde_json` |
| Runtime | síncrono. Sin tokio |

`open(path)` llama a `Database::create`: crea el fichero o reabre el existente. El llamador resuelve `.SddIA/runtime/events.redb`. Si el directorio padre no existe, el error sale de `redb`.

## 2. Tablas

| Tabla | Clave `redb` | Valor |
|-------|--------------|-------|
| `events` | `&str` `event_id` | JSON `StoredEvent` |
| `family_index` | `(&str, i64, &str)` familia, `created_at`, `event_id` | `()` |
| `subscriber_delivery` | `(&str, &str)` `event_id`, `subscriber_id` | JSON `DeliveryWitness` |
| `dead_letter_index` | `&str` `event_id` | JSON `DeadLetter` |
| `dlt_audit_seals` | `&str` `event_id` | JSON `DltSeal` |
| `progress_traces` | `(&str, u64)` `correlation_id`, `seq` | JSON del frame |

`Family`: `telemetry`, `orchestration`, `domain`. `DeliveryStatus`: `pending`, `success`, `failed`, `skipped`. `created_at`: unix millis.

## 3. Operaciones

Cada mutación abre una write-tx y hace `commit` al terminar. Si el cuerpo devuelve error, la transacción se aborta al caer el guard.

| Método | Write-tx |
|--------|----------|
| `insert_event` | `events` + `family_index` + N filas `pending` con witness `{}`. `event_id` repetido = `Conflict` y no altera testigos. |
| `set_delivery` | Solo el par indicado. Par ausente = `NotFound`. |
| `seal_dlt` | Upsert del sello. Exige que el evento exista. `tx_hash`, `digest` y `scope` no vacíos. |
| `append_progress` | Inserta el frame. La misma `(correlation_id, seq)` = `Conflict`. No exige evento. |
| `dead_letter` | Upsert de causa + `failed_subscriber_id`. Exige evento y testigo de ese suscriptor. |
| `replay` | Borra la fila de `dead_letter_index`. El testigo de `failed_subscriber_id` pasa a `pending` y conserva el JSON. El resto de suscriptores no cambia. Sin fila de dead letter = `NotFound`. |

Lecturas (`get_event`, `get_delivery`, `get_seal`, `get_dead_letter`, `list_family`, `list_progress`) usan `begin_read`. `list_family` recorre el índice de esa familia en orden de `created_at` y rehidrata `events`. `list_progress` ordena por `seq`.

Identificadores se recortan. Vacío tras el recorte = `Invalid`. Suscriptores duplicados en el mismo `insert_event` = `Invalid`. Lista vacía de suscriptores es válida.

## 4. Concurrencia

`Database` admite N `begin_read`. `begin_write` es único: la segunda llamada bloquea hasta el `commit` o el abort de la primera. Dos `set_delivery` del mismo `event_id` y distinto `subscriber_id` escriben claves distintas.

## 5. Errores

`StoreError`: `Invalid`, `NotFound`, `Conflict`, `Backend`. El llamador no ve `redb::Error`.

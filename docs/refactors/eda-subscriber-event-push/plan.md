---
feature_name: eda-subscriber-event-push
created: "2026-10-31"
process: refactorization
branch_name: Fix_Events_Multiple
persist_ref: docs/refactors/eda-subscriber-event-push
tracker_ref: AUT-21
phases:
  - id: T1
    title: Puerto de carga con precedencia event, event_id, fichero de laboratorio
  - id: T2
    title: Despacho del router con ECST en línea y event_id
  - id: T3
    title: Autenticidad antes de escritura remota
  - id: T4
    title: Migración de los siete handlers al puerto
  - id: T5
    title: Aduana contra fs::read_to_string del path en handlers
  - id: T6
    title: Verificar subscriber_delivery y documentar delivery_state como foto
  - id: T7
    title: Re-enrutado operativo de success históricos
---

# Plan — AUT-21

Rama `Fix_Events_Multiple`. Un commit por tarea cuando se ejecute. Genoma indexado solo por `entity-manager`.

## T1 — Puerto de carga

Extender el puerto que hoy es `ecst_input_loader::load_ecst_from_inputs`.

Orden: objeto `event`, luego `event_id` con `load_dispatched_event`, luego fichero de `event_file_path`. Si la entrada ya trae `event_id`, no derivar el id del nombre del fichero.

Tests de unidad: en línea sin disco; solo `event_id` en redb sin pending; fixture de laboratorio por path; rechazo cuando no hay ninguna de las tres fuentes.

## T2 — Despacho

En `route_domain_core`, las ramas de `tracker-stamp`, `tracker-sync-replay` y el grupo email / `user-preference` incluyen `event` (el ECST ya cargado) y `event_id`. `event_file_path` se mantiene solo como compatibilidad.

El suscriptor no abre `events.redb` cuando `event` viaja en línea y la verificación no exige recarga.

## T3 — Autenticidad

Ajustar `verify_ecst_matches_bus` para que un ECST en línea sin fila coincidente en store no autorice escritura remota. El handler recarga por `event_id` y opera sobre esa copia. Desajuste explícito sigue en `failed-bus-mismatch`.

## T4 — Handlers

Sustituir la carga local en:

- `tracker_stamp` (ya delega; pasa a la precedencia nueva)
- `tracker_markdown_apply`
- `user_preference`
- `email_triage`
- `email_quick_action`
- `email_digest_preference_reply` (conserva el atajo `callback_data` cuando no hay ECST)
- `tracker_sync_replay` (deja de leer el fichero por su cuenta y de fabricar un ECST mínimo si falta el path)

## T5 — Aduana

Test que recorre `handlers/` y falla ante `fs::read_to_string` del path de evento. El puerto queda fuera de ese directorio.

## T6 — Entrega y contrato

Comprobar con un test de `route-domain-event` que un no-op remoto de `tracker-stamp` queda en `subscriber_delivery` como `skipped-*` o `failed`, no como `success`.

Si el contrato de eventos debe decir que `delivery_state` es la foto del alta, el cambio de ese `.md` indexado va por `entity-manager` y una entrada en `SddIA/evolution/`.

## T7 — Operación

Después de T1–T6, re-enrutar los `PBI_Refined` y `HU_Refined` que el bus marcó `success` sin efecto en Linear. Caso citado: AUT-6, evento `4581b7f6-cc3b-4486-b2b2-efa9a12491cb`. No es cambio de genoma.

## Hecho previo que este plan no repite

AUT-46 ya introdujo el fallback al store por nombre de fichero y `classify_stamp_delivery`. T1 sustituye el fallback como camino principal. T6 solo verifica la persistencia del estado.

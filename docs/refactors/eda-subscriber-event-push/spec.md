---
feature_name: eda-subscriber-event-push
created: "2026-10-31"
base: main
scope: despacho ECST por parámetros y semántica de no-op
process: refactorization
branch_name: Fix_Events_Multiple
tracker_ref: AUT-21
---

# Especificación — entrega del evento por parámetros

## Contrato de entrada del suscriptor

El router, al despachar un proceso suscriptor de dominio, inyecta en el JSON de proceso:

| Campo | Obligatorio | Uso |
|-------|-------------|-----|
| `event` | Cuando el router ya tiene el ECST | Objeto ECST completo |
| `event_id` | Siempre | Trazabilidad y recarga |
| `event_file_path` | Solo laboratorio o binarios antiguos | Último recurso |

Precedencia del puerto de carga (`ecst_input_loader`, o el nombre que tome el puerto único):

1. Si `event` es un objeto con `event_id`, usarlo.
2. Si no, cargar por `event_id` con `load_dispatched_event`.
3. Si no, leer `event_file_path` del disco. Este paso no inventa un `event_id` a partir del nombre cuando ya existe `event_id` en la entrada.

Los handlers no llaman a `fs::read_to_string` sobre `event_file_path`. La lectura de fichero, si ocurre, vive en el puerto.

## Suscriptores

El despacho en `route_domain_core` deja de pasar solo la ruta para:

- `tracker-stamp`
- `tracker-sync-replay`
- `tracker-markdown-apply`
- `user-preference-ingest` (`user_preference`)
- `email-triage-gateway` (`email_triage`)
- `email-quick-action-ingest` (`email_quick_action`)
- `email-digest-preference-reply`

`tracker_sync_replay::load_event` hoy lee el fichero y, si no hay path, fabrica un ECST mínimo. Pasa al mismo puerto. No fabrica un evento que no esté en el bus ni en el fichero de laboratorio.

## Autenticidad

Antes de un efecto remoto de tracker, el ECST en línea se compara con el registro del bus (`verify_ecst_matches_bus`). Si no coincide, el resultado es fallo de entrega y no hay escritura remota. Si el modo store no tiene fila y la entrada trae `event_id`, se recarga del store y se usa esa copia. Un objeto en línea sin ancla en el bus no es suficiente para `update_issue_*` en tracker remoto.

En modo filesystem de laboratorio, los fixtures `.events/domain/*.json` siguen siendo válidos por el tercer peldaño.

## Semántica de no-op

`tracker-stamp` ya distingue `success`, `skipped-*` y `failed` en el envelope (`classify_stamp_delivery`, AUT-46). `dispatch_process_subscriber` propaga `data.delivery_status`. Este refactor comprueba que la fila `subscriber_delivery` persiste ese estado y que un `skipped-*` o `failed` no se reescribe como `success`.

`delivery_state` dentro del blob ECST permanece como foto del alta. La documentación del contrato de eventos lo dice de forma explícita. No se añade un escritor que mantenga ese campo al día.

## Aduana

Un test del motor falla si algún fichero bajo `handlers/` contiene `fs::read_to_string` cuyo argumento es el path resuelto desde `event_file_path`. El puerto de carga es la excepción, fuera de `handlers/`.

## Criterios de aceptación

Coinciden con el cuerpo sellado de AUT-21: sin lectura ciega en handlers, `PBI_Refined` en redb sin fichero pending y sin que el suscriptor abra el store cuando el ECST va en línea, ECST no verificado sin escritura remota, no-op remoto fuera de `success`, relanzado por `event_id`, fixtures de laboratorio verdes.

## Fuera de alcance

Backend remoto del bus. Diseño de `refine-item` (AUT-19). Corte por tier cognitivo (AUT-26). Trait `EventSource` para un backend futuro: queda nombrado, no se implementa.

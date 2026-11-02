---
feature_name: aut-52-refine-fracture-route
created: "2026-11-02"
updated: "2026-11-02"
process: bug-fix
phase: Diseño del fix
agents: dedalo
base: main
scope: refine-item-lock-route-domain-redb-fanout-linear-opaque
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-52-refine-fracture-route
pbi_ref: AUT-52
tracker_ref: AUT-52
execution_id: "f2880068-385c-4403-8c31-9d11bd4b08a1"
---
# Spec — aut-52-refine-fracture-route (AUT-52)

## Hecho

Incidente al reentrar el sellado `refine-item` sobre AUT-25 (execution `ffd581d2-5f0c-4df5-b56b-13563b3bd50a`, HEAD `84c2ee24`). Tres defectos acoplados + deuda de catálogo/cuerpo Linear.

| ID | Defecto | Evidencia / ancla |
|----|---------|-------------------|
| F1 | Contención de lock en sellado: `Database already open. Cannot acquire lock.` sobre `.SddIA/runtime/events.redb` (~3041 ms, `success: false`). En esa pasada no queda `System_Fracture_Detected` usable porque la emisión reabre el mismo store y el fallo se traga (`let _ =`). | `refine_item/run.rs` (`STORE_LOCK_CONTENTION`, `emit_store_lock_fracture`); AUT-50 (`docs/fixes/aut-50-backlog-alta`, APTO) acotó código/budget 10 s pero no la fractura sin store libre |
| F2 | Fan-out mudo: con evento ya en `redb`, `route-domain-event` acusa `success: true`, `data: null` / `dispatch_mode: null` y deja `delivery_state` `pending` para `cumulo.materialize-fracture-pbi`, `mayeuta.enrich-fracture-pbi-kaizen`, `argos.send-telegram-notification` (evento `684ebbf7-…`). | `write_pending_domain_event_file` en modo `redb` (sin FS) devolvía `runtime_db_path(repo)` → `.SddIA/runtime/events.redb`. `route_domain_event` trata esa ruta como fichero JSON; `route_domain_batch` hace `continue` silencioso y **siempre** devolvía `success: true` |
| F3 | `materialize-fracture-pbi` / `invoke_tool_for_process` expone solo `LINEAR_TRANSPORT`: `tool_invoke_error_message` retorna `error.code` y descarta `error.message` (`http-status-N` / `http-post-failed: …`). | `capsules.rs` `tool_invoke_error_message`; `linear-tracker-adapter` emite `LinearFail { code: LINEAR_TRANSPORT, message: … }` |
| F4 | `F-REFINE-ITEM-REDB-LOCK` ausente en catálogo al incidente; `friction_id` no entra al cuerpo remoto porque `materialize_fracture_pbi` filtra con `catalog_contains_id`. | `SddIA/core/fracture-signatures.json`; `materialize_fracture_pbi.rs` |
| F5 | Deuda AUT-24 AC-4: camino Linear de materialización debe usar `build_pbi_body` + labels `pbi`+`fix`. Si queda legacy (`description: &error_trace` / un solo label `fix`), cerrarlo aquí. El nativo `run_remote` ya tiene cuerpo/labels correctos — no reabrir si Argos confirma paridad. | `materialize_fracture_pbi::run_remote`; contrato AUT-24 |

Relacionado: AUT-50 valida contención observable bajo lab; **no** cierra F2 ni F3. Mismo fan-out atrapa `e4432141-…` / `58eb3acc-…` (solo evidencia; drenado manual fuera).

## Corte

### C1 — Lock refine-item: código estable + fractura observable sin store libre

1. Mantener clasificación `STORE_LOCK_CONTENTION` / literales `Database already open` / `Cannot acquire lock` (AUT-50).
2. Ante lock en sellado/emisión: la fractura **debe materializarse sin depender de un segundo `EventStore::open` exitoso**. Opciones aceptables (mínima):
   - sidechannel FS / ECST pending legible por el router aunque el modo default sea `redb`; o
   - artefacto de workspace (`fracture-payload.json`) **y** envelope con `error`/`error_code` estable + `friction_id`.
3. Payload: `source_process: refine-item`, `error_code: STORE_LOCK_CONTENTION`, `friction_id: F-REFINE-ITEM-REDB-LOCK` (tras C4).
4. No rediseñar motor multi-handle; presupuesto open 10 s intacto.
5. Lab: `SDDIA_LAB_SIMULATE_STORE_LOCK` → envelope honesto **y** fractura recuperable sin reentrada muda.

### C2 — Identidad de evento en modo `redb` + router honesto

1. `write_pending_domain_event_file` / `materialize_pending_domain_event` **nunca** deben devolver la ruta del fichero `.redb` como si fuera ECST. En modo store-only: path virtual bajo pending cuyo `file_stem` sea el `event_id` (p. ej. `{pending}/{event_uuid}.json` **sin** exigir existencia en FS), de modo que `route_domain_event` tome `from_store` / `load_dispatched_event(repo, event_id, …)`.
2. Dedupe (`fracture_already_stored` / hash12): path virtual del `event_id` ya persistido, no `runtime_db_path`.
3. `route_domain_event`: si la ruta resuelta es el DB file (`events.redb` / `runtime_db_path`), error explícito (no parse JSON binario).
4. `route_domain_batch`: **prohibido** `success: true` universal. Si una ruta no carga o un hijo falla → `success: false`, `exitCode != 0`, error accionable; no `continue` silencioso que vacíe el lote.
5. Tras el arreglo, re-enrutar `684ebbf7-…` (o lab equivalente con evento ya en `redb`) debe despachar a los tres suscriptores del catálogo (dejan de quedar los tres en `pending` por fallo de lectura de ruta).

### C3 — Envelope Linear accionable

1. `tool_invoke_error_message`: si `error` es objeto con `code` **y** `message` no vacío, exponer ambos (p. ej. `LINEAR_TRANSPORT: http-status-401: …`). No truncar a solo el código cuando el mensaje aporta status/causa.
2. Test: body con `code: LINEAR_TRANSPORT` + `message: http-status-503: …` → el `Err` de `invoke_tool_for_process` contiene el mensaje HTTP, no únicamente `LINEAR_TRANSPORT`.
3. Conservar preferencia de código para auth (`LINEAR_AUTH_MISSING`) si el test legado lo exige; la regla nueva aplica al menos a transporte.

### C4 — Catálogo + corpus

1. Firma `F-REFINE-ITEM-REDB-LOCK` en `SddIA/core/fracture-signatures.json` (consumers `mayeuta`/`cumulo`): match sobre `STORE_LOCK_CONTENTION` / `Database already open` / `Cannot acquire lock` con scope `process_name: refine-item`.
2. Diagnosis: contención de lock redb en sellado; propuesta = C1+C2 (no multi-handle).
3. Caso anclado en `fracture-corpus` **o** `unclassified` justificado en evolution/validacion.
4. Tras C4, `friction_id` pasa `catalog_contains_id` y aparece en `build_pbi_body`.

### C5 — Cuerpo Linear de materialización (deuda AUT-24 AC-4)

1. Verificar camino efectivo (nativo y, si aplica, cápsula) de `create_issue` en fractura: `build_pbi_body` + labels `[pbi, fix]`.
2. Si existe camino legacy (`description: error_trace` crudo / un solo label), eliminarlo o desactivarlo.
3. No bloquea C2/C3; sí bloquea cierre si Linear materializa basura.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AUT52-CA1 | Bajo contención de lock en sellado `refine-item`: no colapso mudo **o** fallo `STORE_LOCK_CONTENTION` con fractura observable (`System_Fracture_Detected` / payload workspace) sin depender de reabrir `events.redb` en la misma pasada |
| AUT52-CA2 | `materialize_pending_domain_event` en modo `redb` devuelve path cuya identidad es el `event_id` (no `.SddIA/runtime/events.redb`) |
| AUT52-CA3 | `route-domain-event` / `route_domain_batch` no acusan `success: true` con `data: null` por pasar la ruta `.redb` como JSON; fallo de carga → `success: false` |
| AUT52-CA4 | Evento lab/`684ebbf7-…` ya en store: tras route, los tres suscriptores del catálogo dejan de quedar en `pending` **por** fallo de lectura de ruta |
| AUT52-CA5 | Fallo Linear de materialize expone mensaje accionable (`http-status-N` / `http-post-failed`), no solo `LINEAR_TRANSPORT` |
| AUT52-CA6 | Firma `F-REFINE-ITEM-REDB-LOCK` en `fracture-signatures.json` + caso corpus/`unclassified` justificado |
| AUT52-CA7 | Materialización Linear de fractura: cuerpo `build_pbi_body` + labels `pbi`+`fix`; `friction_id` catalogado presente en description |
| AUT52-CA8 | AUT-52 → Done en Linear; sin sembrar `docs/todos/` desde Tekton/Argos; nota evolution ligada a AUT-52 |

## Fuera de alcance

- Reintentar sellado funcional de AUT-25 / transporte raw hasta laudo.
- Remedio `PullRequest_Presented` / `tracker_ref` del puente AUT-25.
- Drenado manual de fracturas históricas `e4432141-…` / `58eb3acc-…`.
- Rediseño multi-handle del motor redb más allá de AUT-50.
- Enrich Kaizen solo-Linear (`resolve_enrich_target` / `docs/todos/pending`) salvo fan-out post-materialize; no bloquea si CA1–CA7 + Linear Done se cumplen.

## Touchpoints (Tekton)

| Módulo | Acción |
|--------|--------|
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | C2: `write_pending_domain_event_file`, `route_domain_batch`, defensa `.redb` |
| `SddIA/engine/execute-process/src/engine/handlers/route_domain.rs` | Propagar fallo honesto del batch (si el envelope sigue mintiendo tras C2) |
| `SddIA/engine/execute-process/src/engine/handlers/refine_item/run.rs` | C1: emit fractura sin segundo open bloqueante; `friction_id` |
| `SddIA/engine/execute-process/src/engine/capsules.rs` (+ tests) | C3: `tool_invoke_error_message` |
| `SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs` | C5 verificación / legado; C4 `friction_id` |
| `SddIA/core/fracture-signatures.json` + corpus | C4 |
| `SddIA/sddia-event-store` (solo si hace falta API de carga por id) | Soporte C2 |
| `SddIA/evolution/<uuid>.md` | AUT52-CA8 |

## Blueprint

No aplica: no hay proceso genómico nuevo. Orden de cortes en `plan.md` (secuencia Tekton, no blueprint de códice).

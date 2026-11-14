---

feature_name: man-43-event-manager-desacople-total
created: "2026-11-11"
updated: "2026-11-11"
process: feature
branch_name: feat/man-43-event-manager-desacople-total
persist_ref: docs/features/man-43-event-manager-desacople-total
pbi_ref: MAN-43
parent_ref: MAN-40
historia_ref: MAN-40
document_id: PBI-0f4f0f5f
agent_planificador: dedalo
target_executor: tekton
phase: diseno-blueprint
status: blueprint-ready
version_plan: "1.2.16"
rebaseline: remediation-waves-v1
audit_ref: docs/audits/man-32-lineage-man-43-remediation-20261111.md
phases:
  - H0-precondiciones-forja
  - O0-contencion-integridad
  - H1-contrato-eventuri-v2
  - H2-absorcion-entregas-testigos
  - H3-watcher-sweeper-puerto
  - H4-perifericos-sin-fs
  - H5-lab-demolicion-gates
  - H6-convergencia-documental
rbac_ok: true
scope: "EventManagerPort v2 / EventUri única frontera; cero dualismo Redb+FS ni topología runtime `.events/`"
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-43.jsonl"
refine_workspace: ".SddIA/workspaces/refine-item/c7976303-3b56-4175-9303-415e8b187623"
seed: continue-tekton
execution_id: "3654517b-c915-4e35-a965-3c7550a7b706"
---
# Plan / Blueprint — man-43-event-manager-desacople-total

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md` (L-*), `spec.md` **v1.0.1** (ids **MAN-43** / **PBI-0f4f0f5f** / padre **MAN-40**).

Rebaseline **v1.1.0**: la auditoría post-H1 añade O0, AC-15…AC-19 y convierte el H1 restante en dos olas: **H1R/O0** (integridad) y **H1C/O1** (contrato completo). El merge #453 conserva su sello histórico; no se reejecuta como si H1 estuviera vacío.

Semilla **`continue tekton`** (**L-CONTINUE-TEKTON**): reconfirmación Dedalo; el *qué* no cambia; Ejecución consume este plan en solo lectura. `rbac_ok: true` intacto.

`target_executor_rbac.allowed_policies` alineadas a Tekton: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `chaos-engineering`, `tracker-operations`.

**Lectura de estado:** bóveda MAN-36 viva; kernel H1 parcial en 2.0 y Genoma aún 1.2/v1; deuda F-*/L-P*/R* consolidada en `docs/audits/man-32-lineage-man-43-remediation-20261111.md`. Los paths legacy ausentes quedan supersedidos por esa auditoría; O6 reaudita antes de AC-13. Este blueprint pide forja del desacople total v2, no reabrir MAN-36.

## Viabilidad RBAC (cruzada mecánicamente)

| Cápsula | `context` skill | ¿En Tekton? |
|---------|-----------------|-------------|
| `skill:filesystem-manager` | `filesystem-ops` | sí |
| `skill:shell-executor` | `system-operations` | sí (`cargo test` / clippy / gate) |
| `skill:git-manager` | `source-control` | sí — **única** vía Git (**L-GIT**) |
| `action:execute-process` → entity-manager | `ecosystem-evolution` | sí si muta Genoma/docs normativos (**L-GENOME**) |

**Prohibido:** TODOs bajo `docs/todos/` (solo `agent:cumulo` / `Kaizen_Alert_Required`).
**Prohibido:** reabrir bóveda/precedencia MAN-36; dualidad productiva v1/v2; imponer UUID a `event_id`; eliminar `SddIA/events/` o `.SddIA/events/`.
**rbac_ok:** `true` — ninguna fase requiere contexto fuera de Tekton.

---

## Fases declarativas

```yaml
phases:
  - name: "H0 Precondiciones de forja"
    intent: "Cerrar P1–P6 (inventario v1, ops v2, purga, cutover, tests) y sellar forge_key si el runtime lo exige."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "O0 Contención de integridad"
    intent: "Eliminar matching por substring y errores por texto; probar aislamiento de IDs antes de ampliar el puerto."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "H1 Contrato EventManager 2.0 + EventUri"
    intent: "PublishSeal.event_uri; eliminar logical_rel/target_path; schema/bindings/taxonomía/version_gate v2; suite mock/Redb/externo; rechazo v1."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor", "action:execute-process"]
  - name: "H2 Absorción entregas/testigos"
    intent: "processing/processed/DLQ detrás del puerto; fin de escrituras FS en eda_bus_topology."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "H3 Watcher/sweeper sin FS"
    intent: "claim/listado/cierre/purga solo puerto; cero doble despacho; cero read_dir/is_file SSOT."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "H4 Periféricos sin fallback FS"
    intent: "actions/Kalma2/workspace-server/github_bridge limpios; retirar runtime_db_path/path_is_resolved_redb de consumidores."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "H5 Lab + demolición + gates"
    intent: "Migrar matrices; eliminar eda_port_legacy y sddia-event-store; allowlists vacías; cargo/clippy/suite/gates verdes."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "H6 Convergencia documental"
    intent: "README/Constitución/cumulo/evolution; rematerializar auditorías ausentes; APTO_TOTAL solo con evidencia."
    delegates_to: ["skill:filesystem-manager", "action:execute-process", "skill:git-manager"]
```

---

## Rebaseline de olas

| Orden | Ola | Hito | Dependencia dura | Resultado |
|-------|-----|------|-----------------|-----------|
| 0 | **O0 — Contención de integridad** | H1R | H0 | Persistencia segura: igualdad exacta y errores tipados |
| 1 | **O1 — Contrato v2 íntegro** | H1C | O0 | Puerto, adapters, Genoma y suite v2 completos |
| 2 | **O2 — Estado y testigos** | H2 | O1 | Entregas/terminales sin FS |
| 3 | **O3 — Cutover centinelas** | H3 | O2 | Watcher/sweeper solo puerto |
| 4 | **O4 — Periferia limpia** | H4 | O1; paralela a O2 | Productores/bridges sin FS ni paths físicos |
| 5 | **O5 — Demolición** | H5 | O2–O4 | Legacy y allowlists eliminados |
| 6 | **O6 — Convergencia** | H6 | O5 | AC-1…19 evidenciados y `APTO_TOTAL` |

Regla de corte: ninguna ola posterior puede usar `contains(event_id)` ni matching textual de errores como deuda tolerada.

---

## H0 — Precondiciones de forja (L-FORGE)

| # | Entregable | Detalle |
|---|------------|---------|
| H0.1 | Inventario consumidores v1 | Lista cerrada en `implementation.md` o anexo del feature (motor/daemons/bridges/tools/tests) |
| H0.2 | Ops v2 checklist | Confirmar §4.3 de `spec.md` cubre sweeper/testigos; ampliar contrato si falta algo **antes** de H3 |
| H0.3 | Política residuos `.events/` | Migrar ECST vivos → puerto; higiene FS ops; nunca SSOT |
| H0.4 | Orden cutover | Validar §8 `spec.md`; no watcher huérfano |
| H0.5 | Estrategia tests por hijo | Mapear AC→comandos; contract suite + gates |
| H0.6 | `forge_key` | Materializar si el runtime lo exige antes de código (**ausente** hoy) |

**Commit (si aplica docs-only):** `docs(eda): precondiciones forja MAN-43`.

---

## O0 / H1R — Contención de integridad

Estado: **hecho en rama** (`72108157`, `docs/man-43-post-merge-handoff`). H1 histórico del PR #453 permanece intacto. O1 es el siguiente hito.

### Corte

O0 corrige identidad y errores. No persiste `EventFamily`, no crea schema v2, no corta watcher/sweeper y no toca Genoma. Esas piezas siguen en O1 y se mutan vía `entity-manager`.

### Defecto vivo

| Adaptador | Escritura actual | Lectura/purga actual |
|-----------|------------------|----------------------|
| Redb | `em_deliveries[delivery_id] = DeliveryStatus` | `contains(event_id)` y `contains` sobre claves de traza |
| FS | `{delivery_id}.json = DeliveryStatus`; purga no borra entregas ni trazas | Sin FK recuperable |
| Mock | Guarda `event_id` y purga por igualdad | Filtro de suscriptor usa `delivery_id.contains(subscriber_id)` |
| Bridge | — | Trata como duplicado cualquier `InvalidRequest` cuyo texto contenga `duplicate` |

`RecordDeliveryRequest` no tiene `subscriber_id`, aunque `ListDeliveriesRequest` sí lo filtra. Sin ese campo persistido, el filtro no puede ser exacto.

### Diseño cerrado

```rust
enum EventManagerError {
    AlreadyExists { entity: String, id: String },
    // variantes vigentes se conservan
}

struct DeliveryRecord {
    event_id: String,
    subscriber_id: Option<String>,
    delivery_id: String,
    status: DeliveryStatus,
    attempt: Option<u32>,
}
```

1. Añadir `subscriber_id: Option<String>` a `RecordDeliveryRequest` con `serde(default)`. Los callers actuales siguen compilando.
2. Las tres implementaciones guardan `DeliveryRecord` completo. La clave física puede seguir siendo `delivery_id`; la pertenencia sale del registro.
3. Lectura compatible mediante enum explícito: `Current(DeliveryRecord)` o `LegacyStatus(DeliveryStatus)`.
4. Una fila legacy no tiene `event_id` fiable. `list_deliveries` la excluye de cualquier filtro de evento o suscriptor y no la reinterpreta. `purge_event` no la borra. Queda como residuo diagnosticable, nunca como coincidencia textual.
5. `purge_event` borra entregas actuales solo cuando `record.event_id == req.event_id`.
6. La traza se borra únicamente por la clave exacta `StoredEvent.trace_id`. Una traza cuyo identificador contenga el `event_id` permanece.
7. Publicación duplicada devuelve `AlreadyExists`. `runtime_bridge::publish_ecst` solo absorbe esa variante; cualquier otro error se propaga.
8. ACK repetido del mismo `delivery_id` conserva la idempotencia actual y no se convierte en error.

### Secuencia

| Paso | Archivo | Cambio |
|------|---------|--------|
| 1 | `sddia-core/.../error.rs` | Variante `AlreadyExists`; helper de construcción |
| 2 | `sddia-core/.../contract.rs` | `subscriber_id` opcional en `RecordDeliveryRequest` |
| 3 | `sddia-core/.../memory_mock.rs` | Persistir suscriptor; filtro y purga por igualdad; publicar `AlreadyExists` |
| 4 | `sddia-core/.../runtime_bridge.rs` | Match exclusivo de `AlreadyExists` |
| 5 | `redb_event_adapter/src/lib.rs` | Valor `DeliveryRecord`, lectura versionada, purga/listado exactos |
| 6 | `fs_event_adapter/src/lib.rs` | Mismo registro y las mismas reglas; purga también retira su entrega y su traza exacta |
| 7 | `sddia-core/.../contract_suite.rs` | Casos de colisión compartidos por mock, Redb, FS y cápsula externa |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| Eventos `abc` y `xabcx` | Purga de `abc` conserva `xabcx`, sus entregas y su traza |
| Entregas `abc:sub` y `xabcx:sub` | Cada filtro por `event_id` devuelve exactamente una |
| Suscriptores `sub` y `sub-2` | El filtro no usa substring del `delivery_id` |
| Traza `xabcx-log` | Sobrevive a la purga del evento `abc` |
| Fila legacy con solo `DeliveryStatus` | No se asigna a ningún evento ni se elimina por purga |
| Segunda publicación | `AlreadyExists`; el bridge la trata como idempotente |
| `InvalidRequest("duplicate …")` | El bridge lo devuelve como error |

### Verificación

```bash
cd SddIA && cargo test -p sddia-core -p sddia-infrastructure-redb-event -p sddia-infrastructure-fs-event -p sddia-infrastructure-external-capsule-event
```

El verde de la suite anterior no basta: debe incluir los casos de colisión nuevos.

**Commit:** `fix(eda): identidad exacta en entregas y purga (MAN-43 O0)`.

**AC:** AC-15, AC-18.
**Bloqueo:** O1–O5 no avanzan si los casos de colisión fallan.

---

## O1 / H1C — Contrato v2 íntegro (siguiente hito)

Estado: **hecho en rama** (código + suite; Genoma 2.0 pendiente vía `entity-manager`). Depende de O0 (`72108157`). No corta watcher/sweeper, no demuele store y no reescribe Constitución ni `cumulo.paths.json` (O6).

### Corte

O1 persiste identidad y estado, y hace que los filtros dejen de ser stubs. `EventFamily`, `EventUri`, `CONTRACT_VERSION=2.0.0` y `version_gate` ^2 ya existen en kernel. Lo que falta es el modelo durable, las operaciones que hoy ignoran dimensiones, la suite que lo demuestra y la convergencia Genoma 1.2 → 2.0.

### Defecto vivo (post-O0)

| Pieza | Hoy | Hueco |
|-------|-----|-------|
| `PublishEcstRequest` | `event_id` + `ecst` + `trace_id` | Sin `event_uri`; la familia no entra al puerto |
| `StoredEvent` (mock, Redb, FS) | `event_id`, `ecst`, `trace_id` | Sin `event_uri` ni `state` persistidos; el estado se infiere de tabla/directorio |
| `list_events` (default en `port.rs`) | Reusa `list_pending` y filtra solo `event_type` | Ignora `family` y `cursor`; no hay campo `state` |
| `list_terminal_events` (default) | Solo `list_dlq` | Omite `processed`; ignora familia y cursor |
| `list_deliveries` | Igualdad exacta (O0) | `next_cursor` siempre `None` |
| Genoma | `eda.event_manager.v1.schema.json`; taxonomía `eda:event-manager` **1.2.0** | Kernel declara 2.0.0 (F-02 / L-9) |
| Cápsula dummy | Ops O0 | Sin `list_events` / `list_terminal_events` |

### Diseño cerrado

```rust
struct PublishEcstRequest {
    event_uri: EventUri,
    event_id: String, // debe ser igual a event_uri.event_id
    ecst: Value,
    trace_id: Option<String>,
}

struct StoredEvent {
    event_uri: EventUri,
    state: EventQueueState, // pending | claimed | processed | dead_letter
    ecst: Value,
    trace_id: Option<String>,
}

struct ListEventsRequest {
    family: Option<String>,
    event_type: Option<String>,
    state: Option<EventQueueState>, // campo nuevo, serde(default)
    limit: Option<u32>,
    cursor: Option<String>,
}
```

1. Publicar exige `event_uri` válido. `event_id != event_uri.event_id` → `InvalidRequest`. Familia inválida o estado de cola usado como familia → error de parseo, no se publica.
2. Cada adapter persiste `StoredEvent` completo. El mapa/tabla física puede seguir partido; el campo `state` es la fuente al listar. Claim, ACK, DLQ y replay actualizan `state` y mueven el registro.
3. Lectura versionada: `Current(StoredEvent)` o `Legacy` (fila O0/v1 sin `event_uri`). Una fila legacy no recibe familia inventada. Cualquier filtro `family` o `state` la excluye. Sin filtros puede aparecer en el listado, sin `EventFamily` asignada.
4. `list_events` aplica las cuatro dimensiones. `family` no parseable → `InvalidRequest`. No se ignora un filtro presente.
5. `list_terminal_events` devuelve solo `processed` y `dead_letter`, con `family`, `cursor` y `limit`. Pending y claimed no aparecen.
6. Cursor opaco = último `event_id` (eventos) o `delivery_id` (entregas) de la página, orden lexicográfico ascendente, exclusivo. `next_cursor` es `None` si la página agota el conjunto. Cursor desconocido → `InvalidRequest`.
7. Quitar los cuerpos por defecto de `list_events` y `list_terminal_events` en el trait. Mock, Redb, FS y cápsula los implementan. Un default que consulta solo pending o solo DLQ queda prohibido.
8. `runtime_bridge::publish_ecst` construye `EventUri` desde el token de familia ya aceptado (`pending`→`domain`) y rellena `event_uri` + `event_id`. La idempotencia sigue siendo solo `AlreadyExists` (O0).
9. Genoma al final de la ola, solo vía `./sddia-run.sh --process entity-manager`. Prohibido editar a mano `SddIA/library/norms/capability-contracts/`, `capability-taxonomy.md` y `capability-bindings`. Objetivo: schema v2, catálogo `eda:event-manager` **2.0.0**, bindings alineados. Constitución y Cúmulo quedan en O6.

### Secuencia

| Paso | Archivo | Cambio |
|------|---------|--------|
| 1 | `sddia-core/.../contract.rs` | `event_uri` en `PublishEcstRequest`; `state` en `ListEventsRequest` |
| 2 | `sddia-core/.../port.rs` | `list_events` y `list_terminal_events` obligatorios, sin default |
| 3 | `memory_mock.rs` | `StoredEvent` v2; transiciones de `state`; listados y cursor |
| 4 | `runtime_bridge.rs` | Publicación con `EventUri`; callers internos del request |
| 5 | `redb_event_adapter` / `fs_event_adapter` | Persistencia v2, parseo legacy, mismos listados |
| 6 | Cápsula | `list_events` y `list_terminal_events` en dummy y puente |
| 7 | Call sites del request | `contract_suite`, `eda_port`, `event-watcher`, `event-bus-audit` y cualquier literal que deje de compilar |
| 8 | `contract_suite.rs` | Casos O1 de la tabla siguiente, ejecutados por mock, Redb, FS y cápsula |
| 9 | Genoma | `entity-manager`: schema v2 + taxonomía + bindings 2.0.0 |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| Publicar `domain` y `telemetry` con el mismo `event_type` | `list_events(family=domain)` devuelve solo el de domain |
| `family=pending` | Rechazo; no se persiste |
| `event_id` distinto de `event_uri.event_id` | `InvalidRequest` |
| Claim y luego ACK | `state` pasa pending → claimed → processed |
| `move_to_dlq` | `list_events(state=dead_letter)` lo ve; `list_pending` no |
| `list_terminal_events` | Incluye processed y dead_letter; excluye pending y claimed |
| Página `limit=1` sobre tres ids ordenados | Dos cursores cubren los tres sin duplicar ni omitir |
| Fila legacy sin `event_uri` | Fuera de cualquier filtro `family` o `state`; la purga O0 no cambia |
| Reopen Redb | Familia, estado, entrega y traza del evento nuevo sobreviven |
| Suite O0 | Sigue verde (colisiones, `AlreadyExists`) |

### Verificación

```bash
cd SddIA && cargo test -p sddia-core -p sddia-infrastructure-redb-event -p sddia-infrastructure-fs-event -p sddia-infrastructure-external-capsule-event
```

Tras el paso 7, el workspace que toque esos crates debe compilar. Genoma no se da por cerrado hasta el acuse de `entity-manager` y la taxonomía en 2.0.0.

**Commit de código:** `feat(eda): contrato v2 persistido y listados reales (MAN-43 O1)`.
**Commit de genoma:** el que selle `entity-manager` (no un edit manual).

**AC:** AC-1, AC-2, AC-16, AC-17, AC-19. AC-3 ya cubierto en el kernel (sin `logical_rel` / `target_path`). AC-18 permanece cubierto por O0.
**Fuera de esta ola:** watcher, sweeper, periféricos, demolición del store, Constitución, `cumulo.paths.json`, evolution de cierre.
**Bloqueo:** O2 y O3 no avanzan si familia, estado, terminales o cursor fallan. O4 puede esperar a O1, no a O2.

---

## H1 — Contrato EventManager 2.0 + EventUri (índice)

El detalle ejecutable está en **O1 / H1C**. H1.1, H1.2, H1.4 y H1.6 quedaron en el PR #453. O1 cierra H1.3, H1.5, H1.7, H1.8 y H1.9.

| # | Entregable | Ola |
|---|------------|-----|
| H1.1–H1.2, H1.4, H1.6 | `EventUri`, `PublishSeal`, constantes 2.0.0, `version_gate` | Hecho (#453) |
| H1.3, H1.7–H1.9 | Request con `EventUri`, listados reales, suite, adapters, `StoredEvent` | **O1** |
| H1.5 | Schema/bindings/taxonomía | **O1** vía `entity-manager` |

---

## O2 / H2 — Estado y testigos

Estado: **hecho en rama** (`de2161ed`). Depende de O1 (`b6a73ba2`). Genoma 2.0 sigue pendiente y no bloquea O3. Cabeceras `.events/` y watcher: O3.

### Corte

O2 hace que el testigo de suscriptor sea una entrega del puerto. No retira las cabeceras `.events/{pending,processing,…}`: el watcher aún descubre por directorio y cortarlas aquí lo deja huérfano (L-4). Esas copias (`ensure_state_header`, `copy_header_atomic`) salen en O3, en el mismo corte que `read_dir`.

### Defecto vivo

| Pieza | Hoy | Hueco |
|-------|-----|-------|
| `promote_witness` | `write_json_atomic` de `{event}.{subscriber}.json` y además ACK o `move_to_dlq` | El fichero sigue siendo evidencia; el puerto no es la única |
| `record_delivery_ack` | El nombre de suscriptor entra como `delivery_id`; `subscriber_id` queda `None` | El filtro exacto de O0 no puede usarse |
| `list_port_witnesses` | Acepta `delivery_id.contains(event_id)` y parte el id con `rsplit(':')` | Reabre el matching textual que O0 cerró |
| `list_witnesses` | En test: FS **o** `sddia-event-store` **o** puerto | Doble SSOT bajo `cfg(test)` |
| `terminal_witness_exists` | Prod: sello `delivery_state` dentro del ECST y, si no, `false`. Test: `is_file` | Prod no consulta entregas; test consulta disco |
| `stamp_fractal_delivery_state` | Prod ya llama ACK. `cfg(test)` reescribe el JSON del evento | Mutación residual del fichero |
| `write_processing_witness` | No escribe; devuelve una ruta lógica | El estado `processing` no existe en `DeliveryStatus` |

`github` DL y el espejo de `actions.rs` quedan en O4. `eda_port_legacy` y el crate store quedan en O5: O2 deja de **consultarlos** como testigo, no los borra.

### Diseño cerrado

1. `record_delivery_ack` / `record_delivery_nack` reciben `subscriber_id` y lo persisten. `delivery_id` estable = `{event_id}:{subscriber_id}`. ACK repetido del mismo id sigue siendo noop (O0).
2. `DeliveryStatus` gana `Processing`. `write_processing_witness` registra esa variante en el puerto y no crea fichero. La ruta que devuelve es un token lógico para los callers actuales; nadie hace `is_file` sobre ella como prueba de estado.
3. `promote_witness` no llama a `write_json_atomic`. `processed` → ACK con `subscriber_id`. `dead-letter` → NACK con `subscriber_id` y, si la causa no es contención de lock (MAN-42), `move_to_dlq` como hoy. Devuelve el mismo token lógico, sin fichero.
4. `list_witnesses` es siempre `list_port_witnesses`, también en test. Filtro: `event_id` igual y `subscriber_id` presente. ACK → processed, NACK → dead-letter, `Processing` → processing. El nombre del suscriptor sale del campo, no de un substring del `delivery_id`. Prohibido `contains`.
5. `terminal_witness_exists` consulta solo esas entregas terminales (ACK o NACK) por `subscriber_id`. Deja de leer `delivery_state` del ECST y deja de usar `is_file`.
6. `stamp_fractal_delivery_state` solo registra ACK/NACK en el puerto. Se elimina el bloque `cfg(test)` que reescribe el JSON.
7. `list_fs_witnesses` y `list_store_witnesses_legacy` salen del camino de `list_witnesses`. No se borran el crate ni `eda_port_legacy` en esta ola.

### Secuencia

| Paso | Archivo | Cambio |
|------|---------|--------|
| 1 | `sddia-core/.../contract.rs` | `DeliveryStatus::Processing` |
| 2 | `runtime_bridge.rs` | ACK/NACK con `subscriber_id` y `delivery_id` compuesto |
| 3 | `eda_port.rs` | Misma firma hacia el puerto |
| 4 | `eda_bus_topology.rs` | `promote_witness` y `write_processing_witness` sin FS; listado y terminal solo por entregas |
| 5 | `fractal_bus.rs` | Sello solo por puerto, también en test |
| 6 | `route_domain_core.rs` y `tests/event_store_writers.rs` | Aserciones de testigo vía `list_deliveries` / `terminal_witness_exists`, no `is_file` del testigo |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `promote_witness(..., "processed")` | Existe entrega ACK con ese `subscriber_id`; no aparece `{event}.{subscriber}.json` |
| `promote_witness(..., "dead-letter")` con causa normal | Entrega NACK y evento en DLQ; sin fichero de testigo |
| Causa de contención de lock | Sin `move_to_dlq`; entrega no terminal de DLQ |
| Dos suscriptores `sub` y `sub-2` | El terminal de uno no marca al otro |
| `list_witnesses` | No llama a `read_dir` ni a `EventStore::list_deliveries` |
| `stamp_fractal_delivery_state` | El JSON del evento no cambia; la entrega sí queda en el puerto |
| Suite O0/O1 | Sigue verde |

### Verificación

```bash
cd SddIA && cargo test -p execute-process -p sddia-core --lib
```

El verde de la suite de contrato no basta: debe incluir un test de `promote_witness` sin fichero.

**Commit:** `refactor(eda): testigos de suscriptor detrás del puerto (MAN-43 O2)`.

**AC:** AC-7 en el plano del testigo. AC-5 queda parcial: las cabeceras `.events/` y el watcher siguen hasta O3; periféricos hasta O4.
**Fuera de esta ola:** `ensure_*_header`, event-watcher, sweeper, `actions.rs`, Kalma2, GitHub DL, demolición del store, Genoma.
**Bloqueo:** O3 no arranca si `promote_witness` sigue escribiendo JSON o si el terminal depende de `is_file`.

---

## H2 — Absorción de entregas y testigos (índice)

El detalle ejecutable está en **O2 / H2**. H2.4 (GitHub DL) se ejecuta en O4.

| # | Entregable | Ola |
|---|------------|-----|
| H2.1–H2.3 | Testigo = entrega; sin JSON de suscriptor; fractal sin mutar el fichero | **O2** |
| Cabeceras de estado `.events/` | Se retiran con el watcher | **O3** |
| H2.4 | `move_to_dlq` en el bridge de GitHub | **O4** |

---

## O3 / H3 — Cutover de centinelas (siguiente hito)

Estado: **done en rama** (`f3b1da12` + O3.1). Dominio claim + `list_pending`; fractal/DL por puerto. Progress leaf: indexación de dirs residual. Dependencia dura O2 (`de2161ed`) cumplida: `promote_witness` ya no escribe JSON de suscriptor y `terminal_witness_exists` consulta entregas. Genoma 2.0 (O1 H1.5) y O4 no bloquean este corte. L-4: el watcher no puede quedar sin fuente; el mismo tren retira el descubrimiento por directorio y las cabeceras que solo existían para ese descubrimiento.

### Corte

O3 hace que el ciclo de vida del evento (descubrir, enrutar, cerrar, barrer) lea y escriba solo el puerto. Desaparece el segundo descubrimiento `read_dir` del watcher (F-09) y el barrido que abre `.events/{pending,domain,orchestration,telemetry,progress}`.

En el mismo corte salen `ensure_state_header`, `ensure_processing_header` y `copy_header_atomic` de `eda_bus_topology.rs`, y el borrado de cabeceras dentro de `archive_event_after_sweep` / `finalize_kaizen_terminal`. Esas copias no son SSOT desde O2; si se dejan, el sweeper sigue tratando el fichero como evidencia.

No entra O4 (`actions.rs`, Kalma2, workspace-server, `github_bridge`) ni O5 (`eda_port_legacy`, crate `sddia-event-store`, allowlists). `SddIA/events/` y `.SddIA/events/` permanecen.

### Defecto vivo

| Pieza | Hoy | Hueco |
|-------|-----|-------|
| `daemons/event-watcher/src/main.rs` `run_watcher` | Tras `claim_store_pending`, el bucle recorre `watch_targets` con `read_dir` y lanza `route-domain-event` por cada JSON | Doble despacho (F-09): claim y directorio compiten |
| `claim_store_pending` | `claim_pending` correcto; luego fabrica `.events/pending/{id}.json` y `record_delivery` con `delivery_id = event_id` y sin `subscriber_id` | El enrutado exige un path; el ACK del watcher reabre el id opaco que O0/O2 cerraron |
| `invoke_route_process` | Payload `event_file_path` | El proceso no puede arrancar si la cabecera no existe |
| `eda_bus_topology::try_sweep_event` | Carga el ECST por puerto, pero `archive_event_after_sweep` y `finalize_kaizen_terminal` borran/copian cabeceras; `pending_tracker_sync_failed_for_ref` hace `read_dir` de pending; `processed_subscriber_names` parte el nombre `{event}.{sub}.json` | El terminal de O2 se vuelve a interpretar como fichero |
| `sddia-daemon-runtime/src/eda_sweep.rs` | `sweep_once` lista pending por `read_dir`; `sweep_fractal_dir` lee JSON y decide purga/DLQ con `delivery_state` del cuerpo; `pec_terminal_for_correlation` y `sweep_progress_leaf` barren orchestration/progress en disco | F-5: el barrido no usa `list_terminal_events` / `list_events` / `list_progress` |

`list_terminal_events` ya está en el trait y en Redb/FS/mock (O1). No hace falta ampliar el contrato antes de este corte (L-6 cubierto para el sweeper).

### Diseño cerrado

1. El tick del watcher de dominio es solo `claim_pending`. Se elimina el `read_dir` de `route-domain-event`. Telemetría y orquestación del mismo bucle dejan de descubrir por directorio: o salen del tick, o reclaman por `list_events` de su familia. Un `event_id` no se enruta dos veces en el mismo tick (el book de in-flight sigue, ahora keyed por id, no por path).
2. El spawn de `route-domain-event` recibe `event_id` (y familia `domain`). Prohibido exigir que `.events/pending/{id}.json` exista. La carga sigue siendo `load_dispatched` / `get_event`.
3. El ACK/NACK que el watcher escribe al cerrar la ruta usa `subscriber_id` del suscriptor de ruta y `delivery_id = {event_id}:{subscriber_id}` (`record_delivery` del puerto, misma forma que O2). No se usa el `event_id` pelado como `delivery_id`.
4. `try_sweep_event` decide purga, DLQ y kaizen con entregas del puerto (`list_deliveries` / `terminal_witness_exists`) y con el ECST cargado por id. `pending_tracker_sync_failed_for_ref` busca `Tracker_Sync_Failed` vía `list_events`, igualdad de `event_type` y del ref, sin `read_dir`.
5. `archive_event_after_sweep` y `finalize_kaizen_terminal` no copian ni borran cabeceras. El cierre terminal llama `purge_event` (o la transición de estado que el puerto ya exponga). Si queda un JSON residual en `.events/`, es higiene posterior, no condición del barrido.
6. `sweep_once` obtiene candidatos con `list_pending` + `list_terminal_events` (y `list_events` por familia fractal). `sweep_fractal_dir` no abre ficheros. La terminalidad fractal sale de entregas ACK/NACK, no de `delivery_state` embebido. Progress: `list_progress` + TTL; `Process_Execution_Completed` se localiza con `list_events`, no leyendo `.events/orchestration`.
7. `ensure_state_header`, `ensure_processing_header` y `copy_header_atomic` desaparecen del motor en este commit. Quien aún las llame se corta aquí, no se aplaza a O4.

### Secuencia

| Paso | Archivo | Cambio |
|------|---------|--------|
| 1 | `daemons/event-watcher/src/main.rs` | Tick dominio = `claim_pending`; spawn por `event_id`; ACK/NACK con `subscriber_id`; sin `read_dir` de pending |
| 2 | `engine/.../route_domain_core.rs` | `resolve_route_event_path` acepta `event_id` sin fichero |
| 3 | `engine/.../eda_bus_topology.rs` | Sweeper y archivo sin cabeceras; tracker-sync vía `list_events`; borrar `ensure_*_header` / `copy_header_atomic` |
| 4 | `sddia-daemon-runtime/src/eda_sweep.rs` | `sweep_once` y fractal/progress solo puerto |
| 5 | Tests del watcher y de `try_sweep_event` | Un evento claimado no se relanza por JSON; sweep sin `.events/` presente |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| Pending solo en el puerto, directorio `.events/pending` vacío | El watcher enruta ese `event_id` una vez |
| El mismo id ya está in-flight y además existe un JSON homónimo | Un solo spawn en el tick |
| ACK de cierre del watcher | Entrega con `subscriber_id` de la ruta; `delivery_id` compuesto; sin JSON nuevo |
| `try_sweep_event` con todas las entregas terminales | `purge_event`; no crea ni borra `{id}.json` como prueba |
| `Tracker_Sync_Failed` solo en el puerto | `pending_tracker_sync_failed_for_ref` lo ve sin `read_dir` |
| Fractal con entregas ACK y sin ficheros en `.events/domain` | El sweep lo trata como purgable |
| Suite O0/O1/O2 | Sigue verde, incluido `promote_witness_does_not_materialize_subscriber_json` |

### Verificación

```bash
cd SddIA && cargo test -p execute-process --lib try_sweep
cd SddIA && cargo test -p sddia-daemon-runtime eda_sweep
cd SddIA && cargo test -p event-watcher
```

El verde de O2 no sustituye un test de tick sin `read_dir`.

**Commit:** `refactor(eda): centinelas solo por puerto (MAN-43 O3)`.

**AC:** AC-4 y AC-6 en el plano del centinela. AC-5 sigue parcial hasta O4 (periféricos).
**Fuera de esta ola:** `actions.rs`, Kalma2, workspace-server, GitHub DL, demolición del store, Genoma 2.0, scripts de higiene ops.
**Bloqueo:** O5 no arranca sobre el watcher si `run_watcher` sigue haciendo `read_dir` o si el sweep decide por `is_file` / `delivery_state` del JSON. Ese residuo es **O3.1**, no O4.

---

## O3.1 / H3 — Fractal, progress y DL sin disco (siguiente hito)

Estado: **hecho en rama** (pendiente commit). `f3b1da12` cerró el pending de dominio. El diseño O3 puntos 1 (familias no-pending) y 6 (fractal/progress) no se cumplieron. O4 no sustituye este corte: Kalma2 y GitHub no son la fuente del sweeper.

### Corte

Un tick de telemetría, dominio fractal y orquestación descubre por `list_events` de esa familia, no por `read_dir`. El sweep de esas familias y de progress decide con entregas y listados del puerto. `list_witnesses` del daemon deja de abrir carpetas de suscriptor.

`SddIA/events/` y `.SddIA/events/` permanecen. O4 (`actions.rs`, Kalma2, workspace-server, GitHub) y O5 (borrar store) quedan fuera.

### Defecto vivo

| Pieza | Hoy | Hueco |
|-------|-----|-------|
| `event-watcher` `watch_targets` | Sigue `read_dir` de telemetry, domain y orchestration; el skip fractal lee `delivery_state` del JSON | F-09 residual fuera de pending |
| `has_dead_letter_witnesses` | `read_dir` de `dead_letter/subscribers` con prefijo `{event_id}.` | Reabre el matching textual que O0 cerró |
| `eda_sweep::sweep_fractal_dir` | Lee cada JSON y purga/DLQ según `delivery_state` del cuerpo | AC-6: el disco sigue siendo la prueba |
| `pec_terminal_for_correlation` / `sweep_progress_leaf` | `read_dir` de orchestration y de hojas progress | TTL y PEC no consultan `list_events` / `list_progress` |
| `eda_sweep::list_witnesses` | `read_dir` + `starts_with("{event_id}.")` | El motor ya lista por puerto; el daemon no |

### Diseño cerrado

1. El tick no-pending llama `list_events` con `family` `telemetry` | `domain` | `orchestration` y estado no terminal. Heartbeats se ordenan por `event_type == Daemon_Heartbeat` en el ECST devuelto, sin abrir el fichero. Un `event_id` ya in-flight no se relanza.
2. `has_dead_letter_witnesses` es `list_deliveries` con `event_id` exacto y `status = Nack`. Cero prefijo de nombre.
3. `sweep_fractal_dir` itera `list_events` de la familia. Purga si todas las entregas aplicables son ACK. DLQ si hay NACK y el resto es terminal. No lee ni renombra JSON. El cierre terminal usa `purge_event` / `move_to_dlq`.
4. `pec_terminal_for_correlation` busca `Process_Execution_Completed` con `list_events(family=orchestration, event_type=...)` y compara `payload.correlation_id`. Progress caducado sale de `list_progress` (o del sello temporal del frame), no de `metadata` de un directorio.
5. `list_witnesses` del daemon delega en `list_deliveries` (igual que `eda_bus_topology::list_port_witnesses`): ACK → processed, NACK → dead-letter, Processing → processing. Prohibido `starts_with` / `contains`.
6. Tests: fractal ACK sin ficheros se purga; un JSON homónimo en `.events/domain` no dispara un segundo spawn si el id ya está in-flight; DL de un suscriptor no marca a otro.

### Secuencia

| Paso | Archivo | Cambio |
|------|---------|--------|
| 1 | `sddia-daemon-runtime/src/eda_sweep.rs` | `list_witnesses`, `sweep_fractal_*`, progress/PEC solo puerto |
| 2 | `daemons/event-watcher/src/main.rs` | Tick fractal por `list_events`; DL por `list_deliveries`; skip sin leer JSON |
| 3 | Tests `eda_sweep` / watcher | Casos de la tabla anterior |

### Verificación

```bash
cd SddIA && cargo test -p sddia-daemon-runtime eda_sweep
cd SddIA && cargo build -p event-watcher
```

**Commit:** `refactor(eda): sweep fractal y progress solo por puerto (MAN-43 O3.1)`.

**AC:** AC-6 en fractal/progress. AC-5 sigue parcial hasta O4.
**Fuera:** periféricos F-1…F-4, demolición del crate store, Genoma 2.0.
**Bloqueo:** O4 puede avanzar en paralelo sobre productores; O5 no, mientras `sweep_fractal_dir` o `list_witnesses` del daemon llamen a `read_dir`.

---

## H3 — Watcher y sweeper sin filesystem (índice)

El detalle ejecutable está en **O3 / H3**.

| # | Entregable | Ola |
|---|------------|-----|
| H3.1–H3.2 | Tick = `claim_pending`; cierre ACK/NACK con `subscriber_id`; sin `event_file_path` obligatorio | **O3** |
| H3.3–H3.4 | Pending por `list_pending`; fractal/progress/DL del daemon | **O3** dominio, **O3.1** fractal |
| H3.5 | Un enrutado por `event_id` y tick | **O3** |
| H3.6 | Purga de residuos `.events/` como higiene, no como SSOT | **O3** (ops) / **O6** (docs) |

---

## O4 / H4 — Periféricos sin fallback FS

Estado: **done (rama)**. O1 desbloqueó el puerto. O3.1 (`1a4c84af`) no es dependencia: estos productores no son el watcher. El residual `sweep_progress_leaf` (directorios de correlación) no entra aquí.

### Corte

Cuatro escritores dejan de materializar ECST o DLQ en `.events/`. Quien hoy abre el JSON para reseal, accept-pr o la UI de Kalma2 resuelve por `get_event` / `list_events`. `runtime_db_path` sale de los callers productivos; la función y el crate store siguen hasta O5.

Fuera: Genoma 2.0, demolición de `eda_port_legacy` / `sddia-event-store`, matrices, `sweep_progress_leaf`, `SddIA/events/` y `.SddIA/events/`.

### Defecto vivo

| Pieza | Hoy | Hueco |
|-------|-----|-------|
| `actions.rs` `emit_pr_presented` | `publish_ecst` y además `write_json_atomic` en `.events/domain/{id}.json` | F-1: doble SSOT; el comentario cita ese path como ancla de reseal |
| `kalma2-bridge` `find_domain_event_with_proofs` y barridos de orchestration/progress/telemetry | Si el puerto responde, lo usa; si no, `read_dir` / `is_file` bajo `.events/` | F-2: el disco sigue siendo plan B |
| `sddia-workspace-server` `emit_raw_execution_finished` | `fs::write` de `Raw_Execution_Finished` en `.events/telemetry` | F-3: no pasa por `publish_ecst` |
| `github_bridge` `write_fallback_dead_letter` | JSON en `dead_letter/` cuando IOTA falla | F-4: el DLQ del puerto no ve el fallo |
| `eda_port::runtime_db_path` | Lo abren tests y también ramas de `actions` / vitality / accept-pr | AC-8: el consumidor productivo no debe nombrar el fichero `.redb` |

### Diseño cerrado

1. `emit_pr_presented` publica solo por puerto (`domain` o `pending`, el que ya sella el catálogo) y devuelve `event_id` + `event_uri`. No escribe ancla. Reseal y accept-pr cargan ese id con `get_event`.
2. Kalma2: si `store_active` es falso, la consulta devuelve vacío. Se borran las ramas `is_file` / `read_dir` de `.events/{domain,orchestration,dead-letter,progress,telemetry}`. Las pruebas `.SddIA/proofs/` no son bus EDA y se quedan.
3. `emit_raw_execution_finished` arma el ECST y llama `publish_ecst` familia `telemetry`. `write_capsule_envelope` es artefacto de workspace, no bus: no se toca.
4. El fallo IOTA de GitHub llama `move_to_dlq` con el ECST ya compuesto y la causa. `write_fallback_dead_letter` desaparece. El token `.events/domain/{id}.json` que devuelve `materialize_to_bus` es lógico; nadie hace `is_file` sobre él.
5. Callers productivos de `runtime_db_path` / `EventStore::open` en `actions`, vitality y accept-pr pasan al puerto (`list_events`, `get_event`, `list_deliveries`). Tests y `eda_port_legacy` esperan a O5.

### Secuencia

| Paso | Archivo | Cambio |
|------|---------|--------|
| 1 | `engine/.../actions.rs` | Sin ancla FS; consumidores de Presented por `event_id` |
| 2 | `interfaces/kalma2-bridge/src/main.rs` | Sin fallback `.events/` |
| 3 | `tools/sddia-workspace-server/src/telemetry.rs` | `publish_ecst` telemetry |
| 4 | `sddia-daemon-runtime/src/github_bridge.rs` | DLQ por `move_to_dlq` |
| 5 | vitality / accept-pr | Dejar de abrir `.redb` |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `emit_pr_presented` | Existe el evento en el puerto; no aparece `.events/domain/{id}.json` |
| Kalma2 con puerto vacío y un JSON homónimo en `.events/domain` | No lo lee |
| `Raw_Execution_Finished` | Familia `telemetry` en `list_events`; cero fichero nuevo |
| Fallo IOTA del bridge | El evento está en `list_dlq`; cero JSON en `dead_letter/` |
| Suite O0–O3.1 | Sigue verde |

### Verificación

```bash
cd SddIA && cargo test -p execute-process emit_pr
cd SddIA && cargo test -p kalma2-bridge
cd SddIA && cargo test -p sddia-daemon-runtime github_bridge
cd SddIA && cargo test -p sddia-workspace-server
```

**Commit:** `refactor(eda): periféricos sin fallback FS (MAN-43 O4)`.

**AC:** AC-8 en los callers productivos de este corte. AC-9 en los cuatro escritores. AC-5 deja de estar parcial en F-1…F-4; el crate store y el progress leaf no cuentan como AC-5 cerrado.
**Fuera:** O5, Genoma, `sweep_progress_leaf`.
**Bloqueo:** O5 no borra el store mientras `emit_pr_presented` o `write_fallback_dead_letter` sigan escribiendo JSON — **levantado en rama O4**.

---

## O5 — Lectores residuales sin disco

Estado: **done (rama)**. O4 (`c1700a47`) levantó el bloqueo de escritores. **L-7** impide borrar `sddia-event-store` en este hilo: `execute-process` y `event-bus-audit` siguen dependiendo del crate, `eda_port` bajo `cfg(test)` aún desvía `StoreMode::Filesystem` a `eda_port_legacy`, y las matrices `eda_store_io_*` / `eda_subscriber_*` no están migradas. Eso es **O5.1**.

### Corte

Tres lectores productivos dejan de tratar `.events/` como fuente. El puerto ya responde `load_domain_ecsts` / `list_events` / `pec_terminal_for_correlation`. Un JSON homónimo en disco no cambia el resultado.

| Pieza | Hoy | Hueco |
|-------|-----|-------|
| `accept_pr.rs` `scan_presented_for_branch` | Si `presented_in_store` falla, `read_dir` de `.events/{pending,processed,dead-letter}` | El disco sigue siendo plan B de Presented |
| `tracker_pbi_meta.rs` `presented_ids_in_store` | Tras el puerto, `consider_presented_from_dir` sobre domain/pending/processed/dead-letter | El `event_id` de reseal puede salir de un JSON huérfano |
| `eda_sweep.rs` `sweep_progress_leaf` | `read_dir` de `.events/progress/{correlation}` y `remove_dir_all` si hay PEC terminal o TTL | El barrido decide por directorios; los frames viven en el puerto |

Fuera de este hilo: borrar el crate, `eda_port_legacy`, `runtime_db_path`, ramas `writes_filesystem`, matrices, allowlists, Genoma 2.0, `SddIA/events/` y `.SddIA/events/`. `find_pec_by_correlation` en Kalma2 queda: ya no lo llama `build_status_body`; es helper de test.

### Diseño cerrado

1. `scan_presented_for_branch` devuelve solo `presented_in_store`. Desaparece el bucle `read_dir`.
2. `presented_ids_in_store` recorre solo `load_domain_ecsts`. Desaparecen `consider_presented_from_dir` y las cuatro rutas `.events/`.
3. `sweep_progress_leaf` no lista ni borra directorios. El PEC terminal ya se resuelve por puerto (`pec_terminal_for_correlation`). No hay operación de TTL de traces en el puerto: este hilo no la inventa. Residuos `.events/progress/` dejan de gobernar el sweep.
4. Tests que plantan JSON y esperan que el lector lo vea se reescriben: publican por puerto, o afirman que el JSON solo en disco es invisible (`t_adu_ca7_*`, fixtures de `presented_event_id_for_branch`, `progress_sweep_purges_on_terminal_pec` si sigue exigiendo `remove_dir`).

### Secuencia

| Paso | Archivo | Cambio |
|------|---------|--------|
| 1 | `engine/.../accept_pr.rs` | `scan_presented_for_branch` solo puerto; tests CA7/CA3 alineados |
| 2 | `engine/.../tracker_pbi_meta.rs` | Sin `consider_presented_from_dir` |
| 3 | `sddia-daemon-runtime/src/eda_sweep.rs` | `sweep_progress_leaf` sin `read_dir` |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| Presented solo en `.events/pending/*.json` | `scan_presented_for_branch` = false |
| Mismo evento publicado por puerto | true, y `presented_event_id_for_branch` devuelve ese id |
| Directorio `.events/progress/{cid}` con PEC terminal en puerto | El directorio no se borra por el leaf; el sweep no hace `read_dir` ahí |
| Suite O2–O4 de accept-pr / tracker / sweep | Sigue verde tras reescribir fixtures de disco |

### Verificación

```bash
cd SddIA && cargo test -p execute-process scan_presented
cd SddIA && cargo test -p execute-process presented_in_store
cd SddIA && cargo test -p sddia-daemon-runtime sweep
```

**Commit:** `refactor(eda): lectores Presented y progress sin disco (MAN-43 O5)`.

**AC:** AC-4 y AC-5 en estos tres lectores. AC-6 del leaf de progress. AC-10 no se declara.
**Fuera:** O5.1 (L-7), Genoma, O6.
**Bloqueo O5.1:** no borrar `sddia-event-store` mientras `eda_port` (`cfg(test)`) llame a `eda_port_legacy` ni mientras `execute-process` / `event-bus-audit` declaren la dependencia.

---

## O5.1 — `eda_port` sin modo filesystem (siguiente hilo)

Estado: **done (rama)**. Sin bypass `eda_port_legacy`; `filesystem_eda()` instala `InMemoryEventManagerMock`; unitarios `src/` asertan vía `test_lab` (`get_event`, `list_pending`, `ecst_for_event_id`, `seed_domain_ecst`). O5 (`42519135`) cerró los lectores productivos. **L-7** sigue impidiendo borrar el crate: `tests/common/mod.rs`, `eda_store_io_matrix_*`, `eda_subscriber_ecst_redb`, `event_store_writers` y `event-bus-audit` llaman `sddia_event_store` directo (`publish_ecst` / `EventStore::open` / `runtime_db_path`). Eso es **O5.2**.

### Corte

`eda_port` bajo `cfg(test)` desvía cinco operaciones a `eda_port_legacy` cuando `store_mode()` es filesystem (y `load_domain_ecsts` también lee el store legacy en modo redb). El laboratorio `filesystem_eda()` solo pone ese modo. El puerto productivo ya existe. Este hilo elimina el desvío y el módulo legacy.

| Pieza | Hoy | Hueco |
|-------|-----|-------|
| `eda_port.rs` `publish_ecst`, `record_ecst`, `load_dispatched`, `append_progress`, `load_domain_ecsts` | Rama `cfg(test)` → `eda_port_legacy` | Dualidad de laboratorio: el mismo API escribe JSON o Redb según un thread-local |
| `eda_port_legacy.rs` | Puente a `sddia_event_store` + `.events/{family}` | Único caller es `eda_port` |
| `test_lab::filesystem_eda` y copias locales (`lab_filesystem_eda`) | `override_store_mode(Filesystem)` | Sin la rama, el guard no cambia el destino y los tests que leen `.events/*.json` mienten |
| `gate_event_manager_port` | Allowlist por sufijo `eda_port_legacy.rs` | Reintroducir el fichero no falla el gate |

Fuera: borrar el crate, `runtime_db_path`, `path_is_resolved_redb`, matrices `tests/`, `event-bus-audit`, allowlist de `/tests/` y de `sddia-event-store/`, Genoma, O6.

### Diseño cerrado

1. Las cinco funciones de `eda_port` llaman solo a `with_port` → `EventManagerPort`. Desaparecen los bloques `cfg(test)` que consultan `store_mode` / `writes_filesystem` / `writes_store`.
2. Se borra `eda_port_legacy.rs` y `mod eda_port_legacy` en `engine/mod.rs`.
3. `filesystem_eda()` y cada `lab_filesystem_eda` dejan de fijar `StoreMode::Filesystem`. El guard de exclusión (`ParallelLabGuard`) se queda. Quien publique en el test usa `EventManagerTestHarness` o `record_ecst`/`publish_ecst` contra el puerto del repo temporal.
4. Aserción que lee `.events/**/*.json` como resultado de esas cinco funciones pasa a `get_event` / `list_events`. Un JSON plantado a mano deja de contar (misma regla que O5).
5. Tests y binarios que abren `EventStore` o llaman `sddia_event_store::publish_ecst` directo no se tocan.
6. Allowlist: quitar el sufijo `eda_port_legacy.rs`. El resto de la allowlist espera a O5.2.

### Secuencia

| Paso | Archivo | Cambio |
|------|---------|--------|
| 1 | `engine/.../eda_port.rs` | Sin ramas legacy; `filesystem_eda` sin `StoreMode` |
| 2 | `engine/.../eda_port_legacy.rs` + `mod.rs` | Borrar módulo |
| 3 | Callers de `filesystem_eda` / `override_store_mode(Filesystem)` en `src/` (no `tests/`) | Harness o puerto; sin lectura de `.events/` como SSOT |
| 4 | `tools/sddia-qa/src/gate_event_manager_port.rs` | Fuera el sufijo `eda_port_legacy.rs` |

Callers `src/` conocidos: `route_domain_core`, `delivery_close`, `accept_pr`, `actions`, `daemon_heartbeat`, `workspace_init`, `progress_trace`, `suite_execution_requested`, `capability_di_reactor`, `aiua_intent`, handlers `email_*`, `aiua_stimulus`, `kalma2`, `radamanto_batch_core`.

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `publish_ecst` con `StoreMode::Filesystem` activo | El evento está en el puerto; no aparece `.events/{family}/{id}.json` por esa llamada |
| `load_domain_ecsts` en modo redb con fila solo en `EventStore` legacy y no en el puerto | No la devuelve |
| `rg eda_port_legacy` en `execute-process` | Cero hits |
| Gate con un fichero nuevo `eda_port_legacy.rs` que llame `sddia_event_store::` | Violación |
| `cargo test -p execute-process --lib` | Verde |
| Matrices `tests/eda_store_io_*` | Siguen verdes sin editarlas |

### Verificación

```bash
cd SddIA && cargo test -p execute-process --lib
cd SddIA && cargo test -p execute-process --test eda_store_io_matrix_pending --test eda_store_io_matrix_fractal
cd SddIA && cargo test -p sddia-qa gate_event_manager_port
```

**Commit:** `refactor(eda): eda_port sin bypass filesystem (MAN-43 O5.1)`.

**AC:** AC-5 en el desvío de laboratorio de `eda_port`. AC-10 no se declara: el crate sigue. AC-11 solo en el sufijo `eda_port_legacy.rs`.
**Fuera:** O5.2, Genoma, O6.
**Bloqueo O5.2:** no borrar `sddia-event-store` mientras `execute-process` (dev) o `event-bus-audit` lo declaren en `Cargo.toml`, ni mientras `tests/common/mod.rs` publique por el crate.

---

## O5.2 / H5 — Demolición del crate (siguiente hilo)

Estado: **done (rama)**. Crate `sddia-event-store` eliminado del workspace; matrices y unitarios migrados a `EventManagerPort` + host; gates endurecidos (AC-10/11).

### Corte

Desaparece el crate `sddia-event-store`, su dependencia en workspace y toda referencia productiva o de laboratorio que abra Redb por fuera del host. Las matrices I/O dejan de probar `StoreMode::{Filesystem,Dual,Redb}` del crate y pasan a **RedbEventAdapter** + `run_event_manager_contract_suite` / harness ya usados en O5.1.

| Pieza | Hoy | Hueco |
|-------|-----|-------|
| `SddIA/Cargo.toml` | Miembro `sddia-event-store` | AC-10 |
| `execute-process/Cargo.toml` | `sddia-event-store` path dep | Bloquea borrado del crate |
| `event-bus-audit` | `EventStore::open` + `runtime_db_path` del crate | Consumidor fuera del host |
| `tests/common/mod.rs` | `publish_ecst` / `StoreMode` / `EventStore::open` vía crate | SSOT de matrices pending/fractal |
| `tests/eda_store_io_matrix_{pending,fractal}.rs` | Barrido catálogo × modos store | Cubre store legacy, no solo puerto |
| `tests/eda_subscriber_{ecst_redb,event_push}.rs` | Insert/witness directo en store | Fuera del contrato v2 |
| `tests/event_store_writers.rs` | `runtime_db_path` del crate | Obsoleto tras O5.1 |
| `eda_port::runtime_db_path` / `path_is_resolved_redb` | Públicos en `eda_port.rs`; `route_domain_core` aut52 | AC-8: solo host puede nombrar `.redb` |
| Unitarios `src/` residuales | `override_store_mode` / `EventStore::open` en `accept_pr`, `actions`, `vitality`, `route_domain_core`, handlers tracker, `domain_authority`, etc. | Siguen cableando el crate aunque el productivo ya usa puerto |
| `eda_port::test_lab::redb_eda` | `override_store_mode(Redb)` del crate | Debe usar `RedbEventAdapter` temporal o mock |
| `capability_di_reactor` | `#[cfg(test)]` borra `.events/pending` si `writes_filesystem` | Residuo post-O5.1 |
| `gate_event_manager_port` + `eda-unit-backend.toml` | Allowlist `sddia-event-store/`, `/tests/`, matrices | AC-11 |

Fuera: Genoma 2.0 (`entity-manager`), convergencia README/Constitución (**O6**), `SddIA/events/`, `.SddIA/events/`. `event_store_query` ya delega en `eda_port`; no reabre el crate.

### Diseño cerrado

1. **Nuevo laboratorio de matrices:** `execute-process/tests/common/` publica y aserta solo por `EventManagerPort` (`publish_ecst`, `list_pending`, `get_event`, entregas). Modo “Redb físico” = fichero temporal + `RedbEventAdapter::open`; modo mock = `InMemoryEventManagerMock` (paridad con `test_lab`). **Prohibido** `sddia_event_store::`.
2. **Matrices:** conservar intención (catálogo × familias × pending/fractal) renombrando o sustituyendo escenarios FS/Dual por escenarios puerto/adapter; o fusionar en `sddia-core` contract suite + smoke Redb en `infrastructure/redb_event_adapter`.
3. **`eda_subscriber_*` / `event_store_writers`:** migrar a puerto o eliminar si duplican contract suite O0–O1.
4. **`event-bus-audit`:** inspección vía `sddia-event-manager-host` / `with_productive_port`; quitar dep Cargo.
5. **`eda_port`:** eliminar `runtime_db_path` y `path_is_resolved_redb` de la superficie pública; tests AUT-52 usan `productive_store_present` / `is_resolved_redb_path` del host. `redb_eda()` sin `override_store_mode`.
6. **Barrido `src/`:** sustituir seeds `EventStore::open` en tests por `test_lab::seed_*` o insert vía puerto; quitar `use sddia_event_store` del crate `execute-process`.
7. **Demolición:** quitar miembro workspace, directorio `SddIA/sddia-event-store/`, entradas evolution si aplica.
8. **Gates:** retirar allowlists de store, matrices y blanket `/tests/` en `gate_event_manager_port`; endurecer `FORBIDDEN` (el gate debe fallar si reaparece `sddia_event_store::` fuera host/adapters).

### Secuencia (orden L-7)

| Paso | Alcance |
|------|---------|
| 1 | Reescribir `tests/common/mod.rs` + matrices pending/fractal sin crate |
| 2 | Migrar o borrar `eda_subscriber_*`, `event_store_writers` |
| 3 | Migrar unitarios `src/` que aún llaman `EventStore::open` / `override_store_mode` |
| 4 | `event-bus-audit` sin dep store |
| 5 | Quitar `runtime_db_path` / `path_is_resolved_redb` de consumidores; `capability_di_reactor` sin rama FS |
| 6 | `Cargo.toml` workspace + deps; borrar crate |
| 7 | Allowlists vacías + `cargo test --workspace` + clippy + gates QA |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `rg sddia_event_store` en `execute-process/src` | Cero (tests `src/` incluidos) |
| `rg sddia-event-store` en `Cargo.toml` del workspace | Cero salvo commit de borrado |
| Matrices pending/fractal | Verdes sin `StoreMode` del crate |
| `cargo test -p sddia-core event_manager` + contract suite | Verde |
| Gate `gate_event_manager_port` con allowlist store | Falla si alguien reintroduce `EventStore::open` en productivo |
| `event_store_query` CLI publish/query | Sigue operativo vía puerto |

### Verificación

```bash
cd SddIA && cargo test -p execute-process
cd SddIA && cargo test -p sddia-core event_manager
cd SddIA && cargo test -p sddia-infrastructure-redb-event
cd SddIA && cargo test -p sddia-qa gate_event_manager_port
cd SddIA && cargo test -p event-bus-audit
cd SddIA && cargo clippy --workspace -- -D warnings
```

**Commit:** `test(eda): demolición store legacy + gates v2 (MAN-43 O5.2)`.

**AC:** AC-8 (`runtime_db_path` fuera de consumidores), AC-10, AC-11, AC-12.
**Fuera:** O6.
**Bloqueo:** no borrar el directorio del crate hasta paso 1–4 verdes (L-7).

---

## O6 / H6 — Convergencia documental (siguiente hilo)

Estado: **done (rama)**. Narrativa README/Constitución + evolution + re-auditoría `docs/audits/man-43-o6-remediation-20261113.md`. AC-14 parcial; AC-13 sigue abierto (`NO_APTO_TOTAL`). Índice en § H6.

### Corte

Alinear la narrativa constitucional y el registro de evolución con el runtime ya cortado: `EventUri` + `EventManagerPort` ^1.2.0, persistencia solo vía host/bóveda. `APTO_TOTAL` (AC-13) se escribe **solo** si un re-audit cita evidencia de rama; si queda código productivo que trata `/.events/` como cola, el veredicto sigue `NO_APTO_TOTAL` y se abre deuda, no se maquilla.

### Inventario vivo (pre-hilo)

| Pieza | Hoy | Hueco |
|-------|-----|-------|
| `README.md` raíz | Tabla `/.events/` como runtime; modos `SDDIA_EVENT_STORE` y crate `sddia-event-store` | AC-14: crate demolido en `877c1791` |
| `SddIA/CONSTITUTION_CORE.md` | Aún cita `sddia-event-store` y `filesystem\|dual` como lab | Misma frase obsoleta |
| `SddIA/core/cumulo.paths.json` | `eda_bus` / `eda_fractal` = **nombres de tabla** + JSON de suscripciones; sin cola `.events/` | No borrar claves: el motor y `event-bus-audit` las leen |
| `event-bus-audit` | Defaults `.events/pending` si falta clave | Residuo de lectura; **fuera** de este hilo salvo que el re-audit lo marque bloqueante de AC-4/AC-13 |
| `SddIA/process/`, `SddIA/events/` | Genoma | Inventario; mutación solo `entity-manager` |
| `docs/audits/man-32-lineage-man-43-remediation-20261111.md` | `NO_APTO_TOTAL` histórico | Re-audit nuevo; no reescribir el laudo de 20261111 |
| `validacion.md` del feature | `APTO` = milestone H1 | No promoverlo a feature completo |
| `SddIA/evolution/` | Sin registro del cutover 2.0.0 de este ciclo | AC-14; contrato `evolution_contract` 1.1.2 (`hash_integrity` real) |

### Diseño cerrado

1. **Narrativa (fuera de genoma):** `README.md` y `CONSTITUTION_CORE.md`. Familia = `EventUri`; cola/estado = puerto; `/.events/*` solo como token lógico histórico, no SSOT. Quitar `sddia-event-store` y `StoreMode` filesystem/dual.
2. **Cúmulo:** diff de lectura. Si `eda_bus`/`eda_fractal` siguen siendo IDs de tabla, **no** se toca el JSON. Un path de cola que reaparezca se documenta como deuda, no se borra en caliente.
3. **Genoma:** grep de `sddia-event-store` y de `/.events/` como directorio de cola en `SddIA/process/` y `SddIA/events/`. Lista en `implementation.md`. Cero `Write` sobre esas rutas.
4. **Evolution:** un `{uuid}.md` nuevo (`tipo_operacion: modificacion`, `relacionado` = MAN-43 / `877c1791` / feature) + línea en `Evolution_log.md` según `evolution_contract` 1.1.2. `hash_integrity` calculado, no placeholder.
5. **Auditoría:** informe nuevo bajo `docs/audits/` (fecha del hilo) que relee F-1…F-6 / R1…R3 contra el árbol de `877c1791`. Veredicto `APTO_TOTAL` solo con comandos y salidas citados. Si `event-bus-audit` u otro productivo sigue materializando cola FS, veredicto `NO_APTO_TOTAL` + ítem residual (no es O7 de código en este commit).
6. **Feature docs:** `implementation.md` (O6 done o blocked), `execution.md` si el patrón lo exige, `plan.md` estado. `validacion.md` del H1 intacto.

### Secuencia

| Paso | Alcance |
|------|---------|
| 1 | Inventario README, Constitución, Cúmulo, genoma, audit 20261111 |
| 2 | Parche narrativo README + Constitución |
| 3 | Evolution (UUID + log + hash) |
| 4 | Re-audit con evidencia; veredicto explícito |
| 5 | Cierre `implementation.md` / estado O6 |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| README o Constitución mencionan `sddia-event-store` como crate vivo | Ausente |
| `cumulo.paths.json` pierde `subscriptions` o nombres de tabla que el runtime lee | Diff vacío en esas claves |
| Mutación manual de `SddIA/process/` o `SddIA/events/` | Cero |
| `APTO_TOTAL` sin citas de tests/gates | Prohibido |
| `validacion.md` H1 reescrito a feature completo | Prohibido |
| `hash_integrity` placeholder | Prohibido (`evolution_contract` §2) |

### Verificación

```bash
rg -n "sddia-event-store|SDDIA_EVENT_STORE" README.md SddIA/CONSTITUTION_CORE.md
rg -n "\"pending\":\\s*\"\\./\\.events" SddIA/core/cumulo.paths.json
# evolution: frontmatter conforme a SddIA/evolution/evolution_contract.md
```

**Commit:** `docs(eda): convergencia EventManager v2 (MAN-43 O6)`.

**AC:** AC-14 siempre que evolution + narrativa cierren. AC-13 solo si el re-audit es `APTO_TOTAL` con evidencia. Si no, AC-13 queda abierto y el commit lo dice.
**Fuera:** código de `event-bus-audit`, genoma, `validacion.md` H1, borrado de `SddIA/events/` / `.SddIA/events/`.
**Bloqueo:** no declarar feature Done ni mergear solo con docs si AC-13 sigue `NO_APTO_TOTAL`.

---

## O7 — Auditor sin cola filesystem por defecto (siguiente hilo)

Estado: **done (rama)**. `resolve_bus_paths` sin defaults `/.events/`; ids de tabla ignorados. AC-13 sigue abierto.

### Corte

`event-bus-audit` deja de materializar rutas `/.events/{pending,processing,processed,dead-letter,telemetry,orchestration,domain}` cuando `cumulo.paths.json` no trae un path de directorio. Hoy `resolve_bus_paths` hace `unwrap_or(".events/…")` y, con las claves vigentes (`events`, `subscriber_delivery`, …), junta esos **nombres de tabla** como si fueran carpetas. El censo de eventos y entregas sale solo del puerto productivo. Un barrido de disco, si queda, exige una clave cuyo valor sea un path explícito; no hay default.

### Inventario vivo

| Pieza | Hoy | Hueco |
|-------|-----|-------|
| `tools/event-bus-audit/src/main.rs` `resolve_bus_paths` | Defaults `.events/*`; lee `eda_fractal.telemetry` que **no existe** (la clave real es suscripciones) | F-03 residual |
| Tests del binario | `store_audit_counts_dead_letter_without_json_files` ya exige ausencia de `.events/` | Falta caso «claves = tablas → cero join a `.events/`» |
| `SddIA/process/event-bus-audit.md` | Narrativa `SDDIA_EVENT_STORE=filesystem` | Genoma: inventario, cero `Write` |
| `capability-bindings.md` contrato `1.2.0` | F-02 | **Fuera** (O8 / `entity-manager`) |

### Diseño cerrado

1. Resolver paths solo si el string de Cúmulo parece directorio (`./`, `.events/`, `.SddIA/`). Valores que son ids de tabla (`events`, `subscriber_delivery`, …) se ignoran.
2. Quitar los `unwrap_or(".events/…")`. Sin path explícito, los campos de `BusPaths` quedan vacíos / no escaneados.
3. El informe de dominio, DLQ y entregas sigue por `with_productive_port` (ya usado en tests). No reabrir `sddia-event-store`.
4. Test: repo mínimo con el `cumulo.paths.json` real del Core (tablas, sin `.events/`) → `run_audit` no crea ni lee `.events/` y el censo sale del store.
5. Cero edición de `SddIA/process/`, `SddIA/events/`, `capability-bindings.md`, `validacion.md`.

### Secuencia

| Paso | Alcance |
|------|---------|
| 1 | Separar «path de directorio» vs «id de tabla» en `resolve_bus_paths` |
| 2 | Defaults FS eliminados; scan condicionado |
| 3 | Test con Cúmulo real |
| 4 | `cargo test -p event-bus-audit` + `gate-event-manager-port` |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `unwrap_or(".events/` en `event-bus-audit` | Cero |
| Cúmulo de tablas provoca `repo.join("events")` como cola | Cero |
| Mutación de `SddIA/process/` o bindings | Cero |
| Declarar AC-13 / `APTO_TOTAL` | Prohibido en este hilo |

### Verificación

```bash
cd SddIA && cargo test -p event-bus-audit
cd SddIA && cargo run -q -p sddia-qa -- gate-event-manager-port
rg -n 'unwrap_or\\("\\./\\.events' SddIA/tools/event-bus-audit/src/main.rs
```

**Commit:** `fix(eda): event-bus-audit sin cola FS por defecto (MAN-43 O7)`.

**AC:** acerca F-03; no cierra AC-13.
**Fuera:** F-02 (`capability-bindings` 1.2 vs handshake 2.0), prosa de `SddIA/process/`, `APTO_TOTAL`.
**Siguiente tras O7:** O8 contractual/genoma. Detalle abajo.

---

## O8 — Alineación contractual 2.0.0 (siguiente hilo)

Estado: **done (rama)**. Bindings 2.0.0; narrativa; procesos forjados vía `entity-manager`. F-02 schema/route frontmatter residual. AC-13 abierto.

### Corte

Una sola versión de contrato visible: **2.0.0**. `capability-bindings.md` (no está en directorios de genoma protegido) pasa `contract_version` y `provider_version` del rol `eda:event-manager` a `2.0.0` / `>=2.0.0`. README y Constitución dejan de llamar al puerto vigente `^1.2.0` (ese número queda como hito histórico MAN-35, no como contrato actual). La prosa de `SddIA/process/event-bus-audit.md` y `route-domain-event.md` se actualiza **solo** con `./sddia-run.sh --process entity-manager`. Cero `Write` sobre `SddIA/process/`, `SddIA/events/`, `SddIA/library/`.

### Inventario vivo

| Pieza | Hoy | Hueco |
|-------|-----|-------|
| `sddia-core/.../contract.rs` | `CONTRACT_VERSION` `2.0.0`, rango `^2.0.0` | SSOT de código |
| `sddia-event-manager-host` | `implements_contract_version: 2.0.0` | Alineado |
| `SddIA/core/capability-bindings.md` | `eda:event-manager` `contract_version: 1.2.0`, providers `>=1.2.0`, schema `eda.event_manager.v1.schema.json` | F-02 |
| `README.md` / `CONSTITUTION_CORE.md` | Siguen diciendo puerto `^1.2.0` tras O6 | Misma fractura narrativa |
| `event_manager_factory.rs` test `major_mismatch` | Registra `1.2.0` para demostrar rechazo | **No tocar**: es el caso negativo |
| `SddIA/process/event-bus-audit.md` | `SDDIA_EVENT_STORE=filesystem` escanea `./.events/` | Genoma; O7 ya quitó ese default en el binario |
| `SddIA/process/route-domain-event.md` | `event_file_path` = JSON en `.events/pending/` | Token lógico, no fichero |
| `SddIA/library/norms/.../eda.event_manager.v1.schema.json` | Norma protegida | Fuera: otro ciclo `entity-manager` clase `norm` |

### Diseño cerrado

1. **Bindings:** en la fila `eda:event-manager`, `contract_version: "2.0.0"`, `provider_version: ">=2.0.0"` en default/test/capsule. Subir `version` del fichero (SemVer patch o minor). La prosa del final del archivo dice contrato **2.0.0**. El nombre del schema v1 se deja; no se reescribe la norma.
2. **Narrativa:** sustituir la mención del puerto **vigente** `^1.2.0` por `^2.0.0` en README y Constitución. MAN-35 puede citarse como corte histórico 1.2, no como versión actual.
3. **Procesos (DA-2 / DA-5):** una invocación `entity-manager` `lifecycle_operation: update` por proceso (`event-bus-audit`, `route-domain-event`). Cuerpo: censo por `EventManagerPort`; `event_file_path` es token lógico `eda://` / `.events/pending/<id>.json` resuelto por puerto, sin lectura de JSON en disco. Tras el JSON de acuse del CLI, **prohibido** polling de `./.events/` o del fichero. Si el proceso colapsa: Protocolo Kintsugi, no edición manual.
4. **Re-audit corto** en `docs/audits/` (fecha del hilo): F-02 narrativo/bindings cerrado o aún abierto si `entity-manager` no materializó los procesos. Veredicto del feature sigue `NO_APTO_TOTAL` mientras el schema v1 y el codex no se hayan forjado.

### Secuencia

| Paso | Alcance |
|------|---------|
| 1 | Parche `capability-bindings.md` + README + Constitución |
| 2 | `cargo test -p sddia-core event_manager` (el test de mismatch 1.2 debe seguir rojo al rol `broken`) |
| 3 | Dos inyecciones `entity-manager` (fire-and-forget) |
| 4 | Nota de audit + estado O8 en `implementation.md` |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `contract_version: "1.2.0"` en la fila `eda:event-manager` | Ausente |
| `Write` / `StrReplace` bajo `SddIA/process/` o `SddIA/library/` | Cero |
| Borrar el test que rechaza `implements_contract_version: 1.2.0` | Prohibido |
| `APTO_TOTAL` | Prohibido |
| `sleep` / polling tras acuse de `sddia-run.sh` | Prohibido (DA-5) |

### Verificación

```bash
rg -n 'contract_version: \"1.2.0\"' SddIA/core/capability-bindings.md
cd SddIA && cargo test -p sddia-core event_manager
```

**Commit (parte no genoma):** `docs(eda): contrato EventManager 2.0.0 en bindings (MAN-43 O8)`.

El delta de `SddIA/process/` entra en el mismo PR solo si `entity-manager` lo materializó antes del commit. Si el acuse falla, el commit de bindings no incluye procesos y O8 queda `partial`.

**AC:** F-02 en bindings y narrativa. AC-13 sigue abierto (schema v1 y códices).
**Fuera:** `eda.event_manager.v1.schema.json`, `SddIA/library/codexes/`, `validacion.md` H1.

---

## O8.1 — Frontmatter `route-domain-event` (siguiente hilo)

Estado: **done (rama)**. Frontmatter v1.0.2: inputs como token de puerto; fases sin `fs.persist`. `outputs` de cabecera y schema v1 siguen fuera. AC-13 abierto.

No se llama O9: en `objectives.md` **O9** es la suite de calidad (`cargo test --workspace`), no esta ola.

### Corte

Una sola forja `entity-manager` → `process-creator` sobre `route-domain-event`, con `process_phases` (array no vacío) y `process_inputs`. `patch_process_phases_update` reescribe `phases`, `inputs`, `version` y `hash_signature`. Cero `Write` sobre `SddIA/process/`. Tras el acuse JSON, sin polling (DA-5).

### Inventario vivo

| Pieza | Hoy | Hueco |
|-------|-----|-------|
| Cuerpo `# route-domain-event` | Estados y testigos vía puerto (`4c848f3c`) | Hecho |
| `inputs.event_file_path` | «JSON padre en `.events/pending/`» | Token lógico |
| Fases 1, 3 y 5 | `fs.persist`: leer `pending/`, copiar cabecera, mover testigos | Deben nombrar el puerto |
| `outputs.parent_path` / `processing_header_path` | Siguen diciendo rutas de cabecera | El parche de fases **no** escribe `outputs`; quedan residual menor |
| `eda.event_manager.v1.schema.json` | Describe contrato **v1.2.0**; ningún `.rs` lo carga | **Fuera** (O8.2, clase `norm`) |

### Diseño cerrado

Semilla `semantic_seed`:

- `process_version`: `1.0.2` (hoy `1.0.1`).
- `process_inputs`: tres filas. `event_file_path` = token `.events/pending/<event_id>.json` resuelto por `EventManagerPort`. `event_file_paths` = lista de tokens (batch). `cumulo_topology` = opcional de laboratorio.
- `process_phases`: las cinco fases actuales, mismos `name` y mismos `delegates_to` (`agent:cumulo`, `action:execute-process`, `action:execute-action`). Cambian los `intent` de lectura, materialización y promoción: claim/get por puerto, entregas con `record_delivery` / `list_deliveries`, sin copia de JSON ni `subscribers/*.json`. Quitar `requires_capability` `fs:persist` de esas tres fases. Fan-out y resolución de suscripciones se conservan.

No reenviar el cuerpo ya forjado. No tocar el test de mismatch `1.2.0`. No declarar AC-13.

### Secuencia

| Paso | Alcance |
|------|---------|
| 1 | Una inyección `entity-manager` (`entity_class: process`, `entity_name: route-domain-event`, `lifecycle_operation: update`) |
| 2 | Leer el `.md` resultante: `inputs` y `phases` sin «JSON padre» ni `fs.persist` |
| 3 | Si el acuse es `success: false`: Kintsugi. No editar el fichero a mano |
| 4 | Nota en `implementation.md`. El schema v1 no entra en el commit |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `inputs` sigue diciendo JSON en `.events/pending/` | Ausente |
| Fases de lectura/materialización/promoción con `fs:persist` | Ausente |
| `Write` directo en `SddIA/process/route-domain-event.md` | Cero |
| Reescribir `eda.event_manager.v1.schema.json` | Cero |
| `APTO_TOTAL` | Prohibido |

### Verificación

```bash
rg -n "fs.persist|JSON padre" SddIA/process/route-domain-event.md
```

**Commit:** el delta lo materializa `entity-manager`. Mensaje: `docs(eda): route-domain-event sin cola FS en frontmatter (MAN-43 O8.1)`.

**AC:** cierra el residual de prosa de proceso dejado en O8. AC-13 sigue abierto (schema v1, `outputs` de cabecera, códices).
**Fuera:** norma del schema, `SddIA/library/codexes/`, fábrica Rust para parchear `outputs`.

---

## O9 — Calidad: clippy verde y suite de workspace (siguiente hilo)

Estado: **done (rama)**. Gate canónico `rustc -Dwarnings` verde; clippy con residual documentado. AC-12 parcial (clippy no es gate en `rust-warning-gate` §1).

### Corte (original)

`cargo clippy --workspace --all-targets -D warnings` compila y no emite hallazgos. Hoy **ni siquiera llega a medir**: `sddia-io` falla y arrastra a todo el workspace. Además, el contrato `eda.event_manager` es **inalcanzable por nombre** en el resolvedor de DI. O9 arregla lo primero (código, no genoma) y **diagnostica** lo segundo sin tocar genoma.

### Resultado de la implementación (2026-11-13)

**Hallazgo normativo que reencuadra el hilo:** `SddIA/norms/rust-warning-gate.md` §1 excluye explícitamente los avisos de Clippy del gate (§2 solo exige `rustc` sin warnings). La CI (`.github/workflows/sddia-index-qa.yml`) fija `RUSTFLAGS=-Dwarnings` y **no ejecuta clippy**. Por tanto AC-12 se cumple con `cargo check --workspace` / `cargo build`, no con clippy.

**Corregido (33 ficheros, cambios mecánicos de lint):** `sddia-io` (`question_mark`, desbloquea la medición del workspace), `sddia-core` (`redundant_guards`, `unnecessary_lazy_evaluations`, `assertions_on_constants`), `sddia-event-manager-host` (`iter_nth_zero`), `sddia-daemon-runtime` (`match_like_matches_macro`, `cloned_ref_to_slice_refs`, lifetimes), `sddia-core-memory`, `user-preference-core`, `file-tracker-adapter`, `repo-retention-purger`, `event-bus-audit`, `sddia-workspace-server`, `event-watcher`, `linear-bridge-watcher`, `email-watcher`, `gemini-http-infer`, `telegram-gateway`, `manage-event-receipt`, `io-choke`, `markdown-table-editor`, `iota-publish-relay`, `llm-router`, `shell-executor`, `cryptography-manager`, `lancedb_preferences_repo`, `kalma2-bridge` y 8 ficheros de `execute-process`.

**Verificación:**
- `cargo check --workspace` → **verde** (gate canónico).
- `cargo test -p execute-process --lib` → **816 passed**.
- `cargo clippy --workspace --all-targets` → de **workspace inmedible** a residual solo en `execute-process` (~85 diagnósticos, ninguno funcional).

**Residual clippy (fuera del gate, documentado):** `too_many_arguments` en `handle_subscriber`, `scan_body_residual`, `scan_event_dir`, `make_row`, `post_generate`; `needless_return` dentro de `match` en `route_fractal_core`; `type_complexity`; `unnecessary_unwrap` en `tracker_stamp`. Todos exigen refactor estructural, no parche mecánico.

**Fallos preexistentes verificados con `git stash` (no causados por O9):** `linear_bridge_webhook::ac13_valid_signature_emits_webhook_source` (`NotFound`) y `ecosystem_health_map_absent_still_ok` (`DEGRADED` vs `absent`).

**Incidencia de entorno:** el disco `/tmp` llegó al 100% durante el hilo (27 G de `cargo-target`). Requirió purga del caché; los 4 fallos de una ejecución intermedia fueron `StorageFull`, no de código.

| # | Hallazgo | Evidencia | Tipo |
|---|----------|-----------|------|
| **C-1** | `sddia-io` rompe clippy con `clippy::question_mark`; bloquea el lint de todo el workspace | `sddia-io/src/outbound_lab.rs:57` — `let Some(repo) = repo else { return None; }` | Código |
| **C-2** | El schema del contrato no coincide con el nombre que calcula el motor | Resolver/validador/gate construyen `{contract}.schema.json` → `eda.event_manager.schema.json`; en disco solo existe `eda.event_manager.v1.schema.json` | **Genoma** (sin forge) |
| **C-3** | El schema describe contrato `v1.2.0` y su `publish_ecst` no exige `event_uri` | `$defs.publish_ecst.required` = `[event_id, ecst]` | Contenido genoma |
| **C-4** | `capability-taxonomy.md` sigue con `eda:event-manager` **1.2.0** | `catalog.id: eda:event-manager / version: 1.2.0` | Genoma (`norm`) |
| **C-5** | `route-domain-event.outputs` aún nombra cabeceras en disco | `parent_path`, `processing_header_path` | Genoma; el parche de fases no escribe `outputs` |

**C-2 es el hallazgo grave:** si algún proceso declarase `requires_capability: contract: eda.event_manager`, el gate fallaría con `schema ausente` y el validador de salida abortaría la fase. Hoy nadie lo declara, así que es un fallo **latente**, no activo. `rg 'contract: eda.event_manager'` → cero coincidencias.

### Diseño cerrado

1. **C-1 (único cambio de código):** `let repo = repo?;` en `outbound_lab.rs`. Re-ejecutar clippy para descubrir si `sddia-io` ocultaba más hallazgos en crates dependientes. Iterar hasta verde **solo** con correcciones mecánicas de lint (sin refactor funcional).
2. **C-2, C-3, C-4, C-5:** no se tocan en este hilo. `SddIA/library/norms/` es genoma protegido y **no existe creator de JSON schemas**: `norm-creator` materializa `{name}.md`, no `*.schema.json`. Abrir decisión, no improvisar `Write`.
3. **Documentar** el desajuste de nombre en `implementation.md` y en la nota de auditoría, con la opción técnica de cada vía: renombrar el fichero a `eda.event_manager.schema.json`, o cambiar el resolvedor para leer `contract_schema` de `capability-bindings.md`. Cualquiera de las dos es un corte con su propio laudo.
4. Verificación funcional mínima: `cargo test -p sddia-core event_manager` sigue verde.

### Secuencia

| Paso | Alcance |
|------|---------|
| 1 | Corregir C-1 y re-lanzar `cargo clippy --workspace --all-targets` |
| 2 | Bucle corto: cada hallazgo nuevo, parche mecánico |
| 3 | `cargo test --workspace` (registrar veredicto real, aunque quede rojo) |
| 4 | Nota de auditoría con C-2…C-5 y su vía de forja; estado O9 |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `cargo clippy --workspace --all-targets -- -D warnings` | Cero errores **o** lista residual documentada crate por crate |
| Escribir en `SddIA/library/` con `Write`/`StrReplace` | Cero |
| Renombrar `eda.event_manager.v1.schema.json` sin decisión | Prohibido |
| Cambiar comportamiento de `outbound_lab` más allá del lint | Prohibido |
| `APTO_TOTAL` | Prohibido |

### Verificación

```bash
cd SddIA && cargo clippy --workspace --all-targets -- -D warnings
cd SddIA && cargo test -p sddia-core event_manager
cd SddIA && cargo test --workspace
```

**Commit:** `fix(qa): clippy workspace verde + diagnóstico contrato EDA (MAN-43 O9)`.

**AC:** acerca AC-12 (clippy) y AC-19 (suite). No cierra AC-13: C-2…C-5 son genoma.
**Fuera:** schema JSON, taxonomía, `route-domain-event.outputs`, `SddIA/library/codexes/`.
**Bloqueo de AC-13:** mientras C-2 siga abierto, el contrato `eda.event_manager` no es resoluble por el motor, así que no se puede declarar convergencia contractual completa.

---

## O10 — El motor de DI honra `contract_schema` (C-2) + C-4 taxonomía 2.0.0

Estado: **done (rama)**. C-2 cerrado en motor; C-4 sellado (catálogo `2.0.0`, norma de cauce `entity-manager`).

### Post-laudo Racso (2026-11-14)

- **C-4:** `eda:event-manager` → **2.0.0** en `capability-taxonomy.md` (v1.0.12); la Directriz Core exige mutación vía `./sddia-run.sh --process entity-manager` (prohibido edit IDE). Contenido parcheado tras fallo de `artifact_full_replacements` en tablas con `---`; sello `hash_refresh_only` vía entity-manager.
- **O9 re-gobierno:** parches rustc en `SddIA/tools|skills|daemons/` reaplicados desde `c60ae233` y sellados con `entity-manager` (`hash_refresh_only` en `.md`; daemons con soporte `hash_refresh` en forja nativa).
- **Forge:** `split_md_frontmatter` corregido (delimitador `---` de línea); `artifact_yaml_replacements` + `artifact_body_replacements`; `implementation_source_git_apply` para cuts Rust gobernados.

### Corte

C-2 se cierra en **código del motor**, sin tocar genoma. `capability-bindings.md` ya declara `contract_schema: "eda.event_manager.v1.schema.json"`, pero ningún `.rs` lee esa columna: los tres consumidores reconstruyen el nombre como `{contract}.schema.json`. O10 hace que el motor use la columna declarada y solo recurra al nombre derivado cuando la fila no la trae. `SddIA/engine/` no es genoma protegido (DA-2); `SddIA/library/` no se toca.

### Inventario (verificado 2026-11-13)

| Sitio | Hoy | Consecuencia |
|-------|-----|--------------|
| `capability_di_resolver.rs:127` `load_binding_rows` | Tupla `(contract, provider, provider_version)`; ignora `contract_schema` | La columna del binding no llega al motor |
| `capability_di_resolver.rs:354` | `format!("{contracts_rel}/{contract}.schema.json")` → `ResolvedBinding.contract_schema_rel` | Ruta inexistente para `eda.event_manager` |
| `capability_di_output_validator.rs:59/122` `schema_path(repo, &binding.contract)` | Recalcula el nombre e ignora `binding.contract_schema_rel` | Doble fuente de verdad del path |
| `capability_di_gate.rs:368-373` | `contracts.join(format!("{contract_name}.schema.json"))` desde el requisito del proceso | Gate rechazaría `schema ausente` |
| `capability_di_resolver.rs:410` | `contract_schema_ref: capability_contracts/{contract}` | Referencia lógica; fuera de alcance salvo que un test lo exija |

### Diseño cerrado

1. **Fila tipada:** sustituir la tupla de `load_binding_rows` por un `struct BindingRow { contract, provider, provider_version, contract_schema: Option<String> }`. Leer `contract_schema` con `yaml_str_map`.
2. **Helper único** `pub(crate) fn contract_schema_file(contract: &str, declared: Option<&str>) -> String`: devuelve `declared` si viene y no está vacío; si no, `{contract}.schema.json`. Rechazar valores con `/`, `\` o `..` (`ConfigError`): el binding nombra un fichero dentro de `directories.capability_contracts`, no una ruta.
3. **Resolver:** `contract_schema_rel = {contracts_rel}/{contract_schema_file(..)}`.
4. **Validador de salida:** `schema_path` usa `binding.contract_schema_rel` (relativo a `repo`) en lugar de recomputar desde `binding.contract`. Una sola fuente del path.
5. **Gate:** buscar la fila de bindings por `req.id` (reutilizar el loader del resolver, `pub(crate)`). Si la fila existe y su `contract` coincide con `contract_name`, usar su `contract_schema`; si no, fallback actual. El gate no inventa bindings.
6. Sin cambios en `capability-bindings.md`, en el schema ni en la taxonomía.

### Tests nuevos

| Test | Afirma |
|------|--------|
| `resolver_honours_declared_contract_schema` | Fila con `contract_schema: x.v1.schema.json` → `contract_schema_rel` termina en `x.v1.schema.json` |
| `resolver_falls_back_to_contract_name` | Fila sin columna → `{contract}.schema.json` (comportamiento actual intacto) |
| `resolver_rejects_contract_schema_with_path` | `../x.json` o `a/b.json` → `ConfigError` |
| `validator_uses_binding_contract_schema_rel` | Valida contra el path del binding aunque `{contract}.schema.json` no exista |
| `gate_uses_binding_contract_schema` | Requisito `eda:event-manager` / `eda.event_manager` con fixture → no `schema ausente` |
| `core_bindings_eda_event_manager_resolves_v1_schema` | Con el `capability-bindings.md` real del Core, la ruta resuelta existe en disco |

Los tests existentes de `doc.closure`, `di.binding`, `memory.pref_query` deben seguir verdes sin cambiar fixtures (todos usan el fallback).

### Secuencia

| Paso | Alcance |
|------|---------|
| 1 | `BindingRow` + `contract_schema_file` + tests de resolver |
| 2 | Validador consume `contract_schema_rel` |
| 3 | Gate consulta bindings por `req.id` |
| 4 | `cargo test -p execute-process --lib capability_di` y suite lib completa |
| 5 | Nota `docs/audits/man-43-o10-contract-schema-20261113.md`; estado O10 en `plan.md` / `implementation.md` |

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| Fixture de bindings sin `contract_schema` | Mismo path que hoy |
| `contract_schema` con separador o `..` | `ConfigError`, nunca lectura fuera de `capability_contracts` |
| Validador y resolver con paths distintos para el mismo binding | Imposible por construcción |
| Escribir en `SddIA/library/` o renombrar el schema | Cero |
| `APTO_TOTAL` | Prohibido |

### Verificación

```bash
cd SddIA && cargo test -p execute-process --lib capability_di
cd SddIA && cargo test -p execute-process --lib
cd SddIA && cargo check --workspace
cd SddIA && cargo run -q -p sddia-qa -- gate-event-manager-port
```

**Commit:** `fix(di): resolver/gate/validador honran contract_schema del binding (MAN-43 O10)`.

**AC:** cierra C-2 (latente). AC-13 sigue abierto por C-3 (contenido del schema en `v1.2.0`, `publish_ecst` sin `event_uri`), C-4 (taxonomía `1.2.0`) y C-5 (`route-domain-event.outputs`).
**Fuera:** C-3, C-4, C-5; refactor clippy estructural de `execute-process`; fallos preexistentes `linear_bridge_webhook::ac13_*` y `ecosystem_health_map_absent_still_ok`.
**Decisión pendiente del Vértice Biológico (C-4):** `capability-taxonomy.md` mantiene `eda:event-manager` en `1.2.0` y su regla «Rigor eda 1.2» exige alinear con bindings (ya `2.0.0`): contradicción viva. Es genoma `norm` y su propio texto prescribe `Write` atómico, en conflicto con DA-2. Requiere laudo antes de cualquier hilo que la toque.

---

## O11 — Remediación de forja O10 + C-4 / O9 por cauce real

Estado: **done (rama)** — commit pendiente de hash en esta sesión.

### Post-ejecución Racso (2026-11-14)

- `execute-process` debug reconstruido en `SddIA/target` (2026-11-14 ~12:11).
- Guarda `update` sin parche → `Err` (tool/skill); `run_norm_forge` soporta `implementation_source_git_apply`; delete tool/skill limpia fila de índice.
- Restaurados `shell-executor`, `cryptography-manager`, `event-bus-audit` desde `c60ae233` (diff vs `c60ae233`: solo `hash_signature`).
- Eliminados stub `.md` y filas de `file-tracker-adapter` / `manage-event-receipt` (`Domain_Entity_Deleted`).
- **C-4:** el contenido 2.0.0 de O10 ya estaba en disco; el parche en dos pasos **no se ejecutó**. Sello `hash_refresh_only` vía `entity-manager` + `Domain_Entity_Updated` (trazabilidad; el cuerpo sigue siendo el de O10, editado con IDE).
  - **Corrección (O12):** el motivo declarado («la semilla apunta a 1.2.0») era incompleto. El paso 1 (restaurar desde `c60ae233`) falló porque `o11-norm-restore.json` **no existía** (el lote previo abortó antes de crearlo); lo ejecuté sin recrearlo, vi `No such file`, y seguí al paso 2 sobre un fichero ya en 2.0.0. No se reintentó. El motor ya soportaba el paso 1 (`run_norm_forge` + `implementation_source_git_apply`, compilado en el binario de O11). C-4 **sigue sin cumplir** el laudo «realizado por cauce oficial».
- 4 daemons sellados; 16× `implementation_source_git_apply` O9 (10 tools + 2 skills + 4 daemons; excluye tools fantasma); `src/` diff vacío vs `c60ae233`.
- Tests: `update_existing_*`, `core_bindings_eda_event_manager_resolves_v1_schema`. Suite lib completa pendiente.

Estado previo planificación: prioridad máxima: `419d89d5` dejó genoma corrupto en la rama.

### Causa raíz (verificada 2026-11-14)

`./sddia-run.sh` resuelve `SddIA/target/debug/execute-process` (fechado **2026-11-13 12:14**). Las compilaciones de O10 dentro del sandbox escribieron en `/tmp/cursor-sandbox-cache/.../cargo-target`, no en `SddIA/target`. **Todas** las invocaciones `entity-manager` de O10 corrieron con la forja **anterior** a O10. En esa forja, `update` sin clave de parche reconocida **cae al camino `create` y sobrescribe** el `.md` con un esqueleto y un UUID nuevo.

### Inventario del daño en `419d89d5`

| Artefacto | Estado | Gravedad |
|-----------|--------|----------|
| `SddIA/skills/shell-executor.md` | Esqueleto: UUID nuevo `c1f79172-…` (era `93d23720-…`), inputs/outputs y cuerpo perdidos (44→18 líneas) | **Crítica**: contrato de seguridad (whitelist, anti-git) borrado |
| `SddIA/skills/cryptography-manager.md` | Esqueleto, UUID nuevo (43→18) | **Crítica** |
| `SddIA/tools/event-bus-audit.md` | Esqueleto, UUID nuevo (32→16) | Alta |
| `SddIA/tools/file-tracker-adapter.md`, `manage-event-receipt.md` | **Creados** (no existían) + 2 filas en `tools/index.md` | Media: entidades fantasma |
| 9 tools (`io-choke`, `llm-router`, …) | Solo `hash_signature` refrescado; UUID y contenido intactos | Baja (válido) |
| `capability-taxonomy.md` | Contenido correcto (catálogo 2.0.0, 112 líneas), pero **escrito con `StrReplace` del IDE** tras truncar la forja vieja el fichero a 43 líneas | **Violación DA-2 y del laudo C-4** («realizado por cauce oficial») |
| 4 daemons | No sellados (forja vieja rechaza `hash_refresh_only`) | Pendiente |
| Bus EDA | `Domain_Entity_Created` emitidos con UUIDs falsos (skills/tool sobrescritos, tools fantasma) | Residuo de auditoría; posible DLT si el watcher los anclara |

**Corrección de la narrativa O10:** el «re-gobierno O9» no reaplicó el Rust por `entity-manager`. El contenido Rust sigue siendo el edit manual de `c60ae233` (`git checkout` ida y vuelta, cambio neto cero). Solo se refrescaron hashes de `.md`.

### Diseño cerrado

1. **Binario canónico primero.** `cargo build -p execute-process` con `CARGO_TARGET_DIR=SddIA/target` (fuera de sandbox). Verificar `stat` del ELF posterior al último cambio de `forges/`. Sin esto, ningún paso siguiente arranca.
2. **Guarda anti-sobrescritura (código, `forges/factory.rs`).** En `run_tool_forge`, `run_skill_forge` y `run_daemon_forge`: `update` sobre artefacto existente sin parche reconocido → `Err`, **nunca** caer a `create`. `create` sobre artefacto existente sigue idempotente. Tests: `update_existing_tool_without_patch_is_error`, ídem skill y daemon, y verificación de que UUID y contenido quedan intactos.
3. **Restauración vía `entity-manager`** (`entity_class` = skill/tool, `lifecycle_operation: update`, `semantic_seed.implementation_source_git_apply` con `commit: c60ae233` y `paths: ["SddIA/skills/shell-executor.md"]`, etc.). Restaura UUID, contrato y cuerpo originales; el creator sella `hash_refresh`. Tres invocaciones (shell-executor, cryptography-manager, event-bus-audit). Verificación: `git diff c60ae233 -- <md>` solo en `hash_signature`.
4. **Entidades fantasma:** `entity-manager` con `lifecycle_operation: delete` para `tool:file-tracker-adapter` y `tool:manage-event-receipt` (Fase 2 borra el `.md` y la fila del índice). Verificar `tools/index.md` igual a `c60ae233` salvo hashes.
5. **C-4 por cauce real:** `entity-manager` norm `update` con `implementation_source_git_apply` de `capability-taxonomy.md` desde `c60ae233` **no** basta (devuelve 1.2.0). Secuencia: restaurar a `c60ae233` con `entity-manager` (`implementation_source_git_apply`) y luego un segundo `update` con `artifact_yaml_replacements` + `artifact_body_replacements` + `tactical_norm_version: 1.0.12` (las semillas de `/tmp/em-c4-forge-v2.json`). Aceptación: 112 líneas, UUID `e9c66ec6-…`, catálogo 2.0.0, `Rigor eda 2.0`, Directriz Core con cauce `entity-manager`, sello `Domain_Entity_Updated`.
   - Requiere soporte de `implementation_source_git_apply` en `run_norm_forge` (hoy solo en tool/skill/daemon).
6. **Daemons:** sellar los 4 con `hash_refresh_only` usando el binario nuevo.
7. **O9 Rust de verdad (laudo previo):** reaplicar `SddIA/{tools,skills,daemons}/*/src` desde `c60ae233` con `implementation_source_git_apply` por entidad (18 invocaciones). El contenido final es idéntico; lo que cambia es la trazabilidad (sello EDA por entidad). Diff de `src/` contra `c60ae233` debe ser vacío.
8. **Bus:** listar en la auditoría los `event_id` de los `Domain_Entity_Created` con UUID falso (por puerto, sin leer `.events/`). No se purgan a mano; si hay que anularlos, es decisión del Vértice Biológico (posible proceso de compensación).
9. **Docs:** corregir `implementation.md` § O10 y `docs/audits/man-43-o10-contract-schema-20261114.md` (O9 cosmético, taxonomía por IDE, daños). Nueva nota `man-43-o11-remediation-20261114.md`.

### Secuencia

| Paso | Alcance | Bloquea |
|------|---------|---------|
| 1 | Build canónico en `SddIA/target` | todo |
| 2 | Guarda anti-sobrescritura + tests; rebuild | 3–7 |
| 3 | Restaurar 3 artefactos corruptos | 7 |
| 4 | Borrar 2 tools fantasma | — |
| 5 | `implementation_source_git_apply` en norm-forge; C-4 en dos pasos | — |
| 6 | Sellar 4 daemons | — |
| 7 | 18 reaplicaciones O9 | — |
| 8 | Inventario de eventos falsos | — |
| 9 | Docs + commit | — |

Tras cada acuse JSON de `entity-manager`: verificar el fichero en disco; prohibido polling del bus (DA-5). Si una invocación colapsa: Kintsugi, sin edit IDE de rescate.

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `StrReplace`/`Write` sobre `SddIA/{library,skills,tools,daemons,process}/` | Cero en todo el hilo |
| UUID de skill/tool distinto de `c60ae233` | Cero |
| `update` sin parche sobre artefacto existente | `Err`, test verde |
| `git diff c60ae233 -- SddIA/{tools,skills,daemons}/*/src` | Vacío |
| `capability-taxonomy.md` | 112 líneas, catálogo 2.0.0, sellado por `entity-manager` |
| `SddIA/target/debug/execute-process` más antiguo que `forges/*.rs` | Prohibido invocar `entity-manager` |
| `APTO_TOTAL` | Prohibido (C-3, C-5) |

### Verificación

```bash
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build -p execute-process
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib forges::
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib capability_di
git diff c60ae233 -- SddIA/skills/*.md SddIA/tools/*.md SddIA/tools/index.md
git diff c60ae233 -- 'SddIA/tools/*/src' 'SddIA/skills/*/src' 'SddIA/daemons/*/src'
cd SddIA && cargo run -q -p sddia-qa -- gate-event-manager-port
```

**Commit:** `fix(forge): remediación genoma O10 + C-4/O9 vía entity-manager (MAN-43 O11)`.

**AC:** restaura integridad del genoma; cierra C-4 por cauce oficial. AC-13 sigue abierto por C-3 y C-5.
**Fuera:** C-3 (contenido del schema JSON), C-5 (`route-domain-event.outputs`), clippy estructural.

---

## O12 — Línea base + C-4 oficial + C-5

Estado: **done (rama)** — ver `docs/audits/man-43-o12-baseline-20261114.md`.

### Post-ejecución Racso (2026-11-14)

- **C-4:** restore `c60ae233` + parche `o12-norm-c4` vía `entity-manager`; cuerpo idéntico al snapshot pre-O12; `Domain_Entity_Updated` ×2.
- **C-5:** `patch_process_outputs_update`; process `route-domain-event` 1.1.0; runtime `event_uri` sin rutas fabricadas en modo puerto.
- **Línea base:** 819 tests OK, 4 fallos (validator tempdir + tier_dispatch spawn); `check --workspace` OK; `gate-event-manager-port` OK.
- Higiene: 5 semillas `_lab` inválidas eliminadas.

### Corrección posterior a O12 (2026-11-14, hallada al planificar O13)

O12 clasificó los 4 fallos de la suite como «no bloqueantes». Eso **contradice el plan de O12** («cualquier fallo distinto de los 2 conocidos bloquea B y C») y la auditoría `man-43-o12-baseline-20261114.md` lo repite. Diagnóstico real:

| Fallo | Causa | Tipo |
|-------|-------|------|
| `capability_di_output_validator::ac_r8_schema_mismatch`, `ac_r8_valid_payload`, `ac_tracker_lifecycle_emit_v2_body` | O10 hizo que `schema_path` lea `binding.contract_schema_rel`; los tres tests construyen `ResolvedBinding { contract_schema_rel: "x" }` (fixture heredado). Producción sí rellena el campo (`core_bindings_eda_event_manager_resolves_v1_schema` pasa). | **Regresión mía de O10**: fixtures sin actualizar. La suite completa nunca se ejecutó en O10 (el plan O10 la exigía). |
| `route_domain_core::tier_dispatch::store_backed_native_capsule_does_not_use_cognitive_runtime` | Mi shell tenía exportada `SDDIA_EXECUTE_PROCESS_BIN=SddIA/target/debug/execute-process` (ruta **relativa**); el test espera que la variable no exista. Con `unset`, pasa. | Artefacto de entorno, no defecto del repo; pero el test no es hermético. |

Los dos fallos «conocidos» (`linear_bridge_webhook::ac13_*`, `ecosystem_health_map_absent_still_ok`) **no fallaron** en O12. El veredicto de O12 queda en **`APTO_PARCIAL` con baseline inválida** hasta que O13 re-ejecute la suite con entorno limpio.

Estado previo planificación: **ready**.

### Inventario verificado (2026-11-14, sobre `303ca577`)

| Hallazgo | Evidencia |
|----------|-----------|
| La suite `execute-process --lib`, `cargo check --workspace` y `gate-event-manager-port` **no se han ejecutado desde O9**. O10 y O11 tocaron `forges/`, `entity_manager.rs` y el resolver DI. | O11 solo corrió 3 tests dirigidos |
| **C-4 incumplido** (ver corrección en § O11): el cuerpo de `capability-taxonomy.md` es edit IDE; el hash está sellado, el contenido no pasó por la forja. | `o11-norm-restore.json` nunca existió; `o11-norm-c4.json` falló por texto 1.2.0 ausente |
| **C-5 es peor que «outputs obsoletos».** El runtime emite `parent_path` y `processing_header_path` en 5 puntos de `route_domain_core.rs`. Con `from_store` (puerto) `processing_header` es `.events/pending/{uuid}.json`, una ruta **fabricada que no existe en disco**. Las fases ya dicen «sin copia de cabecera»; los outputs y el JSON de respuesta contradicen. | `route_domain_core.rs:2451`, `2445…2643` |
| Outputs declarados incompletos: el runtime también emite `dispatch_mode`, `sweep` y `transaction_digest` (opcional), que no constan. | `route_domain_core.rs:2640-2670` |
| Único consumidor externo de `parent_path`: `iota_ci_smoke.rs:215` (solo lo vuelca al informe). | grep |
| `run_process_forge` update ignora `process_outputs`: con `process_phases` llama a `patch_process_phases_update` (no escribe `outputs`); sin fases solo admite `markdown_body_replacements` (cuerpo). El hash de proceso cubre solo `phases`, no `outputs`. | `factory.rs:481-560`, `common.rs:252` |
| Higiene: `_lab/` de O11 contiene 5 semillas inválidas o duplicadas: `o11-del-file-tracker-adapter.json` (sin `semantic_seed`, rechazada; el delete real se hizo por stdin), `o11-norm-c4.json` (fallida), `o11-r-skill-event-bus-audit.json` (clase `skill` para una tool), `o11-r-skill-cryptography-manager.json` y `o11-r-tool-event-bus-audit.json` (duplican `o11-r-crypto`/`o11-r-eba`). | `ls _lab` |

### Diseño cerrado

**A. Línea base (primero, sin tocar nada).** Con `CARGO_TARGET_DIR=SddIA/target`, fuera de sandbox: `cargo test -p execute-process --lib` completa, `cargo check --workspace`, `gate-event-manager-port`. Registrar resultado literal. Fallos conocidos preexistentes: `linear_bridge_webhook::ac13_valid_signature_emits_webhook_source`, `ecosystem_health_map_absent_still_ok`. Cualquier otro fallo **bloquea** B y C y se trata como hallazgo de O10/O11.

**B. C-4 por cauce oficial, esta vez con reintento real.**
1. Crear `o12-norm-restore.json`: `entity_class: norm`, `lifecycle_operation: update`, `semantic_seed.implementation_source_git_apply { commit: "c60ae233", paths: ["SddIA/library/norms/capability-taxonomy.md"] }`. Ejecutar por `./sddia-run.sh --process entity-manager`. Verificar: 112 líneas, UUID `e9c66ec6-…`, catálogo `eda:event-manager` en `1.2.0`, versión 1.0.11.
2. Ejecutar `o11-norm-c4` corregido (semilla `/tmp/em-c4-forge-v2.json` envuelta en `entity_class/entity_name/lifecycle_operation/semantic_seed`; copiar a `_lab/o12-norm-c4.json`). Esperado: versión 1.0.12, índice de normas en 1.0.12, `Domain_Entity_Updated`.
3. Aceptación: `git diff HEAD -- SddIA/library/norms/capability-taxonomy.md` **vacío salvo `hash_signature`**. Si difiere en cualquier otra línea, la forja no reproduce el contenido de O10: se detiene (Kintsugi), no se corrige con IDE, y se abre el defecto de forja (candidatos: `patch_norm_content_update`, `split_md_frontmatter`).
4. Si el paso 1 deja el fichero truncado (43 líneas), es el síntoma de O10 y la causa **no** era el binario: parar y diagnosticar `apply_implementation_source_git_apply` antes de seguir.

**C. C-5 — decisión de diseño (laudo Racso 2026-11-14: opción `event_uri`; la alternativa «solo documentar» queda descartada).**
- `route-domain-event` pasa a **1.1.0** (minor: se añade `event_uri`, se retiran dos outputs que ya no son ciertos en modo puerto).
- Outputs nuevos: `success`, `delivery_status`, `event_uri` (`eda://{family}/{event_id}`, token lógico), `dispatch_mode`, `sweep`, `transaction_digest` (opcional). Se eliminan `parent_path` y `processing_header_path` de la declaración.
- Runtime (`route_domain_core.rs`): `event_uri` siempre; `parent_path`/`processing_header_path` solo si `!from_store` (laboratorio FS), nunca una ruta fabricada. Test que fija ambos casos.
- `iota_ci_smoke.rs`: informar `event_uri` en lugar de `parent_path` (con fallback a `parent_path` si existe).
- Motor: `run_process_forge` update acepta `process_outputs` **sin** `process_phases`; escribe `outputs`, sube versión y actualiza `process/index.md`. `patch_process_phases_update` recibe `outputs: Option<&Value>`. Tests: outputs-only preserva `phases`, `inputs`, UUID y cuerpo; el hash no cambia.
- Aplicación: `./sddia-run.sh --process entity-manager` con `entity_class: process`, `process_outputs`, `process_version: "1.1.0"`. Verificar con `git diff`: solo cambian `outputs`, `version` y fila de índice (el reserializado YAML no debe reordenar claves ni tocar el cuerpo; si lo hace, defecto del motor, no se «arregla» a mano).
- **Alternativa (si el Vértice Biológico la prefiere):** mantener los dos outputs y declararlos «token lógico, no ruta FS». Más barata, pero deja un campo llamado `*_path` que no apunta a un fichero. No recomendada.

**D. Higiene.** `git rm` de las 5 semillas inválidas/duplicadas de O11. Añadir a `man-43-o11-remediation-20261114.md` una corrección (C-4 y causa real). Nueva nota `docs/audits/man-43-o12-baseline-20261114.md` con la salida de A, la evidencia de B y C, y el veredicto.

### Secuencia

| Paso | Alcance | Bloquea |
|------|---------|---------|
| 1 | A: suite completa + check + gate | B, C |
| 2 | B: C-4 en dos `entity-manager`, diff vacío salvo hash | — |
| 3 | C: motor `process_outputs` + tests; rebuild en `SddIA/target` | 4 |
| 4 | C: runtime + smoke + tests; rebuild | 5 |
| 5 | C: `entity-manager` process update de `route-domain-event` | — |
| 6 | D: higiene, nota de auditoría, docs, commit | — |

Tras cada acuse JSON: verificar fichero en disco, sin polling del bus (DA-5). Colapso de un proceso oficial: Kintsugi, sin rescate por IDE.

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `StrReplace`/`Write` sobre `SddIA/{library,skills,tools,daemons,process,norms}/` | Cero en todo el hilo |
| Binario `SddIA/target/debug/execute-process` más antiguo que `forges/*.rs` o `route_domain_core.rs` | Prohibido invocar `entity-manager` |
| `capability-taxonomy.md` | Diff vacío contra `HEAD` salvo `hash_signature`; dos sellos `Domain_Entity_Updated` |
| `route-domain-event.md` | Solo cambian `outputs`, `version` y fila de índice; `phases` y `hash_signature` intactos |
| Respuesta del proceso con `from_store` | Nunca contiene una ruta que no exista en disco |
| Cualquier fallo de línea base distinto de los 2 conocidos | Bloquea; se documenta como hallazgo |
| `APTO_TOTAL` | Prohibido: C-3 abierto |

### Verificación

```bash
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build -p execute-process
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo check --workspace
cd SddIA && cargo run -q -p sddia-qa -- gate-event-manager-port
git diff HEAD -- SddIA/library/norms/capability-taxonomy.md
git diff HEAD -- SddIA/process/route-domain-event.md SddIA/process/index.md
```

**Commit:** `fix(eda): C-4 oficial + route-domain-event outputs por event_uri (MAN-43 O12)`.

**AC:** cierra C-4 y C-5 con evidencia de rama; acerca AC-19 (suite). AC-13 sigue abierto por C-3.
**Fuera:** C-3 (O13), clippy estructural, fallos preexistentes ya listados.

### O13 — ver sección «O13» más abajo

---

## O13 — Regresión O10 + clase `capability-contract` + C-3 + identidad de los 17 contratos (cerrado en rama)

Estado: **ready**. Sin commit en la planificación. Laudos Racso 2026-11-14: C-3 con nueva clase `capability-contract` en `entity-manager` (no edit IDE); identidad embebida `x-sddia` (opción A) y migración de los 16 contratos restantes en este mismo hilo.

### Inventario verificado (2026-11-14, sobre `047573df`)

| Hallazgo | Evidencia |
|----------|-----------|
| 3 tests del validador DI rotos por O10 (fixture `contract_schema_rel: "x"`); 1 test no hermético (`SDDIA_EXECUTE_PROCESS_BIN`). Ver § «Corrección posterior a O12». | `capability_di_output_validator.rs:150-165,200`; `route_domain_core.rs:4521` |
| C-3 sigue abierto: schema `eda.event_manager.v1.schema.json` con descripción «v1.2.0»; enum de 18 operaciones (el trait `EventManagerPort` expone 19 con `list_terminal_events` y `list_progress`/`seal_dlt`/`get_dlt_seal` ya presentes; falta `list_terminal_events`); rama `publish_ecst` exige `event_id`+`ecst` pero no `event_uri` (el struct `PublishEcstRequest` lo exige y valida que coincida); `response` es solo envelope. | `contract.rs:162-190`, `port.rs:74`, schema |
| No existe clase de entidad para contratos JSON: `entity_manager.rs` fija 10 clases en `PILOT_CLASSES`, `creator_name`, `dir_by_class` y `materialize_by_inputs`; no hay `capability-contract-creator`. | `entity_manager.rs:10-66`, `factory.rs:1909` |
| `emit-domain-mutation.inputs.entity_class` declara `process\|agent\|skill\|tool\|action\|norm\|codex\|event` (sin `suite`, `daemon`); `entity-manager.inputs.entity_class` declara 10 clases. El sello funciona con `daemon` (O11), así que el enum **no se aplica en runtime**: deriva documental ya existente. | `actions/emit-domain-mutation.md:14`, `process/entity-manager.md:5` |
| El crate `execute-process` ya depende de `sddia-core` y `jsonschema` 0.26: el test de paridad schema↔serde cabe ahí sin dependencias nuevas. | `Cargo.toml:24,33` |
| Vacío normativo: `.cursorrules` §2, `README.md` («Cicatriz digital y estándar atómico») y `.cursorrules` §3 solo reconocían el sustrato `{name}.md`. Los 17 schemas de `capability-contracts/` son `*.schema.json` puros, sin identidad. Resuelto por el laudo en **Norma de Identidad Polimórfica** (ver § siguiente), aplicada **antes** de ejecutar O13. | `ls capability-contracts`, `README.md:324` |

### Validación del laudo (2026-11-14): coherencia y alucinaciones

**Verificado y correcto:** «Cicatriz Digital» existe (`README.md:324`, `CONSTITUTION_CORE.md:50`); las 10 clases del sustrato A coinciden con `PILOT_CLASSES` de `entity_manager.rs`; `x-` es palabra clave de anotación válida en 2020-12; DA-2 y DA-5 existen con ese significado (`external-ai-constraints.md`); `.cursorrules` está fuera de los directorios de genoma de DA-2, y el hook `.cursor/hooks.json` solo guarda `docs/todos/`.

**Incoherencias detectadas y resolución:**

| # | Incoherencia | Resolución |
|---|--------------|------------|
| 1 | El laudo pide editar solo `.cursorrules`, pero el término y la regla «toda entidad = `{name}.md`» viven también en `README.md` (parámetro de restricción obligatorio, `.cursorrules` §7). Dejarlo habría hecho que la norma nueva infrinja el README (error crítico por §7). | `README.md` § «Cicatriz digital y estándar atómico» actualizado en el mismo cambio. `CONSTITUTION_CORE.md:50` no menciona `.md` («estándar de entidades del Core»): compatible, no se toca. |
| 2 | `.cursorrules` §3 marcaba como «Entropía/Código Fósil» toda entidad sin `{name}.md`: con el laudo solo en §2, los 17 contratos seguirían siendo «fósiles» por §3. | §3 actualizado al «sustrato canónico de su clase». |
| 3 | `{name}.schema.json` vs el plan: el fichero real es `eda.event_manager.v1.schema.json` y mi plan usaba `entity_name: eda.event_manager`. | `entity_name` = stem del fichero (ver § Identidad). |
| 4 | §2 exige `id` (kebab-case) y `type`; el sustrato B del laudo solo lleva `uuid`, `version`, `hash_signature`, y los nombres de contrato llevan puntos (no kebab). Además, **ningún** `.md` del genoma lleva hoy `id` ni `type` (usan `name`): §2 ya era aspiracional. | Se mantienen los tres campos del laudo; `id`/`type` del sustrato B se derivan del sustrato (escrito en §2). La deuda preexistente del sustrato A (`id`/`type` ausentes) queda **fuera** de O13 y se anota. Aprovechado para completar el enum `type` de §2 con las 10 clases (antes listaba 5). |
| 5 | «No gobernamos por excepciones» choca con `evolution_contract.md:21`, que **exceptúa** expresamente el estándar atómico para `evolution/`. | Fuera de alcance de O13; anotado como residual para el Vértice Biológico (candidato a un tercer sustrato o a una nota en la norma). No se toca. |
| 6 | La adopción de una identidad nueva se sella como `Domain_Entity_Created` (sin hash previo). Parchear antes de adoptar emitiría `Updated` sobre algo sin nacimiento. | Orden fijado: `adopt` de `eda.event_manager.v1` y después el parche. |
| 7 | «Valor del reinado» no existe como término en el repositorio. | Interpretado como «valor del laudo». |

**Valor del laudo (auditoría):** *a favor* — elimina una excepción ad hoc y la sustituye por una regla con dos sustratos explícitos; respeta la **intención** de Cicatriz Digital (un único archivo con identidad inmutable) y no la letra del `.md`; es barata de implementar (17 inserciones de una clave) y deja el hash recalculable. *Riesgos reales* — (a) la regla vive ahora en tres sitios (`.cursorrules`, `README.md`, y SSOT de contratos): hay que mantenerlos alineados; (b) ningún escáner reconoce aún `x-sddia`: el test de identidad de los 17 (uuid únicos + hash recalculable) es el único guardián, y un edit manual de un schema rompería el hash sin que nada lo detecte hasta correr la suite → candidato a gate permanente (propuesto para O14, no se añade en O13); (c) `evolution_contract.md` sigue siendo una excepción vigente, lo que debilita el «no gobernamos por excepciones».

### Decisión de identidad (laudo Racso 2026-11-14: opción A, y migrar los 16 restantes en este hilo)

- **Modelo:** metadatos embebidos en cada schema, clave de nivel raíz `"x-sddia": { "uuid", "version", "hash_signature" }` (`x-` es palabra clave de anotación en JSON Schema 2020-12: no altera la validación). Un solo fichero por contrato; sin descriptor `.md`.
- **Norma de Identidad Polimórfica (laudo Racso 2026-11-14).** No se gobierna por excepciones: se oficializa el **Polimorfismo de Sustrato de la Cicatriz Digital**, con dos sustratos canónicos en fichero único. **A:** entidades declarativas/orquestación (`agent`, `process`, `action`, `skill`, `tool`, `norm`, `codex`, `event`, `suite`, `daemon`) → `{name}.md` con YAML Frontmatter. **B:** entidades de esquema computacional (`capability-contract`) → `{name}.schema.json` con `"x-sddia": { "uuid", "version", "hash_signature" }` en la raíz. Ya aplicada en `.cursorrules` §2/§3 y `README.md` § «Cicatriz digital y estándar atómico» (antes de O13). El registro en `SddIA/evolution/` documenta esta **evolución del estándar de Cicatriz Digital** (no una excepción), con el UUID de la tarea; si se toca `evolution/`, pasa por la cápsula canónica y `gate-evolution --range` antes del push.
- **Nombre de entidad = stem del fichero.** `{name}.schema.json` se cumple literalmente: `entity_name` = nombre del fichero sin `.schema.json` (`eda.event_manager.v1`, `doc.closure`, `llm-registry`…). Admite namespace con puntos; `id` y `type` se derivan del sustrato (§2). No se usa un parámetro `contract_schema` en la forja: el fichero es `{entity_name}.schema.json`. El binding DI sigue apuntando al fichero por `contract_schema` (O10), sin cambios.
- **`version`** de `x-sddia` = versión del contrato en el catálogo de `capability-taxonomy.md` (p. ej. `1.0.0`; `eda.event_manager` pasa de `1.2.0` a `2.0.0` con el patch C-3). Se pasa explícita en la semilla; un test la contrasta con el catálogo.
- **`hash_signature`** = sha256 del JSON canónico (claves ordenadas) del schema **sin** la clave `x-sddia`. Independiente del formato del fichero.
- **Migración:** los 17 schemas de `capability-contracts/` quedan con identidad (`uuid` v4 generado por `generate_uuid`). Los 16 no tocados por C-3 reciben **solo** la clave `x-sddia`.
- **Formato:** `serde_json` del workspace no tiene `preserve_order`: reserializar reordenaría las claves alfabéticamente y ensuciaría el diff de los 16. Por eso la operación `adopt` inserta el bloque `x-sddia` **por texto** tras la llave de apertura, dejando el resto del fichero byte a byte. Test: tras `adopt`, quitar el bloque insertado devuelve el fichero original idéntico. El único schema que se reserializa es `eda.event_manager` (patch RFC 6902): su diff textual será ruidoso una vez; la verificación es **semántica** (`canon(antes) + patch == canon(después)`, comparada con la salida del test).

### Diseño cerrado

**A. Corregir baseline (bloquea B y C).**
1. Fixtures de `capability_di_output_validator.rs`: `contract_schema_rel` real (`SddIA/library/norms/capability-contracts/doc.closure.schema.json` en el fixture temporal; `…/tracker.lifecycle.schema.json` en el repo real). Añadir test: binding con `contract_schema` de nombre no estándar valida contra ese fichero.
2. Hermetizar `store_backed_native_capsule_does_not_use_cognitive_runtime`: guardar, `remove_var("SDDIA_EXECUTE_PROCESS_BIN")` y restaurar al final.
3. Re-ejecutar la suite completa con `env -u SDDIA_EXECUTE_PROCESS_BIN` (y comprobando `env | rg '^SDDIA'` vacío). Criterio: **0 fallos**. Si aparece cualquier otro, bloquea y se documenta.
4. Corregir `docs/audits/man-43-o12-baseline-20261114.md` (veredicto, causas reales, reconocer que O12 incumplió su propia regla de bloqueo).

**B. Clase `capability-contract` (motor).**
- Forja nativa `run_capability_contract_forge` en `forges/factory.rs`; arm en `materialize_by_inputs`; `PILOT_CLASSES`/`creator_name`/`dir_by_class` en `entity_manager.rs`. Directorio leído de `directories.capability_contracts` (Cúmulo), no cableado. Sin proceso `*-creator`: clase **solo nativa**, sin fallback a proceso (el fallback hoy convierte un fallo nativo en `CAPABILITY_PROVIDER_*`; no debe aplicarse).
- `create`: `schema_json` (objeto) → escribe `{entity_name}.schema.json` con `x-sddia` (uuid nuevo, versión de la semilla, hash). `adopt` (modo de `update` sin parche de contenido, exige `schema_version`): si el schema existe **sin** `x-sddia`, inserta el bloque por texto; si ya lo tiene, devuelve el handoff idempotente (no regenera `uuid`). `update`: `schema_json_patch` = **RFC 6902** (`add`/`replace`/`remove` por JSON Pointer) + `schema_version`; aplica el patch, fija `x-sddia.version`, recalcula `hash_signature`, preserva `uuid`; el patch no puede apuntar a `/x-sddia`. `update` sin patch ni `adopt` → `Err` (regla anti-sobrescritura de O11). `delete` → `Err` explícito (fuera de alcance).
- Seguridad: `entity_name` sin `/`, `\`, `..` (misma regla que `contract_schema_file`, O10); el resultado debe ser JSON Schema válido (se compila con `jsonschema::Validator::new` antes de escribir; si no compila, no se escribe nada). Test de compilación de los **17** schemas con `x-sddia` insertado.
- Documentación de contrato por `entity-manager`: `emit-domain-mutation.inputs.entity_class` y `entity-manager.inputs.entity_class` pasan a `process|agent|skill|tool|action|norm|codex|event|suite|daemon|capability-contract` (action update con `action_inputs`; process update con `process_inputs`). Con ello se cierra de paso la deriva `suite`/`daemon`.
- Tests: create idempotente, `adopt` inserta y es idempotente, `adopt` byte a byte reversible, update por patch con UUID estable, update sin patch = error, patch sobre `/x-sddia` = error, schema inválido tras patch = error sin escribir, nombre con `..` = error, directorio tomado de Cúmulo (fixture con ruta distinta), `x-sddia.version` coincide con el catálogo de `capability-taxonomy.md`.
- **Los schemas embebidos con `include_str!`** (`di.binding.schema.json` en `capability_di_reactor.rs:394` y `cerbero_di_envelope.rs:309`) cambian de bytes al adoptar: los tests que los usan deben seguir verdes (se mide en la suite).

**C. C-3 por la clase nueva.**
- Patch RFC 6902 sobre `eda.event_manager.v1.schema.json` (nombre y `$id` se conservan; O10 lo fijó vía `contract_schema`): `description` → «Contrato EventManager 2.0.0 …»; añadir `list_terminal_events` al enum y su rama `if/then` con los campos reales de `ListTerminalEventsRequest`; rama `publish_ecst`: `event_uri` requerido con patrón `^eda://(telemetry|orchestration|domain)/[^/\\]+$` y nota «`event_id` debe coincidir con `event_uri.event_id`».
- **Oráculo = test de paridad** (`execute-process`, lib): para cada una de las 19 operaciones del trait, construir con tipos de `sddia-core` una petición mínima válida, serializarla como `{operation, payload}` y exigir que valide contra el schema; y su variante sin un campo obligatorio debe **fallar**. Además el enum del schema debe ser igual al conjunto de operaciones del trait. Si el test destapa más discrepancias, se incluyen en el mismo patch si son aditivas; si exigen decisión, se documentan y bloquean C-3.
- Aplicación en **dos** invocaciones, en este orden: (1) `adopt` de `eda.event_manager.v1` (`schema_version: "1.2.0"`, sin hash previo → sello `Domain_Entity_Created`, que es la semántica correcta del nacimiento de la identidad); (2) `update` con `semantic_seed.schema_json_patch` y `schema_version: "2.0.0"` (hash previo presente → `Domain_Entity_Updated`). `entity_class: capability-contract`, `entity_name: eda.event_manager.v1`. Verificar: diferencia semántica == patch (+ `x-sddia`).
- `result` por operación (esquemas de respuesta) queda **fuera**: no forma parte de lo que C-3 declaró.

**C2. Adopción de los 16 restantes (laudo: no quedan como deuda).** Una invocación `entity-manager` por contrato (`entity_class: capability-contract`, `lifecycle_operation: update`, `semantic_seed.adopt: true`, `schema_version` = versión de catálogo). Contratos (`entity_name` = stem): `agenda.persist`, `audit.compliance`, `bus.route`, `channel.ingest`, `di.binding`, `doc.closure`, `fs.persist`, `gov.rbac`, `llm.infer`, `llm.interact`, `llm-registry`, `memory.pref_query`, `memory.pref_write`, `proc.git_sync`, `qa.probe`, `tracker.lifecycle`. Verificación por contrato: el diff contiene **únicamente** el bloque `x-sddia` insertado; el schema compila; `uuid` único en los 17; `hash_signature` recalculable por el test. Las semillas se generan leyendo las versiones del catálogo, no escritas a mano. `di.binding` y `llm-registry` son contratos auxiliares que **no** figuran como capacidad en el catálogo: se adoptan con `1.0.0` y quedan anotados como tales en la auditoría (el test de «versión == catálogo» los excluye explícitamente, no los salta en silencio).

**D. Cierre.** Nota `docs/audits/man-43-o13-contract-schema-20261114.md` con baseline limpia, evidencia de B y C y veredicto. Tras O13 no quedan C-2…C-5 abiertos: la siguiente ola es **re-auditoría AC-13** (O14), único lugar donde puede escribirse `APTO_TOTAL`.

### Secuencia

| Paso | Alcance | Bloquea |
|------|---------|---------|
| 1 | A: fixtures + hermetizar test + suite completa en entorno limpio + corregir auditoría O12 | todo |
| 2 | B: forja nativa + clase en `entity_manager` + tests; rebuild en `SddIA/target` | 3–5 |
| 3 | Test de paridad (rojo contra el schema actual: debe fallar por `event_uri`/`list_terminal_events`) | 4 |
| 4 | `entity-manager`: `emit-domain-mutation` y `entity-manager` (enum de clases) | — |
| 5 | `entity-manager`: `adopt` de `eda.event_manager.v1` y luego patch C-3; test de paridad en verde | — |
| 6 | C2: adopción de los 16 restantes (16 invocaciones) + test de identidad de los 17 | — |
| 7 | Registro en `SddIA/evolution/` de la evolución del estándar de Cicatriz Digital (Norma de Identidad Polimórfica) + `gate-evolution --range` si aplica | 8 |
| 8 | D: auditoría, docs, commit | — |

Tras cada acuse JSON: verificar fichero en disco; sin polling del bus (DA-5). Colapso de un proceso oficial: Kintsugi, sin rescate por IDE. El paso 3 debe verse **rojo** antes del 5; un test que nunca falló no es oráculo.

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `StrReplace`/`Write` sobre `SddIA/{library,skills,tools,daemons,process,actions,norms}/` | Cero en todo el hilo |
| `env \| rg '^SDDIA'` al medir la suite | Vacío |
| Suite `execute-process --lib` | 0 fallos; cualquier fallo bloquea (sin clasificar como «no bloqueante») |
| Test de paridad | Rojo antes del patch, verde después |
| Schema tras el patch | Compila con `jsonschema`; mantiene nombre y `$id` |
| `eda.event_manager.v1.schema.json` | Diferencia semántica == patch RFC 6902 + `x-sddia` (el diff textual puede ser ruidoso por reserialización) |
| 17 schemas de `capability-contracts/` | Todos con `x-sddia` válido; `uuid` únicos; hash recalculable; los 16 adoptados difieren del original **solo** en el bloque insertado |
| Norma de Identidad Polimórfica | `.cursorrules` §2/§3 y `README.md` ya la recogen; registro de evolución del estándar en `evolution/` con referencia al laudo; sin registro no hay commit |
| Binario `SddIA/target/debug/execute-process` más antiguo que `forges/*.rs` o `entity_manager.rs` | Prohibido invocar `entity-manager` |
| `APTO_TOTAL` | Prohibido (lo declara Argos en O15 sobre el dossier de O14) |

### Verificación

```bash
env | rg '^SDDIA' || echo limpio
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build -p execute-process
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo check --workspace
cd SddIA && cargo run -q -p sddia-qa -- gate-event-manager-port
git diff HEAD -- SddIA/library/norms/capability-contracts/
```

**Commit:** `fix(eda): capability-contract + eda.event_manager 2.0 + identidad x-sddia de 17 contratos + fixtures DI (MAN-43 O13)`.

### Resultado de ejecución (2026-11-14)

Auditoría: `docs/audits/man-43-o13-contract-schema-20261114.md` (**APTO_PARCIAL**). Suite lib 849/0; `check --workspace` OK; `gate-event-manager-port` OK; paridad rojo→verde; los 17 contratos con `x-sddia`.

Desviaciones respecto al diseño, todas documentadas en la auditoría: (1) la forja vive en `forges/capability_contract.rs` (no en `factory.rs`; solo el arm está en la fábrica); (2) `event_uri` del schema es un **objeto** `{family, event_id}` (el plan decía cadena `eda://`; error mío, corregido por el tipo `EventUri`); (3) el oráculo destapó más discrepancias aditivas (`processing`, 6 ramas) incluidas en el mismo parche; (4) `domain_mutation.rs` también necesitó la clase (la lista del sello es de runtime); (5) ensayo previo en copia del repo; (6) `SDDIA_PARITY_SCHEMA` permite ensayar un parche contra una copia.

**AC:** cierra C-3; repara la regresión de O10 y la baseline de O12; acerca AC-13 (todos los C-* cerrados). AC-13 lo declara Argos (O15) sobre el dossier de O14.
**Fuera:** esquemas de `result` por operación; `delete` de la clase; clippy estructural. (La migración de los 16 contratos restantes **entra** en O13 por laudo.)

---

## O14 — Desbloqueo pre-auditoría + dossier de evidencia AC-1…AC-19 (siguiente hilo)

**Corrección de alcance.** O10–O13 decían que O14 «declara» AC-13 o escribe `APTO_TOTAL`. No es así: el plan fija «Argos declara; Tekton solo evidencia» (§ Riesgos) y el veredicto previo salió de `pull-request-review` (`validacion.md`). O14 cierra los bloqueos medidos, ejecuta la matriz de evidencia y entrega un **dossier con veredicto propuesto**. La declaración de AC-13 pertenece a O15: push + PR + `pull-request-review` (Argos), vía `delivery-close-cycle`.

### Hallazgos medidos antes de planificar (2026-11-14, HEAD `6f01bfe9`)

| Hallazgo | Evidencia | Afecta |
|----------|-----------|--------|
| `gate-evolution --range`: **123** `EVOL_MATERIAL_UNREGISTERED`. Por último commit que tocó cada ruta: O9 38, O5.2 29, O5.1 16, O10 11, O11 9, O1 6, O12 3, O2 3, O4 3, O8 2, O5 1, O6 1, O0 1. Solo H1, O6 y O13 tienen registro. Base `origin/main` **stale** (3 días). | `sddia-qa gate-evolution --range --json` | AC-14; bloquea el push (pre-push) |
| AC-12 exige literalmente «`cargo test --workspace`, **clippy**, contract suite y gates verdes». O9 lo dio por cumplido solo con el gate `rustc` porque `rust-warning-gate` §1 excluye clippy: **contradicción entre AC y norma**, no resuelta por laudo. Residual O9: ~85 diagnósticos en `execute-process`, sin remedir desde O10–O13. O9 dijo «todos estructurales», pero `needless_return` y `unnecessary_unwrap` son mecánicos. | `man-43-o9-quality-20261113.md` | AC-12 |
| `cargo test --workspace` completo **no tiene evidencia** en ninguna ola: O6/O9/O12/O13 midieron `-p execute-process --lib`, `event-bus-audit` y `check --workspace`. (O9: `/tmp` lleno con `StorageFull`; usar `SddIA/target` y vigilar `df`.) | auditorías O6, O9, O12, O13 | AC-12 |
| AC-3: «ninguna API pública contiene `target_path`». Sobrevive como **token lógico** en salidas CLI: `entity-manager.outputs.target_path`, peaje termodinámico (`.events/telemetry/{id}.json`) y 15 `.md` de genoma. No hay escritura física: tras las 21 invocaciones de O13, `.events/*` tiene 0 ficheros nuevos. Ambigüedad: ¿AC-3 abarca solo los tipos Rust del puerto o también el contrato JSON de procesos y acciones? | `rg target_path SddIA/{process,actions}`; `find .events -newermt` | AC-3 |
| `DualVersionManifest::builtin_v2.schema_uri` apunta a `eda.event_manager.v2.schema.json`, que **no existe** (el fichero es `…v1…`, `$id` v1). Resto de F-02. | `sddia-core/.../contract.rs:528` | AC-1, F-02 |
| La matriz AC-1…AC-19 / F-01…F-12 no se ha re-ejecutado desde O6 (`NO_APTO_TOTAL`, F-03 y F-09/F-10 parciales). | `man-43-o6-remediation-20261113.md` | AC-13 |

### Laudos del Vértice Biológico (2026-11-14)

- **D-1 = (a).** Clippy en verde (`cargo clippy --workspace --all-targets`, cero avisos) dentro de O14. Los avisos mecánicos se corrigen; `too_many_arguments`/`type_complexity` se refactorizan o reciben `#[allow]` justificado uno a uno en la nota de cierre. AC-12 conserva su texto literal.
- **D-2 = (b).** AC-3 abarca también el contrato JSON de procesos y acciones: `target_path` se retira (→ `event_uri`) en motor y genoma vía `entity-manager`. Va en un hilo propio, **O14b**, que se planifica aparte. Consecuencia: la matriz y el dossier (C, D) pasan a **O14c**, tras O14b; sin O14b, AC-3 no tiene evidencia y no hay veredicto propuesto posible.

### Diseño

**A. Base sincronizada.** `git fetch origin main` (red) y `gate-evolution --range --sync-base`. Si `origin/main` avanzó, repetir la agrupación de hallazgos sobre la base nueva antes de B1.

**B1. Evolución de la rama (AC-14, desbloquea push).** Un registro consolidado `MAN-43 O0–O12 desacople total` en `SddIA/evolution/`, con una sección por ola (commit, qué cambió, auditoría) y `relacionado` generado de los hallazgos reales del gate, **sin prefijos de directorio amplios que oculten rutas** (lista exacta). Hash por `sddia-qa evolution-rehash`; fila en `Evolution_log.md`. Criterio: `gate-evolution --range` → `EVOL_OK`. Los registros existentes (H1, O6, O13) no se tocan.

**B2. `schema_uri` (F-02).** `builtin_v2` apunta al fichero real (`eda.event_manager.v1.schema.json`; el nombre es estable y la versión vive en `x-sddia.version`). Test nuevo: el `schema_uri` del manifiesto resuelve a un fichero existente de `capability_contracts` cuyo `x-sddia.version` == `CONTRACT_VERSION`. Rojo antes, verde después.

**B3. AC-12 (D-1 a).** Medir `cargo clippy --workspace --all-targets` y corregir por categorías (mecánicos primero). Cada `#[allow]` lleva justificación en la nota de cierre; la suite queda igual o mejor.

**B4. AC-3 (D-2 b) → O14b.** Retirada de `target_path` en salidas CLI (motor, peaje termodinámico, `entity-manager`, 15 `.md` de genoma vía `entity-manager`) + test de escaneo del puerto y de los contratos de proceso. Hilo propio.

**C. Matriz de evidencia.** Para cada AC-1…AC-19 y F-01…F-12: comando, resultado y log en `docs/audits/man-43-o14-*.log`. Mínimo:
- `cargo test --workspace` (completo, entorno `SDDIA*` vacío, `SddIA/target`).
- `cargo test -p sddia-core event_manager`; suite contractual sobre mock, Redb y externo (AC-19: `redb_event_adapter`, `fs_event_adapter`, `external_capsule_event_adapter/tests/e2e_dummy.rs`).
- `cargo check --workspace` con `RUSTFLAGS=-Dwarnings` (gate canónico); `cargo clippy --workspace --all-targets` con cero avisos (D-1 a).
- `gate-event-manager-port`, `gate-evolution --range`, `verify-process-integrity`, `audit-eda-coverage --scan`.
- AC-4/AC-5: `rg` de `.events/` y de ops FS (`read_dir`, `is_file`, `fs::write`) en código productivo de los crates EDA, **excluyendo tests de forma verificable** (lista de exclusiones en el dossier, no a ojo). Hoy hay ~30 ficheros `.rs` no-test que citan `.events/`; cada uno se clasifica (productivo, comentario, `cfg(test)`, token lógico).
- AC-6: watcher/sweeper sin `read_dir`/`is_file`/`event_file_path`.
- AC-10/AC-11: `test ! -d SddIA/sddia-event-store`; allowlist del gate vacía.
- Residuales O13 (`evolution_contract.md` §1, `id`/`type` del sustrato A, `dlt_threshold_ok`): fuera de los AC; se listan como deuda, no bloquean.

**D. Dossier.** `docs/audits/man-43-o14-reaudit-ac13-20261114.md`: capa posterior a `man-32-lineage-…` y `man-43-o6-…` (no se reescriben), tabla AC/F con evidencia, **veredicto propuesto** (`APTO_TOTAL_PROPUESTO` solo si todos los AC tienen evidencia verde; si no, `NO_APTO_TOTAL` con lista de bloqueos) y handoff a Argos. Tekton no escribe `APTO_TOTAL` en `validacion.md` ni en auditorías parentales.

### Secuencia

| Paso | Alcance | Bloquea |
|------|---------|---------|
| 1 | A: fetch + base sincronizada | 2 |
| 2 | B1: registro de evolución consolidado → `gate-evolution --range` `EVOL_OK` | 6 |
| 3 | B2: test rojo → fix `schema_uri` → verde | 6 |
| 4 | B3 (D-1 a): medir clippy, corregir por categorías, suite igual o mejor | 6 |
| 5 | `cargo test --workspace` completo como baseline (primera evidencia de AC-12) | 6 |
| 6 | Nota `docs/audits/man-43-o14-preaudit-20261114.md` + docs + commit | O14b |
| — | **O14b**: retirada de `target_path` (B4, D-2 b) — se planifica aparte | O14c |
| — | **O14c**: C + D (matriz AC-1…AC-19 / F-01…F-12 y dossier con veredicto propuesto) | O15 |

Tras cada acuse JSON del CLI, sin polling (DA-5). Un fallo de proceso oficial → Kintsugi. Cualquier fallo de la matriz C **bloquea** el veredicto propuesto: no se reclasifica como «no bloqueante» (lección O12).

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `StrReplace`/`Write` sobre `SddIA/{library,skills,tools,daemons,process,actions,norms}/` | Cero (`evolution/` sí, con hash por CLI canónica) |
| `relacionado` con prefijos amplios para silenciar el gate | Prohibido; lista exacta de rutas |
| `gate-evolution --range` | `EVOL_OK` sobre base sincronizada |
| `cargo test --workspace` | 0 fallos con `env \| rg '^SDDIA'` vacío |
| Veredicto `APTO_TOTAL` escrito por Tekton | Prohibido; solo `…_PROPUESTO` + handoff |
| `#[allow(clippy::…)]` sin justificación escrita | Prohibido; cada uno listado en la nota |

### Verificación

```bash
env | rg '^SDDIA' || echo limpio
git fetch origin main && SddIA/target/debug/sddia-qa gate-evolution --range --sync-base
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test --workspace
cd SddIA && RUSTFLAGS=-Dwarnings CARGO_TARGET_DIR=$PWD/target cargo check --workspace
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo clippy --workspace --all-targets   # D-1 a: cero avisos
cd SddIA && cargo run -q -p sddia-qa -- gate-event-manager-port
```

**Commit:** `fix(eda): evolution O0–O12 + schema_uri + clippy verde (MAN-43 O14)`.

**AC:** AC-14 (evolución de rama), F-02 residual, AC-12 (clippy + workspace). **O14b:** AC-3. **O14c:** dossier AC-13. **O15:** `delivery-close-cycle` → PR → `pull-request-review` (Argos) declara.
**Fuera:** `evolution_contract.md` §1; `id`/`type` en sustrato A; `dlt_threshold_ok`; esquemas `result` del contrato eda.

### Resultado de ejecución (2026-11-14)

Auditoría: `docs/audits/man-43-o14-preaudit-20261114.md` (**APTO_PARCIAL**). Evolution `d0bb10b2`; clippy workspace 0 errores; F-02 cerrado; `execute-process --lib` 849/0. Residual: `cargo test --workspace` — `linear-tracker-adapter` AUT-68 (diagnóstico en O14b: test obsoleto frente al catálogo MAN-39, no «preexistente» sin causa). Próximo: O14b (`target_path`), O14c (dossier).

---

## O14b — Retirada de `target_path` / `event_file_path` de los contratos de evento (siguiente hilo)

Laudo D-2 (b): AC-3 abarca el contrato JSON de procesos y acciones. Esta ola retira el token de ruta del contrato de eventos y lo sustituye por `event_uri`. Incluye el paso 0: cierre del residual AC-12 de O14.

### Hallazgos medidos (2026-11-14, HEAD `c0fde1ce`)

| Hallazgo | Evidencia | Afecta |
|----------|-----------|--------|
| **Residual AC-12 de O14 = test obsoleto, no fallo del adaptador.** `aut68_missing_label_names_team` espera `LINEAR_LABEL_UNKNOWN` para la etiqueta `kaizen` en el equipo MAN, pero el catálogo de laboratorio de MAN incluye `Kaizen` desde MAN-39 (`bcf50739`: «label Kaizen lab MAN»). Los otros 10 fallos de `linear-tracker-adapter` son `PoisonError` en cascada del mutex `TEST_ENV`. Corrige mi nota O14, que lo llamó «preexistente» sin diagnosticar la causa. | `lab_team_label_catalog` (`main.rs:106`); `-- --test-threads=1` | AC-12 |
| `target_path` es un **token fabricado**, no una ruta: `eda_bus.rs:205` hace `format!(".events/{family}/{event_id}.json")` sobre un evento que ya está en Redb (comentario «Token lógico… no ruta física»). Cero ficheros nuevos en `.events/`. | `eda_bus.rs:203`, `fractal.rs:64` | AC-3 |
| El token es el primer eslabón de una **tubería**: `seal.target_path` → `route-domain` con `{"event_file_path": target}` (`telemetry_compliance_core.rs:77`, `radamanto_batch_core.rs:486`). `event_file_path` es el mismo token con otro nombre (64 apariciones en 18 `.rs`; 14 `.md` de genoma, 12 procesos). Retirar solo `target_path` dejaría `event_file_path` colgando. | `rg event_file_path` | AC-3 |
| `target_path` de **evento** (productores/consumidores): `eda_bus`, `fractal`, `domain_mutation`, `entity_manager`, `executor`, `phase_capsules`, `residual_runner`, `delivery_close`, `accept_pr`, `aiua_intent`, `suite_execution_requested`, `user_preference_change_requested`, `system_vitality`, `telemetry_compliance_core`, `radamanto_batch_core`, `governance_daemon`/`daemon_kill_switch` (`orchestration_event_path`), `sddia-qa/eda_e2e_lab` (`handoff.target_path`). Genoma: 9 `actions/emit-*`, `materialize-*`, `process/entity-manager`, `delivery-close-cycle`, `accept-pr`, `events/domain/fracture-pbi-materialized`, `skills/intent-transpiler`, `tools/event-bus-audit`. | `rg -c target_path` | AC-3 |
| `target_path` de **fichero** (otro concepto, ruta real): `fracture_pbi.rs` (PBI en `docs/todos/…`), `materialize_*`, `enrich_fracture_pbi_kaizen` (ruta o ref del PBI), `markdown-table-editor`, `filesystem-manager` (+ norma `skill-io-filesystem-manager-frozen`), `sddia-workspace-server`, `event-bus-audit.kaizen_target_path`. No son contrato de evento. | lectura de cada uso | AC-3 |
| El puerto (`sddia-core/ports/event_manager`, adaptadores, host) ya tiene **cero** `target_path`/`logical_rel`/`event_file_path`. El residual vive en el motor y el genoma, no en el puerto. | `rg` sobre esas rutas | AC-3 |
| `process/route-domain-event` ya declara `event_uri` como **salida** (`eda://{family}/{event_id}`, `domain_event_uri_token`, `route_domain_core.rs:2326`); falta como **entrada**. | `route-domain-event.md`; `route_domain_core.rs:340-372` | AC-3 |

### Laudos del Vértice Biológico (2026-11-14)

- **E-1 = (a).** O14b retira `target_path` **y** `event_file_path` (→ `event_uri` / `event_uris`) en la misma ola.
- **E-2 = (a).** El `target_path` de fichero real se conserva como parámetro de operaciones de fichero. Lista blanca explícita en el oráculo y en la nota: `fracture_pbi`, `materialize_*`, `enrich_fracture_pbi_kaizen` (ref/ruta del PBI), `markdown-table-editor`, `filesystem-manager` (+ norma congelada), `sddia-workspace-server`, `event-bus-audit.kaizen_target_path`. Cualquier otro uso es contrato de evento y se retira.

### Diseño

**Contrato sustituto.** El sello de publicación devuelve `{event_id, event_uri, family}` con `event_uri` = `EventUri::as_canonical()` (`eda://{family}/{event_id}`). Los consumidores resuelven por puerto (`get_event`), nunca por ruta. Corte duro: todos los consumidores están en el repo y se mueven en la misma ola; sin alias de transición.

**0. Residual AC-12.** Test `aut68_missing_label_names_team`: usar una etiqueta ausente del catálogo de MAN (p. ej. `other`), manteniendo la intención (el mensaje nombra equipo y etiqueta). Sin tocar el catálogo. Verificar `linear-tracker-adapter` completo y `cargo test --workspace`. Los tests siguen mutando `.SddIA/lab-linear-store.json` (versionado): no se incluye en el commit.

**1. Oráculo rojo.** Test `eda_event_token_hygiene` (execute-process, al estilo de `eda_contract_parity`): (i) el sello de `write_fractal_event` no contiene `target_path` y sí `event_uri`; (ii) frontmatter de procesos/acciones del contrato de eventos sin `target_path`/`event_file_path` en `inputs`/`outputs`, con lista blanca de ficheros de E-2; (iii) fuente del puerto, adaptadores y host sin los tres tokens. Rojo antes, verde después.

**2. Productores.** Sello `eda_bus`/`fractal` y propagadores (`phase_capsules`, `executor`, `residual_runner`, `delivery_close`, `accept_pr`, `aiua_intent`, `suite_execution_requested`, `user_preference_change_requested`, `system_vitality`, `domain_mutation`, `entity_manager`) pasan de `target_path` a `event_uri`. `orchestration_event_path` → `orchestration_event_uri`.

**3. Consumidores.** `route-domain` acepta `event_uri` (resuelve el `event_id`, carga el ECST por puerto); `telemetry_compliance_core` y `radamanto_batch_core` pasan `event_uri`. Con E-1 (a): `event_file_path`/`event_file_paths` → `event_uri`/`event_uris` en `route_domain_core`, `route_fractal_core`, `ecst_input_loader`, `residual_runner`, handlers y `event-watcher`; `eda_e2e_lab` lee `handoff.event_uri`.

**4. Genoma por `entity-manager`** (DA-2; ensayo previo en copia `/tmp/o14b-lab` con `target` enlazado): `actions/emit-*`, `materialize-*`, `process/entity-manager`, `route-domain-event` y los 12 procesos con `event_file_path`, skills `compliance-auditor`/`rbac-governor`, `events/events-contract.md`, `delivery-close-cycle`, `accept-pr`. SemVer: **MAJOR** por entidad cuyo I/O cambia (precedente: contrato eda 2.0.0); índices y catálogo sincronizados por la forja.

**5. Evolution.** Un registro nuevo con `relacionado` en lista exacta, rehash por `sddia-qa evolution-rehash`, `gate-evolution --range` en `EVOL_OK`.

**6. Nota** `docs/audits/man-43-o14b-event-token-20261114.md` (APTO_PARCIAL/APTO según evidencia; sin `APTO_TOTAL`).

### Secuencia

| Paso | Alcance | Bloquea |
|------|---------|---------|
| 0 | Test AUT-68 + `cargo test --workspace` completo | 6 |
| 1 | Inventario `target_path` evento vs fichero: fijar la lista blanca E-2 (hay usos ambiguos: `transit-event-payload`, `manage-event-receipt`, `enrich_fracture_pbi_kaizen` se clasifican por lectura) | 2 |
| 2 | Oráculo `eda_event_token_hygiene` en rojo | 3 |
| 3 | Productores + consumidores en motor (hasta verde en `--lib`) | 4 |
| 4 | Ensayo en copia → genoma por `entity-manager` | 5 |
| 5 | Evolution + gate | 6 |
| 6 | Nota + docs + commit | O14c |

Tras cada acuse JSON del CLI, sin polling (DA-5). Fallo de proceso oficial → Kintsugi.

### Casos que bloquean el cierre

| Caso | Resultado exigido |
|------|-------------------|
| `StrReplace`/`Write` sobre `SddIA/{library,skills,tools,daemons,process,actions,norms,events}/` | Cero (solo `entity-manager`) |
| `target_path`/`event_file_path` en contrato de evento | Cero, salvo lista blanca E-2 documentada |
| Alias de transición (`target_path` junto a `event_uri`) | Prohibido |
| `cargo test --workspace` y clippy | 0 fallos / 0 errores, `SDDIA*` vacío |
| `gate-evolution --range` | `EVOL_OK` |
| Test que pasa por haberse debilitado el assert | Prohibido; el AUT-68 conserva su intención |

### Verificación

```bash
env | rg '^SDDIA' || echo limpio
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p linear-tracker-adapter
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib eda_event_token_hygiene
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test --workspace
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo clippy --workspace --all-targets
rg -n 'target_path|event_file_path' SddIA/{process,actions,events} SddIA/engine SddIA/sddia-core   # solo lista blanca E-2
SddIA/target/debug/sddia-qa gate-evolution --range --sync-base
```

**Commit:** `refactor(eda): event_uri sustituye target_path/event_file_path en contratos (MAN-43 O14b)`.

**AC:** AC-3 (contrato JSON), AC-12 (workspace completo). **O14c:** matriz + dossier.
**Fuera:** `target_path` de fichero (lista blanca E-2); esquemas `result` del contrato eda; refactor estructural de aridad clippy.

### Resultado de ejecución (2026-11-14, rama)

Auditoría: `docs/audits/man-43-o14b-event-token-20261114.md` (**APTO_PARCIAL**). Oráculo `eda_event_token_hygiene` 5/0; motor y genoma contractual migrados; `execute-process --lib` 854/0; `linear-tracker-adapter` 14/0. Residual workspace: `ephemeral-cache-purger::jail::tests::purge_deletes_fixture` (fuera del alcance token EDA). Pendiente: evolution + gate + commit único antes de O14c.

---

## H4 — Consumidores periféricos (índice)

El detalle ejecutable está en **O4 / H4**.

| # | Entregable | Ola |
|---|------------|-----|
| H4.1–H4.4 | actions, Kalma2, workspace-server, GitHub DL | **O4** |
| H4.5 | Callers productivos sin `runtime_db_path` | **O4**; borrar el símbolo, **O5** |

---

## H5 — Laboratorio, demolición y gates

| # | Entregable | Detalle |
|---|------------|---------|
| H5.1 | Migrar matrices | Fuera de `sddia-event-store`; harness/mock/contract suite |
| H5.2 | Borrar `eda_port_legacy` | Módulo + ramas cfg |
| H5.3 | Borrar crate `sddia-event-store` | Workspace + deps |
| H5.4 | Allowlists legacy vacías | Gates detectan reintroducción |
| H5.5 | Gates v2 | Veto `.events/` runtime, símbolos path, store legacy, major 1 |
| H5.6 | Verificación | `cargo test --workspace`, clippy, contract suite, gates |

**Commit(s):** `test(eda): demolición store legacy + gates v2 (MAN-43 H5)`.

**AC:** AC-10, AC-11, AC-12.

**Condición L-7:** no borrar store hasta H5.1–H5.4 listos.

---

## H6 — Convergencia documental

| # | Entregable | Detalle |
|---|------------|---------|
| H6.1 | README / Constitución / procesos / contratos | Narrativa `EventUri` + puerto v2; cero topología runtime `.events/` |
| H6.2 | `cumulo.paths.json` | Solo IDs/tablas/suscripciones lógicas |
| H6.3 | Auditorías | Reejecutar la auditoría consolidada 20261111 sobre el resultado; parental + legacy → `APTO_TOTAL` **solo con evidencia** |
| H6.4 | `SddIA/evolution/` | Registro cutover 2.0.0 |
| H6.5 | Docs forja | `implementation.md` + `execution.md` |

**Commit:** `docs(eda): convergencia EventManager v2 / APTO_TOTAL (MAN-43 H6)`.

**AC:** AC-13, AC-14.

---

## Orden de ejecución Tekton

1. **H0** precondiciones (+ `forge_key` si aplica).
2. **O0/H1R** integridad exacta — bloquea expansión del adapter.
3. **O1/H1C** contrato/URI/modelo/suite/Genoma — desbloquea el cutover.
4. **O2/H2** entregas/testigos (fuente de estado).
5. **O3/H3** watcher/sweeper (cutover sin huérfano; mismo tren que deja de necesitar espejos).
6. **O4/H4** periféricos (paralelizable con O2 tras O1).
7. **O5/H5** lab + demolición (tras consumidores limpios).
8. **O6/H6** docs/evolution/auditorías → handoff Argos/`validacion.md`.
9. **O7** `event-bus-audit` sin default `/.events/` (F-03 residual). AC-13 sigue abierto.
10. **O8** bindings + narrativa a contrato `2.0.0`; procesos solo `entity-manager` (DA-5). AC-13 sigue abierto.
11. **O8.1** frontmatter `route-domain-event` vía `process_phases` (no es el O9 de calidad).
12. **O9** calidad: clippy verde + suite workspace; diagnóstico del schema (C-2…C-5 son genoma).
13. **O10** motor DI honra `contract_schema` del binding (cierra C-2 en código). C-3/C-4/C-5 siguen abiertos; C-4 pide laudo.
14. **O11** remediación del genoma dañado por O10 (binario de forja obsoleto) + C-4 y O9 rehechos de verdad por `entity-manager`.
15. **O12** línea base de verificación + C-4 por cauce oficial (reintento real) + C-5 (`route-domain-event.outputs`). C-3 pasa a O13, con decisión previa.
16. **O13** repara la regresión de O10 (fixtures DI) y la baseline de O12; clase `capability-contract` en `entity-manager`; C-3 por esa vía.
17. **O14** desbloqueo pre-auditoría: evolución O0–O12, `schema_uri`, clippy verde (D-1 a), `cargo test --workspace`. **O14b** retirada de `target_path`/`event_file_path` → `event_uri` (D-2 b; paso 0 cierra el residual AC-12). **O14c** matriz AC-1…AC-19 + dossier con veredicto **propuesto**.
18. **O15** `delivery-close-cycle` → PR → `pull-request-review`: Argos declara AC-13.

No declarar AC-1…AC-19 sin evidencia. No seeds Kaizen desde Tekton. Git solo `skill:git-manager`.

## Criterios de salida del blueprint

- AC-1…AC-19 según `spec.md` / `objectives.md`.
- Laudos L-1…L-7, L-CHILDREN, L-SUPERSEDE-T0, L-AUDIT-GAP, L-AUDIT-DISK, L-CONTINUE-TEKTON, L-NON-GOALS, L-PROCESS, L-GIT, L-GENOME, L-TRACKER, L-FORGE respetados.
- Inventarios F-1…F-6 / L-P* / R1…R3 **cerrados** o reducidos a higiene ops no-SSOT.
- Tracker: **MAN-43** / documento **PBI-0f4f0f5f** / padre **MAN-40** / `execution_id` **3278ec5b-21ae-4fe0-af14-62a7e5ba0e07**.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| Consumidores externos aún en v1 | `version_gate` v2 + inventario H0; rechazo handshake |
| Ops insuficientes para sweeper | Ampliar puerto en H1 **antes** de H3 (**L-6**) |
| Eventos residuales solo en `.events/` | Política H0.3; higiene ≠ SSOT |
| Watcher sin fuente en cutover | H2 completo + H3 atómico (**L-4**) |
| Cambio simultáneo CLI/procesos | H1 primero; H4/H6 alinean después |
| README/Constitución con `.events/` permitido | H6 obligatorio (AC-14) |
| Reabrir MAN-36 | Diff review: cero cambios de precedencia/bóveda salvo API faltante |
| Inventar APTO / auditorías ausentes | H6 rematerializa (**L-AUDIT-DISK**); Argos declara; Tekton solo evidencia |
| `forge_key` ausente | H0.6 antes de código; no inventar éxito de forja |
| IDs opacos solapados | O0: igualdad exacta + tests de colisión antes de ampliar consumidores |
| Migración de filas v1 | Versión de registro + migrador explícito; prohibida inferencia silenciosa |
| Defaults del trait ocultan soporte parcial | Adapters v2 implementan operaciones obligatorias; suite falla ante default incompleto |

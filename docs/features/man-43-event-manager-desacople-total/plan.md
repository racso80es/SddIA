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
version_plan: "1.2.1"
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

## O6 / H6 — Convergencia documental (después de O5.2)

Estado: **ready**. O5.2 cierra AC-10/11/12 en rama. Índice en § H6 más abajo.

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

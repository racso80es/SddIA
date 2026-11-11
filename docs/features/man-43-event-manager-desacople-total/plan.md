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
version_plan: "1.1.1"
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

## O0 / H1R — Contención de integridad (primer hito pendiente)

Estado: **listo para ejecutar**. H1 histórico del PR #453 permanece intacto. O1–O6 no empiezan hasta que esta ola tenga sus tests verdes.

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

## H1 — Contrato EventManager 2.0 + EventUri

| # | Entregable | Detalle |
|---|------------|---------|
| H1.1 | `EventFamily` + `EventUri` | Enum + parse/serialización/validación; rechazo familias-estado |
| H1.2 | `PublishSeal` v2 | Solo `event_uri` (+ accessors); eliminar `logical_rel`/`target_path` |
| H1.3 | Ops reales | `list_deliveries`, listados por familia/tipo/estado/cursor, barrido terminales; `PublishEcstRequest` con `EventUri` |
| H1.4 | Constantes | `CONTRACT_VERSION=2.0.0`, `REQUIRED_CONTRACT_RANGE=^2.0.0` |
| H1.5 | Schema/bindings/taxonomía | Genoma vía entity-manager |
| H1.6 | `version_gate` / handshake | Rechazo explícito consumidores v1 |
| H1.7 | Contract suite | mock + Redb + adaptador externo: lifecycle, colisiones, entregas, familia, estado, terminales, cursor, duplicado y reopen |
| H1.8 | Adapters host | Implementar ops nuevas sin defaults incompletos ni exponer path |
| H1.9 | Modelo persistente | `StoredEvent` conserva `EventUri` + `EventQueueState`; migración explícita de filas v1 |

**Commit:** `feat(eda): EventManagerPort 2.0 + EventUri (MAN-43 H1)`.

**AC:** AC-1, AC-2, AC-3, AC-16, AC-17, AC-19.

---

## H2 — Absorción de entregas y testigos

| # | Entregable | Detalle |
|---|------------|---------|
| H2.1 | `eda_bus_topology` | Sin `write_json_atomic` de testigos/cabeceras; estado vía puerto |
| H2.2 | Terminal / witnesses | Solo puerto; sin OR `is_file` |
| H2.3 | `fractal_bus` | Sin mutación fichero residual en prod |
| H2.4 | github DL fallback | Preferir `move_to_dlq` (puede completarse en H4) |

**Commit:** `refactor(eda): entregas/testigos detrás del puerto (MAN-43 H2)`.

**AC:** AC-5, AC-7 (parcial hasta H3/H4).

**Bloqueo:** no iniciar H3 si ops de listado/terminal de H1 no están verdes.

---

## H3 — Watcher y sweeper sin filesystem

| # | Entregable | Detalle |
|---|------------|---------|
| H3.1 | `event-watcher` L-P1 | Eliminar lote `read_dir`; solo `claim_pending` |
| H3.2 | L-P2 | Cierre/DLQ sin FS |
| H3.3 | `eda_sweep` L-P3/F-5 | Barrido solo puerto |
| H3.4 | L-P4 | Consultas dominio vía `list_events` |
| H3.5 | Anti doble despacho | Un enrutado por evento/tick |
| H3.6 | Higiene L-H* | Purga vía `purge_event`; scripts ops fuera de SSOT |

**Commit:** `refactor(eda): watcher/sweeper 100% EventManagerPort v2 (MAN-43 H3)`.

**AC:** AC-4, AC-5, AC-6.

---

## H4 — Consumidores periféricos

| # | Entregable | Detalle |
|---|------------|---------|
| H4.1 | F-1 `actions` | Sin doble materialización FS |
| H4.2 | F-2 Kalma2 | Sin fallback FS |
| H4.3 | F-3 workspace-server telemetry | Publicación puerto `telemetry` |
| H4.4 | F-4 github_bridge | DLQ por puerto |
| H4.5 | AC-8 `eda_port` | Retirar `runtime_db_path` / `path_is_resolved_redb` de consumidores |

**Commit:** `refactor(eda): periféricos sin fallback FS (MAN-43 H4)`.

**AC:** AC-8, AC-9.

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

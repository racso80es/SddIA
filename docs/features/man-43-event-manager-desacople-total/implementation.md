---

feature_name: man-43-event-manager-desacople-total
created: "2026-11-11"
updated: "2026-11-11"
process: feature
branch_name: feat/man-43-event-manager-desacople-total
persist_ref: docs/features/man-43-event-manager-desacople-total
pbi_ref: MAN-43
parent_ref: MAN-40
document_id: PBI-0f4f0f5f
agent: tekton
phase: ejecucion
status: remediation-ready
seed: continue-tekton
norm_ref: features-documentation-pattern@1.3.1
execution_id: "3654517b-c915-4e35-a965-3c7550a7b706"
rebaseline: remediation-waves-v1
audit_ref: docs/audits/man-32-lineage-man-43-remediation-20261111.md
---
# Implementation — man-43-event-manager-desacople-total

Tekton Ejecución (`continue tekton`). Blueprint Dedalo v1.0.1 en solo lectura.

## H0 — Precondiciones de forja

### H0.1 Inventario consumidores v1 (cerrado)

| Área | Touchpoints residuales |
|------|------------------------|
| Motor | `eda_port`, `eda_bus_topology`, `fractal_bus`, `actions` (espejo FS F-1), `route_*`, handlers con `event_file_path` |
| Daemons | `event-watcher` (L-P1/P2), `eda_sweep` (L-P3/F-5) |
| Bridges | Kalma2 fallback FS (F-2), `github_bridge` DL FS (F-4), `linear_bridge` tokens |
| Tools | `event-bus-audit`, `repo-retention-purger`, `sddia-qa` gates/allowlists, workspace-server telemetry (F-3) |
| Lab | `eda_port_legacy`, crate `sddia-event-store`, matrices `eda_store_io_*` / `eda_subscriber_*` |

Excepciones permanentes: `SddIA/events/`, `.SddIA/events/`.

### H0.2 Ops v2 checklist

Cubiertas en contrato/código H1:

- `list_deliveries` real en `InMemoryEventManagerMock` (Redb ya tenía implementación).
- `list_terminal_events` añadida al trait (barrido terminales / sweeper).
- `EventUri` / `EventFamily` / `EventQueueState` tipados.
- `PublishSeal` v2 solo `event_uri` (AC-3 en sello).

Pendiente endurecer filtros familia/estado en adapters Redb/FS y schema Genoma vía entity-manager (H1.5).

### H0.3 Política residuos `.events/`

Detectar → migrar ECST vivos al puerto → `purge_event` / higiene ops. Nunca SSOT. Ejecución real en H2–H3/H6.

### H0.4 Orden cutover

Validado contra `spec.md` §8: H1 → H2 → H3 atómico → H4 → H5 → H6. Sin watcher huérfano.

### H0.5 Estrategia tests

| AC | Comando / evidencia |
|----|---------------------|
| AC-1..3 | `cargo test -p sddia-core event_manager` + contract suite mock |
| AC-4..9 | H2–H4 + gates |
| AC-10..12 | H5 workspace/clippy/gates |
| AC-13..14 | H6 auditorías/evolution |

**Bloqueo verificación:** `.SddIA/project.md` sin `mcp.allowed_executables` → `run_check`/`cargo` denegados vía Cerbero MCP en esta sesión.

### H0.6 forge_key

**Ausente** (`tracker_ref: MAN-43` no en `.SddIA/state/forge-pbi/`). Materialización legítima = proceso `forge-pbi` (no inventar ledger). Gate documentado; código H1 forjado bajo semilla runtime `continue tekton` + PBI ya refinada **PBI-0f4f0f5f**.

---

## H1 — Contrato EventManager 2.0 + EventUri (parcial materializado)

| Entregable | Estado |
|-------------|--------|
| H1.1 `EventFamily` + `EventUri` | **hecho** — `contract.rs` |
| H1.2 `PublishSeal` v2 | **hecho** — solo `event_id` + `event_uri`; callers motor/tools/host actualizados |
| H1.3 Ops | **parcial** — `list_deliveries` mock; `list_terminal_events` trait; filtros avanzados pendientes adapters |
| H1.4 Constantes 2.0.0 / `^2.0.0` | **hecho** |
| H1.5 Schema/bindings/taxonomía Genoma | **pendiente** — requiere `entity-manager` (L-GENOME); no mutado en esta pasada |
| H1.6 version_gate / host 2.0.0 | **hecho** — tests gate + `sddia-event-manager-host` binding |
| H1.7 Contract suite | **pendiente verificación** (Cerbero sin cargo) |
| H1.8 Adapters host | **parcial** — handshake usa `CONTRACT_VERSION`; ops terminales default en trait |

### Archivos tocados (H1)

- `SddIA/sddia-core/src/ports/event_manager/{contract,runtime_bridge,port,memory_mock,version_gate,mod}.rs`
- `SddIA/sddia-event-manager-host/src/lib.rs`
- `SddIA/engine/execute-process/src/engine/{actions,domain_mutation,event_store_query,eda_port_legacy}.rs`
- `SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs`
- `SddIA/tools/event-bus-audit/src/main.rs`
- `SddIA/tools/sddia-qa/src/iota_ci_smoke.rs`

### Fuera de esta pasada (siguen plan)

H2 entregas/testigos FS · H3 watcher/sweeper · H4 periféricos F-1..F-4 / AC-8 · H5 demolición store · H6 docs/auditorías APTO.

## Notas honestas

- Espejo FS en `emit_pr_presented` eliminado (O4).
- Crate `sddia-event-store` demolido en `877c1791` (O5.2). O6 documental en rama; **AC-13 abierto** (`NO_APTO_TOTAL` en `docs/audits/man-43-o6-remediation-20261113.md`).
- Genoma `SddIA/process/` (event-bus-audit, route-domain-event) y codex library: citas legacy FS — solo inventario; refinar vía `entity-manager`.
- No se declara AC-1…AC-19 verdes sin evidencia de tests/gates.

---

## Rebaseline post-auditoría — O0…O6

Auditoría canónica: `docs/audits/man-32-lineage-man-43-remediation-20261111.md`.

El merge #453 materializó H1 parcial. La siguiente implementación **no** comienza en H2: comienza en **O0/H1R**, porque el adapter Redb vigente selecciona entregas/trazas mediante coincidencias textuales y puede cruzar IDs opacos solapados.

| Ola | Estado | Touchpoints principales |
|-----|--------|-------------------------|
| **O0 / H1R** | `done` (rama; sin merge) | `DeliveryRecord`, `AlreadyExists`, purga/listado exactos (Redb/FS/mock), suite `o0_containment_integrity`, cápsula dummy + puente; `cargo test` verde en los cuatro crates del plan § O0 |
| **O1 / H1C** | `done` (rama; sin merge) | `event_uri` en publish, listados reales, `em_processed` / FS `processed`, suite `o1_listing_integrity`; Genoma 2.0 **pendiente** (`entity-manager`) |
| **O2 / H2** | `done` (rama; sin merge) | Testigos vía entregas del puerto; `DeliveryStatus::Processing`; sin JSON de suscriptor |
| **O3 / H3** | `done` (rama) | Dominio + fractal watcher/sweep por puerto; progress leaf aún indexa dirs |
| **O3.1** | `done` (rama; sin merge) | `sweep_fractal_family`, `list_witnesses` puerto, watcher `list_events`, DL por Nack |
| **O4 / H4** | `done` (rama) | F-1…F-4; `actions` test helper sin `EventStore::open`; Kalma2 sin fallback `.events/` en dominio/telemetría/progress |
| **O5** | `done` (rama) | `scan_presented_for_branch`, `presented_ids_in_store`, `sweep_progress_leaf` sin disco |
| **O5.1** | `done` (rama) | `eda_port` solo `EventManagerPort`; borrado `eda_port_legacy`; `test_lab` ampliado; ~44 unitarios migrados de lectura `.events/` |
| **O5.2 / H5** | `done` (rama) | Crate `sddia-event-store` demolido; matrices/harness puerto; gates v2; `runtime_db_path` retirado de `eda_port` |
| **O6 / H6** | `done (rama)` | README/Constitución convergentes; evolution `c4e8f2a1-…`; audit O6; AC-13 abierto |
| **O7** | `done (rama)` | `event-bus-audit` sin default `/.events/`; AC-13 abierto |
| **O8** | `done (rama)` | Bindings 2.0.0; procesos forjados; F-02 schema abierto |
| **O8.1** | `done (rama)` | `route-domain-event` v1.0.2 sin `fs.persist` en fases; schema v1 fuera |
| **O9** | `done (rama)` | Gate `rustc` verde; clippy residual acotado a `execute-process` (excluido por norma); C-2 latente |
| **O10** | `done (rama)` | `contract_schema` en resolver/gate/validador; C-2 cerrado; C-4 taxonomía 1.0.12 / eda 2.0.0; O9 tools/skills/daemons sellados vía entity-manager |
| **O12** | `done` | Línea base; C-4 por cauce oficial; C-5 `event_uri` + process 1.1.0; 4 fallos lib documentados |
| **O11** | `done` | Remediación forja O10: binario canónico, guarda anti-sobrescritura, restauración 3 `.md`, delete stubs, sellos daemons/O9, `hash_refresh` taxonomía (contenido 2.0.0 heredado de O10) |

### Backlog técnico O0

Implementado según `plan.md` § O0 / H1R (2026-11-11). Evidencia: `sddia-core` (`error`, `contract`, `memory_mock`, `runtime_bridge`, `contract_suite`), `redb_event_adapter`, `fs_event_adapter`, `external_capsule_event_adapter` (+ `capsule_dummy`). Pendiente: PR/commit y cierre documental en rama.

### Backlog técnico O1

Kernel y adapters en `b6a73ba2`. Genoma 2.0 sigue pendiente vía `entity-manager` y no bloquea O2.

### Backlog técnico O2

Cerrado en `de2161ed`. Secuencia en `plan.md` § O2 / H2.

### Backlog técnico O3

O3.1 en `1a4c84af`. Residual de progress (`sweep_progress_leaf`) fuera de O4.

### Backlog técnico O4

Cerrado en `c1700a47`. Residual de lectores (Presented por disco, progress leaf) = hilo **O5** en `plan.md` v1.1.7. La demolición del crate queda en **O5.1** (L-7).

### Backlog técnico O5

Cerrado en rama: Presented y `presented_event_id_for_branch` solo puerto; `sweep_progress_leaf` no-op sin `read_dir`. Siguiente: **O5.1**.

### Backlog técnico O5.1

Cerrado en `0d37e9d6`. Verificación: `cargo test -p execute-process --lib` (816), matrices IO, `gate_event_manager_port`.

### Backlog técnico O5.2

Cerrado en rama: `tests/common` + matrices pending/fractal + `event_store_writers` vía puerto; eliminados `eda_subscriber_*`; `event-bus-audit` sin dep store; `eda_port` sin `runtime_db_path`/`path_is_resolved_redb`; workspace sin `sddia-event-store/`; allowlists QA endurecidas. Verificación: `cargo build/test -p execute-process`, `gate_event_manager_port`, `event-bus-audit`.

### Backlog técnico O6

Cerrado en `f9ef35d9`. F-02 y prosa de `SddIA/process/` → **O8** vía `entity-manager`. AC-13 sigue abierto.

### Backlog técnico O7

Cerrado en rama: `cumulo_string_is_legacy_fs_dir` + `optional_queue_dir`; sin `unwrap_or(".events/…")`; tests `cumulo_table_ids_*` y `run_audit_with_core_cumulo_*`. Verificación: `cargo test -p event-bus-audit`, `gate-event-manager-port`.

### Backlog técnico O8

Cerrado en rama: `capability-bindings` v1.10.0 / contrato `2.0.0`; README/Constitución `^2.0.0`; `event-bus-audit` y `route-domain-event` actualizados vía `entity-manager`. Re-audit `man-43-o8-contract-20261113.md`. Residual de cabecera → **O8.1**. Schema v1 → ciclo `norm` posterior. AC-13 abierto.

### Backlog técnico O8.1

Cerrado en rama: `entity-manager` reescribió `phases` e `inputs` de `route-domain-event` a v1.0.2 (token de puerto, sin `fs.persist`). `context` restaurado (`event-routing`, `ecosystem-evolution`). `outputs` de cabecera y `eda.event_manager.v1.schema.json` siguen fuera. AC-13 abierto.

### Backlog técnico O9

Cerrado en rama. **Hallazgo normativo:** `rust-warning-gate` §1 excluye Clippy; la CI usa `RUSTFLAGS=-Dwarnings` sin clippy, así que AC-12 se mide con `cargo check/build`. Partiendo de un workspace **inmedible** (`sddia-io` fallaba), se corrigieron 33 ficheros de lint mecánico. `cargo check --workspace` verde; `execute-process --lib` 816/816. Residual clippy (~85) acotado a `execute-process` y de tipo estructural (`too_many_arguments`, `needless_return` en `match`, `type_complexity`). Fallos preexistentes confirmados con `git stash`: `linear_bridge_webhook::ac13_*` y `ecosystem_health_map_absent_still_ok`. C-2 (`eda.event_manager.schema.json` inalcanzable) sigue latente: ningún proceso declara `requires_capability` de ese contrato. AC-13 abierto.

### Backlog técnico O10

Cerrado en rama: `BindingRow` + `contract_schema_file`; validador usa `contract_schema_rel`; gate lee fila de bindings; forja norm/tool/skill/daemon ampliada (`artifact_yaml/body`, `hash_refresh`, `implementation_source_git_apply`); `capability-taxonomy` **1.0.12** / catálogo **2.0.0**; O9 genoma sellado vía entity-manager. Verificación: test `core_bindings_eda_event_manager_resolves_v1_schema` (tras `cargo test -p execute-process --lib`). C-3 (schema v1.2.0) y C-5 (`route-domain-event.outputs`) siguen abiertos; AC-13 abierto.

### Backlog técnico O12 (cerrado en rama)

Ejecutado (`plan.md` § O12). Auditoría: `docs/audits/man-43-o12-baseline-20261114.md`. Línea base completa (suite lib, `cargo check --workspace`, `gate-event-manager-port`; no corridas desde O9). C-4 por cauce oficial en dos `entity-manager` (restore desde `c60ae233` + parche); en O11 el paso 1 nunca se ejecutó porque su semilla no existía, no por límite del motor. C-5: `route-domain-event` 1.1.0 con `event_uri`; el runtime deja de fabricar `processing_header_path` en modo puerto; `run_process_forge` update admite `process_outputs` sin fases. Higiene de 5 semillas `_lab` de O11. C-3 pasa a O13 (laudo: nueva clase `capability-contract` en `entity-manager`; sin edit IDE). C-5 decidido: `event_uri`.

### Backlog técnico O11 (cerrado en rama)

Ejecutado (`plan.md` § O11). **Corrige O10:** `SddIA/target/debug/execute-process` era del 2026-11-13 12:14; las invocaciones `entity-manager` de O10 usaron la forja anterior, que en `update` sin parche sobrescribe con `create`. Daño en `419d89d5`: `shell-executor.md`, `cryptography-manager.md`, `event-bus-audit.md` con esqueleto y UUID nuevo; `file-tracker-adapter.md` y `manage-event-receipt.md` creados; `capability-taxonomy.md` escrita con `StrReplace` (incumple DA-2 y el laudo C-4); daemons sin sellar; el «re-gobierno O9» solo refrescó hashes. Pasos: build canónico → guarda anti-sobrescritura → restaurar desde `c60ae233` vía `implementation_source_git_apply` → delete de fantasmas → C-4 en dos `update` → sellar daemons → 18 reaplicaciones O9 → inventario de eventos con UUID falso → docs.

No se modifica el veredicto histórico de `validacion.md`: su `APTO` corresponde al milestone H1. El feature completo sigue `NO_APTO_TOTAL`.

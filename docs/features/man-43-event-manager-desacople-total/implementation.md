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

- Espejo FS en `emit_pr_presented` (`.events/domain/...`) **aún presente** → F-1 / H4.
- `runtime_db_path` / `path_is_resolved_redb` aún en superficie consumidores → AC-8 / H4.
- No se declara AC-1…AC-19 verdes sin evidencia de tests/gates.

---

## Rebaseline post-auditoría — O0…O6

Auditoría canónica: `docs/audits/man-32-lineage-man-43-remediation-20261111.md`.

El merge #453 materializó H1 parcial. La siguiente implementación **no** comienza en H2: comienza en **O0/H1R**, porque el adapter Redb vigente selecciona entregas/trazas mediante coincidencias textuales y puede cruzar IDs opacos solapados.

| Ola | Estado | Touchpoints principales |
|-----|--------|-------------------------|
| **O0 / H1R** | `done` (rama; sin merge) | `DeliveryRecord`, `AlreadyExists`, purga/listado exactos (Redb/FS/mock), suite `o0_containment_integrity`, cápsula dummy + puente; `cargo test` verde en los cuatro crates del plan § O0 |
| **O1 / H1C** | `done` (rama; sin merge) | `event_uri` en publish, listados reales, `em_processed` / FS `processed`, suite `o1_listing_integrity`; Genoma 2.0 **pendiente** (`entity-manager`) |
| **O2 / H2** | `ready` | `eda_bus_topology`, `fractal_bus`, entregas/testigos |
| **O3 / H3** | `blocked-by-O2` | event-watcher, `eda_sweep` |
| **O4 / H4** | `blocked-by-O1` | actions, Kalma2, workspace-server, GitHub bridge, superficie `eda_port` |
| **O5 / H5** | `blocked-by-O2-O4` | store legacy, matrices, allowlists, gates |
| **O6 / H6** | `blocked-by-O5` | README, Constitución, Cúmulo, evolution, auditorías |

### Backlog técnico O0

Implementado según `plan.md` § O0 / H1R (2026-11-11). Evidencia: `sddia-core` (`error`, `contract`, `memory_mock`, `runtime_bridge`, `contract_suite`), `redb_event_adapter`, `fs_event_adapter`, `external_capsule_event_adapter` (+ `capsule_dummy`). Pendiente: PR/commit y cierre documental en rama.

### Backlog técnico O1

Implementado en kernel y adapters (2026-11-11): `event_query`, `PublishEcstRequest.event_uri`, `list_events`/`list_terminal_events` obligatorios, persistencia processed, paginación cursor. Pendiente cierre Genoma: schema/bindings/taxonomía **2.0.0** vía `./sddia-run.sh --process entity-manager` (no edit manual).

No se modifica el veredicto histórico de `validacion.md`: su `APTO` corresponde al milestone H1. El feature completo sigue `NO_APTO_TOTAL`.

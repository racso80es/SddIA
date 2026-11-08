---
feature_name: event-manager-port-cutover
created: "2026-11-08"
updated: "2026-11-08"
process: refactorization
branch_name: refactor/event-manager-port-cutover
persist_ref: docs/features/event-manager-port-cutover
pbi_ref: MAN-35
parent_ref: MAN-33
document_id: PBI-176f9f2f
related_adr: ADR-EDA-002
execution_mode: BREAKING_CUTOVER
agent_planificador: dedalo
target_executor: tekton
phase: diseno-blueprint
status: blueprint-ready
phases:
  - T0-norma-redb-exclusiva
  - T1-contrato-event-manager-1-2
  - T2-composition-root
  - T3-loader-route-domain-estricto
  - T4-fractal-daemons
  - T5-productores
  - T6-audit-purger
  - T7-retirada-legacy
  - T8-pruebas-gate
rbac_ok: true
scope: "Cutover EventManagerPort 1.2 — Genoma + composition + motor + productores + aduana"
norm_ref: features-documentation-pattern@1.3.1
execution_id: "8b39f170-38b9-4aea-961b-31c4c63512b7"
tracker_channel: ".SddIA/state/tracker-outbound/MAN-35.jsonl"
---
# Plan / Blueprint — event-manager-port-cutover

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md` (L-*), `spec.md` v1.1.0 (ids **MAN-35** / **PBI-176f9f2f**).

`target_executor_rbac.allowed_policies` esperadas: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `quality-assurance`.

## Viabilidad RBAC

| Cápsula | ¿Permitida? |
|---------|-------------|
| `skill:filesystem-manager` | sí |
| `skill:shell-executor` | sí (`cargo test` / clippy / gate script) |
| `skill:git-manager` | sí — **única** vía Git (**L-GIT**) |
| `action:execute-process` → entity-manager / evolution | sí para mutaciones Genoma (**L-GENOME**) |

Si falta `quality-assurance`, T8 degrada a verificación documentada vía `shell-executor` solo si el skill lo permite; si no, **abortar fase** — no improvisar.

**Prohibido:** TODOs bajo `docs/todos/` (Cumulo / `Kaizen_Alert_Required` únicamente).

---

## Fases declarativas

```yaml
phases:
  - name: "T0 Norma Redb exclusiva"
    intent: "Convergencia normativa: Redb único runtime; prohibir filesystem/dual; separar ECST de proofs/config/estado local; alinear ids MAN-35."
    delegates_to: ["skill:filesystem-manager", "action:execute-process"]
  - name: "T1 Contrato EventManager 1.2"
    intent: "Extender schema/puerto/mock/adapters/host/suite con list_events, list_deliveries, DLT, admin y verify_inline."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor", "action:execute-process"]
  - name: "T2 Composition root único"
    intent: "Factory/host único; consumidores reciben Arc<dyn EventManagerPort>; solo adapter conoce físico."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T3 Loader y route_domain_core estrictos"
    intent: "Carga por event_id; verify inline; sin precedencia/mutaciones/barridos filesystem ECST."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T4 Bus fractal y daemons"
    intent: "route_fractal, fractal_bus, heartbeat, memory evolution, radamanto, compliance, event-watcher → puerto."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T5 Productores"
    intent: "kalma2-bridge, lancedb_thought_repo, instance_creator, Cerbero, output validator, github bridge sin buses/DLQ paralelos."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T6 Auditoría y purger"
    intent: "event-bus-audit solo contrato; purger sin EventStore::open; purge vía op administrativa."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T7 Retirada legacy"
    intent: "Eliminar usos productivos sddia-event-store, StoreMode, SDDIA_EVENT_STORE, fs_event_adapter prod; docs AC-9."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T8 Pruebas y gate"
    intent: "Suite contractual 1.2; cargo test --workspace; gate CI arquitectónico; implementation/execution."
    delegates_to: ["skill:shell-executor", "skill:filesystem-manager", "skill:git-manager"]
```

---

## T0 — Norma Redb exclusiva

| # | Entregable | Detalle |
|---|------------|---------|
| T0.1 | Norma / docs runtime | Declarar `.SddIA/runtime/events.redb` SSOT; prohibir modos filesystem/dual en runtime |
| T0.2 | Delimitación ECST vs no-evento | Documentar rutas SSOT de proofs/config/estado local (**L-NON-EVENT-FS**) |
| T0.3 | `cumulo.paths` / Constitución (si aplica) | Apuntar runtime canónico; sin dual |
| T0.4 | Alinear `norma-redb-exclusiva.md` | Frontmatter → `pbi_ref: MAN-35`, `document_id: PBI-176f9f2f` (corrige ids lab MAN-101) |

**Salida:** base normativa para T1–T7. **Commit** (`git-manager`): `docs(eda): fijar Redb exclusivo como runtime de eventos`.

---

## T1 — Contrato EventManager 1.2

| # | Entregable | Detalle |
|---|------------|---------|
| T1.1 | Schema `eda.event_manager.v1.schema.json` | Añadir ops §3.1 spec; description 1.2.0 |
| T1.2 | Taxonomía | Catalog `1.2.0`; bump + hash + `norms/index.md` vía entity-manager |
| T1.3 | Bindings `1.9.0` | `contract_version: 1.2.0`; sin fallback FS productivo |
| T1.4 | `sddia-core` ports | Trait + tipos + `CONTRACT_VERSION`/`REQUIRED_CONTRACT_RANGE=^1.2.0` + mock |
| T1.5 | Adapters | Redb + External (+ mock) implementan ops nuevas; path Redb canónico |
| T1.6 | Suite parcial | Casos nuevos 1.2 en verde sobre mock + Redb tempdir |

**Commit:** `feat(eda): extender eda.event_manager a 1.2.0 (list/query/DLT/admin)`.

---

## T2 — Composition root

| # | Entregable | Detalle |
|---|------------|---------|
| T2.1 | Módulo host canónico | Factory + builders Redb/mock/external; path `.SddIA/runtime/events.redb` |
| T2.2 | Cableado `execute-process` | Inyectar `Arc<dyn EventManagerPort>` al arranque |
| T2.3 | Cableado `event-watcher` / hosts | Misma factory; cero open store en `main` |
| T2.4 | Tests DI | Rol `test` → mock; rechazo MAJOR / <1.2 |

**Salida:** **L-COMPOSE** cumplida. **Commit:** `feat(eda): composition root único EventManagerPort`.

---

## T3 — Loader + `route_domain_core`

| # | Entregable | Detalle |
|---|------------|---------|
| T3.1 | `ecst_input_loader.rs` | `get_event` / verify; sin FS ECST |
| T3.2 | `route_domain_core.rs` | Inline verify antes de efectos; delivery vía puerto; sin mutar ECST |
| T3.3 | `event_store_query.rs` / `eda_bus.rs` | Consultas → `list_*` / `get_event` |
| T3.4 | Tests | Ausente/divergente → rechazo (**AC-4**); sin crear `.events` (**AC-5**) |

**Commit:** `refactor(eda): route_domain_core estricto vía EventManagerPort`.

---

## T4 — Fractal + daemons

| # | Entregable | Detalle |
|---|------------|---------|
| T4.1 | `route_fractal_core` + `fractal_bus` + `fractal` | ACK/NACK/DLQ/DLT puerto |
| T4.2 | heartbeat / memory evolution / radamanto / compliance | Sin EventStore directo |
| T4.3 | `dlt_telemetry_anchor` | `seal_dlt` / `get_dlt_seal` |
| T4.4 | `event-watcher` | claim/record_delivery puerto |

**Commit:** `refactor(eda): fractal y daemons sobre EventManagerPort`.

---

## T5 — Productores

| # | Entregable | Detalle |
|---|------------|---------|
| T5.1 | kalma2-bridge | Publish/consulta puerto; eliminar DLQ paralelo |
| T5.2 | `lancedb_thought_repo` | Sin store directo |
| T5.3 | instance_creator + Cerbero + output validator | DI puerto |
| T5.4 | github-bridge-watcher | Publish vía puerto |

**Commit:** `refactor(eda): productores al puerto EventManager 1.2`.

---

## T6 — Audit + purger

| # | Entregable | Detalle |
|---|------------|---------|
| T6.1 | `event-bus-audit` | Solo ops contractuales |
| T6.2 | `repo-retention-purger` / `purge_repo_retention` | `purge_event`; **cero** `EventStore::open` |

**Commit:** `refactor(eda): audit y purge administrativos vía puerto`.

---

## T7 — Retirada legacy

| # | Entregable | Detalle |
|---|------------|---------|
| T7.1 | Grep/allowlist | Eliminar imports productivos `sddia_event_store` fuera adapter |
| T7.2 | StoreMode / env | Retirar filesystem/dual/`SDDIA_EVENT_STORE` de consumidores |
| T7.3 | `fs_event_adapter` | Fuera de bindings productivos |
| T7.4 | Docs AC-9 | README, Constitución, cumulo.paths, evolution |

**Commit:** `chore(eda): retirar sddia-event-store y modos legacy del plano productivo`.

---

## T8 — Pruebas y gate

| # | Hecho | Verificación |
|---|--------|----------------|
| T8.1 | Contract suite 1.2 | Mock + Redb + external (**AC-7**) |
| T8.2 | `cargo test --workspace` | Verde (**AC-8**) |
| T8.3 | Gate arquitectónico | Veto patterns **L-GATE**; CI `run_id` o `PENDIENTE-CI` |
| T8.4 | Docs forja | `implementation.md` + `execution.md` |
| T8.5 | Evolution / EDA | Normas mutadas selladas |

**Commit(s):** cierre verificación; Git solo `skill:git-manager`.

---

## Orden de ejecución Tekton

1. T0 (norma) → T1 (contrato/adapters) → T2 (composition).
2. T3 (motor) → T4 (fractal/daemons) → T5 (productores) → T6 (audit/purger).
3. T7 (retirada) → T8 (suite + gate + docs).

No adelantar T7 antes de migrar consumidores (rompe el build). No declarar AC-8/AC-10 sin evidencia de tests/gate.

## Criterios de salida del blueprint

- AC-1…AC-10 según `spec.md` / `objectives.md`.
- Laudos L-DOMAIN, L-RUNTIME, L-LOGICAL-PATH, L-ECST, L-OPS-PORT, L-CONTRACT-12, L-COMPOSE, L-STRICT-CORE, L-FRACTAL, L-PRODUCERS, L-AUDIT-ADMIN, L-RETIRE, L-VERIFY, L-GATE, L-GIT, L-GENOME, L-TRACKER respetados.
- Sin dual-mode; sin Kaizen TODOs desde Tekton.
- Tracker canónico de este ciclo: **MAN-35** / documento **PBI-176f9f2f** / padre **MAN-33**.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| Romper camino caliente al retirar FS | Orden T2→T3→…→T7; suite en cada tajo |
| Contaminar consumidores con Redb path | **L-COMPOSE**; gate CI T8 |
| Cápsulas 1.0.x incompatibles con `^1.2.0` | Actualizar adapters en T1; handshake falla hasta bump (esperado) |
| Purger/audit reabren store | Allowlist estricta + veto `EventStore::open` |
| Residuos `.events` JSON | No dual; limpieza admin / fractura (**L-LOGICAL-PATH**) |
| Norma Genoma huérfana EDA | T1/T8 vía entity-manager + cobertura |
| Ids lab MAN-101 residuales en persist_ref | T0.4 + frontmatter Dedalo ya corregidos en spec/plan |

---

feature_name: eda-event-manager-redb-cutover
created: "2026-11-07"
updated: "2026-11-07"
process: refactorization
branch_name: refactor/eda-event-manager-redb-cutover
persist_ref: docs/features/eda-event-manager-redb-cutover
pbi_ref: MAN-33
tracker_ref: MAN-33
historia_ref: MAN-32
document_id: PBI-e4403517
execution_mode: BREAKING_CUTOVER
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version_target: "1.1.0"
productive_provider: "adapter:redb-event-adapter"
productive_role: default
agent_planificador: dedalo
target_executor: tekton
phase: diseno-blueprint
status: blueprint-ready
phases:
  - T0-contrato-1-1
  - T1-di-composition-roots
  - T2-migrate-publishers
  - T3-migrate-watcher-daemon
  - T4-migrate-tools-catalog
  - T5-retire-store-verify
rbac_ok: true
scope: "Cutover breaking EventManager default=Redb — Genoma 1.1.0 + DI + inventario + retiro sddia-event-store"
norm_ref: features-documentation-pattern@1.3.1
execution_id: "c7d9f208-857a-411a-b5b0-5333170ecf67"
---
# Plan / Blueprint — eda-event-manager-redb-cutover

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md` (L-*), `spec.md` v1.0.0.

`target_executor_rbac.allowed_policies` esperadas: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `quality-assurance`.

## Viabilidad RBAC

| Cápsula | ¿Permitida? |
|---------|-------------|
| `skill:filesystem-manager` | sí |
| `skill:shell-executor` | sí (cargo test / clippy / builds) |
| `skill:git-manager` | sí — **única** vía Git (**L-GIT**) |
| evolution / entity-manager | sí si el runtime lo inyecta; Genoma vía Write atómico + sello EDA en T0/T5 |

Materialización piloto solo vía Workspace Server MCP (aduana 1×N). Prohibido improvisar dual-path si una fase falla: **abortar y escalar**.

---

## Fases declarativas

```yaml
phases:
  - name: "T0 Contrato 1.1.0 y adapters"
    intent: "Cerrar L-GAP: ops get_event/list_pending/list_dlq/list_traces/purge_event; bump Genoma; puerto+mock+suite+3 adapters; version_gate ^1.1.0."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T1 DI composition roots"
    intent: "Cablear EventManagerDiFactory+Redb builder en binarios host; helper open path Cúmulo; sin imports adapter fuera de roots."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T2 Migrar publishers"
    intent: "execute-process, kalma2-bridge, sddia-daemon-runtime, lancedb_thought_repo → solo EventManagerPort.publish_ecst/append_trace."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T3 Migrar watcher y daemon drain"
    intent: "event-watcher + daemon sweep a claim_pending/record_delivery/move_to_dlq/replay_dlq (L-MODEL)."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T4 Tools y catálogo"
    intent: "event-bus-audit, sddia-qa, repo-retention-purger, sddia-eda-catalog sin EventStore; ops 1.1.0."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T5 Retiro store + verificación"
    intent: "Quitar sddia-event-store del workspace; residual search; builds sandbox/debug/release; docs implementation/execution; Genoma final."
    delegates_to: ["skill:shell-executor", "skill:filesystem-manager", "skill:git-manager"]
```

---

## T0 — Contrato **1.1.0** y adapters (**L-GAP** primero)

| # | Entregable | Detalle |
|---|------------|---------|
| T0.1 | Schema `eda.event_manager` | Enum + payloads ops nuevas §3.2 spec; descripción v1.1.0 |
| T0.2 | Taxonomía + bindings | `contract_version` / catalog version → `1.1.0`; hash taxonomía; bump doc bindings |
| T0.3 | Puerto Rust | Métodos + tipos; `REQUIRED_CONTRACT_RANGE = ^1.1.0` |
| T0.4 | Mock + contract suite | Casos lectura/purge + lifecycle 1.0 |
| T0.5 | Redb / Fs / External adapters | Implementar 1.1.0; manifiesto `implements_contract: 1.1.0`; Redb open-budget provider-neutral |
| T0.6 | Tests | `cargo test` crates tocados en verde |

**Salida:** contrato listo **antes** de migrar hosts. **Prohibido** empezar T2–T4 con gaps abiertos.

**Commit** (`skill:git-manager`): `feat(eda): evolucionar eda.event_manager a 1.1.0 para cutover`.

---

## T1 — Composition roots DI (**L-DI**)

| # | Entregable | Detalle |
|---|------------|---------|
| T1.1 | Roots binarios | Registrar builder Redb + rol `default` + `resolve` en execute-process, event-watcher, kalma2-bridge, event-bus-audit, sddia-qa, repo-retention-purger |
| T1.2 | Libs inyectables | `sddia-daemon-runtime`, `sddia-eda-catalog`, `lancedb_thought_repo` reciben `Arc<dyn EventManagerPort>` (sin abrir DB) |
| T1.3 | Path Cúmulo | Resolver `.SddIA/runtime/events.redb` sin gate `SDDIA_EVENT_STORE` |
| T1.4 | Test DI | default→Redb; rol desconocido→error; grep/arch-test: no `RedbEventAdapter` en módulos no-root |

**Commit:** `feat(eda): cablear EventManagerDiFactory default=Redb en hosts`.

---

## T2 — Publishers (**L-MODEL** write-path)

| # | Entregable | Detalle |
|---|------------|---------|
| T2.1 | `execute-process` | Sustituir `EventStore`/`publish_ecst`/`record_ecst`/`write_pending_event` FS por puerto; eliminar ramas dual/FS |
| T2.2 | `kalma2-bridge` | `open_runtime_store` → puerto; traces vía `append_trace` |
| T2.3 | `sddia-daemon-runtime` | Bridges usan puerto inyectado |
| T2.4 | `lancedb_thought_repo` | Emit pensamiento vía `publish_ecst` del puerto |

**Commit:** `refactor(eda): migrar publishers al EventManagerPort`.

---

## T3 — Watcher / drain (**L-MODEL** lifecycle)

| # | Entregable | Detalle |
|---|------------|---------|
| T3.1 | `event-watcher` | Loop: `claim_pending` → route → `record_delivery` ack/nack / `move_to_dlq` |
| T3.2 | Daemon sweep | `eda_sweep` / loaders sin `load_dispatched_event` del store; usar `get_event` / claim flow |
| T3.3 | Paridad | Test publish→claim→ack y publish→dlq→replay sobre Redb tempdir o runtime de lab |

**Commit:** `refactor(eda): event-watcher y daemon sobre claim/lease EventManager`.

---

## T4 — Tools y catálogo

| # | Entregable | Detalle |
|---|------------|---------|
| T4.1 | `event-bus-audit` | list/get vía ops 1.1.0; seed con `publish_ecst` |
| T4.2 | `sddia-qa` | smokes solo puerto |
| T4.3 | `repo-retention-purger` | `list_dlq` + `purge_event` |
| T4.4 | `sddia-eda-catalog` | Sin `EventStore`; deliveries vía puerto o catálogo propio |
| T4.5 | DLT seals | Rewire fuera del store (tool IOTA) o eliminar acoplamiento — **no** reabrir store |

**Commit:** `refactor(eda): tools y catálogo sin sddia-event-store`.

---

## T5 — Retiro + verificación (**L-CLEAN** / **L-VERIFY**)

| # | Hecho | Verificación |
|---|--------|----------------|
| T5.1 | Deps Cargo limpias | Cero `sddia-event-store` en miembros productivos |
| T5.2 | Borrar crate + workspace member | `SddIA/sddia-event-store` retirado |
| T5.3 | Residual search | Símbolos/env/paths legacy en verde (**AC-3**) |
| T5.4 | Tests AC-4 | DI + suite + paridad |
| T5.5 | Builds | sandbox aislado + debug + release (**AC-5**) |
| T5.6 | Clippy alcance | `-D warnings` donde el repo lo exija |
| T5.7 | Docs forja | `implementation.md` + `execution.md` |
| T5.8 | Genoma / evolution | Bindings/taxonomy/docs describen **único** camino final; EDA coverage si mutó norma |
| T5.9 | Datos | Documentar en execution: drain previo **o** cold-cut explícito (**L-SCHEMA**) |

**Commit(s):** `refactor(eda): retirar sddia-event-store y sellar cutover MAN-33`.

**Prohibido:** TODOs bajo `docs/todos/` / semillas Kaizen (Cumulo / `Kaizen_Alert_Required`).

---

## Orden de ejecución Tekton

1. **T0** (contrato — bloquea el resto si falla).
2. **T1** (DI roots — puede compilar con stubs temporales **solo** si no reintroducen store; preferir T1+T2 acoplados por crate).
3. **T2 → T3 → T4** (inventario; serializar commits por superficie).
4. **T5** (retiro + sello).

Quiesce/drain de runtime real: coordinación operativa **antes** del merge (spec §4.2); Tekton documenta el modo elegido en `execution.md`.

## Criterios de salida del blueprint

- AC-1…AC-6 de `spec.md` / `objectives.md`.
- Laudos L-MISSION, L-DI, L-INPROC, L-SINGLE, L-GAP, L-CLEAN, L-CONFIG, L-VERIFY, L-GENOMA, L-GIT + L-MODEL/L-SCHEMA del spec.
- Inventario 9/9 cerrado; sin `SDDIA_EVENT_STORE`; sin shims.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| L-GAP incompleto a mitad de migración | T0 obligatorio primero; no mezclar store «temporal» |
| Esquema `events*` vs `em_*` en mismo fichero | Cold-cut o migrator one-shot borrado en T5; nunca dual-read |
| Semántica subscriber_delivery ≠ lease | **L-MODEL** — reescribir watcher, no emular matriz |
| Contaminar consumers con tipo concreto Redb | Arch/test + code review roots-only |
| Lock `DatabaseAlreadyOpen` multi-proceso | Open-budget en adapter/root; un writer disciplinado |
| DLT seals huérfanos | Rewire tool IOTA en T4.5 |
| MAN-33 cuerpo Linear ausente | No bloquea; cierre tracker en delivery |

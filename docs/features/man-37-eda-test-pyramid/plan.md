---
feature_name: man-37-eda-test-pyramid
created: "2026-11-09"
updated: "2026-11-09"
process: refactorization
branch_name: refactor/man-37-eda-test-pyramid
persist_ref: docs/features/man-37-eda-test-pyramid
pbi_ref: MAN-37
parent_ref: MAN-35
document_id: PBI-94396a84
capability_id: "eda:event-manager"
contract: "eda.event_manager"
agent_planificador: dedalo
target_executor: tekton
phase: diseno-blueprint
status: blueprint-ready
version_plan: "1.1.0"
spec_ref: "spec.md@1.1.0"
phases:
  - T0-event-manager-test-harness
  - T1-inventario-unitarios-eda
  - T2-migrar-unitarios-harness
  - T3-reclasificar-contract-integration-e2e
  - T4-gate-ci-unit-no-real-backend
  - T5-documentar-taxonomia
  - T6-verificacion-validacion
rbac_ok: true
scope: "Pirámide tests EDA — harness RAII, migración unitarios, gate anti-backend, taxonomía"
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-37.jsonl"
execution_id: "47b9678f-7ee1-4080-bd3e-12cf0d366069"
---
# Plan / Blueprint — man-37-eda-test-pyramid

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md` (L-*), `spec.md` **v1.1.0** (ids **MAN-37** / **PBI-94396a84**).

`target_executor_rbac.allowed_policies` esperadas: `ecosystem-evolution` (si hace falta), `filesystem-ops`, `source-control`, `quality-assurance`.

## Viabilidad RBAC

| Cápsula | ¿Permitida? |
|---------|-------------|
| `skill:filesystem-manager` | sí |
| `skill:shell-executor` | sí (`cargo test` / gate script) |
| `skill:git-manager` | sí — **única** vía Git (**L-GIT**) |
| `action:execute-process` → entity-manager | solo si T5 muta Genoma (preferir docs en `persist_ref` sin Genoma) |

**Prohibido:** TODOs bajo `docs/todos/` (Cumulo / `Kaizen_Alert_Required` únicamente).

---

## Fases declarativas

```yaml
phases:
  - name: "T0 EventManagerTestHarness"
    intent: "Forjar RAII que instala/restaura InMemoryEventManagerMock vía eda_port::install_port; guía mínima + tests FaultInjection canónicos."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T1 Inventario unitarios EDA"
    intent: "Tabla machine-readable de unitarios execute-process + consumidores EDA que tocan eventos/store; clasificar migrate|reclassify|allowlist."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T2 Migrar unitarios al harness"
    intent: "Migrar inventario a harness; retirar L-STRIP de unitarios; cubrir fail_write/steal_lease/duplicate_ack_noop."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T3 Reclasificar contract/integration/e2e"
    intent: "Aislar Redb/FS/host/CLI/cápsulas en capas no-unit; matrices I/O en tests/."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T4 Gate CI unitario"
    intent: "Gate sddia-qa inverso a MAN-35 (escanea #[cfg(test)]); allowlist versionada; veto backend real; cablear CI."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T5 Documentar taxonomía"
    intent: "Sección canónica unit/contract/integration/e2e en persist_ref (+ refs README si aplica; sin Genoma por defecto)."
    delegates_to: ["skill:filesystem-manager"]
  - name: "T6 Verificación"
    intent: "cargo test unitarios verdes sin backend real; contract suite; gate verde; validacion APTO."
    delegates_to: ["skill:shell-executor", "skill:filesystem-manager", "skill:git-manager"]
```

---

## T0 — EventManagerTestHarness

| # | Entregable | Detalle |
|---|------------|---------|
| T0.1 | Tipo `EventManagerTestHarness` | RAII: `new` / `with_faults` / `mock` / `port` / `set_faults` / Drop → `clear_port` (**spec §3**) |
| T0.2 | Locus | `execute-process` junto a `eda_port` (canal TLS existente; no segundo TLS) |
| T0.3 | Tests canónicos FaultInjection | Tres unitarios mínimos: `fail_write`, `steal_lease`, `duplicate_ack_noop` |
| T0.4 | Guía mínima | Comentario módulo o párrafo en `persist_ref` / doc-comment |
| T0.5 | No regresión | `install_port` ad hoc (`scoped_tests`) puede coexistir **temporalmente** hasta T2; documentar deprecación |
| T0.6 | Lab mutex | Si se adquiere `ParallelLabGuard`, **no** fijar StoreMode FS/Redb; solo serializar TLS |

**Salida:** O1 iniciado; base para T2. **Commit** (`git-manager`): `test(eda): añadir EventManagerTestHarness RAII con FaultInjection`.

---

## T1 — Inventario

| # | Entregable | Detalle |
|---|------------|---------|
| T1.1 | Escaneo | `src/**` `#[cfg(test)]` + crates consumidores EDA; patrones L-STRIP |
| T1.2 | Tabla machine-readable | Añadir a `execution.md` (o anexo en este plan al cerrar): path, símbolo, decisión `migrate` \| `reclassify` \| `allowlist` |
| T1.3 | Semilla | Partir de focos en `spec.md` §6.1; **cero** filas `pending` al cierre de T2/T3 |
| T1.4 | Alcance crates | Prioridad: `execute-process` (incl. `handlers/*`, `actions`, `eda_port`); luego interfaces/daemons/tools que usen eventos en unitarios |
| T1.5 | Matrices conocidas | `tests/eda_store_io_matrix_*`, `event_store_writers`, `eda_subscriber_*`, `memory_evolution_ingest` → candidatos `reclassify`/`allowlist` integration |

**Commit:** `docs(eda): inventario unitarios EDA MAN-37 (T1)`.

---

## T2 — Migración al harness

| # | Entregable | Detalle |
|---|------------|---------|
| T2.1 | Reemplazo | Cada fila `migrate` → `EventManagerTestHarness` |
| T2.2 | Strip | Retirar `sddia_event_store`, `EventStore::open`, `StoreMode`, `override_store_mode`, inspección `.events/*` |
| T2.3 | Asserts | Contra mock/puerto; faults deterministas (**AC-2**) |
| T2.4 | Deprecar wiring ad hoc | Preferir harness sobre `install_port` suelto en unitarios |
| T2.5 | Bridges `#[cfg(test)]` | Unitarios no deben empujar StoreMode que active `eda_port_legacy` |

**Commit(s):** `refactor(eda): migrar unitarios EDA a EventManagerTestHarness`.

---

## T3 — Reclasificación de capas

| # | Entregable | Detalle |
|---|------------|---------|
| T3.1 | Matrices I/O | Mantener en `execute-process/tests/**` como **integration** |
| T3.2 | `test_lab::{filesystem_eda,redb_eda}` | Solo helpers integration; no unitarios |
| T3.3 | Contract suite | Intacta; peers en allowlist (**O7**) |
| T3.4 | E2E | CLI/cápsulas/`sddia-qa` labs fuera de `#[cfg(test)]` unitarios |

**Commit:** `test(eda): aislar backends reales en contract/integration/e2e`.

---

## T4 — Gate CI unitario

| # | Entregable | Detalle |
|---|------------|---------|
| T4.1 | Módulo `sddia-qa` | Escaneo **dentro** de bloques test (inverso a MAN-35 `productivo_lines`) |
| T4.2 | Allowlist versionada | Adapter/suite/integration/e2e + gate self + legacy bridge (**L-ALLOWLIST**) |
| T4.3 | Patrones | Lista `spec.md` §8.2 |
| T4.4 | CI / hook | Job o paso que falle en violaciones; documentar `run_id` o `PENDIENTE-CI` |
| T4.5 | Tests del gate | Fixture: unitario con `EventStore::open` → KO; allowlisted → OK |
| T4.6 | No debilitar MAN-35 | Gate productivo intacto; nuevo gate es sibling |

**Commit:** `feat(qa): gate CI anti-backend-real en unitarios EDA (MAN-37)`.

---

## T5 — Taxonomía documentada

| # | Entregable | Detalle |
|---|------------|---------|
| T5.1 | Consolidar | Tabla unit/contract/integration/e2e ya en `spec.md` §5; asegurar refs desde `objectives`/`clarify` |
| T5.2 | Operativa | Cómo elegir capa + enlace al harness + allowlist |
| T5.3 | Genoma | **Solo si** se exige norma indexada; default = docs en `persist_ref` sin mutación Genoma |

**Commit:** `docs(eda): taxonomía pirámide de tests EventManager`.

---

## T6 — Verificación y cierre

| # | Hecho | Verificación |
|---|--------|----------------|
| T6.1 | Unitarios EDA | `cargo test` relevantes verdes **sin** backend real |
| T6.2 | Contract suite | `run_event_manager_contract_suite` verde |
| T6.3 | Gate MAN-37 | Verde; MAN-35 productivo intacto |
| T6.4 | Inventario | Cero pendientes (**AC-5**) |
| T6.5 | Docs forja | `implementation.md` + `execution.md` |
| T6.6 | `validacion.md` | **APTO** solo tras AC-1…AC-7 (**AC-8**) |

**Commit(s):** cierre verificación; Git solo `skill:git-manager`.

---

## Orden de ejecución Tekton

1. T0 (harness + faults canónicos) → T1 (inventario).
2. T2 (migración) en paralelo parcial con T3 (reclasificar lo que no quepa en unit).
3. T4 (gate) **después** de que el inventario migrado no rompa en rojo permanente; puede aterrizar en modo warn→fail si hace falta, pero Done exige fail.
4. T5 (docs) puede solaparse tras T0 taxonomía base.
5. T6 cierra.

No declarar AC-6/AC-8 sin evidencia de gate/tests. No adelantar strip masivo sin harness (rompe unitarios).

## Criterios de salida del blueprint

- AC-1…AC-8 según `spec.md` / `objectives.md`.
- Laudos L-DOMAIN, L-HARNESS, L-FAULTS, L-TAXONOMY, L-STRIP, L-ALLOWLIST, L-INVENTORY, L-GATE, L-DOCS, L-GIT, L-GENOME, L-TRACKER respetados.
- Gate MAN-35 productivo **no** debilitado; composición scoped MAN-38 **no** reabierta.
- Sin Kaizen TODOs desde Tekton.
- Tracker: **MAN-37** / **PBI-94396a84** / padre **MAN-35**.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| Unitarios acoplados a `.events/` FS | Reclasificar a integration (T3) en vez de forzar mock incompleto |
| Gate MAN-37 rojo masivo al día 1 | Completar T2/T3 antes de modo fail; allowlist mínima explícita |
| Doble TLS / harness vs `install_port` ad hoc | Deprecar ad hoc en T2; un solo canal |
| `test_lab` StoreMode contamina unitarios | Prohibir en path unit; gate veto `override_store_mode` |
| Bridges `#[cfg(test)]` → legacy en `eda_port` | Unitarios no activan StoreMode; strip T2 |
| Confundir contract suite con unit | Allowlist + taxonomía (**L-ALLOWLIST**) |
| Debilitar gate MAN-35 | Gate nuevo **complementario**; no omitir productivo |

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
version_plan: "1.2.0"
spec_ref: "spec.md@1.2.0"
phases:
  - T0-event-manager-test-harness
  - T1-inventario-unitarios-eda
  - T2-migrar-unitarios-harness
  - T3-reclasificar-contract-integration-e2e
  - T4-gate-ci-unit-no-real-backend
  - T5-documentar-taxonomia
  - T6-verificacion-validacion
rbac_ok: true
scope: "Pirámide tests EDA — residual: inventario cerrado, adopción harness, paridad allowlist/gate, validacion"
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-37.jsonl"
execution_id: "74fe273f-1c3d-446f-bcea-0e83d6829dac"
---
# Plan / Blueprint — man-37-eda-test-pyramid

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md` (L-*), `spec.md` **v1.2.0** (ids **MAN-37** / **PBI-94396a84**).

`target_executor_rbac.allowed_policies` esperadas: `ecosystem-evolution` (si hace falta), `filesystem-ops`, `source-control`, `quality-assurance`.

**Estado de forja al sellado Dedalo (execution_id `08a7564d-…`):** T0 y T4 **ya materializados** en working tree; T2 parcial; T1/T3 documentados vía allowlist; residual = cerrar inventario, adoptar harness donde falte, paridad allowlist↔gate, T5 refs, T6 verde + `validacion` APTO. **No** re-forjar harness/gate desde cero.

## Viabilidad RBAC

| Cápsula | ¿Permitida? |
|---------|-------------|
| `skill:filesystem-manager` | sí |
| `skill:shell-executor` | sí (`cargo test` / gate CLI) |
| `skill:git-manager` | sí — **única** vía Git (**L-GIT**) |
| `action:execute-process` → entity-manager | solo si T5 muta Genoma (preferir docs en `persist_ref` sin Genoma) |

**Prohibido:** TODOs bajo `docs/todos/` (Cumulo / `Kaizen_Alert_Required` únicamente).

---

## Fases declarativas

```yaml
phases:
  - name: "T0 EventManagerTestHarness"
    intent: "Harness RAII + FaultInjection canónicos — YA FORJADO; solo verificar/regresión."
    status: forged
    delegates_to: ["skill:shell-executor"]
  - name: "T1 Inventario unitarios EDA"
    intent: "Cerrar tabla machine-readable migrate|reclassify|allowlist sin filas pending (spec §6.1 + execution.md)."
    status: residual
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T2 Migrar unitarios al harness"
    intent: "Migrar residuales no allowlisted; strip L-STRIP; preferir harness sobre install_port ad hoc."
    status: residual-partial
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T3 Reclasificar contract/integration/e2e"
    intent: "Mantener/ajustar allowlist integration_src; opcional mover a tests/** (kaizen no bloqueante)."
    status: residual-documented
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T4 Gate CI unitario"
    intent: "Gate + allowlist + CI — YA FORJADO; verificar verde y paridad toml↔path_allowlisted."
    status: forged
    delegates_to: ["skill:shell-executor", "skill:filesystem-manager"]
  - name: "T5 Documentar taxonomía"
    intent: "Asegurar refs cruzadas a spec §5 desde objectives/clarify/README si aplica; sin Genoma."
    status: residual-docs
    delegates_to: ["skill:filesystem-manager"]
  - name: "T6 Verificación"
    intent: "cargo test + contract suite + ambos gates verdes + validacion APTO."
    status: pending
    delegates_to: ["skill:shell-executor", "skill:filesystem-manager", "skill:git-manager"]
```

---

## T0 — EventManagerTestHarness (**forjado**)

| # | Entregable | Detalle |
|---|------------|---------|
| T0.1 | Tipo `EventManagerTestHarness` | Vivo en `eda_port::test_harness` |
| T0.2 | Locus | `execute-process` / `eda_port.rs` |
| T0.3 | Tests canónicos FaultInjection | Tres unitarios presentes |
| T0.4 | Guía mínima | Doc-comments en módulo |
| T0.5 | `scoped_tests` | Ya vía harness |
| T0.V | Verificación residual | `cargo test -p execute-process test_harness --lib` |

**No reabrir semántica.** Commit histórico esperado: `test(eda): añadir EventManagerTestHarness RAII con FaultInjection`.

---

## T1 — Inventario (**residual: cerrar**)

| # | Entregable | Detalle |
|---|------------|---------|
| T1.1 | Escaneo residual | Unitarios EDA fuera de harness y fuera de allowlist |
| T1.2 | Tabla machine-readable | Actualizar `execution.md` (path, símbolo, `migrate`\|`reclassify`\|`allowlist`, estado) |
| T1.3 | Cierre | **Cero** filas `pending` (**AC-5**) |
| T1.4 | Semilla | Partir de `spec.md` §6.1 + allowlist `eda-unit-backend.toml` |

**Commit:** `docs(eda): inventario unitarios EDA MAN-37 (T1 cierre)`.

---

## T2 — Migración al harness (**residual parcial**)

| # | Entregable | Detalle |
|---|------------|---------|
| T2.1 | Ya migrado | `domain_mutation`, `ecst_input_loader`, `tracker_pbi_meta`, `scoped_tests` |
| T2.2 | Pendiente | Cada fila `migrate` restante → `EventManagerTestHarness` |
| T2.3 | Strip | Retirar L-STRIP de unitarios no allowlisted |
| T2.4 | Asserts | Contra mock/puerto; faults deterministas (**AC-2**) |
| T2.5 | Bridges | Unitarios no empujan StoreMode → `eda_port_legacy` |

**Commit(s):** `refactor(eda): migrar unitarios EDA a EventManagerTestHarness`.

---

## T3 — Reclasificación de capas (**documentado / residual**)

| # | Entregable | Detalle |
|---|------------|---------|
| T3.1 | Matrices I/O | `execute-process/tests/**` = integration |
| T3.2 | `test_lab` | Solo helpers integration |
| T3.3 | `integration_src` | Allowlist = reclasificación válida (**D4.7**) |
| T3.4 | Paridad | Si se añade path a toml → espejar en `path_allowlisted` |
| T3.5 | Kaizen opcional | Mover integration-in-src a `tests/**` — no bloquea Done |

**Commit:** solo si hay cambios de código/allowlist; si no, documentar en `execution.md`.

---

## T4 — Gate CI unitario (**forjado**)

| # | Entregable | Detalle |
|---|------------|---------|
| T4.1 | Módulo `sddia-qa` | `gate_eda_unit_no_real_backend.rs` |
| T4.2 | Allowlist | `allowlists/eda-unit-backend.toml` |
| T4.3 | CLI + CI | `gate-eda-unit-no-real-backend`; workflow index-qa |
| T4.4 | Tests del gate | Fixture KO + allowlist skip |
| T4.V | Verificación residual | `sddia-qa gate-eda-unit-no-real-backend --json` exit 0; MAN-35 intacto |
| T4.P | Paridad | toml ↔ hardcode gate al tocar entradas |

**No debilitar MAN-35.** Commit histórico: `feat(qa): gate CI anti-backend-real en unitarios EDA (MAN-37)`.

---

## T5 — Taxonomía documentada (**residual docs**)

| # | Entregable | Detalle |
|---|------------|---------|
| T5.1 | Canónica | `spec.md` §5 (**v1.2.0**) |
| T5.2 | Refs | objectives/clarify/plan apuntan a §5; opcional README engine |
| T5.3 | Genoma | **No** por defecto (**L-GENOME**) |

**Commit:** `docs(eda): taxonomía pirámide de tests EventManager` (si hay delta documental).

---

## T6 — Verificación y cierre

| # | Hecho | Verificación |
|---|--------|----------------|
| T6.1 | Unitarios EDA | `cargo test` relevantes verdes **sin** backend real |
| T6.2 | Contract suite | `run_event_manager_contract_suite` verde |
| T6.3 | Gate MAN-37 | Verde; MAN-35 productivo intacto |
| T6.4 | Inventario | Cero pendientes (**AC-5**) |
| T6.5 | Docs forja | `implementation.md` + `execution.md` actualizados |
| T6.6 | `validacion.md` | **APTO** solo tras AC-1…AC-7 (**AC-8**) |

Comandos de referencia (`execution.md`):

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo test -p execute-process test_harness --lib
cd SddIA && CARGO_TARGET_DIR=target cargo build -p sddia-qa
SddIA/target/debug/sddia-qa gate-eda-unit-no-real-backend --json
SddIA/target/debug/sddia-qa gate-event-manager-port --json
```

**Commit(s):** cierre verificación; Git solo `skill:git-manager`.

---

## Orden de ejecución Tekton (residual)

1. **T0.V / T4.V** — smoke harness + gate (no reescritura).
2. **T1** — cerrar inventario machine-readable.
3. **T2** — migrar residuales; **T3** — ajustar allowlist solo si hace falta.
4. **T4.P** — paridad allowlist si T3 tocó entradas.
5. **T5** — refs documentales.
6. **T6** — verde + `validacion` APTO.

No declarar AC-6/AC-8 sin evidencia de gate/tests. No inventar éxito de inventario cerrado sin tabla sin `pending`.

## Criterios de salida del blueprint

- AC-1…AC-8 según `spec.md` **v1.2.0** / `objectives.md`.
- Laudos L-DOMAIN … L-TRACKER respetados.
- Gate MAN-35 productivo **no** debilitado; composición scoped MAN-38 **no** reabierta.
- Sin Kaizen TODOs desde Tekton.
- Tracker: **MAN-37** / **PBI-94396a84** / padre **MAN-35**.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| Re-forjar T0/T4 por spec stale | Usar **v1.2.0** (evidencia Presente); solo residual |
| Allowlist toml ≠ hardcode gate | Checklist T4.P en cada cambio |
| Unitarios acoplados a `.events/` FS | Reclassify/allowlist (T3) vs forzar mock incompleto |
| Inventario con `pending` al Done | Bloquear AC-5 / validacion |
| Debilitar gate MAN-35 | Gate MAN-37 sibling; no omitir productivo |
| Kaizen TODOs desde Tekton | Solo Cumulo / `Kaizen_Alert_Required` |

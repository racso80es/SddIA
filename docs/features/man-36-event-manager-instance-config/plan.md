---
feature_name: man-36-event-manager-instance-config
created: "2026-11-09"
updated: "2026-11-09"
process: refactorization
branch_name: refactor/man-36-event-manager-instance-config
persist_ref: docs/features/man-36-event-manager-instance-config
pbi_ref: MAN-36
parent_ref: MAN-35
document_id: PBI-0604fb3c
agent_planificador: dedalo
target_executor: tekton
phase: diseno-blueprint
status: blueprint-ready
version_plan: "1.1.0"
phases:
  - T0-contrato-boveda
  - T1-host-resolucion
  - T2-desacoplo-consumidores
  - T3-cumulo-logico
  - T4-documentacion
  - T5-tests-validacion
rbac_ok: true
scope: "Bóveda soberana Event Manager — cierre residual CA-4 + verificación; Cúmulo lógico-only"
norm_ref: features-documentation-pattern@1.3.1
execution_id: "4fc6eadd-78b9-4022-8991-ed39d911e9af"
tracker_channel: ".SddIA/state/tracker-outbound/MAN-36.jsonl"
forge_key: "582d8a29c4f772596e7de259a8af20da3b0352e2f1354317c12574eadd31fa16"
---
# Plan / Blueprint — man-36-event-manager-instance-config

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md` (L-*), `spec.md` **v1.1.0** (ids **MAN-36** / **PBI-0604fb3c**).

`target_executor_rbac.allowed_policies` alineadas a Tekton: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `chaos-engineering`, `tracker-operations`.

**Lectura de estado:** T0–T4 están **mayormente materializados** en el árbol (host/`config.rs`, Cúmulo 1.12.0, plantilla bóveda, Constitución, bridge lógico). Este blueprint **no** pide reescribir lo hecho: prioriza **delta CA-4** (`eda_port::runtime_db_path`) + **T5 verificación** + docs/gates residuales.

## Viabilidad RBAC (cruzada mecánicamente)

| Cápsula | `context` skill | ¿En Tekton? |
|---------|-----------------|-------------|
| `skill:filesystem-manager` | `filesystem-ops` | sí |
| `skill:shell-executor` | `system-operations` | sí (`cargo test` / clippy / gate) |
| `skill:git-manager` | `source-control` | sí — **única** vía Git (**L-GIT**) |
| `action:execute-process` → entity-manager | `ecosystem-evolution` | sí si muta Genoma/docs normativos (**L-GENOME**) |

**Prohibido:** TODOs bajo `docs/todos/` (solo `agent:cumulo` / `Kaizen_Alert_Required`).
**Prohibido:** rehabilitar FS productivo o tocar ops EventManagerPort 1.2.
**rbac_ok:** `true` — ninguna fase requiere contexto fuera de Tekton.

---

## Fases declarativas

```yaml
phases:
  - name: "T0 Contrato de bóveda"
    intent: "Auditar plantilla SDDIA_EVENT_MANAGER_* y precedencia; no recrear si ya cumple L-VARS."
    delegates_to: ["skill:filesystem-manager"]
  - name: "T1 Resolución en host"
    intent: "Auditar resolve/validate/factory productiva Redb; no reabrir salvo regresión."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T2 Desacoplo consumidores (delta CA-4)"
    intent: "Cerrar runtime_db_path: sin fallback DEFAULT_REDB_REL en eda_port; consumidores solo puerto/host APIs."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T3 Cúmulo lógico-only"
    intent: "Confirmar cumulo.paths.json v1.12.0 sin stores físicos; no reintroducir event_bus/*.store."
    delegates_to: ["skill:filesystem-manager"]
  - name: "T4 Documentación límite Core/instancia"
    intent: "Cerrar drift README/contratos/gates vs Constitución+env.example (CA-8)."
    delegates_to: ["skill:filesystem-manager", "action:execute-process"]
  - name: "T5 Tests y cierre forja"
    intent: "Matriz host + gates; implementation/execution actualizados; evidencia CA-1…8 sin inventar APTO."
    delegates_to: ["skill:shell-executor", "skill:filesystem-manager", "skill:git-manager"]
```

---

## T0 — Contrato de bóveda (auditoría)

| # | Entregable | Detalle |
|---|------------|---------|
| T0.1 | `.dev/.env.example` | Verificar bloque PROVIDER/REDB_PATH/FS_ROOT + precedencia; legacy marcado retirado |
| T0.2 | Guía instancia | Override en `.SddIA/.dev/.env` sin tocar Cúmulo |
| T0.3 | Sin `.env` en cápsulas | Confirmar no hay env nuevos bajo skills/tools/daemons |

**Salida:** checklist OK o parche mínimo. Commit solo si hay diff: `docs(eda): alinear plantilla bóveda Event Manager (MAN-36)`.

---

## T1 — Host (auditoría)

| # | Entregable | Detalle |
|---|------------|---------|
| T1.1 | `config.rs` | Precedencia + L-PATH-POLICY + productive rejects FS |
| T1.2 | `lib.rs` | Factory/scoped open desde config; APIs host-only de path |
| T1.3 | Regresión | Si falta cobertura, añadir test — no rediseñar API |

**Commit** solo si hay diff: `fix(eda): host Event Manager desde bóveda (MAN-36)`.

---

## T2 — Desacoplo consumidores (**prioridad**)

| # | Entregable | Detalle |
|---|------------|---------|
| T2.1 | `eda_port::runtime_db_path` | Retirar **o** `Result` puro vía `resolved_redb_path` **sin** `DEFAULT_REDB_REL` en el consumidor |
| T2.2 | Call sites | Migrar a `with_event_port` / `productive_store_present` / `is_resolved_redb_path` |
| T2.3 | Grep allowlist | Cero `events_redb_path`; cero fallback cableado fuera host |
| T2.4 | Compilación | `execute-process` + daemons/tools afectados |

**Commit:** `refactor(eda): cerrar runtime_db_path sin default cableado (MAN-36)`.

---

## T3 — Cúmulo (confirmación)

| # | Entregable | Detalle |
|---|------------|---------|
| T3.1 | `cumulo.paths.json` | Persiste v1.12.0 lógico-only |
| T3.2 | Lectores | Topology/bus sin asumir `*.store` físico |

**Commit** solo si hay diff.

---

## T4 — Documentación residual

| # | Entregable | Detalle |
|---|------------|---------|
| T4.1 | README / contratos EDA | Límite Core/instancia + vars + precedencia si drift |
| T4.2 | Gates / normas cutover | Veto stores en Cúmulo / path fuera host |
| T4.3 | Genoma | Solo vía entity-manager; sin bump 1.2.0 ops |

**Commit:** `docs(eda): soberanía de instancia Event Manager (MAN-36)` si aplica.

---

## T5 — Tests y cierre forja

| # | Hecho | Verificación |
|---|--------|----------------|
| T5.1 | Matriz precedencia | SO > instancia > global (**CA-6**) |
| T5.2 | Matriz rutas | rel/abs/vacía/`..` (**CA-5**) |
| T5.3 | Contract suite | MemoryMock / FS / Redb (**CA-7**) |
| T5.4 | `cargo test -p sddia-event-manager-host` (+ afectados) | Verde |
| T5.5 | Gates QA | `gate-event-manager-port`, `gate-eda-unit-no-real-backend` |
| T5.6 | Docs forja | Actualizar `implementation.md` + `execution.md` con delta T2 |

**Commit(s):** verificación; Git solo `skill:git-manager`.

---

## Orden de ejecución Tekton

1. T0/T1 **auditoría rápida** (no reescritura).
2. **T2 delta CA-4** (bloqueante de diseño para CA-4).
3. T3 confirmación.
4. T4 docs residuales.
5. T5 tests + artefactos forja → handoff Argos/`validacion.md`.

No declarar CA-1…CA-8 sin evidencia. No abrir EventManagerPort 1.2. No seeds Kaizen desde Tekton.

## Criterios de salida del blueprint

- CA-1…CA-8 según `spec.md` / `objectives.md`.
- Laudos L-VAULT, L-PRECEDENCE, L-VARS, L-COMPOSE, L-REDB-DEFAULT, L-FS-LIMIT, L-PATH-POLICY, L-CUMULO-LOGICAL, L-LOGICAL-EVENTS, L-NO-CAPSULE-ENV, L-DOCS, L-VERIFY, L-GIT, L-GENOME, L-TRACKER respetados.
- Residuo `runtime_db_path` **cerrado** o acotado sin fallback cableado.
- Tracker: **MAN-36** / documento **PBI-0604fb3c** / padre **MAN-35** / `execution_id` **4fc6eadd-78b9-4022-8991-ed39d911e9af**.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| Reescritura ciega de T0–T1 ya vivos | Auditoría primero; diffs mínimos |
| `runtime_db_path` fallback viola CA-4 | T2.1 obligatorio; gate grep |
| Call sites rompen al retirar PathBuf | Migrar a APIs host-only en el mismo tajo |
| FS se cuela en prod | Ya gate en `resolve_productive_backend_config`; test negativo |
| Timeout runtime Tekton | `execution.md` — elevar timeout bóveda si el orquestador reentra |
| Contaminar cápsulas con `.env` | L-NO-CAPSULE-ENV; review T0/T4 |

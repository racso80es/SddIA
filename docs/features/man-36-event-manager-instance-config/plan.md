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
version_plan: "1.0.1"
phases:
  - T0-contrato-boveda
  - T1-host-resolucion
  - T2-desacoplo-consumidores
  - T3-cumulo-logico
  - T4-documentacion
  - T5-tests-validacion
rbac_ok: true
scope: "Bóveda soberana Event Manager — host resuelve proveedor/ruta; Cúmulo lógico-only"
norm_ref: features-documentation-pattern@1.3.1
execution_id: "4fc6eadd-78b9-4022-8991-ed39d911e9af"
tracker_channel: ".SddIA/state/tracker-outbound/MAN-36.jsonl"
forge_key: "582d8a29c4f772596e7de259a8af20da3b0352e2f1354317c12574eadd31fa16"
---
# Plan / Blueprint — man-36-event-manager-instance-config

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md` (L-*), `spec.md` v1.0.1 (ids **MAN-36** / **PBI-0604fb3c**).

`target_executor_rbac.allowed_policies` alineadas a Tekton: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `chaos-engineering`, `tracker-operations`.

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
    intent: "Documentar y plantillar SDDIA_EVENT_MANAGER_*; precedencia SO>instancia>global; retirar docs productivas SDDIA_EVENT_STORE."
    delegates_to: ["skill:filesystem-manager"]
  - name: "T1 Resolución en host"
    intent: "Implementar resolve/validate en sddia-event-manager-host; factory productiva solo Redb desde config; fallo pre-open."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T2 Desacoplo consumidores"
    intent: "Eliminar events_redb_path/runtime_db_path productivos y PublishSeal físico; consumidores solo puerto."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T3 Cúmulo lógico-only"
    intent: "Quitar event_bus y eda_*.store físicos; bump cumulo; alinear lectores/gates."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T4 Documentación límite Core/instancia"
    intent: "README, Constitución §3.1, contratos EDA; CA-8."
    delegates_to: ["skill:filesystem-manager", "action:execute-process"]
  - name: "T5 Tests y cierre forja"
    intent: "Matriz precedencia/rutas; contract suite intacta; implementation/execution; cargo test host+afectados."
    delegates_to: ["skill:shell-executor", "skill:filesystem-manager", "skill:git-manager"]
```

---

## T0 — Contrato de bóveda

| # | Entregable | Detalle |
|---|------------|---------|
| T0.1 | `.dev/.env.example` | Bloque `SDDIA_EVENT_MANAGER_PROVIDER` / `REDB_PATH` / `FS_ROOT` + nota de precedencia; deprecar/retirar `SDDIA_EVENT_STORE` productivo |
| T0.2 | Comentarios guía instancia | Documentar que `.SddIA/.dev/.env` overridea clone sin tocar Cúmulo |
| T0.3 | Sin `.env` en cápsulas | Verificar no se añaden ficheros env bajo skills/tools/daemons |

**Salida:** contrato documental listo para T1. **Commit** (`git-manager`): `docs(eda): plantilla bóveda Event Manager (MAN-36)`.

---

## T1 — Resolución en `sddia-event-manager-host`

| # | Entregable | Detalle |
|---|------------|---------|
| T1.1 | `resolve_backend_config` | Lectura SO + bóvedas; defaults Redb; provider enum |
| T1.2 | `validate_configured_path` | **L-PATH-POLICY** (rel/abs/vacía/`..`) |
| T1.3 | Factory productiva | Solo Redb; path validado; crear parents; open con budget existente |
| T1.4 | FS gate | Provider `fs` rechazado en rol `default` / `resolve_productive_port` |
| T1.5 | API path | Dejar de exportar `events_redb_path` pública (o internalizar) |

**Commit:** `feat(eda): host resuelve backend Event Manager desde bóveda`.

---

## T2 — Desacoplo consumidores

| # | Entregable | Detalle |
|---|------------|---------|
| T2.1 | `runtime_bridge.rs` | Sin `events_redb_path`; `PublishSeal` sin path físico Redb |
| T2.2 | `eda_port.rs` | Retirar `runtime_db_path` productivo |
| T2.3 | Grep consumidores | `event-watcher`, audit, purger, daemon-runtime: solo puerto/host scoped |
| T2.4 | Compilación | Crates afectados compilan sin imports de path builder |

**Commit:** `refactor(eda): consumidores sin ruta física Redb (MAN-36)`.

---

## T3 — Cúmulo lógico-only

| # | Entregable | Detalle |
|---|------------|---------|
| T3.1 | `cumulo.paths.json` | Eliminar `event_bus`, `eda_bus.store`, `eda_fractal.store`; bump versión |
| T3.2 | Lectores | Confirmar `load_bus_topology` / `load_eda_bus` con defaults `.events`; arreglar aserciones rotas |
| T3.3 | Gate | Veto a stores físicos en Cúmulo / path cableado fuera host |

**Commit:** `refactor(cumulo): Event Manager sin stores físicos en paths`.

---

## T4 — Documentación

| # | Entregable | Detalle |
|---|------------|---------|
| T4.1 | `CONSTITUTION_CORE.md` §3.1 | Ubicación física = bóveda; default histórico solo si no hay config |
| T4.2 | README / contratos EDA | Límite Core/instancia + vars + precedencia (**CA-8**) |
| T4.3 | Genoma (si aplica) | Solo texto/bindings descriptivos vía entity-manager; **sin** bump 1.2.0 de ops |

**Commit:** `docs(eda): soberanía de instancia Event Manager (MAN-36)`.

---

## T5 — Tests y cierre forja

| # | Hecho | Verificación |
|---|--------|----------------|
| T5.1 | Matriz precedencia | SO > instancia > global (**CA-6**) |
| T5.2 | Matriz rutas | rel/abs/vacía/`..`/tempdir (**CA-5**) |
| T5.3 | Contract suite | MemoryMock / FS / Redb intacta (**CA-7**) |
| T5.4 | `cargo test -p sddia-event-manager-host` (+ afectados) | Verde |
| T5.5 | Docs forja | `implementation.md` + `execution.md` |

**Commit(s):** verificación; Git solo `skill:git-manager`.

---

## Orden de ejecución Tekton

1. T0 (plantillas) → T1 (host) → T2 (consumidores).
2. T3 (Cúmulo) preferible **después** de T1 para no dejar window sin default host.
3. T4 (docs) tras T3 (descripción alineada al JSON final).
4. T5 cierra con tests + artefactos de forja.

No declarar CA-1…CA-8 sin evidencia. No abrir EventManagerPort 1.2. No seeds Kaizen desde Tekton.

## Criterios de salida del blueprint

- CA-1…CA-8 según `spec.md` / `objectives.md`.
- Laudos L-VAULT, L-PRECEDENCE, L-VARS, L-COMPOSE, L-REDB-DEFAULT, L-FS-LIMIT, L-PATH-POLICY, L-CUMULO-LOGICAL, L-LOGICAL-EVENTS, L-NO-CAPSULE-ENV, L-DOCS, L-VERIFY, L-GIT, L-GENOME, L-TRACKER respetados.
- Tracker: **MAN-36** / documento **PBI-0604fb3c** / padre **MAN-35** / `execution_id` **4fc6eadd-78b9-4022-8991-ed39d911e9af**.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| Doble `events_redb_path` (host + core) deja residuos | T1+T2: eliminar export; gate grep |
| `PublishSeal.target_path` rompe callers | Sustituir por token lógico; actualizar call sites en el mismo tajo |
| Lectores Cúmulo asumen `*.store` | T3 + tests daemon-runtime / topology |
| FS se cuela en prod | Gate provider en `resolve_productive_port`; test negativo |
| Absoluta mal validada | Suite CA-5 con `..` y vacía |
| Contaminar cápsulas con `.env` | L-NO-CAPSULE-ENV; review T0/T4 |

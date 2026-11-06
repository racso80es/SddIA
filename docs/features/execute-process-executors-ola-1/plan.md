---
feature_name: execute-process-executors-ola-1
created: "2026-11-05"
process: refactorization
branch_name: refactor/execute-process
persist_ref: docs/features/execute-process-executors-ola-1
pbi_ref: AUT-77
document_id: PBI-9e2df289
historia_ref: MAN-30
execution_id: "f41c572c-b6d1-4599-b76b-bf5cba84b7c0"
phases: 6
agent_planificador: dedalo
---

# Blueprint — Ejecutores por clase desde execute-process (Ola 1)

Entrada: `objectives.md`, `clarify.md`, `spec.md`. Ejecutor destino Tekton: contextos `ecosystem-evolution`, `filesystem-ops`, `source-control` (proceso `refactorization`).

## 0. Estado de la entrega

| Bloque | Estado | Evidencia |
|--------|--------|-----------|
| Clarificación (Mayeuta) | ✅ | `clarify.md` D0–D9 |
| Objetivos | ✅ | `objectives.md` O1–O10 |
| Especificación (Dedalo) | ✅ | `spec.md` |
| Planificación (Dedalo) | ✅ | este documento |
| E0…E5 (Tekton) | ⏳ | pendiente fase Ejecución |
| Verificación Argos | ⏳ | `validacion.md` |
| Cierre | ⏳ | `delivery-close-cycle` + Linear `done` |

**Decisión arquitectónica:** ejecutores = despacho por clase + capa compartida de invocación de cápsulas; orquestador = aduana universal; sin genoma nuevo por defecto; sin `execute-event`.

---

## E0 — Sellado de inventario

**Intent:** Congelar Ola 1 vs diferidos y bridge Python vacío antes de mutar código.

| # | Entregable | Detalle |
|---|------------|---------|
| E0.1 | Inventario en expediente | Ya en `spec.md` §3; Tekton copia tabla operativa a `implementation.md` |
| E0.2 | Etiqueta P17 | Documentar ausencia física de `execute-action.py` (no inventar poda de fichero) |
| E0.3 | Trazabilidad | Confirmar enlaces MAN-30 ↔ AUT-77 en issue Linear (lectura/stamp vía tracker; sin `docs/todos/`) |

**Delegates_to:** `skill:filesystem-manager`

**Commit sugerido:** `docs(refactor): E0 — inventario ejecutores ola 1 AUT-77`

**Criterio de salida:** `implementation.md` cita inventario; sin código de despacho aún **o** solo docs.

---

## E1 — Capa compartida `capsule-invoke`

**Intent:** Extraer resolve + spawn + parse JSON + fallbacks WASM→native fuera del monolito concentrador.

| # | Entregable | Detalle |
|---|------------|---------|
| E1.1 | Frontera física | Preferir crate lib `SddIA/engine/capsule-invoke` (workspace `engine/*`); alternativa válida: `execute-process/src/engine/executors/common.rs` promocionable en el mismo PR |
| E1.2 | Mover | `capsule_paths` (o API mínima), `invoke_capsule_subprocess`, `invoke_capsule_json_labeled`, unwrap/finalize |
| E1.3 | Facade | `execute_process::engine::capsules` reexporta temporalmente |
| E1.4 | Tests | Unitarios de resolve/fallback/errores tipados que ya vivían en `capsules.rs` |

**Delegates_to:** `skill:filesystem-manager`

**Commit sugerido:** `refactor(engine): E1 — extraer capa capsule-invoke`

**Criterio de salida:** `cargo test -p execute-process` (y crate nuevo si aplica) verde; semántica CLI intacta.

---

## E2 — Ejecutor action + paridad O3

**Intent:** Desconcentrar `invoke_action` / fachada sobre `try_run_native`.

| # | Entregable | Detalle |
|---|------------|---------|
| E2.1 | API `dispatch_action` | Concentra nativo → cápsula → error; handlers `actions.rs` sin duplicar dominio |
| E2.2 | CLI | `main.rs::run_action_cli` llama ejecutor |
| E2.3 | Consumidores calientes | `phase_capsules` sello presented, `route_domain_core::invoke_execute_action`, etc. vía facade |
| E2.4 | Paridad O3 | Smoke `--action` representativo pre/post |

**Delegates_to:** `skill:filesystem-manager`, `action:execute-process`

**Commit sugerido:** `refactor(engine): E2 — ejecutor action + paridad CLI`

**Criterio de salida:** CA-4 (O3) verde; camino monolítico action retirado o reducido a reexport.

---

## E3 — Ejecutor tool + paridad O5

**Intent:** Mover `invoke_tool_for_process` / `invoke_tool_capsule_json` preservando gates Linear/lab y `file-tracker-adapter` in-process.

| # | Entregable | Detalle |
|---|------------|---------|
| E3.1 | API tool | Incluye gates + outbound recording actuales |
| E3.2 | CLI | `run_tool_cli` → ejecutor |
| E3.3 | Golden | `capsule-invoke-smoke` (`tool:io-choke`) |
| E3.4 | Paridad O5 | Smoke CLI `--tool` y/o proceso smoke |

**Delegates_to:** `skill:filesystem-manager`, `action:execute-process`

**Commit sugerido:** `refactor(engine): E3 — ejecutor tool + capsule-invoke-smoke`

**Criterio de salida:** CA-6 (O5) verde.

---

## E4 — Wrappers skill + fan-out + paridad O4

**Intent:** Especializaciones `git-manager` / `shell-executor` y `try_invoke_delegates` delegan en la capa común sin cambiar semántica (skip git-manager en fan-out).

| # | Entregable | Detalle |
|---|------------|---------|
| E4.1 | Wrappers skill | `invoke_git_manager(_for)`, `invoke_shell_executor` |
| E4.2 | Fan-out | `try_invoke_delegates` → API ejecutor; conservar label homogenizado actual |
| E4.3 | Paridad O4 | Camino skill representativo (git en init/delivery o shell acotado) |
| E4.4 | Executor/residual | Confirmar peaje Cerbero **antes** del fan-out (aduana intacta) |

**Delegates_to:** `skill:filesystem-manager`, `skill:git-manager`, `action:execute-process`

**Commit sugerido:** `refactor(engine): E4 — wrappers skill + fan-out delegates`

**Criterio de salida:** CA-5 (O4) + CA-3 (aduana) verdes.

---

## E5 — Retarget, higiene, evolution, cierre documental de forja

**Intent:** Eliminar concentrador muerto; sellar trazabilidad; preparar Argos.

| # | Entregable | Detalle |
|---|------------|---------|
| E5.1 | Retarget | Consumidores densos usan facades estables; borrar código muerto en `capsules.rs` monolito |
| E5.2 | Higiene | Comentarios D-P5.1-R/`execute-action.py` alineados a inventario vacío |
| E5.3 | Evolution | Entrada en `SddIA/evolution/` si mutación material Core |
| E5.4 | Docs forja | `implementation.md` + `execution.md` (un commit/tarea anotada) |
| E5.5 | Trazabilidad padre | Verificar referencia bidireccional MAN-30 ↔ AUT-77 en Linear (vía proceso/tool tracker; **no** `docs/todos/`) |

**Delegates_to:** `skill:filesystem-manager`, `skill:git-manager`, `tool:linear-tracker-adapter`

**Commit sugerido:** `refactor(engine): E5 — retarget consumidores + evolution ola 1`

**Criterio de salida:** CA-1…CA-10 listos para Argos; sin ficheros nuevos en `docs/todos/{pending,done}/`.

---

## Secuencia de verificación

| Hito | Gate |
|------|------|
| Fin E1 | Tests capa común + CLI process smoke básico |
| Fin E2 | O3 action |
| Fin E3 | O5 tool (`capsule-invoke-smoke`) |
| Fin E4 | O4 skill + Cerbero pre-fase |
| Fin E5 | Diff sin monolito; evolution; handoff Argos |

## Orden de delegación (runtime `refactorization`)

1. `agent:dedalo` — `spec.md` + `plan.md` (**este hito**).
2. `agent:tekton` — E0…E5 → `implementation.md` + `execution.md` (un commit por tarea vía `skill:git-manager`).
3. `agent:argos` — `validacion.md`.
4. `action:execute-process` → `delivery-close-cycle` (cierre; Linear `done`).

## Rollback

Cada hito E1–E4 deja facades reexportando. Revertir = restaurar módulo previo en el commit del hito; `./sddia-run.sh` no cambia de binario. Sin coexistencia de dos orquestadores.

## Viabilidad RBAC (Dedalo §4)

| Cápsula en blueprint | Contextos típicos | ∩ `refactorization` |
|----------------------|-------------------|---------------------|
| `skill:filesystem-manager` | filesystem-ops | ✅ |
| `skill:git-manager` | source-control | ✅ |
| `action:execute-process` | (orquestador; peaje por proceso hijo) | ✅ vía contrato runtime |
| `tool:linear-tracker-adapter` | tracker-operations / ecosystem | ✅ si binding instancia; uso solo lectura/stamp trazabilidad |

No se referencian cápsulas fuera del catálogo indexado. No se exige forja de entidades nuevas en Ola 1.

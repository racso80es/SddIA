---
feature_name: execute-process-executors-ola-1
created: "2026-11-05"
process: refactorization
base: main
scope: execute-process-physical-dispatch-ola-1
branch_name: refactor/execute-process
persist_ref: docs/features/execute-process-executors-ola-1
pbi_ref: AUT-77
document_id: PBI-9e2df289
historia_ref: MAN-30
execution_id: "f41c572c-b6d1-4599-b76b-bf5cba84b7c0"
related:
  - docs/features/execute-process-executors-ola-1/objectives.md
  - docs/features/execute-process-executors-ola-1/clarify.md
  - docs/features/migracion-execute-process-rust/implementation.md
  - SddIA/norms/capsule-json-io.md
  - SddIA/actions/actions-contract.md
  - SddIA/process/process-contract.md
  - SddIA/core/cumulo.paths.json
---

# Especificación técnica — Ejecutores por clase (Ola 1)

## 0. Fuentes y límites de evidencia

| Fuente | Uso |
|--------|-----|
| `objectives.md` / `clarify.md` (D0–D9) | `refined_requirements` + laudos ontológicos |
| Inventario estático crate `SddIA/engine/execute-process` | Puntos de despacho físico reales |
| Antecedente `migracion-execute-process-rust` | Contexto de aduana Rust ya soberana |
| Cuerpo Linear `AUT-77` live | **No materializado en esta sesión Dedalo** (Shell/Auto-review); SSOT operativa = objetivos + clarify coherentes con caché Mayeuta `PBI-9e2df289` |

## 1. Propósito

Desconcentrar del orquestador Rust nativo el **despacho físico** de invocación por clase (`action` / `skill` / `tool`), materializando:

1. una **capa compartida** de resolución + spawn de cápsulas (JSON stdin/stdout);
2. **ejecutores de despacho por clase** (modelo: `action` de despacho + wrappers tipados),

sin rediseñar el grafo de fases del códice SE, sin `execute-event`, y **sin** reescritura de consumidores de `./sddia-run.sh`.

## 2. Fronteras

| In scope (Ola 1) | Out of scope |
|------------------|--------------|
| Inventario documentado de puntos de invocación action/skill/tool | Agents como ejecutor |
| Extracción incremental de `capsules.rs` + fan-out `try_invoke_delegates` + CLI `--tool`/`--action` | Nuevos entrypoints process-as-subprocess |
| Frontera orquestador (aduana) ↔ ejecutores (despacho físico) | Extracción del enrutado EDA / bus |
| Paridad ≥1 camino representativo por familia | Big-bang / reescritura CLI |
| Etiquetado bridge Python residual | Poda forzada de Python en todo el ecosistema |
| Evolution si mutación material bajo `SddIA/` | Cambios UX tracker / `refine-item` / `forge-*` |

## 3. Inventario Ola 1 vs diferidos

### 3.1 Núcleo a desconcentrar (Ola 1)

| Punto | Clase | Rol actual |
|-------|-------|------------|
| `engine/capsules.rs::invoke_capsule_subprocess` | skill\|tool (runtime) | Spawn WASM/nativo |
| `engine/capsules.rs::invoke_capsule_json` / `invoke_tool_capsule_json` | skill / tool (label) | Resolve + JSON + fallback WASM→native |
| `engine/capsules.rs::invoke_tool` / `invoke_tool_for_process` | tool | Gates Linear/lab + `file-tracker-adapter` in-process + outbound |
| `engine/capsules.rs::invoke_git_manager(_for)` | skill | Especialización tipada `git-manager` |
| `engine/capsules.rs::invoke_shell_executor` | skill | Especialización tipada `shell-executor` |
| `engine/capsules.rs::invoke_action` | action | `actions::try_run_native` → fallback cápsula |
| `engine/actions.rs::try_run_native` | action | Registro in-process de handlers nativos |
| `engine/phase_capsules.rs::try_invoke_delegates` | skill\|tool | Fan-out `delegates_to`; salta `skill:git-manager`; hoy llama `invoke_tool_capsule_json` para ambas |
| `main.rs::run_tool_cli` / `run_action_cli` | tool / action | Entrypoints CLI cortos |

### 3.2 Consumidores (retarget a API de ejecutor; negocio no se mueve)

`executor.rs`, `residual_runner.rs`, `workspace_init.rs`, `delivery_close.rs`, `accept_pr.rs`, `pull_request_review.rs`, `route_domain_core.rs`, `agent_runtime.rs` (solo peaje skill git), handlers tracker/telegram/email/kalma2, `purge_*`, `sync_entity_index`, `capsule_invoke_smoke`, etc. Siguen siendo **orquestación/dominio**; dejan de importar el monolito de spawn.

### 3.3 Diferidos (clarify D3)

| Punto | Motivo |
|-------|--------|
| `agent_runtime::invoke_agent_phase` | Agent fuera de ola |
| `invoke_orchestrator::*` | Recursión process existente; no nuevos subprocess process |
| Extracción bus EDA / `execute-event` | Event ≠ clase ejecutable |
| Fan-out fase `action:` hoy **simulated** | Cablear despacho en grafo = mejora futura; no bloquea Ola 1 |

### 3.4 Bridge Python de acciones (P17 / D-P5.1-R)

**Estado físico verificado en árbol actual:** no existe `execute-action.py` ni bridge `invoke_action`→Python en el crate. Cadena vigente = nativo → cápsula homónima → error tipado. Menciones residuales = comentarios de paridad histórica. Ola 1 **etiqueta** inventario vacío; poda documental de comentarios obsoletos permitida; no hay subprocess Python que retirar del concentrador.

## 4. Arquitectura objetivo

```text
 ./sddia-run.sh
        │
        ▼
 ┌──────────────────────────────────────────────────────────┐
 │  execute-process (ADUANA UNIVERSAL)                        │
 │  · CLI parse / normalize_request                           │
 │  · grafo de fases / phase_invocations / execution_report   │
 │  · Cerbero DI + capability DI                              │
 │  · telemetría / peaje termodinámico fail-soft              │
 │  · invoke_orchestrator (process recursion)                 │
 │  · decide CUÁNDO invocar                                   │
 └───────────────┬───────────────────────┬────────────────────┘
                 │                       │
                 ▼                       ▼
      ┌──────────────────┐    ┌─────────────────────────────┐
      │ action-dispatch  │    │ capsule-invoke (capa común) │
      │ · try_run_native │───►│ · resolve (cumulo paths)    │
      │ · fallback name  │    │ · spawn WASM|ELF            │
      └──────────────────┘    │ · JSON stdout / fallbacks   │
                              │ · labels skill|tool         │
                              └──────────────┬──────────────┘
                                             │
                              ┌──────────────┴──────────────┐
                              │ skill/tool wrappers           │
                              │ · git-manager / shell-executor│
                              │ · invoke_tool_for_process     │
                              │ · try_invoke_delegates (fan)  │
                              └───────────────────────────────┘
```

### 4.1 Decisión de frontera física (Dedalo)

| Pieza | Forma | Genoma |
|-------|-------|--------|
| Capa compartida | Crate lib interno `SddIA/engine/capsule-invoke` (miembro `engine/*`) **o**, si el blast radius de crate nuevo es inaceptable en el primer commit, módulo `execute-process/src/engine/executors/common.rs` promovible a crate en el mismo PR | **Sin** entidad indexada (reubicación interna; D4) |
| Ejecutor action | Módulo/API `dispatch_action` que concentra `invoke_action` + fachada sobre `try_run_native` | Sin genoma nuevo salvo promoción explícita a `action:dispatch-*` (fuera del default Ola 1) |
| Ejecutor skill/tool | Misma capa + wrappers tipados; fan-out delega aquí | Skills/tools siguen siendo tipos de cápsula |
| Handlers nativos `actions.rs` | Permanecen en el host orquestador **o** se mueven junto al ejecutor action **sin** duplicar negocio de dominio en otra cápsula | No forjar actions nuevas por handler existente |

**Prohibido:** processes `executor-*` multi-fase; `execute-event`; duplicar lógica de negocio fuera de cápsulas.

### 4.2 Contrato preservado (aduana)

Invariantes (clarify D5 / objectives O7):

- `capsule-json-io` (stdin/stdout JSON, `exitCode`, errores tipados).
- Peaje Cerbero / contextos de ejecución en fases del proceso.
- Telemetría / peaje termodinámico fail-soft.
- Resolución `./sddia-run.sh` → binario orquestador; flags `--process` / `--tool` / `--action` / `--forge` **sin** cambio de semántica pública.
- Envelope de proceso (`success`, `execution_report`, …) intacto para `--process`.

Entrypoints CLI cortos (`--tool`, `--action`) **pueden** seguir residiendo en `main.rs` del orquestador; el cuerpo de despacho se delega al ejecutor (el peaje de proceso completo no aplica a esos entrypoints cortos — paridad con hoy).

## 5. Estrategia de migración

| Regla | Aplicación |
|-------|------------|
| Big-bang | Prohibido |
| Unidad | Por familia / sub-familia (`common` → `action` → `tool` → `skill` wrappers → fan-out) |
| Gate de retirada del camino monolítico | Paridad verificable **antes** de borrar el camino concentrado equivalente |
| Facades | Durante transición, `engine::capsules::{invoke_*}` pueden reexportar la nueva API hasta que consumidores internos se retargeteen en el mismo hito |

Orden canónico:

1. **E0** — Inventario sellado en expediente (este `spec.md` + `plan.md`).
2. **E1** — Extraer capa común (resolve/spawn/JSON/fallback).
3. **E2** — Extraer despacho `action` + paridad O3.
4. **E3** — Extraer despacho `tool` (+ gates) + paridad O5 (`capsule-invoke-smoke` / CLI `--tool`).
5. **E4** — Wrappers `skill` (`git-manager`, `shell-executor`) + fan-out `try_invoke_delegates` + paridad O4.
6. **E5** — Retarget consumidores densos a facades estables; limpiar monolito; evolution; higiene comentarios bridge Python.

## 6. Paridad (criterios de verificación)

| ID | Familia | Camino representativo mínimo | Medio |
|----|---------|------------------------------|-------|
| O3 | action | `./sddia-run.sh --action <emit-*\|purge-*>` **o** sello `emit-pr-presented-event` vía delivery | Smoke CLI / fase caliente; `success` + `exitCode` idénticos pre/post |
| O4 | skill | `invoke_git_manager` en init/delivery **o** `invoke_shell_executor` acotado | Smoke/proceso ya verde en rama; sin cambio de envelope |
| O5 | tool | `process: capsule-invoke-smoke` (`tool:io-choke`) y/o `./sddia-run.sh --tool …` | Golden D-P5.2 + CLI |

No se exige golden histórico Python; oráculo = comportamiento pre-extracción en la misma rama.

## 7. Criterios de aceptación (S+)

| ID | Criterio | Verificación |
|----|----------|--------------|
| CA-1 | Inventario Ola1 vs diferidos documentado | `spec.md` §3 + `implementation.md` |
| CA-2 | Despacho físico no concentrado en monolito `capsules.rs` único | Diff: capa/ejecutores extraídos; orquestador solo aduana + llamadas |
| CA-3 | Aduana intacta (Cerbero, telemetría, grafo, `execution_report`, `sddia-run.sh`) | Smokes proceso + grep de peajes pre-fase |
| CA-4 | Paridad action ≥1 | O3 |
| CA-5 | Paridad skill ≥1 | O4 |
| CA-6 | Paridad tool ≥1 | O5 |
| CA-7 | Sin lógica de dominio nueva fuera de cápsulas; sin genoma espurio | Revisión Diff + DA-2 |
| CA-8 | CLI consumidores sin reescritura | Diff touchpoints host limitado a facades internas |
| CA-9 | Trazabilidad MAN-30 ↔ AUT-77 / `PBI-9e2df289` | Frontmatter expediente + issue Linear |
| CA-10 | Evolution si mutación material Core | `SddIA/evolution/` |
| CA-11 | Cierre | Un PR + `validacion.md` APTO + issue en `state_map.done` (`provider: linear`) |

## 8. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Regresión Linear gates dentro de `invoke_tool_for_process` | Extraer gates **con** el wrapper tool; no “simplificar” en el corte |
| `try_invoke_delegates` homogeniza skill/tool vía `invoke_tool_capsule_json` | Preservar semántica; label solo diagnóstico |
| Blast radius crate nuevo | Preferir módulo interno primero o crate lib sin binario; un PR incremental |
| Confundir consumidores EDA con el corte | Retarget llamadas; no mover `route_domain_core` |
| Docs históricos citan `execute-action.py` | Etiquetar inventario vacío; no inventar poda de fichero inexistente |

## 9. Dependencias normativas / topología

- `SddIA/core/cumulo.paths.json` — `compiled_capsules`, `execution_capsules`, directorios genoma.
- `SddIA/norms/capsule-json-io.md`, contratos actions/skills/tools/process.
- `features-documentation-pattern` v1.3.1 bajo `persist_ref`.
- Git solo `skill:git-manager`.
- Genoma: mutaciones indexadas solo vía `entity-manager` / proceso autorizado (esta ola **no** exige entidades nuevas por defecto).

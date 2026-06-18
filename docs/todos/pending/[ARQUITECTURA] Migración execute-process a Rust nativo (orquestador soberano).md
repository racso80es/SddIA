---
document_id: PBI-MIGRACION-EXECUTE-PROCESS-RUST
title: "[ARQUITECTURA] Migración execute-process.py a Rust nativo (orquestador soberano)"
format: markdown
version: "1.0.0"
created: "2026-06-18"
status: "abierto"
priority: alta
process: feature
related:
  - docs/features/migracion-rust-wasi/spec.md
  - docs/features/refactor-execute-process-engine/objectives.md
  - SddIA/norms/external-ai-constraints.md
  - SddIA/norms/capsule-json-io.md
  - SddIA/Cargo.toml
---

# [ARQUITECTURA] Migración `execute-process.py` a Rust nativo (orquestador soberano)

## 1. Contexto Arquitectónico y Restricción de Sustrato

La migración estratégica hacia Rust/WASI (`docs/features/migracion-rust-wasi`) transmutó con éxito las habilidades (*skills*) y herramientas (*tools*) a cápsulas `wasm32-wasip1`. No obstante, excluyó deliberadamente el componente central de orquestación, dejando el intérprete dinámico de Python (`execute-process.py`) como una deuda técnica residual y el último consumidor activo de `PyYAML` en el entorno de ejecución del negocio.

Este PBI liquida dicha deuda mediante la reescritura íntegra del orquestador a **Rust nativo**, preservando estrictamente la paridad funcional y el contrato de invocación canónico ya certificado. El nuevo componente se consolida como la Aduana Universal de Ejecución del ecosistema SddIA.

### Restricción Crítica del Sustrato (Lección D8)
El orquestador tiene la responsabilidad de realizar la forja física y el *spawning* de subprocesos dinámicos (invocaciones a `wasmtime run`, comandos de sistema `git`, llamadas a herramientas externas). El estándar WASI (`wasm32-wasip1`) no soporta operaciones de *subprocess spawning* con herencia parametrizada de directorios o entornos (`--dir=.`).

Por lo tanto, **el target obligatorio de este componente es un binario nativo de Rust**, homólogo a la arquitectura de los centinelas (*daemons*), haciendo uso estricto de `std::process::Command`. Esto garantiza que la frontera entre el Core y la Instancia, así como el contrato estricto de E/S JSON (`SddIA/norms/capsule-json-io.md`), permanezcan inmaculados.

## 2. Superficie a Portar (Mapeo de Paridad Estricta)

La transición desde el entorno interpretado al entorno compilado preservará la modularidad actual mediante la siguiente distribución de responsabilidades en el Crate orquestador:

| Componente Python Actual | Responsabilidad Operativa | Destino Arquitectónico Rust |
| :--- | :--- | :--- |
| `SddIA/scripts/qa/execute-process.py` | Interfaz de línea de comandos (CLI), parseo de flags (`--process`, `--inputs`, `--inputs-file`), lectura de `stdin` y gestión del ciclo de vida base. | `main.rs` e interfaz de entrada (vía crate `clap` o parseo directo libre de entropía). |
| `execute_process_core.py` | Carga y parseo de frontmatter YAML de procesos, resolución canónica de nombres, alias, validación de definiciones y normalización estructural de requerimientos (`load_process_def`). | Módulo `core::parser` y `core::resolver`. |
| `execute_process_capsules.py` | Maquinaria de ejecución de fases secuenciales, invocación dinámica de cápsulas WASM y cálculo del Peaje Termodinámico (emisión de telemetría). | Módulo `engine::executor` y `engine::capsules`. |
| `execute_process_forges.py` | Forjas físicas especializadas por categoría funcional (`tool` / `action` / `process`). | Módulo `forges::factory`. |
| Cores Satélite (`route_domain_event_core`, `kalma2_interact_core`, etc.) | Manejadores específicos de flujos inyectados durante el ciclo de ejecución de la fase. | Submódulos dedicados bajo `engine::handlers::*`. |
| `env_loader.py` | Resolución jerárquica de archivos de entorno y secretos locales. | Módulo `core::env` (utilizando inyección explícita). |

### Reutilización de Activos S+ Grade
Queda prohibida la duplicación de estructuras de datos e I/O. El nuevo orquestador debe integrarse directamente dentro del Cargo Workspace central (`SddIA/Cargo.toml`) para compartir de manera nativa el árbol de compilación (`target/`) y reutilizar de forma obligatoria:
- El crate de abstracción de datos e intercambio universal `SddIA/sddia-io`.
- Los esquemas estructurados de sobres de error y éxito (`success`, `exitCode`, `feedback`, `result`).

## 3. Aislamiento y Gobernanza (Preservación del Desacoplamiento)

### Directriz de Ceguera Espacial
Para impedir el acoplamiento y el colapso de responsabilidades, los centinelas locales (`event-watcher`, `telegram-watcher`) y las interfaces materiales (Cliente Kalma2) mantendrán su naturaleza de **Despertadores Inertes**. El orquestador **no se integrará como una librería compilada dentro de los daemons**; se mantendrá como un **binario independiente desacoplado**.

El contrato de invocación (`--process <name> --inputs <json>` → línea JSON final en `stdout` con `success`/`exitCode`) **no cambia**: los consumidores siguen operando sin reescritura lógica, sustituyendo únicamente el ejecutable invocado.

```text
Centinelas (Rust nativo)        Git Hooks / Wrapper / Cliente
 event-watcher, telegram-watcher  hook_common, sddia-run.sh, kalma2
        │                                  │
        └──────────── invocan ─────────────┘
                       ▼ (std::process::Command — binario desacoplado)
        [ orquestador execute-process ]  ← este PBI: Python → Rust nativo
                       │  JSON stdin/stdout (capsule-json-io)
        ┌──────────────┼───────────────┐
        ▼              ▼               ▼
   wasmtime run    git-manager     execute-action
   (skills.wasm)   (subprocess)    (tools / acciones)
```

## 4. Actualizaciones de Uso del Nuevo Fichero (Touchpoints físicos)

| Consumidor | Fichero | Cambio |
|------------|---------|--------|
| Centinela eventos | `SddIA/daemons/event-watcher/src/main.rs` | Sustituir `python_bin() + execute-process.py` por invocación al binario orquestador (`std::process::Command`, desacoplado) |
| Centinela Telegram | `SddIA/daemons/telegram-watcher/src/main.rs` | Idem |
| Lanzadores daemon | `SddIA/scripts/daemons/*.{sh,bat}`, `_exec_daemon.py`, `_launch.sh` | Apuntar al binario nativo / runtime |
| Aduana local (hooks) | `SddIA/scripts/qa/git-hooks/hook_common.py` (`invoke_process`) | Actualizar invocación |
| Wrapper laboratorio | `sddia-run.sh` | Reemplazar `python … execute-process.py` por `cargo run`/binario; reevaluar venv + `pip install` |
| Cliente Kalma2 | `.SddIA/client/sddia-client-bridge.py` (`invoke_engine`) | Invocar binario orquestador en lugar de subprocess Python |
| QA / E2E lab | `SddIA/scripts/qa/run-eda-e2e-lab.py`, `route_domain_event_core.py` | Actualizar referencias de invocación |

## 5. Documentación Viva a Actualizar (excluir histórica)

Solo documentos **operativos vigentes** (no `docs/todos/done/`, no `SddIA/evolution/`, no features cerradas como `refactor-execute-process-engine`, que se citan como antecedente sin mutar):

| Documento | Cambio |
|-----------|--------|
| `README.md` | §«Aduana Universal (CLI)» y ejemplos `python SddIA/scripts/qa/execute-process.py …` → invocación binario nativo |
| `SddIA/norms/external-ai-constraints.md` | DA-3: vía canónica de invocación del orquestador (tabla de invocaciones obligatorias) |
| `SddIA/norms/capsule-json-io.md` | Confirmar contrato I/O del orquestador nativo (reuso, sin ruptura) |
| Contratos de proceso que citan el CLI | `SddIA/process/feature.md`, `bug-fix.md`, `delivery-close-cycle.md`, `route-domain-event.md` (vía proceso autorizado — **no** bisturí directo sobre genoma) |
| Skill dependiente | `SddIA/skills/intent-transpiler.md` (referencias de ejecución) |
| Feature antecedente | Cerrar el *out of scope* declarado en `docs/features/migracion-rust-wasi/spec.md` §2 (anotar en la feature de este PBI, no reescribir la cerrada) |

> **Genoma protegido (DA-2):** cambios en `SddIA/process/*`, `skills/*`, `norms/*` se ejecutan vía `entity-manager`/proceso autorizado, nunca por escritura directa del IDE.

## 6. Elementos Adicionales a Contemplar

- **PyYAML / `requirements.txt`:** tras el porte, el orquestador deja de requerir PyYAML. Reevaluar eliminación de `requirements.txt` (condicional `migracion-rust-wasi/clarify.md` D3): mantener solo si scripts QA residuales (`verify-process-integrity.py`, `execute-action.py`, `audit-doc-parity.py`) aún lo consumen.
- **Safety net (anti-panic):** todo `panic!`/`unwrap` debe capturarse y devolverse como envelope JSON `success:false`, `exitCode>0`. El fallo del orquestador no debe romper el formato esperado por centinelas/agentes (paridad con `OPERATIVO-PBI-Migracion-Rust-WASI` §3).
- **Peaje Termodinámico:** preservar emisión de `Raw_Execution_Finished` (telemetría) y eventos de orquestación post-éxito; fail-soft D3.13 intacto.
- **Kill-switch (CEN-03):** mantener opt-in `SDDIA_ENGINE_KILL_SWITCH` y semántica one-shot vs supervisor.
- **CI/CD:** añadir `cargo build`/`cargo test` del crate orquestador; smoke E2E (`run-eda-e2e-lab`, `eda-bus-e2e-smoke`) verdes con el binario nativo.

## 6.bis Deudas Residuales P5 (cápsulas `wasmtime` nativas)

El hito P5 cerró la invocación nativa de cápsulas (`engine::capsules`), portó `delivery-close-cycle` a Rust (`engine::delivery_close`) y habilitó `try_invoke_delegates` para fases `skill:`/`tool:`. Golden **13/13** verde (incl. `capsule-invoke-smoke`). Las tres deudas explícitas quedaron **liquidadas** en el commit de cierre D-P5.1/2/3.

### D-P5.1 — `invoke_action` aún vía `execute-action.py` (subprocess Python)

**Estado:** ✅ **cerrado.** `engine::actions` implementa handlers nativos para `emit-pr-presented-event`, `emit-pr-merged-event`, `emit-pr-audited-event` y **`emit-domain-mutation`** (`engine::domain_mutation`). `invoke_action` intenta: nativo → cápsula wasm/native → bridge `execute-action.py` (solo red de seguridad para acciones no portadas).

**Criterio de cierre cumplido:** fase «Sello Presentación ECST» de `delivery-close-cycle` con `status:executed` sin *spawn* de `execute-action.py`; golden `delivery-close-cycle` verde.

> [!IMPORTANT]
> **Deuda residual explícita (D-P5.1-R) — `execute-action.py` permanece como fallback para acciones no portadas.**
>
> `engine::actions::try_run_native` cubre el camino caliente EDA/cierre: `emit-pr-*` y **`emit-domain-mutation`** (Sello Universal). El resto del catálogo `SddIA/actions/` **sigue ejecutándose vía bridge `execute-action.py`** (subprocess Python). Esto es **intencional** (red de seguridad), no un olvido, pero **mantiene vivo un consumidor del runtime Python** y bloquea la retirada del intérprete (P17).
>
> **Inventario de acciones (estado de portado):**
>
> | Acción | Estado Rust | Notas |
> |--------|-------------|-------|
> | `emit-pr-presented-event` | ✅ nativo | `engine::actions::emit_pr_presented` |
> | `emit-pr-merged-event` | ✅ nativo | `engine::actions::emit_pr_merged` |
> | `emit-pr-audited-event` | ✅ nativo | `engine::actions::emit_pr_audited` |
> | `emit-domain-mutation` | ✅ nativo | `engine::domain_mutation` + `ecst_validation` + `eda_coverage` |
> | `emit-suite-execution-requested` | 🔶 bridge Python | sin portar |
> | `sync-entity-index` | 🔶 bridge Python | muta índices (genoma) |
> | `materialize-kaizen-alert-doc` | 🔶 bridge Python | materializa doc |
> | `materialize-fracture-pbi` | 🔶 bridge Python | materializa PBI |
> | `enrich-fracture-pbi-kaizen` | 🔶 bridge Python | enriquecimiento PBI |
> | `policy-validator` | 🔶 bridge Python | validación de políticas |
> | `crypto-broker` | ✅ nativo | `engine::crypto_broker` → `cryptography-manager` |
>
> **Artefactos `emit-domain-mutation` nativo:**
> - `engine/domain_mutation.rs` — validación inputs, ensamblaje ECST, idempotencia, persistencia `pending/`
> - `engine/ecst_validation.rs` — aduana ECST (REQUIRED/OPTIONAL/FORBIDDEN desde `SddIA/events/`)
> - `engine/eda_bus.rs` — `find_existing_domain_event` (idempotencia)
> - `engine/eda_coverage.rs` — upsert/remove `eda-coverage.json` (SSOT correlación)
>
> **Deudas de ecosistema ajenas al handler (siguen abiertas):** cableado de invocadores en `*-creator`, suscripciones `Domain_Entity_*` con fan-out efectivo, contexto RBAC Cerbero en runtime. El handler cumple contrato `emit-domain-mutation.md` v1.1.0; tests unitarios `emit_domain_mutation_*` verdes.
>
> **Gate residual P17:** portar acciones de materialización/sync restantes; retirar `invoke_action_python_bridge` cuando el inventario 🔶 quede vacío.

### D-P5.2 — Fases `feature`/`bug-fix` con `skill:`/`tool:` caen a `simulated`

**Estado:** ✅ **cerrado (golden).** Proceso lab `capsule-invoke-smoke` invoca `tool:io-choke` nativo; golden verifica fase `executed` con `handler:capsule-tool-io-choke`. Fases sin cápsula compilada siguen `simulated` con `note` explícita (`capsula ausente` vs `agentes IDE`).

**Criterio de cierre cumplido:** golden `capsule-invoke-smoke` 13/13; matriz de delegados documentada en `implementation.md` §7.2.

### D-P5.3 — Resolución de cápsulas vía `SddIA/target` (no declarada en `cumulo.paths.json`)

**Estado:** ✅ **cerrado (Opción A).** Clave `compiled_capsules` en `cumulo.paths.json` (`native_root`, `wasm_root`, `profiles`). Módulo `engine::capsule_paths` (Rust) y `capsule_resolve.py` (Python) consumen el mismo SSOT con fallback release→debug y wasm→native.

**Criterio de cierre cumplido:** test unitario `loads_compiled_capsule_roots_from_cumulo`; paridad Rust↔Python en resolución de artefactos.

## 6.ter Deudas Residuales tras porte `emit-domain-mutation`

El porte nativo de `emit-domain-mutation` (`engine::domain_mutation` + `ecst_validation` + `eda_bus` + `eda_coverage`) cerró el Sello Universal EDA en Rust. Quedan dos frentes de deuda explícita.

### D-P6T.1 — `execute-action.py` aún es fallback de acciones no portadas

`engine::actions::try_run_native` cubre `emit-pr-*` y `emit-domain-mutation`. El resto del catálogo `SddIA/actions/` **sigue resolviéndose vía bridge `execute-action.py`** (subprocess Python), deliberadamente como red de seguridad:

| Acción | Estado | Notas |
|--------|--------|-------|
| `emit-suite-execution-requested` | 🔶 bridge Python | emisión de evento de suite |
| `sync-entity-index` | 🔶 bridge Python | muta `index.md` (genoma) |
| `materialize-kaizen-alert-doc` | 🔶 bridge Python | materializa doc Kaizen |
| `materialize-fracture-pbi` | 🔶 bridge Python | materializa PBI de fractura |
| `enrich-fracture-pbi-kaizen` | 🔶 bridge Python | enriquece PBI de fractura |
| `policy-validator` | 🔶 bridge Python | validación de políticas |
| `crypto-broker` | ✅ nativo | `engine::crypto_broker` → `cryptography-manager` |

**Criterio de cierre:** portar las acciones 🔶 a `engine::actions` (o cápsula nativa); retirar `invoke_action_python_bridge` cuando el inventario quede vacío; grep sin `execute-action.py` desde el crate.

**Gate:** P17 (poda). Mantener el bridge mientras exista cualquier acción 🔶.

### D-P6T.2 — Deudas de ecosistema fuera del handler `emit-domain-mutation`

El handler nativo cumple el contrato `emit-domain-mutation.md` v1.1.0, pero la consciencia EDA del genoma **no es efectiva end-to-end** hasta resolver el cableado de ecosistema:

- **Cableado en `*-creator`:** los procesos de forja (`process-creator`, `tool-creator`, `skill-creator`, …) no invocan `emit-domain-mutation` tras la mutación física del artefacto. Sin esta llamada, las creaciones/updates no sellan evento de dominio.
- **Fan-out `Domain_Entity_*`:** los tipos `Domain_Entity_{Created,Updated,Deleted}` requieren suscriptores efectivos en `event-domain-subscriptions.json`; mientras no existan, `route-domain-event` documenta no-op y mueve a `processed/`.
- **Cerbero runtime:** el gate RBAC por `context: ecosystem-evolution` está declarado en `execution-contexts.md` pero su aplicación efectiva en el runtime del orquestador (no solo en el contrato) sigue pendiente.

**Criterio de cierre:** invocadores `*-creator` emiten el sello tras forja; suscripciones `Domain_Entity_*` con fan-out real; gate Cerbero aplicado en runtime. Cada sub-deuda puede cerrarse en hito independiente (no bloquea el porte del handler).

**Gate:** ninguno técnico sobre el handler (ya nativo y testeado); depende de procesos de forja y topología de suscripciones.

## 7. Criterios de Aceptación (Protocolo de Acero)

- [ ] Ciclo `feature` completo bajo `persist_ref` (spec, clarify, plan, implementation, validacion) — Argos APTO.
- [ ] Binario Rust nativo del orquestador compila en el workspace (`cargo build`) sin warnings lógicos.
- [ ] Paridad funcional estricta: batería de procesos (`feature`, `bug-fix`, `route-domain-event`, `delivery-close-cycle`, `entity-manager`) produce el mismo envelope que la versión Python.
- [ ] Centinelas, hooks, `sddia-run.sh` y cliente Kalma2 invocan el binario nativo desacoplado y operan E2E.
- [ ] Ningún flujo de **orquestación** requiere intérprete Python ni PyYAML; `requirements.txt` reevaluado/podado según D3.
- [ ] Errores devueltos como JSON válido (`exitCode>0`), sin panic crudo en stdout.
- [ ] Documentación viva (`README.md`, `external-ai-constraints.md`, contratos vía proceso autorizado) refleja el nuevo orquestador.
- [x] **Deudas P5 liquidadas (§6.bis):** (a) `invoke_action` nativo sin `execute-action.py` [D-P5.1]; (b) golden de fase `skill:`/`tool:` `executed` con cápsula presente [D-P5.2]; (c) resolución de artefactos de cápsula desde SSOT única (Rust↔Python) [D-P5.3].
- [ ] **Deudas post-`emit-domain-mutation` (§6.ter):** (a) portar acciones 🔶 restantes y retirar `invoke_action_python_bridge` [D-P6T.1]; (b) cableado `*-creator`, fan-out `Domain_Entity_*` y Cerbero runtime [D-P6T.2].
- [ ] Este TODO movido a `docs/todos/done/` en el mismo PR (cierre documental en rama).

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

El hito P5 cerró la invocación nativa de cápsulas (`engine::capsules`), portó `delivery-close-cycle` a Rust (`engine::delivery_close`) y habilitó `try_invoke_delegates` para fases `skill:`/`tool:`. Golden 12/12 verde. No obstante, persisten tres deudas explícitas que **no bloquean** el gate maestro P9 pero **sí condicionan** la poda final (P17). Se detallan aquí como unidades de trabajo cerradas.

### D-P5.1 — `invoke_action` aún vía `execute-action.py` (subprocess Python)

**Estado actual:** `engine::capsules::invoke_action` hace *spawn* de `python3 SddIA/scripts/qa/execute-action.py --action <name> --inputs <json>`. Las acciones `action:*` (p. ej. `emit-pr-presented-event`, `emit-domain-mutation`, `emit-pr-merged-event`) se ejecutan en el intérprete Python, no como cápsula `wasm`/nativa directa.

**Por qué es deuda:** mantiene un consumidor activo del runtime Python en el camino caliente de orquestación (`delivery-close-cycle`, fan-out EDA). Contradice el objetivo de soberanía total del orquestador y mantiene vivo `execute-action.py` + sus dependencias (`PyYAML` transitivo).

**Objetivo:** portar la invocación de acciones a cápsula directa, replicando la resolución de `execute_process_capsules.invoke_capsule_action`:
- Resolver la cápsula `action:<name>` vía `capsule_resolve` (wasm32-wasip1 → nativo) en lugar de `execute-action.py`.
- Mapear el contrato de E/S de la acción (envelope `SddiaResponse`) con `unwrap` equivalente al actual `invoke_capsule_action` (devuelve `data` en éxito; error en `success:false`).
- Conservar `_ACTIVE_CAPSULE_CAPTURE_STATE` (last_capsule_id / envelope) si el Peaje Termodinámico lo requiere para el recibo de telemetría.

**Criterio de cierre:** `emit-pr-presented-event` (fase «Sello Presentación ECST» de `delivery-close-cycle`) se ejecuta sin *spawn* de `execute-action.py`; golden `delivery-close-cycle` sigue verde con la acción nativa; grep sin referencias a `execute-action.py` desde el crate Rust.

**Gate:** requiere que las acciones `emit-*` existan como cápsula compilada (`SddIA/target`). Mientras no existan, el bridge a `execute-action.py` es la red de seguridad (no retirar).

### D-P5.2 — Fases `feature`/`bug-fix` con `skill:`/`tool:` caen a `simulated`

**Estado actual:** en `engine::executor::execute_phase`, las fases con delegados `skill:`/`tool:` que **no** resuelven a una cápsula compilada bajo `SddIA/target` se marcan `simulated` (vía `try_invoke_delegates` → `None` → rama `simulated`). Es **paridad exacta** con el comportamiento Python (`execute_process_capsules.execute_phase`), pero no es ejecución física real.

**Por qué es deuda:** las fases productivas de `feature`/`bug-fix` (Diseño, Ejecución, Verificación) delegan en `agent:*` (IDE) y en cápsulas que en laboratorio no tienen binario; el orquestador no las ejecuta de verdad. Es correcto para el lab actual, pero el cierre real exige que, cuando la cápsula exista, se invoque.

**Objetivo:**
- Confirmar que `try_invoke_delegates` invoca la cápsula cuando está compilada (ya implementado) y documentar la matriz de delegados soportados (`skill:filesystem-manager`, `skill:git-manager` ya nativo, `tool:*`).
- Añadir caso golden con una fase `skill:`/`tool:` **con cápsula presente** que produzca `executed` (no `simulated`), verificando paridad de `phase_report` con Python.
- Distinguir explícitamente en el `phase_report` el motivo del `simulated`: `"capsula ausente"` vs `"agentes IDE"` (ya diferenciado en `note`).

**Criterio de cierre:** test/golden que demuestre una fase `tool:` ejecutada nativamente (`status:executed`, `handler:capsule-tool-<name>`); documentación de la matriz de delegados en `implementation.md`.

**Gate:** depende de disponer de al menos una cápsula `tool:`/`skill:` de fase compilada para el fixture. No bloquea P9 (gate maestro) porque la paridad `simulated`↔`simulated` ya es verde.

### D-P5.3 — Resolución de cápsulas vía `SddIA/target` (no declarada en `cumulo.paths.json`)

**Estado actual:** `engine::capsules::resolve_capsule_{wasm,native}` y su homólogo `capsule_resolve.py` buscan binarios bajo rutas **cableadas**: `SddIA/target/wasm32-wasip1/{release,debug}/<name>.wasm` y `SddIA/target/{release,debug}/<name>`. `cumulo.paths.json` declara `execution_capsules` (directorios de *fuente* `skills/`, `tools/`, `daemons/`) pero **no** las rutas de *artefactos* compilados.

**Por qué es deuda:** viola parcialmente el principio SSOT (Protocolo §3: rutas validadas contra `cumulo.paths.json`). La ruta de artefactos compilados es convención implícita compartida Rust↔Python; un cambio de layout de `target/` rompería ambos sin punto único de verdad.

**Objetivo (a decidir vía proceso autorizado, genoma DA-2):**
- Opción A: añadir a `cumulo.paths.json` una clave `compiled_capsules` (`{ wasm_root, native_root }`) y consumirla desde `engine::capsules` y `capsule_resolve.py` (paridad SSOT).
- Opción B: documentar formalmente la convención de `SddIA/target` como contrato estable en `capsule-json-io.md` / norma de build, aceptándola como invariante de workspace Cargo.
- Mantener el fallback de perfiles `release`→`debug` y wasm→native en cualquier caso.

**Criterio de cierre:** Rust y Python resuelven la ruta de artefactos desde la misma fuente de verdad (clave en `cumulo.paths.json` **o** contrato documentado citado por ambos); sin rutas mágicas divergentes.

**Gate / soberanía:** si se elige la Opción A, la mutación de `cumulo.paths.json` se realiza vía `entity-manager`/proceso autorizado (no bisturí directo sobre genoma).

## 7. Criterios de Aceptación (Protocolo de Acero)

- [ ] Ciclo `feature` completo bajo `persist_ref` (spec, clarify, plan, implementation, validacion) — Argos APTO.
- [ ] Binario Rust nativo del orquestador compila en el workspace (`cargo build`) sin warnings lógicos.
- [ ] Paridad funcional estricta: batería de procesos (`feature`, `bug-fix`, `route-domain-event`, `delivery-close-cycle`, `entity-manager`) produce el mismo envelope que la versión Python.
- [ ] Centinelas, hooks, `sddia-run.sh` y cliente Kalma2 invocan el binario nativo desacoplado y operan E2E.
- [ ] Ningún flujo de **orquestación** requiere intérprete Python ni PyYAML; `requirements.txt` reevaluado/podado según D3.
- [ ] Errores devueltos como JSON válido (`exitCode>0`), sin panic crudo en stdout.
- [ ] Documentación viva (`README.md`, `external-ai-constraints.md`, contratos vía proceso autorizado) refleja el nuevo orquestador.
- [ ] **Deudas P5 liquidadas (§6.bis):** (a) `invoke_action` nativo sin `execute-action.py` [D-P5.1]; (b) golden de fase `skill:`/`tool:` `executed` con cápsula presente [D-P5.2]; (c) resolución de artefactos de cápsula desde SSOT única (Rust↔Python) [D-P5.3].
- [ ] Este TODO movido a `docs/todos/done/` en el mismo PR (cierre documental en rama).

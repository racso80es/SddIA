---
feature_name: kaizen-rust-capsule-structure
created: "2026-06-15"
process: refactorization
branch_name: feat/kaizen-rust-capsule-structure
persist_ref: docs/features/kaizen-rust-capsule-structure
pbi_ref: docs/todos/pending/kaicen Estructura de Cápsulas Rust.md
impacts_doc: true
---

# Especificación técnica — Kaizen Estructura de Cápsulas Rust

## 1. Contexto

El ecosistema SddIA mantiene **duplicidad termodinámica**: cápsulas Rust canónicas bajo `SddIA/skills/` y `SddIA/tools/`, pero el SSOT (`cumulo.paths.json`) y el runtime laboratorio (`execute_process_capsules.py`) siguen resolviendo `execution_capsules` hacia `scripts/skills/` y `SddIA/scripts/tools/` con artefactos Python. Los centinelas operan solo como scripts `.py`/`.sh` en `SddIA/scripts/daemons/` sin binarios Rust en `SddIA/daemons/{name}/`.

**Objetivo:** consolidar la capa ejecutable en Rust nativo/WASI, eliminar el camino operativo Python para skills, tools y daemons, y alinear contratos + Cúmulo.

**Orden de consolidación obligatorio (decisión D8):**

```text
1. Skills  →  2. Tools  →  3. Daemons
```

Cada ola cierra cableado SSOT + retirada de shims Python **antes** de iniciar la siguiente.

## 2. Estado actual vs objetivo

### 2.1 `execution_capsules` (Cúmulo hoy)

```json
{
  "skills": "scripts/skills/",
  "tools": "scripts/tools/",
  "daemons": "SddIA/scripts/daemons/"
}
```

### 2.2 `execution_capsules` (objetivo)

```json
{
  "skills": "SddIA/skills/",
  "tools": "SddIA/tools/",
  "daemons": "SddIA/daemons/"
}
```

Cada entidad ejecutable reside en `{capsule_root}/{name}/` con `Cargo.toml` + artefacto compilado. El launcher resuelve:

| Sustrato | Artefacto | Invocación |
|----------|-----------|------------|
| WASI skill/tool | `target/wasm32-wasip1/release/{name}.wasm` | `wasmtime run --dir=. {wasm}` |
| Nativo skill (fallback git) | `target/release/{name}` | subprocess directo stdin/stdout JSON |
| Daemon nativo | `target/release/{name}` | proceso long-running; sin stdin orquestación |

### 2.3 Inventario por ola

#### Ola 1 — Skills (`scripts/skills/` → `SddIA/skills/`)

| name | Python legacy | Rust canónico | Invocador lab actual |
|------|---------------|---------------|----------------------|
| `bus-operator` | `bus-operator.py`, `bus-operator.sh` | `SddIA/skills/bus-operator/` | `execute_process_capsules`, `route_domain_event_core` |
| `git-manager` | `git-manager.py` | `SddIA/skills/git-manager/` | `invoke_git_manager` → WASI + fallback `.py` |
| `shell-executor` | `shell-executor.py` | `SddIA/skills/shell-executor/` | `invoke_shell_executor` → WASI + fallback `.py` |
| `cryptography-manager` | — | `SddIA/skills/cryptography-manager/` | `crypto()` WASI + fallback `.py` |

Skills solo documentales (sin cápsula en esta iteración): `filesystem-manager`, `intent-transpiler`, `text-metrics`.

#### Ola 2 — Tools (`SddIA/scripts/tools/` → `SddIA/tools/`)

| name | Python legacy | Rust canónico |
|------|---------------|---------------|
| `io-choke` | `io_choke.py` | `SddIA/tools/io-choke/` |
| `markdown-table-editor` | `markdown_table_editor.py` | `SddIA/tools/markdown-table-editor/` |
| `manage-event-receipt` | `manage_event_receipt.py` | `SddIA/tools/manage-event-receipt/` |
| `read-event-subscriptions` | `read_event_subscriptions.py` | `SddIA/tools/read-event-subscriptions/` |
| `sandbox-breacher` | `sandbox_breacher.py` | `SddIA/tools/sandbox-breacher/` |
| `schema-corruptor` | `schema_corruptor.py` | `SddIA/tools/schema-corruptor/` |
| `send-telegram-notification` | `main.py`, `telegram_api.py` | `SddIA/tools/send-telegram-notification/` |
| `telegram-gateway` | `main.py`, `transmute.py` | `SddIA/tools/telegram-gateway/` |
| `transit-event-payload` | `transit_event_payload.py` | `SddIA/tools/transit-event-payload/` |
| `iota-immutable-publisher` | Node + `install-deps.sh` | `SddIA/tools/iota-immutable-publisher/` (Rust) |

#### Ola 3 — Daemons (`SddIA/scripts/daemons/` → `SddIA/daemons/{name}/`)

| name | Python/shim legacy | Genoma `.md` | Estado Rust |
|------|-------------------|--------------|-------------|
| `event-watcher` | `.py`, `.sh`, `.bat` | `event-watcher.md` | **A forjar** |
| `telegram-watcher` | `.py`, `.sh`, `.bat` | `telegram-watcher.md` | **A forjar** |
| `github-bridge-watcher` | `.py`, `.sh` | `github-bridge-watcher.md` | **A forjar** |
| `event-sweeper` | `.py`, `.sh` | no indexado | **A forjar** + indexar |

Runtime compartido hoy: `daemon_centinel_runtime.py` (heartbeat, lock) — lógica a portar a crate común `sddia-daemon-runtime` o módulo interno.

## 3. Contrato I/O (invariante)

Todas las cápsulas skills/tools obedecen:

- Norma: `SddIA/norms/capsule-json-io.md` (schema v2.0)
- Contratos familia: `skills-contract.md`, `tools-contract.md`
- Coherencia: `exitCode === 0` ⟺ `success === true`

Daemons **no** usan capsule-json-io en operación normal; emiten ECST al bus (`daemons-contract.md` §8).

## 4. Cambios en contratos (K2)

### 4.1 `skills-contract.md`

| Sección | Cambio |
|---------|--------|
| §2 Consciencia espacial | `execution_capsules.skills` → `SddIA/skills/{name}/`; prohibido `scripts/skills/` en nuevas entregas |
| §4 Sustrato | Jerarquía: WASI release → nativo release → *(retirar)* fallback Python tras Ola 1 |
| §4 Excepción temporal | Acotar a `git-manager` solo hasta Ola 1 cerrada; luego nativo o WASI con `wasi:cli` |

### 4.2 `tools-contract.md`

| Sección | Cambio |
|---------|--------|
| §3 / §7 | `implementation_path_ref` resuelve a `SddIA/tools/{name}/`; `execution_capsules.tools` alineado |
| §7 | Eliminar referencia implícita a `scripts/tools/` |

### 4.3 `daemons-contract.md`

| Sección | Cambio |
|---------|--------|
| §3 | Artefactos bajo `SddIA/daemons/{name}/target/release/{name}` |
| §4 `execution.entrypoint` | Ejemplo: `SddIA/daemons/event-watcher/target/release/event-watcher` |
| §4 `execution.runtime` | `native-rust` (sustituye `python3` en centinelas migrados) |
| §2 Ceguera | Aclarar: **subprocess CLI inerte** (`execute-process`, `route-domain-event`) permitido como delegación OS sin lectura de genoma; prohibida orquestación lógica inline |

## 5. Ola 1 — Skills (detalle)

### 5.1 Resolución de launcher

Función canónica `resolve_skill_capsule(repo, name)` en runtime (lab → futuro Rust):

```text
1. SddIA/target/wasm32-wasip1/release/{name}.wasm  (si wasmtime + wasm)
2. SddIA/skills/{name}/target/release/{name}       (nativo)
3. ERROR — sin fallback Python tras cierre Ola 1
```

### 5.2 Mutaciones runtime lab (`execute_process_capsules.py`)

| Función | Cambio |
|---------|--------|
| `invoke_git_manager` | Eliminar `_invoke_git_manager_native`; rutas desde `resolve_skill_capsule` |
| `invoke_shell_executor` | Idem |
| `crypto()` / bus-operator | Idem; retirar referencias a `scripts/skills/*.py` |
| `CAPSULE_SKILL_REGISTRY` | Apuntar a resolución Cúmulo |

### 5.3 Poda Ola 1

Retirar del camino operativo (no borrar hasta verificación):

- `scripts/skills/bus-operator.py`, `bus-operator.sh`
- `scripts/skills/git-manager.py`
- `scripts/skills/shell-executor.py`

### 5.4 Criterios de salida Ola 1

| ID | Criterio |
|----|----------|
| SK-CA1 | `cumulo.execution_capsules.skills` = `SddIA/skills/` |
| SK-CA2 | `invoke_git_manager` + `shell-executor` + `bus-operator` sin `.py` en `scripts/skills/` |
| SK-CA3 | Tests lab existentes (`test_execute_suite`, EDA smoke) pasan |
| SK-CA4 | Workspace-init de `refactorization` opera solo con skills Rust/WASI |

## 6. Ola 2 — Tools (detalle)

### 6.1 Resolución

`resolve_tool_capsule(repo, name)` — misma jerarquía WASI → nativo; sin `.py`.

Invocadores: `invoke_chaos_tool_capsule`, `route_domain_event_core` (IOTA), `telegram_gateway_core`, tests chaos.

### 6.2 `cumulo.paths.json`

`execution_capsules.tools`: `SddIA/tools/`

### 6.3 Poda Ola 2

Retirar árbol operativo `SddIA/scripts/tools/` (excepto `node_modules` iota si CI aún lo exige — deuda Kaizen CI WASI).

### 6.4 Criterios de salida Ola 2

| ID | Criterio |
|----|----------|
| TL-CA1 | `cumulo.execution_capsules.tools` = `SddIA/tools/` |
| TL-CA2 | Chaos tools + IOTA + telegram invocados vía WASM/nativo |
| TL-CA3 | Sin invocación a `SddIA/scripts/tools/*.py` en `execute_process_capsules` |

## 7. Ola 3 — Daemons (detalle)

### 7.1 Estructura crate

```text
SddIA/daemons/
  event-watcher/
    Cargo.toml
    src/main.rs
  telegram-watcher/
  github-bridge-watcher/
  event-sweeper/
  sddia-daemon-runtime/   # crate compartido (lock, heartbeat, state cursor)
```

Dependencias recomendadas: `tokio`, `notify` (event-watcher), `reqwest` (telegram/github), `serde_json`.

### 7.2 Comportamiento invariante (port desde Python)

| Centinela | Estímulo físico | Emisión / delegación |
|-----------|-----------------|----------------------|
| `event-watcher` | FS poll `eda_bus.pending` | Subprocess `execute-process --process route-domain-event` **o** import core solo en lab; preferir subprocess en Rust |
| `telegram-watcher` | Long-poll Telegram API | ECST domain (`Manual_Task_Requested`, `Kaizen_Idea_Captured`) |
| `github-bridge-watcher` | Webhook/poll GitHub | ECST domain (`PullRequest_Presented`, etc.) |
| `event-sweeper` | Scan processed/dead-letter | Purga `pending/` padre; sin orquestación |

### 7.3 `execution` en frontmatter

Actualizar cada `SddIA/daemons/{name}.md`:

```yaml
execution:
  entrypoint: "SddIA/daemons/event-watcher/target/release/event-watcher"
  runtime: "native-rust"
  heartbeat_interval_seconds: 30
```

Recalcular `hash_signature` §7 contrato.

### 7.4 Indexar `event-sweeper`

Forja vía `daemon-creator` / `entity-manager`: `event-sweeper.md` + fila en `index.md`.

### 7.5 Criterios de salida Ola 3

| ID | Criterio |
|----|----------|
| DM-CA1 | `cumulo.execution_capsules.daemons` = `SddIA/daemons/` |
| DM-CA2 | `governance-daemon-manager` arranca binarios Rust |
| DM-CA3 | `event-watcher --once` + `daemon-heartbeat-audit` APTO |
| DM-CA4 | Sin `.py` operativo en `SddIA/scripts/daemons/` |

## 8. Poda final y certificación (K6)

| Acción | Detalle |
|--------|---------|
| Retirar shims | `SddIA/scripts/tools/`, `scripts/skills/`, `SddIA/scripts/daemons/` del índice operativo |
| E2E | `run-eda-e2e-lab.py`, `test_chaos_immunity_eda.py`, `verify-process-integrity` |
| V1 | Smoke en contenedor/entorno sin `python3` para skills+tools+daemons migrados |
| V2 | Peaje termodinámico en ejecución refactorization/feature con binarios Rust |
| V3 | Documentar matriz única Rust (+ WASM) en `implementation.md` |

## 9. Fuera de alcance

- Migración `SddIA/scripts/qa/` (intérprete `execute-process.py`).
- Purga completa Node `iota-immutable-publisher/node_modules`.
- Sustitución WASM de todos los binarios nativos.
- Forja manual de genoma `.md` sin `entity-manager`.

## 10. Criterios de aceptación globales

| ID | Criterio | Ola |
|----|----------|-----|
| V1 | Core crítico sin Python | K6 |
| V2 | Telemetría + peaje correctos | K6 |
| V3 | Matriz Rust única para skills/tools/daemons | K6 |
| SK-CA* | Skills consolidados | Ola 1 |
| TL-CA* | Tools consolidados | Ola 2 |
| DM-CA* | Daemons consolidados | Ola 3 |
| DOC | `validacion.md` APTO + PBI en `done/` | Cierre |

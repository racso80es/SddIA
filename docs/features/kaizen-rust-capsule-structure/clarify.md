---
feature_name: kaizen-rust-capsule-structure
created: "2026-06-15"
process: refactorization
purpose: Kaizen — erradicación entropía Python y transición a cápsulas Rust S+ Grade
updated: "2026-06-15"
---

# Clarificación — Kaizen Estructura de Cápsulas Rust

Transcript de decisiones (2026-06-15) para el PBI Kaizen sobre migración Python → Rust.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`refactorization`** v1.2.0 (Kaizen arquitectónico; paridad documental con `feature`) |
| Nombre operativo | **kaizen-rust-capsule-structure** |
| Rama | `feat/kaizen-rust-capsule-structure` ✅ |
| `persist_ref` | `docs/features/kaizen-rust-capsule-structure` |
| Manifiesto | `docs/todos/pending/kaicen Estructura de Cápsulas Rust.md` |
| Contrato I/O | `SddIA/norms/capsule-json-io.md` (envelope stdin → result/feedback/exitCode stdout) |

---

## D2 — Proceso vs feature

| Pregunta | Decisión |
|----------|----------|
| ¿Por qué `refactorization` y no `feature`? | Sin capacidad funcional nueva; purga de entropía y consolidación tecnológica |
| ¿Paridad con `feature`? | **Sí** — cadena V5, `features-documentation-pattern`, fase 1 `workspace-init` ejecutada |
| Beta laboratorio | Fase 1 **ejecutada** vía `execute-process.py`; fases 2–6 **simuladas** (agentes IDE) |

---

## D3 — Estado termodinámico actual (auditoría Fase 1)

### Tools

| Entidad | Python legacy (`SddIA/scripts/tools/`) | Rust canónico (`SddIA/tools/`) |
|---------|----------------------------------------|----------------------------------|
| io-choke | `io_choke.py` | `io-choke/` ✅ |
| markdown-table-editor | `markdown_table_editor.py` | `markdown-table-editor/` ✅ |
| manage-event-receipt | `manage_event_receipt.py` | `manage-event-receipt/` ✅ |
| read-event-subscriptions | `read_event_subscriptions.py` | `read-event-subscriptions/` ✅ |
| sandbox-breacher | `sandbox_breacher.py` | `sandbox-breacher/` ✅ |
| schema-corruptor | `schema_corruptor.py` | `schema-corruptor/` ✅ |
| send-telegram-notification | `main.py`, `telegram_api.py` | `send-telegram-notification/` ✅ |
| telegram-gateway | `main.py`, `transmute.py` | `telegram-gateway/` ✅ |
| transit-event-payload | `transit_event_payload.py` | `transit-event-payload/` ✅ |
| iota-immutable-publisher | Node/TS + `install-deps.sh` | `iota-immutable-publisher/` (Rust) ✅ |

### Skills

| Entidad | Python legacy (`scripts/skills/`) | Rust canónico (`SddIA/skills/`) |
|---------|-----------------------------------|----------------------------------|
| bus-operator | `bus-operator.py` + `bus-operator.sh` | `bus-operator/` ✅ |
| git-manager | `git-manager.py` | `git-manager/` ✅ |
| shell-executor | `shell-executor.py` | `shell-executor/` ✅ |
| cryptography-manager | — | `cryptography-manager/` ✅ |
| filesystem-manager | — | solo `.md` (sin cápsula) |
| intent-transpiler | — | solo `.md` (sin cápsula) |

### Daemons (centinelas)

| Entidad | Python/shim legacy (`SddIA/scripts/daemons/`) | Genoma (`SddIA/daemons/`) |
|---------|-----------------------------------------------|---------------------------|
| event-watcher | `.py`, `.sh`, `.bat` | `event-watcher.md` (sin binario Rust) |
| telegram-watcher | `.py`, `.sh`, `.bat` | `telegram-watcher.md` |
| github-bridge-watcher | `.py`, `.sh` | `github-bridge-watcher.md` |
| event-sweeper | `.py`, `.sh` | no indexado en `daemons/` |

### Núcleo QA / orquestación (`SddIA/scripts/qa/`)

| Área | Artefactos | Nota |
|------|------------|------|
| Intérprete procesos | `execute-process.py`, `execute_process_capsules.py`, cores | **Deuda separada** — fuera del alcance inmediato de poda `scripts/tools/skills/daemons` |
| Route/bus EDA | `route_domain_event_core.py`, `route_fractal_event_core.py` | Invocados por centinelas; migración coordinada con daemons |
| Gates/hooks | `git-hooks/`, `verify-*.py` | Mantener hasta sustituto Rust o WASM |

---

## D4 — Topología objetivo (PBI §2–3)

| Destino | Contenido |
|---------|-----------|
| `SddIA/tools/{name}/` | `Cargo.toml` + binario capsule-json-io |
| `SddIA/skills/{name}/` | Idem para skills con lógica ejecutable |
| `SddIA/daemons/{name}/` | Binario Rust concurrente (notify/tokio) + `{name}.md` |
| `cumulo.paths.json` | `execution_capsules` apunta a binarios compilados, no `.py` |
| **Poda** | Retirar `SddIA/scripts/tools/`, `scripts/skills/`, `SddIA/scripts/daemons/` del camino operativo |

---

## D5 — Contratos a adecuar (Fase 2)

| Contrato | Archivo |
|----------|---------|
| Daemons | `SddIA/daemons/daemons-contract.md` |
| Tools | `SddIA/tools/tools-contract.md` |
| Skills | `SddIA/skills/skills-contract.md` |

Ampliar con requisitos de cápsula Rust, rutas de binario, telemetría y dogma Ceguera Espacial (daemons invocan CLI inerte: `execute-process` / `route-domain-event`).

---

## D6 — Fuera de alcance explícito (esta iteración)

- Migración completa de `SddIA/scripts/qa/` a Rust (intérprete de procesos).
- Purga de `iota-immutable-publisher/node_modules` y toolchain Node (deuda Kaizen CI WASI ya parcialmente cerrada).
- WASM runtime sustituto de todos los binarios nativos (futuro; matriz Rust + WASM).

---

## D7 — Criterios de aceptación (PBI §4)

| ID | Criterio |
|----|----------|
| V1 | Core operativo (repos, IOTA, orquestación EDA) sin Python instalado |
| V2 | Telemetría y peaje termodinámico correctos en Aduana Universal con binarios Rust |
| V3 | Matriz única Rust (+ WASM futuro); eliminación cerebro dividido scripting |

---

## D8 — Orden de consolidación Python (decisión operador)

| Pregunta | Decisión |
|----------|----------|
| ¿Secuencia de poda? | **Skills → Tools → Daemons** (innegociable) |
| ¿Por qué skills primero? | Base de invocación del runtime lab (`git-manager`, `shell-executor`, `bus-operator`); sin skills Rust el resto del pipeline no es auditable |
| ¿Por qué tools antes que daemons? | Daemons y EDA dependen de tools (IOTA, telegram-gateway, chaos); tools consolidados antes de transmutar centinelas |
| ¿Gate entre olas? | SK-CA* → TL-CA* → DM-CA* (`spec.md` §5.4, §6.4, §7.5) |

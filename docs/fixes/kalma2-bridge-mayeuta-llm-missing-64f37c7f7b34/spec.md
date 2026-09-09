---
feature_name: kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
created: "2026-09-09"
process: bug-fix
base: main
scope: kalma2-bridge-mayeuta-llm-prosthetic-preflight
version_spec: "1.0.0"
branch_name: fix/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
persist_ref: docs/fixes/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
pbi_ref: docs/todos/pending/[FIX] kalma2-bridge — fractura sistémica (64f37c7f7b34).md
document_id: PBI-FIX-FRACTURE-64f37c7f7b34
execution_id: "aee88988-ea8f-434d-a0bd-8bc67e5748b9"
incident_ref: "System_Fracture_Detected — 64f37c7f7b34"
fracture_hash: "64f37c7f7b34"
---

# Especificación — prótesis `mayeuta-llm` ausente (`64f37c7f7b34`)

## Diagnóstico

| Campo | Valor |
|-------|-------|
| Sello | `64f37c7f7b34` (`SHA-256[:12]` de la traza) |
| Emisor | `kalma2-bridge` `handle_chat` (`POST /api/chat`) |
| `fracture_kind` | `prosthetic_collapse` |
| `attempted_action` | `sse_chat_stream` |
| Traza literal | `mayeuta-llm no encontrado en SddIA/target/{debug,release}` |

`resolve_mayeuta_llm` exige magia `\x7fELF` en override `SDDIA_MAYEUTA_LLM_BIN` o en `SddIA/target/{debug,release}/mayeuta-llm`. Fallo de candidatos default → `emit_system_fracture` + HTTP 500 con esa `message`. **No** es 404 `/api/aiua/interact`. **No** es `exit 1` de `cbe0c30b3695`. **No** existe `friction_id` en el payload.

Causa estructural: `kalma2-bridge` no declara crate-dependencia de `mayeuta-llm`; el lanzador solo garantiza el ELF del puente. Host limpio / `cargo clean` → el demonio arranca y colapsa al primer chat.

## Corrección de esta ola

### H1 — Host (no git)

`cargo build --release -p mayeuta-llm` → `SddIA/target/release/mayeuta-llm` ELF ejecutable. `target/` gitignored.

### H2 — Orden release-first

`resolve_mayeuta_llm`: candidatos default **release luego debug**, paridad con `_sddia_resolve_daemon_binary`. Override `SDDIA_MAYEUTA_LLM_BIN` intacto (si set y no ELF → traza distinta, otro sello). Sin chequeo fósil mtime en satélites.

### H3 — Preflight lanzador (WARN, no kill)

`kalma2-bridge.sh` avisa en stderr **antes** del `exec` si no hay ELF de `mayeuta-llm` (override o `target/{release,debug}`), con `cd SddIA && cargo build --release -p mayeuta-llm`. **No `exit 1`:** el puente sirve estáticos, `/api/status` y `/api/aiua/interact`.

### H4 — Test hermético

Tempdir + magia `\x7fELF` y/o override env. Caso ausente = traza literal. Sin depender de `SddIA/target` real. Comando: `cargo test --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml --bin kalma2-bridge -- resolve_mayeuta_llm`.

### H5 — No-regresión de ESTE sello

`POST /api/chat` con `prompt` no vacío **no** responde 500 cuya `message` sea la traza literal. No exigir stream LLM.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| KALMA-LLM-CA1 | ELF ejecutable `SddIA/target/release/mayeuta-llm` (host; no artefacto git) |
| KALMA-LLM-CA2 | Resolver prioriza release sobre debug; override env intacto |
| KALMA-LLM-CA3 | Test hermético de resolución + traza literal |
| KALMA-LLM-CA4 | Lanzador WARN sin `exit 1` si falta prótesis |
| KALMA-LLM-CA6 | Con prótesis resoluble, `POST /api/chat` no 500 con la traza de este sello |
| KALMA-LLM-CA-CI | Checks GitHub Actions del PR verdes con `run_id` antes de `global: APTO` y `accept-pr` |

KALMA-LLM-CA5 (cubo léxico Kaizen) **diferido**. No gate.

## Alcance prohibido

| Prohibido | Motivo |
|-----------|--------|
| Mutar `SddIA/skills/mayeuta-llm` (salvo compile) | DA-2 genoma |
| Frontend `interfaces/kalma2/app.js` | Fuera de sello |
| `/api/aiua/interact`, recycle 404, sanitización 503 | Otros PBI |
| Alinear `resolve_orchestrator` | Mismo olor; otro sello |
| Retry/backoff en handler | DA-5 |
| `kalma2-chat-sse-live-smoke.sh` como gate | Arranca debug propio; exige inferencia |
| Predicado «cero fracturas en pending» | Cola compartida/volátil |
| Inventar `friction_id` / `F-*` | Alucinación v1.1.0 |
| Alterar `fracture_hash` o la traza literal | A-FRACTURE-HASH-INMUTABLE |

## Corte de esta fase (Dedalo)

Diseño: `spec.md` + `plan.md`. Commit de planificación. Ejecución = Tekton bajo el mismo `persist_ref`.

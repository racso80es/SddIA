---
feature_name: kalma2-bridge-aiua-interact-stale-elf
created: "2026-09-09"
process: bug-fix
base: main
scope: kalma2-bridge-stale-elf-documentary-seal
version_spec: "1.0.0"
branch_name: fix/kalma2-bridge-aiua-interact-stale-elf
persist_ref: docs/fixes/kalma2-bridge-aiua-interact-stale-elf
pbi_ref: docs/todos/pending/[FIX] kalma2-bridge ELF release fósil — POST api-aiua-interact 404.md
document_id: PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF
execution_id: "4ccdf721-f1d4-40e1-b7ca-40002e9d5da5"
incident_ref: "Auditoría empírica WUI 2026-09-09 — 404 ruta desconocida en /api/aiua/interact"
---

# Especificación — ELF fósil `kalma2-bridge` (CA-6)

## Diagnóstico

| Síntoma | Evidencia |
|---------|-----------|
| WUI `#aiua-pulse` → `[error] ruta desconocida` | Auditoría `docs/audits/kalma2-wui-tormentosa-chat-20260909.md` ~15:08 CEST |
| `POST /api/aiua/interact` HTTP 404 | Cuerpo `{"success":false,"message":"ruta desconocida","exit_code":1}` |
| Estáticos frescos / binario fósil | `GET /` lee `interfaces/kalma2/` del disco; match POST vive en el ELF en memoria |
| PID 6151 | Unidad `sddia-kalma2-bridge@home-racso-Proyectos-SddIA.service`; arranque 2026-09-07 14:34:55 CEST |
| ELF release mtime 2026-09-06 06:48:40 CEST | Anterior a PR #276 (head `492c673` 09:14 CEST; merge `4b212aec` 09:27 CEST) |
| Código HEAD ya tenía la ruta | `dispatch` `(Method::Post, "/api/aiua/interact")` en `main.rs` |

**Causa raíz:** split-brain ciclo de vida. `_sddia_resolve_daemon_binary` compara mtime ELF ≥ fuente **al spawn**. Un proceso ya instanciado no reevalúa. Tras merge, faltó `cargo build --release -p kalma2-bridge` + `systemctl --user restart`.

**No es defecto de `main.rs`.** No falta guarda en el lanzador.

## Estado físico (pre-ciclo, 2026-09-09 15:17–15:18 CEST)

Remediación host ya ejecutada: rebuild release + restart unidad. PID `1244544`, ELF mtime 15:17:48 CEST, probe ≠404 (HTTP 500/81s por 503 Gemini = entrada a handler). CA-1…CA-5 del PBI marcados APTO in situ.

Este ciclo **no** reescribe el puente. Sella **CA-6** (archivo documental) y re-verifica que el órgano vivo sigue ≠404.

## Corrección de este sello

### H1 — Verificar órgano vivo (sin rebuild salvo fósil)

MainPID systemd, `strings` ELF (`handle_aiua_interact` / `aiua/intera`), `POST /api/aiua/interact` ≠404, `POST /api/chat` operativo. Rebuild+restart **solo** si el predicado de frescura falla. Prohibido compile en hot-path (`A-NO-AUTO-COMPILE-HOT-PATH`).

### H2 — Cascada documental

`spec.md` (este), `plan.md`, `implementation.md`, `execution.md`, `validacion.md` bajo `persist_ref`. Incluir auditoría empírica si sigue untracked.

### H3 — Archivo PBI (CA-6)

Mover `docs/todos/pending/[FIX] kalma2-bridge ELF release fósil — POST api-aiua-interact 404.md` → `docs/todos/done/` conservando `document_id`. `validacion.md`: `global: APTO` (CI = `PENDIENTE-CI` hasta run verde), `pbi_archived: true`. Prohibido `pbi_archived: true` con PBI en `pending/`.

### H4 — Evolution + PR único

`sddia-qa evolution-register`. Diff del PR: persist_ref + PBI en `done/` + evolution + auditoría. **Cero** diffs de `SddIA/interfaces/kalma2-bridge/` y `interfaces/kalma2/app.js` por este sello. DCC → PR. CA-CI con `run_id` verde. Luego `accept-pr`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| KALMA-STALE-CA1 | ELF del MainPID / disco: mangled `handle_aiua_interact` y substring `aiua/intera` |
| KALMA-STALE-CA2 | `POST /api/aiua/interact` ≠404 `ruta desconocida`. 500/503 proveedor cumple |
| KALMA-STALE-CA3 | 1ª pulsación `#aiua-pulse` no escribe `[error] ruta desconocida` (equivalente: probe HTTP ≠404; live WUI no gate si probe cubre) |
| KALMA-STALE-CA4 | `POST /api/chat` operativo post-recycle (no-regresión enrutamiento) |
| KALMA-STALE-CA5 | Filename plano del PBI (sin `/` en basename); hipervínculos lógicos, no `file://` de host |
| KALMA-STALE-CA6 | PBI en `docs/todos/done/`; `pbi_archived: true`; PR **sin** diffs `kalma2-bridge`/`app.js` |
| KALMA-STALE-CA-CI | Checks GitHub Actions del PR verdes con `run_id` antes de `global: APTO` definitivo y `accept-pr` |

## Alcance prohibido

| Prohibido | Motivo |
|-----------|--------|
| Mutar `kalma2-bridge` `main.rs` / WUI `app.js` por este sello | Ruta ya en `main` vía PR #276; `A-NO-DUPLICATE` de entrega |
| Inventar guarda mtime en el lanzador | `_sddia_resolve_daemon_binary` ya existe |
| Retry/backoff ante 503 Gemini | DA-5; PBI-OPERATIVO-KALMA2-AIUA-503-SANITIZE |
| Failover multi-LLM | PBI-MULTI-LLM-ROUTER (kitchen) |
| Fractura `64f37c7f7b34` (`mayeuta-llm`) | PBI hermano; endpoint distinto |
| `pgrep -f` como PID canónico | Wrappers del shell; usar MainPID systemd |
| Enlaces `file:///home/racso/...` | Agnosticismo Core |

## Corte de esta fase (Dedalo)

Diseño: `spec.md` + `plan.md`. Commit de planificación. Ejecución = Tekton bajo el mismo `persist_ref`.

---
document_id: PBI-KALMA2-LLM-LIVE-V2
uuid: f0f1b1ec-4b79-47c6-85e2-a0ac2ca3164b
title: "[FEATURE] kalma2-llm-live — Interacción S+ Grade y Prótesis de Ejecución"
format: markdown
version: "2.3.2"
created: "2026-07-20"
refined: "2026-07-20"
status: "done"
priority: alta
process: feature
suggested_feature_name: kalma2-llm-live
suggested_branch: feat/kalma2-llm-live
pr_url: https://github.com/racso80es/SddIA/pull/123
depends_on:
  - docs/features/kalma2-full-cycle
baseline_delivered:
  - docs/features/kalma2-event-bus-integration
  - docs/features/kalma2-mayeuta-llm-router
  - docs/features/kalma2-process-dispatch
related:
  - SddIA/skills/mayeuta-llm/
  - SddIA/interfaces/kalma2-bridge/
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.sh
  - interfaces/kalma2/
  - .dev/.env.example
  - docs/features/kalma2-llm-live/
  - docs/features/kalma2-llm-live/runbook-infer.md
  - docs/features/kalma2-llm-live/runbook-agent.md
  - docs/features/kalma2-llm-live/runbook-sqlite.md
inherited_from: docs/todos/done/[FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre (527007fa).md
supersedes: "v2.3.0 — reabre en-curso para ejecutar deuda host (cursor-agent live) en este mismo PBI/PR"
evidence:
  - "2026-07-20 lab APTO: SSE/execute/STREAM/SQLite/smokes S1–S5; PR #123"
  - "2026-07-20 host APTO: CLI+login; CHAT_STREAM backend=cli; AGENT_PHASE executed cli; HOST-D omitido"
---

# [FEATURE] kalma2-llm-live — Interacción S+ Grade y Prótesis de Ejecución (v2.3.2)

## Estado

**v2.3.2 — Done (lab + host A–C).** PR [#123](https://github.com/racso80es/SddIA/pull/123). Merge pendiente operador.

### Ya forjado (lab — no reabrir)

| Tramo | Evidencia |
|-------|-----------|
| `/api/chat` SSE + watchdog + fractura | bridge + `kalma2-sse-fracture-smoke.sh` |
| `/api/execute` → `Kalma2_Process_Requested` | bridge + mode execute |
| `mayeuta-llm` STREAM | skill + smokes |
| Dual-mode `.py` + SQLite lab | smokes S3/S4 |
| UI Chat / Forjar | `interfaces/kalma2/` |
| Runbooks infer/agent/sqlite | `docs/features/kalma2-llm-live/runbook-*.md` |

## 1–8 (resumen normativo)

Laudos L-EP…L-WAL, AC1–AC9 y fases 1–8a: ver v2.3.0 / `docs/features/kalma2-llm-live/`.  
**Gate Done de este PBI:** lab APTO **y** §9 Host Live cerrado (o laudo Racso de alcance reducido explícito).

## 9. Deuda host — a realizar en este PBI (obligatoria)

> Alcance de instancia/host, no genoma. Se ejecuta en la **misma** rama/PR `#123`. No abrir PBI paralelo.

### HOST-A — Instalar Cursor Agent CLI

| Campo | Detalle |
|-------|---------|
| Síntoma | `command -v cursor-agent` / `agent` → vacío; bóveda `cursor-agent --print` no resuelve |
| Acción | `curl https://cursor.com/install -fsS \| bash` · `export PATH="$HOME/.local/bin:$PATH"` · `agent --version` · auth si aplica (`agent login`) |
| Ref | `docs/features/kalma2-llm-live/runbook-infer.md` |
| Criterio | `cursor-agent --print` o `agent --print` responde a prompt corto con stdout no vacío |
| Evidencia 2026-07-20 | CLI `2026.07.17-3e2a980`; `agent login` → `racso80es@gmail.com`; smoke `cursor-agent --print --mode ask --trust` → `ok` |

- [x] HOST-A instalado (CLI en PATH)
- [x] HOST-A auth (`agent login`)

### HOST-B — Chat live (cierra S1/S2 · AC6 real)

| Campo | Detalle |
|-------|---------|
| Acción | Bóveda `.dev/.env` (no versionar): `SDDIA_LLM_INFER_COMMAND=cursor-agent --print` (o path absoluto); `SDDIA_LLM_REQUIRE_INFER=1`; **unset** `SDDIA_LLM_CHAT_MOCK` / `SDDIA_AGENT_RUNTIME_MOCK`; reiniciar bridge |
| Smoke | UI Chat o `POST /api/chat` → meta `[kalma2-meta] {"backend":"cli",...}` y tokens **≠** `sqlite-ack` / `[infer-lab]` |
| Criterio | AC6 live: backend cli real |
| Evidencia 2026-07-20 | Bóveda con paths + `--trust` + `REQUIRE_*=1`. Prótesis `CHAT_STREAM` → meta `backend=cli`, cuerpo `pong` (≠ sqlite-ack / infer-lab) |

- [x] HOST-B completado (prótesis live; reiniciar bridge si UI SSE aún con bóveda vieja)

### HOST-C — Agent phases live (cierra S3 · AC7 real)

| Campo | Detalle |
|-------|---------|
| Acción | `SDDIA_AGENT_RUNTIME_COMMAND=SddIA/scripts/tools/kalma2-agent-runtime-cursor.sh`; `SDDIA_AGENT_RUNTIME_CLI=cursor-agent --print`; `SDDIA_AGENT_RUNTIME_REQUIRE_CLI=1`; daemons up |
| Smoke | UI Forjar Proceso / `POST /api/execute` → TQM → hijo → fase agent `executed` o handoff con `backend: cli` (no soft solo por CLI missing) |
| Ref | `runbook-agent.md` |
| Criterio | AC7 live |
| Evidencia 2026-07-20 | `AGENT_PHASE` live → `{"success":true,"data":{"status":"executed","backend":"cli",...}}` (~114s) |

- [x] HOST-C completado

### HOST-D — SQLite live opcional bajo L-WAL (refuerza AC8)

| Campo | Detalle |
|-------|---------|
| Acción | Cursor cerrado **o** copia `sqlite3 … .backup`; `SDDIA_CURSOR_VSCDB` + `SDDIA_CURSOR_SQLITE_WRITE=1`; chat una vez; verificar composer `Kalma2:` en DB/UI |
| Ref | `runbook-sqlite.md` |
| Criterio | Keys `composerData`/`bubbleId` en DB real o backup verificado |

- [ ] HOST-D completado (recomendado; no bloquea si A–C OK y lab AC8 ya APTO) — **omitido** (A–C OK; lab AC8 APTO)

### HOST-E — Cierre tras host

| Campo | Detalle |
|-------|---------|
| Acción | Actualizar `validacion.md`: `host_cursor_agent_live: APTO`; evidencia comandos; PBI → `docs/todos/done/` con checkboxes HOST-*; merge PR #123 |
| Criterio | Done = lab + host (A–C) en el mismo PR |

- [x] HOST-E completado (docs + archive; merge PR pendiente operador)

## 10. Plan de acción (host)

```text
1. HOST-A install CLI + PATH + version
2. HOST-B bóveda chat + smoke SSE live
3. HOST-C bóveda agent + execute E2E (daemons)
4. HOST-D (opc) SQLite live
5. HOST-E validacion + archive PBI + merge
```

## 11. Fuera de alcance (sin cambio)

- Disparo autónomo del agente UI solo por insert SQLite (L-IDE).
- Re-forjar ECST/TQM.
- Versionar secretos / `.dev/.env`.

## 12. Mandato

Ejecutar §9 HOST-A…E en `feat/kalma2-llm-live` / PR #123.  
Hasta HOST-A–C cerrados, **no** declarar Done total del PBI (lab solo = APTO parcial).

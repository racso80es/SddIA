---
feature_name: kalma2-llm-live
created: "2026-07-20"
process: feature
purpose: Runbook S3 — fases agent live desde Kalma2 execute (AC7)
---

# Runbook — Agent runtime live (S3)

## Objetivo

Tras `/api/execute` → EDA → TQM → hijo, las fases `agent:` no quedan en soft `awaiting_agents` **solo** por CLI ausente.

## Bóveda

```bash
# Producción Cursor CLI (comillas + --trust no-interactivo)
SDDIA_AGENT_RUNTIME_COMMAND=SddIA/scripts/tools/kalma2-agent-runtime-cursor.sh
SDDIA_AGENT_RUNTIME_CLI="/home/racso/.local/bin/cursor-agent --print --trust"
SDDIA_AGENT_RUNTIME_BACKEND=cli
# Demo live: no enmascarar CLI missing
SDDIA_AGENT_RUNTIME_REQUIRE_CLI=1
# unset SDDIA_AGENT_RUNTIME_MOCK
```

Instalar CLI + auth: ver `runbook-infer.md` (`curl https://cursor.com/install | bash` + `agent login` + `--trust`).

## Lab (cableado, sin Cursor)

```bash
SDDIA_AGENT_RUNTIME_COMMAND=SddIA/scripts/tools/kalma2-agent-runtime-lab.sh
SDDIA_AGENT_RUNTIME_LAB_AUTO=1   # handoff + status=executed
./SddIA/scripts/tools/kalma2-agent-phase-smoke.sh
```

## E2E Kalma2 (requiere Sistema Nervioso)

1. `start-sddia` / daemons up (`event-watcher`).
2. UI **Forjar Proceso** o `POST /api/execute` con prompt allowlist + `pbi_ref`.
3. Verificar PEC/`cycle_phase` y `_agent_handoff.md` con `backend: cli|sdk` o lab `executed`.

## Criterios AC7

| Check | OK |
|-------|-----|
| MOCK/LAB_AUTO → `status=executed` | smoke |
| CLI missing + `REQUIRE_CLI=1` → `failed` (no awaiting) | smoke |
| Host con `cursor-agent` → fase `executed` o handoff cli | live |

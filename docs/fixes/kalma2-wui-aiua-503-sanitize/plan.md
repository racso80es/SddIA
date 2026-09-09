---
feature_name: kalma2-wui-aiua-503-sanitize
created: "2026-09-09"
process: bug-fix
phases:
  - classify-sanitize-503
  - hermetic-tests
  - evolution-register
  - archive-pbi-validacion
  - delivery-close-cycle
  - confirm-ci
  - accept-pr
branch_name: fix/kalma2-wui-aiua-503-sanitize
persist_ref: docs/fixes/kalma2-wui-aiua-503-sanitize
execution_id: "8abff97c-9ce3-4d83-bdd8-4b08ac0d75d0"
---

# Plan — sanitización 503 Gemini en epidermis

Blueprint Tekton. Mutación acotada: `sanitize_bridge_message` + tests en `SddIA/interfaces/kalma2-bridge/src/main.rs`. Cero `app.js`. Cero `gemini-http-infer`.

## Fase 0 — Diseño (hecho, Dedalo)

- `spec.md` + este `plan.md` bajo `persist_ref`.
- Init: `execution_id` `8abff97c-9ce3-4d83-bdd8-4b08ac0d75d0`; rama `fix/kalma2-wui-aiua-503-sanitize`.
- PBI v1.2.0 (Filtro A: cifras 171/188/91; campo `message`; prefijo suficiente).
- Commit de planificación. Working tree sucio ajeno (`ecosystem-health.json`, PBI Telegram kitchen) **fuera** del commit.

## Fase 1 — Clasificador (CA1–CA3)

En `sanitize_bridge_message`, antes del recorte 240:

1. Si `raw` contiene `http-status-503` → devolver constante canónica.
2. Else si `"code":503` o `"code": 503` y marcador de indisponibilidad (case-insensitive) → canónico.
3. Else comportamiento actual (newlines + `take(240)`).

Prohibido `std::thread::sleep`, retry, backoff, polling.

## Fase 2 — Tests (CA4–CA5)

```bash
cargo test --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml --bin kalma2-bridge
```

Casos: fixture incidente; `http-status-503` sin UNAVAILABLE; `gemini-model-unavailable`; existente `"gemini 503"`.

## Fase 3 — Evolution + archivo

`sddia-qa evolution-register` (`modificacion` o `alta`). PBI `pending/` → `done/`. `validacion.md` con CA locales APTO y CA-CI `PENDIENTE-CI` hasta run verde.

## Fase 4 — DCC / PR

`./sddia-run.sh --process delivery-close-cycle` con `source_process: bug-fix`, `persist_ref`, `branch_name`.

## Fase 5 — CI + accept-pr

Un log de checks. Si rojo: un parche + un push (DA-6). Si verde: sellar `run_id` en `validacion.md` (`global: APTO`) y `./sddia-run.sh --process accept-pr`.

## Fuera

Retry 503; failover multi-LLM; mutar tool/frontend; fractura `64f37c7f7b34`; reciclo de daemon (ya fresco).

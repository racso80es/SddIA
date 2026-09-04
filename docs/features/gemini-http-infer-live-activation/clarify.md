---
feature_name: gemini-http-infer-live-activation
created: "2026-09-04"
process: feature
purpose: Estabilización Filtro A del PBI v1.2.0 tras init lab
version_clarify: "1.0.0"
execution_id: "0926e45d-db83-42ea-8a5b-3bafcdb00b57"
pbi_ref: "docs/todos/pending/[OPERATIVO] Activación live gemini-http-infer — catálogo de modelos y humo de instancia.md"
document_id: PBI-GEMINI-HTTP-INFER-LIVE-ACTIVATION
pbi_uuid: "d7f1f8aa-0d51-4b2d-ac27-ed17c8a23d09"
---

# Clarificación — gemini-http-infer-live-activation

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery + `SDDIA_LAB_ALLOW_DIRTY=1` (worktree ajeno). `execution_id` `0926e45d-db83-42ea-8a5b-3bafcdb00b57`. Rama `feat/gemini-http-infer-live-activation`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

## Decisiones (PBI v1.2.0, sin laudo nuevo)

| ID | Laudo |
|----|-------|
| L-MODEL | `request.model` gana; else `SDDIA_GEMINI_MODEL`; else error. Cero slug en Rust. |
| L-404 | `ureq::Error::Status` → body JSON; prefijo `gemini-model-unavailable` si catálogo. |
| L-EMPTY | 200 + text vacío → `success: false`. No gate `finishReason == STOP`. |
| L-SKILL-ARGV | Sin `--print` bare. `--output-format json` + `-p` + `--sandbox` default. |
| L-SKILL-AUTH | Substring `not logged into antigravity` ∪ `authentication required`. |
| L-FORGE | Crate = delivery de ED ya forjada. EM `update` genérico **prohíbe** (regenera UUID). Sello: `hash_refresh_only` solo si se toca `{name}.md`. |
| L-SKILL-INVOKE | Humo skill: ELF. SUCCESS live no gate. |
| L-SMOKE-MODEL | Slug del día; no `gemini-2.5-flash` si 404. |

## Fuera

Tiers LLM, `--skill` CLI, Vertex, CI contra Google/`agy` logueado.

---
feature_name: bundle-consumer-tormentosa-chain
created: "2026-09-25"
process: bug-fix
branch_name: fix/bundle-consumer-tormentosa-chain
persist_ref: docs/fixes/bundle-consumer-tormentosa-chain
pbi_document_id: PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN
pbi_ref: docs/todos/pending/[FIX] bundle consumer — cadena Tormentosa (13 ELF), llm-registry y active-domain-profile en instance-creator.md
execution_id: "4480e892-4647-48c5-a1de-934800e768d7"
---

# Especificación — bundle consumer cadena Tormentosa

## Diagnóstico

| ID | Síntoma | Causa |
|----|---------|-------|
| F-BUNDLE-TORMENTOSA-CHAIN | `POST /api/aiua/interact` → `cápsula tool 'llm-router' no encontrada` | `CONSUMER_BINS` = 9; crate `skills/` ignorado; `aiua-stimulus-processing` no se escanea |
| F-CREATOR-LLM-REGISTRY | `.SddIA/llm-registry.json` ausente post-creator | `materialize_*` solo cubre `local.paths.json` |
| F-CREATOR-DOMAIN-PROFILE | Autoridad software legado (`git_required: true`, slug null) | Creator no escribe `active-domain-profile.json` |
| F-SYSTEMD-EMAIL-LOGRATE | `sddia-email-watcher@` sin techo de log | Plantilla excluida de PR #291 |

## Diseño

1. **`build-release-bundle.sh`:** `CONSUMER_BINS` = 13 (`llm-router`, `gemini-http-infer`, `antigravity-cli-executor`, `thought-graph-access`). `_sddia_crate_root` + `local_pkgs` + `_scan_md_for_tools` cubren `SddIA/skills/`. Escanear `aiua-stimulus-processing.md`. Núcleo fail-closed: + `llm-router`, `thought-graph-access`.
2. **`instance-creator` v1.4.0 (entity-manager) + handler:** `materialize_llm_registry` (copia example si ausente); `materialize_domain_profile` solo `runtime_profile=consumer`, slug = `inputs.codex_slug` ≻ `MANIFEST.json.codex`, `git_required: false`. Nunca sobrescribir. Smoke informativo.
3. **Plantilla email:** `LogRateLimitIntervalSec=30s` / `LogRateLimitBurst=500`.

## CA

Ver PBI §4: CA-BINS, CA-SKILL-CRATE, CA-FAILCLOSED, CA-SMOKE-FILTRO-C, CA-CREATOR-REGISTRY, CA-CREATOR-PROFILE, CA-AUTHORITY, CA-EMAIL-LOGRATE, CA-RESOLVER, CA-BUILD, CA-GOV, CA-DOC.

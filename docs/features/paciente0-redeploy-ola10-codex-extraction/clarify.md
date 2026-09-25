---
feature_name: paciente0-redeploy-ola10-codex-extraction
created: "2026-09-25"
process: feature
purpose: Estabilización operativa ola 10; laudos Q1/Q2/Q3 ya cerrados
version_clarify: "1.0.0"
execution_id: "3098fa27-fb71-48e7-8ee2-aab2bee2c468"
pbi_ref: docs/todos/pending/[OPERATIVO] Paciente 0 SddIA_AP — redeploy ola 10 post-extracción del códice de ingeniería de software.md
document_id: PBI-OPERATIVO-PACIENTE0-REDEPLOY-OLA10-CODEX-EXTRACTION
pbi_uuid: "26df818b-f3f6-4a91-b228-84b065ad49df"
pbi_version: "1.1.0"
---

# Clarificación — paciente0-redeploy-ola10-codex-extraction

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `3098fa27-fb71-48e7-8ee2-aab2bee2c468`. Rama `feat/paciente0-redeploy-ola10-codex-extraction`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

Semilla: PBI operativo v1.1.0 + prompt DEUDA v1.6.0 §1 + delta §1 del operativo. CA-PREREQ cerrado (FIX PR #298, `CONSUMER_BINS=13`, `instance-creator` v1.4.0). Instancia ausente → wipe.

## Decisiones (cerradas; no reabrir)

| ID | Laudo |
|----|-------|
| Q1 | Tormentosa operativa: 13 ELF + registro + bóveda. `cápsula tool 'llm-router' no encontrada` = NO APTO G3 v2. |
| Q2 | `.SddIA/active-domain-profile.json` = `{"codex_slug":"codex-kalma2-assistant","git_required":false}`. Lo escribe creator v1.4.0. |
| Q3 | Ola 9 = errata documental. No citar su audit/Kaizen como SSOT. Ola 10 = nueva línea base. |
| L-NO-PROCESS | Prohibido forjar `paciente0-deploy`. Canal = bundle + `instance-creator` + `start-sddia.sh`. |
| L-NO-PATCH | Prohibido parchear `{instancia}/SddIA/`. Mutación de genoma = forja + PR. |
| L-OLA9 | Baseline de contraste = `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825` (done). Fricciones ola 9 absorbidas en Core (PR #292) se verifican en runtime. |
| L-VAULT-GEMINI | `CONFIG_SOURCE` tiene `GEMINI_API_KEY`, no `SDDIA_GEMINI_MODEL`. Staging instancia debe inyectar `SDDIA_GEMINI_MODEL` (oráculo fallback). Slug no se hardcodea en genoma. |
| L-FILTRO-C-ENV | Instancia omite `SDDIA_AGENT_RUNTIME_*` (5 claves en CONFIG_SOURCE). `SDDIA_RUNTIME_PROFILE=consumer`. `SDDIA_SENSORIAL_JURISDICTION=systemd`. |
| L-CONST | `PREPROD_VAULT` ausente. Staging materializa `constitution/` (`meta.product=SddIA_AP`) y `codexes/codex-kalma2-assistant` desde starter-kit / forja. Instancia, no git. |
| L-AGY | `agy` en host (`~/.local/bin/agy`). `SDDIA_AGY_PATH` no se escribe si el PATH del servicio lo resuelve. Ausente → G-agy-host degradado si G3 v2 pasa por `oracle-gemini`. |
| L-G5 | G5 First Blood opt-out (DA-5). Plano B DLT fuera. Chat `/api/chat` degradado esperado. |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo tras checks verdes. |
| L-DA5 | Tras acuse JSON de CLI: cero poll EDA / sleep. Ignición: wait interno de `start-sddia.sh` (no Tekton). |

## Fuera (este ciclo)

Forja `paciente0-deploy`. `mayeuta-llm` en bundle. ABSTRACT-04b poda histórica Core. Migración Paciente 0 a `.SddIA/projects/`. Anclaje retrospectivo ola 9.

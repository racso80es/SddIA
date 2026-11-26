---


feature_name: kaizen-env-safe-dotenv-load
created: "2026-11-26"
process: feature
branch_name: feat/kaizen-env-safe-dotenv-load
persist_ref: docs/features/kaizen-env-safe-dotenv-load
pbi_ref: AUT-120
tracker_ref: AUT-120
document_id: PBI-KAIZEN-ENV-SAFE-LOAD-01
execution_id: "f8cd9845-38a9-4173-a15a-5aa0287f212f"
---
# Objetivos — kaizen-env-safe-dotenv-load (AUT-120)

## Objetivo

Eliminar la fricción `F-ENV-BASH-SOURCE-STDIN-HANG`: carga literal de bóvedas `.env` en shell, diagnóstico preventivo sin fuga de secretos y escritura segura desde scripts.

## Criterios de aceptación

- [x] **KA-ENV-1:** `test-vault-literal-dotenv-load.sh` valida `_sddia_load_vault` con `|`, `&`, `$(...)`, espacios.
- [x] **KA-ENV-2:** `gate-env-dotenv-shell-hazard` reporta solo `archivo:línea:CLAVE` en plantillas versionadas.
- [x] **KA-ENV-3:** Gate audita scripts sin `source`/`.` sobre `.env`.
- [x] **KA-ENV-4:** `_sddia_quote_dotenv_value` / `_sddia_write_dotenv_kv` y guía en README + `.env.example`.
- [x] **KA-ENV-5:** Paso en `sddia-index-qa.yml` (gate + smoke shell) y cierre con `validacion.md` APTO.
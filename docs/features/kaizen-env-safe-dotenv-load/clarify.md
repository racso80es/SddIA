---
feature_name: kaizen-env-safe-dotenv-load
created: "2026-11-26"
process: feature
tracker_ref: AUT-120
execution_id: "246ab6e5-e75c-40f2-82e8-8385f3bbc040"
---

# Clarificación — kaizen-env-safe-dotenv-load (AUT-120)

| ID | Decisión |
|----|----------|
| **D-LOAD** | `_sddia_load_vault` es el único cargador shell canónico; proscripción de `source`/`.` sobre rutas `.env`. |
| **D-GATE** | `sddia-qa gate-env-dotenv-shell-hazard`: plantillas versionadas + audit de scripts; flag `--include-local-vault` solo operador (sin valores en salida). |
| **D-QUOTE** | Helpers `_sddia_quote_dotenv_value` / `_sddia_write_dotenv_kv` para escritura segura desde scripts. |
| **D-TEST** | `test-vault-literal-dotenv-load.sh` en `sddia-index-qa.yml` (job integrity). |

Fricción sellada: `F-ENV-BASH-SOURCE-STDIN-HANG`.
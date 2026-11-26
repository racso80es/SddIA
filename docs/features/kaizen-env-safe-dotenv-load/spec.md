---









feature_name: kaizen-env-safe-dotenv-load
created: "2026-11-26"
process: feature
tracker_ref: AUT-120
execution_id: "c8242b6c-0ea9-4797-a4f4-a211a3c0d5a3"
---
# Spec — AUT-120

## Touchpoints

| Componente | Ruta |
|------------|------|
| Cargador shell | `SddIA/scripts/common/sddia_shell_lib.sh` |
| Gate QA | `SddIA/tools/sddia-qa/src/gate_env_dotenv_shell_hazard.rs` |
| Smoke | `SddIA/scripts/qa/test-vault-literal-dotenv-load.sh` |
| CI | `.github/workflows/sddia-index-qa.yml` |
| Guía | `README.md`, `.dev/.env.example`, starter-kit `.env.example` |
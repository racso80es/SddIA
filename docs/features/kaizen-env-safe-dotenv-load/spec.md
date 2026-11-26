---
feature_name: kaizen-env-safe-dotenv-load
created: "2026-11-26"
process: feature
tracker_ref: AUT-120
execution_id: "246ab6e5-e75c-40f2-82e8-8385f3bbc040"
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
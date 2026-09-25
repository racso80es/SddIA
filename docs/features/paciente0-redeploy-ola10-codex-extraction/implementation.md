---
feature_name: paciente0-redeploy-ola10-codex-extraction
created: "2026-09-25"
process: feature
items:
  - vault-staging
  - bundle-13
  - instance-creator
  - systemd-fallback
  - overlay-conscience
  - gates
  - audit-deuda
---

# Implementación

| Touchpoint | Cambio |
|------------|--------|
| `/home/racso/Proyectos/SddIA_AP.deploy-vault` | Staging Filtro C + `SDDIA_GEMINI_MODEL` + constitution/codexes. Fuera de git. |
| `SddIA/scripts/build-release-bundle.sh` | Rsync `SddIA/conscience`. ONBOARDING §7. |
| `SddIA/scripts/qa/test-build-release-bundle-filtro-c.sh` | Gate `conscience/aiua_core.md`. |
| `start-sddia.sh` | No-op hooks si raíz no es git. |
| `SddIA/evolution/faa18af8-60e1-4d3f-b824-990f89cee208.md` | Registro + `Evolution_log`. |
| Instancia `SddIA_AP` | Wipe bundle + creator v1.4.0 + unidades `@%f` + overlay conscience. |
| `docs/audits/paciente0-deploy-20260925T114629Z.md` | Cicatriz ola 10. |
| DEUDA `PBI-DT-PACIENTE0-DEPLOY-PROCESS` | v1.7.0. |

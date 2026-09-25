---
feature_name: sddia-deterministic-installer
created: "2026-09-25"
process: feature
items:
  - bundle-full-node
  - installer-engine
  - wrapper-smoke-ci
document_id: PBI-ARQUITECTURA-DEPLOY-DETERMINISTA
execution_id: "7b22f932-c162-4104-b38d-b1c9c6068414"
---

# Implementación — sddia-deterministic-installer

| Item | Path | Nota |
|------|------|------|
| Perfil `full-node` | `SddIA/scripts/build-release-bundle.sh` | Discovery de crates nativos; `--list-capsules`; Filtro C intacto. |
| Motor | `SddIA/scripts/sddia-installer.sh` | `deploy`/`teardown`; L-PATH; live-gate 2; teardown `--force` 3; `--dry-run`. |
| Fachada | `sddia-installer.sh` | `exec` al motor. |
| Smoke | `SddIA/scripts/qa/test-sddia-installer.sh` | Dry-run + discovery; cero `enable`. |
| CI | `.github/workflows/sddia-index-qa.yml` | Job `sddia-installer-smoke`. |

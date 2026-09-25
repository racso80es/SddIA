---
feature_name: sddia-deterministic-installer
created: "2026-09-25"
process: feature
branch: feat/sddia-deterministic-installer
global: PENDIENTE-CI
pbi_archived: false
git_changes:
  - SddIA/scripts/build-release-bundle.sh
  - SddIA/scripts/sddia-installer.sh
  - SddIA/scripts/qa/test-sddia-installer.sh
  - sddia-installer.sh
  - .github/workflows/sddia-index-qa.yml
  - docs/features/sddia-deterministic-installer/
  - docs/todos/pending/PBI-ARQUITECTURA-DEPLOY-DETERMINISTA.md
  - SddIA/evolution/e0b636bf-099c-4907-a5d0-70e62e6c6f6e.md
  - SddIA/evolution/Evolution_log.md
checks:
  CA-ATOMIC:
    verdict: APTO
    evidence: dry-run deploy sin prompts; motor encadena bundle + instance-creator + systemd (no ejercido live)
  CA-PAYLOAD:
    verdict: APTO
    evidence: --list-capsules full-node incluye CONSUMER_BINS + sddia-qa|github-bridge-watcher
  CA-TEARDOWN:
    verdict: APTO
    evidence: dry-run teardown esc=systemd-escape -p ROOT; wipe real exige --force
  CA-LIVE:
    verdict: APTO
    evidence: stub .SddIA + deploy --dry-run → exit 2
  CA-FORGE:
    verdict: APTO
    evidence: --root FORGE dry-run abort ≠ 0
  CA-CI:
    verdict: PENDIENTE-CI
    evidence: job sddia-installer-smoke; run_id pendiente post-PR
---

# Validación — sddia-deterministic-installer

Smokes locales APTO. `global` no es APTO hasta `run_id` verde del PR (L-CI).

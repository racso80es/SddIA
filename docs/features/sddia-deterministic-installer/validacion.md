---
feature_name: sddia-deterministic-installer
created: "2026-09-25"
process: feature
branch: feat/sddia-deterministic-installer
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/300
ci_head: 39876b3cec9fe4fb752ec53b7f11b73f6b687b8f
ci_run_pr: "36133200778"
ci_run_push: "36133194114"
evolution_id: e0b636bf-099c-4907-a5d0-70e62e6c6f6e
execution_id: "7b22f932-c162-4104-b38d-b1c9c6068414"
git_changes:
  - SddIA/scripts/build-release-bundle.sh
  - SddIA/scripts/sddia-installer.sh
  - SddIA/scripts/qa/test-sddia-installer.sh
  - sddia-installer.sh
  - .github/workflows/sddia-index-qa.yml
  - docs/features/sddia-deterministic-installer/
  - docs/todos/done/PBI-ARQUITECTURA-DEPLOY-DETERMINISTA.md
  - SddIA/evolution/e0b636bf-099c-4907-a5d0-70e62e6c6f6e.md
  - SddIA/evolution/Evolution_log.md
checks:
  CA-ATOMIC:
    verdict: APTO
    evidence: dry-run deploy sin prompts; motor encadena bundle + instance-creator + systemd
  CA-PAYLOAD:
    verdict: APTO
    evidence: --list-capsules full-node incluye CONSUMER_BINS + crate extra
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
    verdict: APTO
    evidence: "PR #300 head 39876b3; runs 36133200778 (pull_request) y 36133194114 (push) success. Job sddia-installer-smoke pass."
---

# Validación — sddia-deterministic-installer

CI post-PR verde. PBI archivado en `docs/todos/done/`. `accept-pr` autorizado.

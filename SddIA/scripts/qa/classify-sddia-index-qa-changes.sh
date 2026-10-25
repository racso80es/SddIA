#!/usr/bin/env bash
# Expone lance_memory, linear_e2e e installer en GITHUB_OUTPUT para poda por diff en PR.
# En push a main: las tres flags true (guardián AC-6).
set -euo pipefail

lance_memory=false
linear_e2e=false
installer=false

if [[ "${GITHUB_EVENT_NAME:-}" == "push" && "${GITHUB_REF:-}" == "refs/heads/main" ]]; then
  lance_memory=true
  linear_e2e=true
  installer=true
else
  git fetch --no-tags origin main
  while IFS= read -r f; do
    [[ -z "$f" ]] && continue
    case "$f" in
      SddIA/core/memory/*|SddIA/infrastructure/adapters/lancedb*|SddIA/tools/evolution-register/*|SddIA/engine/execute-process/*|SddIA/Cargo.toml|SddIA/Cargo.lock|.github/workflows/sddia-index-qa.yml)
        lance_memory=true
        ;;
    esac
    case "$f" in
      SddIA/engine/execute-process/*|SddIA/sddia-daemon-runtime/*|SddIA/sddia-event-store/*|SddIA/tools/*tracker-adapter/*|SddIA/tools/sddia-qa/*|docs/_fixtures/lab-done-gate-project-both/*|SddIA/Cargo.toml|SddIA/Cargo.lock|.github/workflows/sddia-index-qa.yml)
        linear_e2e=true
        ;;
    esac
    case "$f" in
      sddia-installer.sh|SddIA/scripts/*|SddIA/engine/execute-process/*|.github/workflows/sddia-installer-qa.yml|.github/workflows/sddia-index-qa.yml)
        installer=true
        ;;
    esac
  done < <(git diff --name-only origin/main...HEAD)
fi

{
  echo "lance_memory=$lance_memory"
  echo "linear_e2e=$linear_e2e"
  echo "installer=$installer"
} >> "${GITHUB_OUTPUT:?GITHUB_OUTPUT required}"

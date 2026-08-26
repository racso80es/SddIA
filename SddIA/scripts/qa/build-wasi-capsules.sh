#!/usr/bin/env bash
# Build WASI artifacts for skills/tools/interfaces — excludes native daemons (DT-WASI-NATIVE-DAEMON-SPLIT).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TARGET="${WASI_TARGET:-wasm32-wasip1}"

# Native-only crates: orchestrator, QA aduana, centinelas sensoriales (IMAP/TLS, etc.).
EXCLUDE=(
  execute-process
  sddia-qa
  event-watcher
  event-sweeper
  email-watcher
  telegram-watcher
  github-bridge-watcher
)

packages=()
for dir in skills tools interfaces; do
  for cargo in "${ROOT}/${dir}"/*/Cargo.toml; do
    [[ -f "${cargo}" ]] || continue
    name="$(grep -E '^name = ' "${cargo}" | head -1 | sed 's/name = "\(.*\)"/\1/')"
    skip=false
    for ex in "${EXCLUDE[@]}"; do
      if [[ "${name}" == "${ex}" ]]; then
        skip=true
        break
      fi
    done
    if $skip; then
      continue
    fi
    main_rs="${cargo%/*}/src/main.rs"
    if [[ -f "${main_rs}" ]]; then
      packages+=("${name}")
    fi
  done
done

if [[ ${#packages[@]} -eq 0 ]]; then
  echo '{"success":false,"exitCode":1,"message":"no WASI packages discovered"}' >&2
  exit 1
fi

cd "${ROOT}"
args=()
for pkg in "${packages[@]}"; do
  args+=(-p "${pkg}")
done
cargo build --target "${TARGET}" "${args[@]}"

printf '{"success":true,"exitCode":0,"message":"wasi capsules built","result":{"target":"%s","package_count":%d}}\n' \
  "${TARGET}" "${#packages[@]}"

#!/usr/bin/env sh
set -e
cd "$(dirname "$0")"
if [ "$#" -gt 0 ]; then
  CAPSULE_JSON="$*"
  exec npx --yes ts-node index.ts "$CAPSULE_JSON"
else
  exec npx --yes ts-node index.ts
fi

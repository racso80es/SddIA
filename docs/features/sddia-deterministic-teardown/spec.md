---
feature_name: sddia-deterministic-teardown
created: "2026-09-26"
process: feature
base: main
scope: scripts
branch_name: feat/sddia-deterministic-teardown
persist_ref: docs/features/sddia-deterministic-teardown
execution_id: "0e8d3078-8bcb-4d45-91b9-5dade5b6b641"
---

# Spec — sddia-deterministic-teardown

Motor único: `SddIA/scripts/sddia-installer.sh teardown`. Defaults = deploy (last-resort Aplicaciones, no `SddIA_AP`).

Orden: SIGTERM/KILL scoped a `{ROOT}/start-sddia.sh` ∪ cwd/exe ∈ ROOT → `stop`/`disable`/`reset-failed` `sddia-*@${ESC}` → locks → `rm -rf ROOT` si existe (huérfano systemd también). Abort `/`, `$HOME`, forja. Atajo host `SddIA_Teardown.sh` → `teardown --force` (salvo `--dry-run` / comando explícito).

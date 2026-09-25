---
feature_name: sddia-deterministic-installer
created: "2026-09-25"
process: feature
branch_name: feat/sddia-deterministic-installer
persist_ref: docs/features/sddia-deterministic-installer
execution_id: "7b22f932-c162-4104-b38d-b1c9c6068414"
items_applied:
  - init-feature
  - plan-commit
  - bundle-full-node
  - installer-engine
  - wrapper-smoke-ci
---

# Ejecución — sddia-deterministic-installer

## Init

`SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 ./sddia-run.sh --process feature --inputs-file .tmp/feature-sddia-deterministic-installer.json`

`execution_id` `7b22f932-c162-4104-b38d-b1c9c6068414`. workspace-init **executed**. Mayeuta…DCC phase-barrier / relevo IDE. Commit planificación `9ffc3ee`.

## Tests

```text
bash SddIA/scripts/qa/test-sddia-installer.sh
# OK test-sddia-installer

bash SddIA/scripts/qa/test-build-release-bundle-filtro-c.sh
# OK build-release-bundle-filtro-c
```

Cero `systemctl enable` en smoke. Deploy live a ruta PBI = fuera de gate.

`sddia-qa evolution-register` → `e0b636bf-099c-4907-a5d0-70e62e6c6f6e` (`EVOL_OK`, `alta`).

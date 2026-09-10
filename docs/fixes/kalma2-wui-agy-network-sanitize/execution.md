---
feature_name: kalma2-wui-agy-network-sanitize
created: "2026-09-10"
process: bug-fix
branch: fix/kalma2-wui-agy-network-sanitize
execution_id: "2dab4bf4-408b-4dc6-919a-680c92588cf7"
items_applied:
  - classify-agy-network
  - hermetic-tests
  - intact-skill-appjs
---

# Execution — kalma2-wui-agy-network-sanitize

## Init

```bash
SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  ./sddia-run.sh --process bug-fix --inputs-file .tmp/bug-fix-kalma2-wui-agy-network-sanitize.json
```

`execution_id` `2dab4bf4-408b-4dc6-919a-680c92588cf7`. workspace-init **executed**. Diseño `simulated`.

## Código

`is_agy_network_issue` + `AIUA_AGY_NETWORK_MSG` (82 chars). Cero retry.

## Host (fuera del diff)

`nmcli connection modify EstherRacsoWifi_5G ipv6.method disabled` + recycle `kalma2-bridge`. No entra en el PR.

## Tests

```text
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p kalma2-bridge --bin kalma2-bridge -- sanitize_maps_agy
# 2 passed (filtro); suite completa en CI
```

`sddia-qa evolution-register` → `89661fec-bed6-4c3d-8169-3922ddbe7a2f` (`EVOL_OK`, `alta`).

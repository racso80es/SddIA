---
feature_name: iota-dlt-publish-fetch-failed
created: "2026-09-01"
process: bug-fix
branch_name: fix/iota-dlt-publish-fetch-failed
persist_ref: docs/fixes/iota-dlt-publish-fetch-failed
execution_id: "479390a2-db58-46c7-857f-445dd26364c2"
items_applied:
  - relay-error-format
  - server-catch-cause
  - node-test-relay-error
  - diagnose-testnet-fullnode
  - capsule-publish-ca3
  - recycle-relay-leftover-node
---

# Ejecución — fractura `a90fad3fa8fa`

## Inicio

```bash
SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 ./sddia-run.sh --process bug-fix --inputs-file .tmp/bug-fix-a90fad3fa8fa-init.json
```

`execution_id`: `479390a2-db58-46c7-857f-445dd26364c2`. Diseño `simulated`. DCC barrera `prior_agent_phase_not_executed`.

Commit Diseño: `9473110`.

## F2 tests

```text
.node-v22 --test relay-error.test.mjs
# tests 5  pass 5  fail 0
```

## F3 causa (CA2)

Sello original (2026-08-30 20:07): `relay.log` solo `listening`; catch no logueaba. Cause de `a90fad3fa8fa` **no recuperable**.

Reproducción isomorfa 2026-09-01T07:47Z desde el mismo Node v22 del hijo:

```text
fullnode_url https://api.testnet.iota.cafe
http_status 200
body_prefix {"jsonrpc":"2.0","id":1,"result":"258893231"}
```

Literal: Testnet alcanzable **hoy**. Candidato 3 del PBI (SDK/red) para el sello histórico; no hay ENOTFOUND/timeout en la ventana de ejecución.

## Reciclo hijo (instancia)

Restart systemd dejó huérfano pid 67195 (Node del 30 ago, `:8787`). Hijos nuevos exit 1 (EADDRINUSE). Kill 67195. Supervisor respawn pid **653722**. `/health` 200.

## F4 publish (CA3)

```bash
./sddia-run.sh --tool iota-immutable-publisher --prefer-native --inputs-file .tmp/iota-publish-ca3.json
```

`SIMULATE=0`. Cápsula, no `curl` POST.

```text
success: true
mode: relay
transaction_digest: 8TaP3rFM27J76NDzKoGRPd72ysAr7TWSkJdKnk5HpAdG
object_id: 0xf8f49b7c4be31b938b10bc70a6415923513e792b41d49372b0067d6e7c12ee7f
```

`/health` 200 y publish 2xx = checks distintos (CA4).

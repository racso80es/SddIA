---





feature_name: linear-update-state-graphql-types
created: "2026-11-06"
process: bug-fix
branch_name: fix/linear-update-state-graphql-types
persist_ref: docs/fixes/linear-update-state-graphql-types
document_id: PBI-cd8405c2
uuid: cd8405c2-2394-4d3e-b846-ab2ffc5f38cc
execution_id: "65f1bd41-ca2d-4139-83b3-edc774251e56"
---
# Ejecución — linear-update-state-graphql-types

## Comandos

```bash
cd SddIA && env -u CARGO_TARGET_DIR cargo test -p linear-tracker-adapter aut70
cd SddIA && env -u CARGO_TARGET_DIR cargo build -p linear-tracker-adapter --release
cd SddIA && env -u CARGO_TARGET_DIR cargo build -p linear-tracker-adapter --target wasm32-wasip1 --release
```

## Verificación física

```bash
./sddia-run.sh --tool linear-tracker-adapter --inputs '{"request":{"operation":"update_issue_state","issue_ref":"MAN-27","state_name":"Todo","team_key":"AUT","project_id":"359ebe2e-311b-46f9-8623-fa665179bac3"}}'
./sddia-run.sh --tool linear-tracker-adapter --inputs '{"operation":"fetch_issue","issue_ref":"MAN-27","project_slug":"core"}'
```

Acuse esperado: `success: true` en `update_issue_state`; `fetch_issue.state` coherente con **Todo** (nombre Linear según `state_map`).

## Gate evolution

```bash
SddIA/target/release/sddia-qa gate-evolution --json --range origin/main..HEAD
```

(Si `release` ausente, `debug/sddia-qa` equivalente.)

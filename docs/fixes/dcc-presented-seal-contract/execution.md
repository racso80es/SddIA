---
feature_name: dcc-presented-seal-contract
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-presented-seal-contract
tracker_ref: AUT-48
execution_id: "9e85f016-c9be-46de-af20-92455e7817b8"
---

# Ejecución — AUT-48

## Comandos

```bash
./sddia-run.sh --process entity-manager --inputs-file .tmp/entity-manager-aut48-seal-body.json
./sddia-run.sh --process entity-manager --inputs-file .tmp/entity-manager-aut48-version.json

rg 'opcional en payload ECST v1.1' SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
# (sin coincidencias)

export TMPDIR=$PWD/.tmp CARGO_TARGET_DIR=$PWD/.tmp/cargo-target
cargo test -p execute-process emit_presented_blocks_missing_pr_url

SddIA/target/debug/sddia-qa evolution-rehash --id 20628bfd-f3f1-4dff-9409-4ef70e82c985
```

## entity-manager

- `handoff_entity_uuid`: `5417c92c-da7f-4d46-b245-55cf1b17961a`
- `handoff_version`: `1.6.1`
- `hash_signature` fases: sin cambio de digest (`sha256:2478e088…`) — solo cuerpo MD y bump SemVer

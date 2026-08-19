---
feature_name: kalma2-mvp-sync-activos
created: "2026-08-19"
process: feature
branch_name: feat/kalma2-mvp-sync-activos
persist_ref: docs/features/kalma2-mvp-sync-activos
document_id: PBI-KALMA2-MVP-01B
uuid: "ed2f20b8-6e3d-4dbf-931c-d62e53ddf7c4"
status: executing
---

# Ejecución — kalma2-mvp-sync-activos

## Comandos

```bash
# Build
cd SddIA && CARGO_TARGET_DIR=target cargo build -p github-raw-fetcher -p execute-process -p kalma2-bridge

# Cápsula T6
echo '{"meta":{"schemaVersion":"2.0","entityKind":"tool","entityId":"github-raw-fetcher"},"request":{"asset_path":"SddIA/library/codexes/codex-kalma2-assistant.md","ref":"main"}}' \
  | SddIA/target/debug/github-raw-fetcher

# Proceso T7 (smoke local)
./sddia-run.sh --process sync-client-assets --inputs '{
  "asset_id":"c43544f3-c557-4cc3-8a03-7175282f2c88",
  "asset_family":"library_codexes",
  "correlation_id":"test-sync-01b-smoke",
  "execution_id":"test-sync-01b-smoke"
}'

# Gate G7
rg 'github-raw-fetcher' SddIA/process/sync-client-assets.md SddIA/actions/download-remote-asset.md
# → exit 1 (sin coincidencias)

# Tests
cd SddIA && CARGO_TARGET_DIR=target cargo test -p execute-process sync_client_assets
```

## Smokes (2026-08-19)

| ID | Resultado |
|----|-----------|
| `github-raw-fetcher` local read + SHA-256 | OK · `exitCode:0 ⟺ success:true` |
| `sync-client-assets` 4 fases | OK · inyección en `.SddIA/library/codexes/codex-kalma2-assistant.md` |
| G7 grep acción/proceso | OK · 0 coincidencias |
| PEC + telemetría | OK · `correlation_id=test-sync-01b-smoke` |

---
feature_name: merge-thermo-02-net-prune
created: "2026-10-03"
process: feature
branch_name: feat/merge-thermo-02-net-prune
persist_ref: docs/features/merge-thermo-02-net-prune
---

# Ejecución

```bash
SddIA/scripts/qa/test-merge-thermo-02-net-prune.sh
cd SddIA && cargo test -p sddia-qa resolve_base_fresh_origin_main_skips_fetch_ca6 -- --test-threads=1
cd SddIA && cargo test -p execute-process resolve_timeout_secs_honors_hook_env_180 -- --test-threads=1
env -u CARGO_TARGET_DIR cargo build -p sddia-qa --release
```

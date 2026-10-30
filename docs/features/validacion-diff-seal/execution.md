---
feature_name: validacion-diff-seal
created: "2026-10-30"
process: feature
pbi_ref: AUT-39
---

# Ejecución

```bash
cd SddIA/engine/execute-process
cargo test -p execute-process --lib diff_seal sync_dcc_process_hash_signature dcc_fracture_suppressed_on_diff_seal
cargo build --release -p execute-process
```

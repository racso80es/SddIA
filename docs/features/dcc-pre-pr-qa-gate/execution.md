---
feature_name: dcc-pre-pr-qa-gate
created: "2026-10-30"
process: feature
pbi_ref: AUT-38
branch: feat/dcc-pre-pr-qa-gate
---

# Ejecución

```bash
cd SddIA && cargo test -p execute-process --lib worktree_allows_only_validacion sync_dcc_process_hash_signature dcc_fracture_suppressed_on_pre_pr_qa_gate_block
cd .. && SddIA/target/release/sddia-qa verify-process-integrity
```

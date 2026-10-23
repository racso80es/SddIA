---
feature_name: tracker-lifecycle-sink
created: "2026-10-23"
process: feature
command: cargo test -p execute-process --lib -- --test-threads=1 file_default_and_rejects_both_and_contradiction materialize_fracture_pbi_creates_and_idempotent materialize_kaizen_alert_doc_creates_and_idempotent ci_chronic_materialize_creates_and_idempotent
cwd: SddIA
exit_code: 0
---

# Stdout — tests del sumidero

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 9.66s
Running unittests src/lib.rs (execute_process-67fde4d64d95f9f5)

running 4 tests
test engine::materialize_ci_chronic_failure_pbi::tests::ci_chronic_materialize_creates_and_idempotent ... ok
test engine::materialize_fracture_pbi::tests::materialize_fracture_pbi_creates_and_idempotent ... ok
test engine::materialize_kaizen_alert_doc::tests::materialize_kaizen_alert_doc_creates_and_idempotent ... ok
test engine::project_binding::tests::file_default_and_rejects_both_and_contradiction ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 669 filtered out; finished in 0.22s
```

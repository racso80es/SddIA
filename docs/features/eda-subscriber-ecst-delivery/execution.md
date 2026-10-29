---
feature_name: eda-subscriber-ecst-delivery
pbi_ref: AUT-46
branch: feat/eda-subscriber-ecst-delivery
---

# Ejecución

```bash
cd SddIA && cargo test -p execute-process --test eda_subscriber_ecst_redb -- --test-threads=1
cd SddIA && cargo test -p execute-process --test event_store_writers terminal_witness -- --test-threads=1
cd .. && SddIA/target/release/sddia-qa gate-evolution --json --range origin/main...HEAD
```

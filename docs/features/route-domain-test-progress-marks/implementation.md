---
feature_name: route-domain-test-progress-marks
created: "2026-11-01"
tracker_ref: AUT-59
---

# Implementación — AUT-59

- `route_domain_test_progress_mark` en `tests/eda_subscriber_ecst_redb.rs`: stderr + `.tmp/route-domain-test-progress.log` del checkout (`find_repo_root`).
- Cinco tests `route_domain_*`: `enter` / `leave` alrededor de `route_domain_event`.

Observación: `cargo test … route_domain_ -- --nocapture --test-threads=1` y `tail -f .tmp/route-domain-test-progress.log`.

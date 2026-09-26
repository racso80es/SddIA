---
feature_name: sddia-deterministic-teardown
created: "2026-09-26"
process: feature
items:
  - installer-teardown
  - qa-smoke
  - host-shortcut
execution_id: "0e8d3078-8bcb-4d45-91b9-5dade5b6b641"
---

# Implementación — sddia-deterministic-teardown

| Item | Path |
|------|------|
| Motor | `SddIA/scripts/sddia-installer.sh` (`resolve_root` inseguro; `signal_instance_procs`; reset-failed) |
| Smoke | `SddIA/scripts/qa/test-sddia-installer.sh` (§8–10) |
| Atajo | `/home/racso/Aplicaciones/SddIA/SddIA_Teardown.sh` (fuera de git) |

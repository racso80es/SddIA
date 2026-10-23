---
feature_name: tracker-lifecycle-sink
process: feature
created: "2026-10-23"
branch: feat/tracker-lifecycle-sink
---

# Spec

`file` escribe `pending/` y `done/`. `linear` usa el provider resuelto de `tracker:lifecycle` y no crea ficheros en esos buckets. `done_gate: both` no es legal.

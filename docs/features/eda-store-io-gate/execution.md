---
feature_name: eda-store-io-gate
created: "2026-10-30"
items_applied:
  - publish_ecst y load_domain_ecsts en sddia-event-store
  - escritores residuales (heartbeat, kalma2, capability-di, lancedb, iota smoke, installer)
  - lectores presented y find_existing_domain_event
  - claim fractal y route_fractal_event desde store
  - flag CLI --publish-ecst
---

# Ejecución

Fachada `publish_ecst(repo, family, event, fs_dir)`: store si `writes_store`, JSON si `writes_filesystem`.

`load_domain_ecsts` une store y directorios FS según el modo.

Claim fractal (`telemetry` / `orchestration`): ventana 6 h, tope 16 por tick, testigo `event-watcher.fractal-claim`. En `redb`, JSON bajo esas carpetas es `F-EDA-RESIDUAL-FRACTAL`.

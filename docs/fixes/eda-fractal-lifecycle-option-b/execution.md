---
feature_name: eda-fractal-lifecycle-option-b
created: "2026-07-16"
process: bug-fix
items_applied:
  - route-fractal-purge-after-domain
  - stamp-delivery-state
  - telegram-ack-first
  - sweeper-fractal
---

# Execution

```bash
cargo build -p execute-process -p event-watcher -p event-sweeper -p telegram-watcher -p sddia-daemon-runtime
```

Resultado: `Finished dev profile` (2026-07-16).

## Post-merge empírico (2026-07-16)

Dominio local: **145** archivos; **0** fully-terminal; **145** con `failed` (IOTA dominante).

Detalle y plan de continuación: `continuation.md`.

---
document_id: PBI-FEATURE-KALMA2-BRIDGE-RUST
title: "[FEATURE] kalma2-bridge — puente HTTP nativo Rust"
format: markdown
version: "1.0.0"
created: "2026-06-19"
status: done
priority: alta
process: feature
branch_name: feat/kalma2-bridge-rust
feature_ref: docs/features/kalma2-bridge-rust
validacion_ref: docs/features/kalma2-bridge-rust/validacion.md
uuid: a18693e0-226a-4602-a1bd-9952a87cae54
closed: "2026-06-19"
---

# PBI-FEATURE: kalma2-bridge — puente HTTP nativo Rust

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-FEATURE-KALMA2-BRIDGE-RUST` |
| **Estatus** | ✅ Done — PR pendiente merge |
| **Feature** | [`docs/features/kalma2-bridge-rust/`](../../features/kalma2-bridge-rust/) |
| **Validación** | [`validacion.md`](../../features/kalma2-bridge-rust/validacion.md) |
| **Rama** | `feat/kalma2-bridge-rust` |

## Entregables

| Fase | Entregable | Estado |
|------|------------|--------|
| A | Crate + workspace `interfaces/*` | ✅ |
| B | Servidor estático GET | ✅ |
| C | POST /api/interact passthrough | ✅ |
| D | Switch start-sddia + sddia-run.sh | ✅ |
| E | Poda sddia-client-bridge.py | ✅ |

## Operador

```bash
cd SddIA && cargo build -p kalma2-bridge -p execute-process
./start-sddia.sh
```

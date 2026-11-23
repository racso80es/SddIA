---
feature_name: erradicar-env-bypass-por-di
created: "2026-11-23"
process: refactorization
branch_name: refactor/erradicar-env-bypass-por-di
persist_ref: docs/refactors/erradicar-env-bypass-por-di
tracker_ref: AUT-116
---
# Especificación Técnica — erradicar-env-bypass-por-di (`AUT-116`)

| ID | Criterio | Oráculo mecánico ejecutable (Ola 0) |
| --- | --- | --- |
| AC-1 | Inventario clasificado | Tabla en `implementation.md` |
| AC-2 | DI en fronteras migradas | `cargo test -p execute-process --lib aut114_pull_request_merged_from_store_invokes_notify_humanized` |
| AC-3 | 0 `SDDIA_LAB_SKIP_*` en fronteras migradas | `sddia-qa gate-no-env-bypass` (`violation_count: 0`) |
| AC-4 | Gate estático `gate-no-env-bypass` | `cargo test -p sddia-qa gate_no_env_bypass` |
| AC-5 | Compilación limpia `-Dwarnings` | `RUSTFLAGS="-Dwarnings" cargo check --workspace` |

---
feature_name: resolucion-estricta-capsulas-skill-tool
created: "2026-11-23"
process: refactorization
branch_name: refactor/resolucion-estricta-capsulas-skill-tool
persist_ref: docs/refactors/resolucion-estricta-capsulas-skill-tool
tracker_ref: AUT-118
---
# Especificación Técnica — resolucion-estricta-capsulas-skill-tool (`AUT-118`)

| ID | Criterio | Oráculo mecánico ejecutable (Ola 0) |
| --- | --- | --- |
| K-CROSS | Cero invocaciones cruzadas en Rust | `sddia-qa gate-capsule-kind` (`violation_count: 0`) |
| K-RT | Rechazo estricto `CapsuleKindMismatch` en `resolve_capsule` + registro Cerbero | `cargo test -p execute-process --lib aut118_resolve_capsule_rejects_kind_mismatch` |
| K-QA | Gate estático en `sddia-qa` + `verify-tools-index` | `cargo test -p sddia-qa gate_capsule_kind` + `sddia-qa verify-tools-index` |
| K-TEST | Compilación limpia `-Dwarnings` | `RUSTFLAGS="-Dwarnings" cargo check --workspace` |

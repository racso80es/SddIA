---
feature_name: event-manager-port-cutover
created: 2026-11-08
process: refactorization
pbi_ref: MAN-35
document_id: PBI-176f9f2f
branch: refactor/event-manager-port-cutover
global: APTO
pbi_archived: false
validated_base_sha: b5de29c291ed27ed59ce7a8d3513d5291c50eea5
validated_head_sha: 99f71349581ff087fa0ba191c49670aefad77711
validated_tree_sha: fbebfa87aa1e6878b16e47ba2194b7d88fafd097
diff_digest: sha256:e16dedca9e3c7d9f2e165791d205ae9ed666fecdbc9f5c5eb4a444fe456ef3cc
---

# Validación — event-manager-port-cutover (MAN-35)

## Alcance verificado

- Composition root `sddia-event-manager-host` + `eda_port` en motor.
- Contrato **1.2.0** (core + Redb + mock + external capsule + suite).
- T3 loader / `route_domain` estricto; T4 `fractal_bus` solo puerto en release.
- T5 productores (kalma2, lancedb) vía host; T6 audit/purger contractual.
- T7 `eda_port` productivo; legacy en `eda_port_legacy` + `#[cfg(test)]`.
- T8 `sddia-qa gate-event-manager-port` + tests contrato en CI (`sddia-index-qa`).
- Taxonomía **1.0.11** + bindings **1.9.0** (`eda.event_manager` **1.2.0**, sin fallback FS).
- Convergencia documental **AC-9**: `README.md`, `SddIA/CONSTITUTION_CORE.md` (MAN-35).

## Gate CI

| Check | Estado |
|-------|--------|
| `gate-event-manager-port` | job `sddia-index-integrity` |
| Contract/tests EM | job `sddia-index-integrity` (MAN-35 step) |
| `run_id` | PENDIENTE-CI — se sella tras merge PR en `main` |

## Criterio global

`APTO` — invariantes L-GATE/L-OPS-PORT cubiertas en rama; cierre Linear MAN-35 tras merge a `main` con checks verdes.
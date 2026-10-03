---
feature_name: prepush-argos-qa-witness
created: "2026-10-02"
process: bug-fix
---

# Implementación

| Artefacto | Cambio |
|-----------|--------|
| `route_domain_core.rs` | Test ancla witness `a55f1d12…` |
| `phase_terminal.rs` | Test fase `blocked` PPR → `status_code` 1 |
| `residual_runner.rs` / `executor.rs` | `argos_verdict: block` si agente PPR `blocked` |
| `docs/features/prepush-argos-qa-witness/` | Dictamen R-1/R-2 (no solo dead-letter) |

Hook sin cambio: comportamiento ya coherente con contrato.

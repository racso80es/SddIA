---
feature_name: kaizen-ciclo-jurisdiccion-todos
created: "2026-08-29"
process: feature
items:
  - run_norm_forge_dependencies_hard_constraints
  - patch_norm_content_update
  - patch_hash_signature_refresh
  - emit_dcc_phase_fractures
  - eda_coverage_ssot_exempt
  - gitignore_nested_tmp
  - obediencia_colapso_mudo
execution_id: "1550128b-c2ef-4c4d-8cbb-181a15a66940"
---

# Implementation — kaizen-ciclo-jurisdiccion-todos

| CA | Touchpoint | Vía |
|----|------------|-----|
| CA1 | `factory.rs` `run_norm_forge`, `common.rs` helpers, test `run_norm_forge_emits_dependencies_and_hard_constraints` | IDE |
| CA2 | `SddIA/library/norms/todos-jurisdiction.md` v1.1.0 | `--forge` + `entity-manager` |
| CA3 | `github-raw-fetcher.md`, `download-remote-asset.md` hash real | `entity-manager` `hash_refresh_only` |
| CA3b | `delivery-close-cycle.md`, `features-documentation-pattern.md` | `entity-manager` `markdown_body_replacements` |
| CA4 | `delivery_close.rs` `emit_dcc_phase_fractures`, test `dcc_fracture_emits_on_blocked_phase` | IDE |
| CA5 | `sddia-evolution-register/src/lib.rs` exención `eda-coverage.json` | IDE |
| CA6 | `.gitignore` `**/.tmp/` | IDE |
| CA7 | `SddIA/norms/obediencia-procesos.md` v1.2 colapso mudo | DA-4 |
| L8 | `SddIA/evolution/a8f3c2e1-9d4b-4a7f-b6e5-1c2d3e4f5a6b.md`, `Evolution_log.md` | `evolution-rehash` |

Invariante forge: compilar con `CARGO_TARGET_DIR=SddIA/target` antes de `sddia-run` (binario release en `SddIA/target/release/execute-process`).

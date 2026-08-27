---
feature_name: kaizen-tekton-evolution-gate-no-poll
created: "2026-08-27"
process: feature
items:
  - T1-capsule-format-rehash-verdict
  - T2-gate-fidelity-read-blob
  - T3-evolution-rehash-cli
  - T4-gate-all-universe
  - T5-fossil-saneamiento
  - T6-pre-push-if-touched
  - T7-da6-norm-rule
  - T8-contract-1.1.2
branch_name: feat/kaizen-tekton-evolution-gate-no-poll
execution_id: "96471044-003a-457a-bf59-041e94053b12"
---

# Implementación — kaizen-tekton-evolution-gate-no-poll

## Touchpoints

| Área | Archivo | Cambio |
|------|---------|--------|
| Cápsula | `SddIA/skills/sddia-evolution-register/src/lib.rs` | Formato hash; `verdict` valida delta sin L-SELF ciego; `audit: universe`; `operation: rehash` |
| Parser | `SddIA/engine/execute-process/src/core/parser.rs` | `parse_frontmatter_from_str`, `frontmatter_yaml_to_json` |
| Gate CLI | `SddIA/tools/sddia-qa/src/gate_evolution.rs` | `read_blob(rev)`; fm desde `raw`; `--all`, `--if-touched`; `evolution-rehash` |
| CLI | `SddIA/tools/sddia-qa/src/main.rs` | Subcomando `evolution-rehash` |
| Contrato | `SddIA/evolution/evolution_contract.md` | v1.1.2 documental |
| Skill | `SddIA/skills/sddia-evolution-register.md` | `rehash` + `audit: universe` |
| Norma | `SddIA/norms/external-ai-constraints.md` | v1.6.0 DA-6 |
| Rule | `.cursor/rules/tekton-fire-and-forget.mdc` | DA-6 |
| Hook | `SddIA/scripts/qa/git-hooks/pre_push_gate.sh` | `gate-evolution --range --if-touched` antes de `route-domain-event` |
| CI | `.github/workflows/sddia-index-qa.yml` | Step `evolution gate (universe)` |
| Fósiles | `SddIA/evolution/*.md` (8 registros) | `evolution-rehash` + canonicalización `7e3c1a90` |

## Decisiones de implementación

- `invoke_register` sigue priorizando WASM (CI); WASM rebuild obligatorio tras cambios en cápsula.
- `--all` lee blobs `HEAD` (coherente con checkout CI).
- `rehash` lee working tree del registro (operador re-ancla WT).

---
feature_name: aut-132-gate-anti-anemia-reglas-contractuales
created: "2026-11-27"
updated: "2026-11-27T09:00:00Z"
process: refactorization
branch: refactor/aut-132-gate-anti-anemia-reglas-contractuales
branch_name: refactor/aut-132-gate-anti-anemia-reglas-contractuales
persist_ref: docs/refactors/aut-132-gate-anti-anemia-reglas-contractuales
tracker_provider: linear
tracker_ref: AUT-132
source_process: refactorization
global: APTO
pbi_archived: false
delivery_state: success
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  AC-4: APTO
  AC-5: APTO
  AC-6: APTO
  AC-7: APTO
  AC-8: APTO
  global: APTO
  branch: APTO
  F3_TECH_GATE: APTO
  DOC_EVOLUTION: APTO
---
# Validación — AUT-132

## Resumen

Entrega documental y evolution del hijo **AUT-132**. La lógica del peaje hereda `main` post-ratchet AUT-125; regresiones verificadas en rama.

## Oráculos ejecutados

| Oráculo | Resultado |
| --- | --- |
| `cargo test -p execute-process entity_no_anemia --lib` | 18 passed, 0 failed |
| `cargo test -p sddia-qa gate_entity` | 6 passed, 0 failed |
| Scripts `rebuild_actions_index.py` / `remediate_bulk.py` | ausentes en árbol |
| Evolution `a8f3c2e1…` `relacionado` | incluye `eda-coverage.json` y `docs/audits/AUT-125.md` en main |

## Criterios de aceptación

| ID | Estado | Evidencia |
| --- | --- | --- |
| AC-1 | APTO | `entity_no_anemia.rs` + tests `CONTRACT_SECTION_MISSING` |
| AC-2 | APTO | tests `truncated_tail_*` |
| AC-3 | APTO | tests `scan_changed_*` |
| AC-4 | APTO | `split_frontmatter` + tests YAML |
| AC-5 | APTO | `gate_entity_no_anemia.rs` tests CLI |
| AC-6 | APTO | `default_base_ref` + in-process en `phase_capsules` |
| AC-7 | APTO | scripts retirados; evolution padre completo en main |
| AC-8 | APTO | suites cargo verdes (tabla oráculos) |

**global: APTO**
---
feature_name: gate-anti-anemia-entidades-dominio
created: "2026-11-26"
process: refactorization
branch_name: refactor/gate-anti-anemia-entidades-dominio
persist_ref: docs/refactors/gate-anti-anemia-entidades-dominio
tracker_ref: AUT-125
---

# Spec — gate anti-anemia entidades dominio

## Oráculo mecánico (KA-DOC-2)

| ID | Oráculo | Comando |
| --- | --- | --- |
| AC-1 | Gate global dominio .md | `sddia-qa gate-entity-no-anemia --json` → `violation_count: 0` |
| AC-2 | Contrafactual testigo | `cargo test -p execute-process entity_no_anemia` (4 tests) |
| AC-3 | Peaje DCC/PPR si toca genoma | `gate-entity-no-anemia --if-touched` en índices (phase_capsules + PPR) |
| AC-4 | Forja create/update acción | `orchestration_logic` con ≥2 `##` + post-validación anti-anemia |

## Validador `entity_no_anemia`

- Frontmatter obligatorio (`uuid` v4).
- Cuerpo: ≥2 secciones `##` con texto; sin stubs (`TODO`/`TBD`/`FIXME`/línea `...`).
- Keywords contractuales por familia (propósito + operación/orquestación/payload).
- `NAME_FILENAME_MISMATCH` exento para `codex`.
- Dedupe de paths en `scan_all` (event rank > codex).

## Integración

| Superficie | Cambio |
| --- | --- |
| `SddIA/tools/sddia-qa` | subcomando `gate-entity-no-anemia` |
| `phase_capsules.rs` | aduana índices `--if-touched` |
| `pull_request_review.rs` | `scan_changed` bloqueante |
| `factory.rs` | create/update acción; yaml-only update event/process/suite; descripción índice una línea |
| CI | `.github/workflows/sddia-index-qa.yml` |

## Testigo

`SddIA/actions/switch-llm-provider.md` — anémico pre-refactor; remediado vía `entity-manager`.
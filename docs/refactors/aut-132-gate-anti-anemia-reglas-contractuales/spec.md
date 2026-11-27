---
feature_name: aut-132-gate-anti-anemia-reglas-contractuales
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-132-gate-anti-anemia-reglas-contractuales
persist_ref: docs/refactors/aut-132-gate-anti-anemia-reglas-contractuales
tracker_ref: AUT-132
historia_ref: AUT-125
execution_id: "47facb2e-84b4-48b8-ba38-0429be4605fd"
base: clarify.md
scope: peaje-gate-entity-no-anemia
---
# Spec — AUT-132

Endurecer y sellar el peaje `gate-entity-no-anemia` / `entity_no_anemia` según REJECT `3899742f-04f5-480b-a1f6-866d93d1d5a3` (hijo de AUT-125). Canal PBI: Linear `AUT-132` / forge-pbi `088f93c4…` (`PBI-9d7f7811`). Fuera: AUT-131 (forjas), AUT-133 (línea base → 0).

## Oráculos (O-1…O-7 → AC)

| ID | Objetivo | Oráculo mecánico |
| --- | --- | --- |
| AC-1 | Keywords contractuales efectivas (`purpose_kw` + `second_kw` en `##` distintas; sin atajos por conteo/longitud) | `cargo test -p execute-process purpose_kw_enforced_even_with_long_first_section second_kw_enforced_with_two_sections second_kw_must_be_distinct_section_from_purpose` |
| AC-2 | `scan_changed`: `base...HEAD`, `--diff-filter=d`, rank + límite `/` | `cargo test -p execute-process scan_changed_uses_merge_base_and_rank scan_changed_skips_deleted_files scan_changed_rejects_unknown_base classify_prefers_rank_over_config_order classify_respects_directory_boundary` |
| AC-3 | Frontmatter estricto + `FRONTMATTER_YAML_INVALID` | `cargo test -p execute-process split_frontmatter_requires_leading_delimiter frontmatter_yaml_error_is_reported missing_leading_delimiter_is_no_frontmatter` |
| AC-4 | CLI `--base` estricto; errores respetan `--json` | `cargo test -p sddia-qa gate_entity_no_anemia` |
| AC-5 | Peaje PPR/`phase_capsules` in-process + `base_ref` configurable | `cargo test -p execute-process index_anemia_gate_ tech_triage_` (+ aserción: sin spawn de `sddia-qa gate-entity-no-anemia`) |
| AC-6 | Scripts ad-hoc fuera del árbol; evolution con AUT-132 | Ausencia de `remediate_bulk.py` / `rebuild_actions_index.py` bajo `docs/refactors/gate-anti-anemia-entidades-dominio/`; `relacionado` incluye `AUT-132` en `SddIA/evolution/a8f3c2e1-9d4b-4f7a-b6e5-1c2d3e4f5a6b.md` (o alta hija si Tekton decide traza propia) |
| AC-7 | Suite peaje verde (regresión REJECT) | `cargo test -p execute-process entity_no_anemia` + `cargo test -p sddia-qa gate_entity_no_anemia` → 0 failed |
| AC-TRUNC | `BODY_TRUNCATED` solo firmas mecánicas (Q7) | `cargo test -p execute-process truncated_tail_ignores_lists_and_prose_without_period truncated_tail_detects_mechanical_cuts` |

**Ratchet heredado (laudo padre, no reabre AUT-133):** CI/PPR exigen `violation_count: 0` en `--if-touched`; línea base global **322** sin incremento.

## Contrato del validador

| Regla | Comportamiento canónico |
| --- | --- |
| Keywords | `purpose_kw` y `second_kw` se evalúan por heading `##` con cuerpo no vacío; la sección operativa debe ser **distinta** del propósito. Prohibido bypass por `sections.len() >= 2` o longitud ≥40 de la primera sección. |
| `scan_changed` | `git diff --name-only --diff-filter=d {base}...HEAD`; clasifica con `classify_entity_path` (rank event/process > resto > codex; a igual rank, prefijo más largo; límite `/`). |
| Frontmatter | Primera línea `---` (BOM tolerado) + cierre; YAML no-mapa o inválido → `FRONTMATTER_YAML_INVALID` (nunca silencio). |
| Truncado | Solo fence impar o prosa sin `\n` final y sin puntuación de cierre. |
| Base por defecto | `default_base_ref` → `origin/{default_branch}` desde `.SddIA/project.md`, fallback `origin/main`. |

## Superficies

| Path | Rol |
| --- | --- |
| `SddIA/engine/execute-process/src/engine/entity_no_anemia.rs` | Validador, `scan_all` / `scan_changed`, `default_base_ref`, tests de regresión |
| `SddIA/tools/sddia-qa/src/gate_entity_no_anemia.rs` | CLI JSON-determinista (`--json`, `--if-touched`, `--base`) |
| `SddIA/engine/execute-process/src/engine/phase_capsules.rs` | `index_anemia_gate` → `scan_changed` in-process; `base_ref` input / default |
| `SddIA/engine/execute-process/src/engine/pull_request_review.rs` | `anemia_triage` → `scan_changed` in-process; detalle en payload |
| `SddIA/evolution/a8f3c2e1-9d4b-4f7a-b6e5-1c2d3e4f5a6b.md` | Traza padre con `relacionado: AUT-132` |
| `.github/workflows/sddia-index-qa.yml` | Ratchet CI `--if-touched` (sin cambio de contrato salvo drift) |

## Estado observado (herencia)

La lógica de AC-1…AC-5, AC-TRUNC y la limpieza de scripts (AC-6 scripts) **ya está en el árbol**. Este ciclo Dedalo fija el contrato documental AUT-132 y el blueprint de verificación; Tekton solo toca código si Argos encuentra regresión o hueco frente a esta spec.

## Fuera de alcance

- Remediación masiva / oráculo global `violation_count: 0` → **AUT-133**.
- Forjas / `entity_manager` update sin defaults → **AUT-131**.
- Aceptación/merge de AUT-125 (#480) ni higiene QA del padre.
- Ampliar `genome_entity_dirs` a `capability_contracts` / `infrastructure_adapters` (Kaizen menor del REJECT; no bloqueante aquí).

---
feature_name: aut-132-gate-anti-anemia-reglas-contractuales
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-132-gate-anti-anemia-reglas-contractuales
persist_ref: docs/refactors/aut-132-gate-anti-anemia-reglas-contractuales
tracker_ref: AUT-132
historia_ref: AUT-125
execution_id: "47facb2e-84b4-48b8-ba38-0429be4605fd"
purpose: Endurecer peaje gate-entity-no-anemia con reglas contractuales efectivas
---
# Clarificación — AUT-132

Origen: auditoría PR #480 (`audit_event_reference` `3899742f-04f5-480b-a1f6-866d93d1d5a3`, REJECT). Hallazgo de peaje: bypass de keywords contractuales, `scan_changed` defectuoso, frontmatter frágil, CLI/JSON opaco y peaje DCC/PPR por binario externo (`F-GATE-*` / §5 del informe `docs/audits/pr-audit-aut-125-gate-anti-anemia-20261127.md`). Hijo de AUT-125; hermanos AUT-131 (forjas) y AUT-133 (deuda genoma).

**Canal PBI:** `tracker_ref` Linear `AUT-132` (forge-pbi `088f93c4…` / `PBI-9d7f7811`; outbound `.SddIA/state/tracker-outbound/AUT-132.jsonl`). El cuerpo Markdown del issue no está materializado como fichero local bajo el piloto; el alcance se estabiliza desde semilla de ejecución + REJECT + cascada padre + estado del código en rama.

## Decisiones de alcance

| ID | Decisión |
| --- | --- |
| Q1 | Keywords contractuales por familia son **efectivas**: `purpose_kw` y `second_kw` en headings `##` con cuerpo no vacío; sección operativa **distinta** del propósito. Prohibidos atajos por `sections.len() >= 2` o longitud de primera sección. |
| Q2 | `scan_changed` usa rango `base_ref...HEAD` (merge-base), `--diff-filter=d` (sin borrados), clasificación por `rank` + prefijo con límite `/`, compartida con `scan_all`. |
| Q3 | Frontmatter estricto: `---` en primera línea (BOM tolerado) + cierre; YAML inválido → `FRONTMATTER_YAML_INVALID` (no silencio). |
| Q4 | CLI `sddia-qa gate-entity-no-anemia`: `--base` estricto; errores respetan `--json`. |
| Q5 | Peaje DCC/PPR **in-process** (`entity_no_anemia::scan_changed`) con `base_ref` configurable (`default_base_ref` desde `.SddIA/project.md`). |
| Q6 | Scripts ad-hoc de remediación masiva (`rebuild_actions_index.py` / `remediate_bulk.py`) fuera del árbol; evolution canónico retiene traza AUT-132. |
| Q7 | `BODY_TRUNCATED` solo por firmas mecánicas (fence abierto o corte sin `\n` final en prosa). |

## Fuera de alcance

- Remediación global del genoma anémico / oráculo `violation_count: 0` global → **AUT-133**.
- Preservación/rollback de forjas, sync de índice de acciones, `entity_manager` update sin defaults → **AUT-131**.
- Rediseño de contratos de acción/familia ni cambios de SemVer/UUID de entidades ajenas al peaje.
- Aceptación/merge de AUT-125 (PR #480) ni higiene de atestación QA del padre.

## Herencia main / estado observado

La lógica funcional del peaje endurecido ya figura en el árbol (módulo `entity_no_anemia.rs`, CLI, `phase_capsules`, `pull_request_review`, tests de regresión keywords/`scan_changed`/frontmatter). Este ciclo **sella** regresiones, trazabilidad documental AUT-132 y evolution sin reabrir forjas ni la deuda de genoma.

**Ratchet vigente (laudo padre):** CI/PPR exigen `0` en `--if-touched`; línea base heredada **322** sin incremento (fuera de este PBI bajarla a 0).

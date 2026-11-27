---

feature_name: gate-anti-anemia-entidades-dominio
created: "2026-11-26"
process: refactorization
branch_name: refactor/gate-anti-anemia-entidades-dominio
persist_ref: docs/refactors/gate-anti-anemia-entidades-dominio
tracker_ref: AUT-125
execution_id: "982404c8-5fc6-49c1-94ac-4a66ee48dc56"
---
# Spec — gate anti-anemia entidades dominio

## Oráculo mecánico (KA-DOC-2)

| ID | Oráculo | Comando |
| --- | --- | --- |
| AC-1 | Ratchet anti-anemia | CI y PPR: `sddia-qa gate-entity-no-anemia --if-touched --base origin/main --json` → `violation_count: 0` en el delta. Línea base heredada fijada en **322** (`audit-inventory.json`); este cierre no admite incremento. El oráculo global a 0 queda fuera de AUT-125. |
| AC-2 | Contrafactual testigo | `cargo test -p execute-process entity_no_anemia` |
| AC-3 | Peaje DCC/PPR si toca genoma | `gate-entity-no-anemia --if-touched` en índices (phase_capsules + PPR) |
| AC-4 | Forja create/update acción | `orchestration_logic` con ≥2 `##` + post-validación anti-anemia con rollback |

AC-1 reformulado por laudo Racso (2026-11-27) vía `refine-item` `c3a91e04-7b2d-4f58-9a16-8e0d5c4b7a21` (`PBI_Refined` `eda://domain/b7529cd9-ae9c-4351-91a3-55e5b08ad7a6`). Sustituye el oráculo global `violation_count: 0`. La remediación de la línea base hasta 0 es **AUT-133**. **AUT-131** (forjas) y **AUT-132** (gate) son los otros hijos de la auditoría `REJECT` `3899742f…`.

## Validador `entity_no_anemia`

- Frontmatter obligatorio: `---` en la primera línea y cierre `---`; YAML inválido → `FRONTMATTER_YAML_INVALID`; `uuid` v4.
- Cuerpo: ≥2 secciones `##` con texto; sin stubs (`TODO`/`TBD`/`FIXME`/línea `...`) fuera de bloques de código.
- Keywords contractuales por familia en headings: sección de propósito **y** sección operativa distinta. Sin atajos por número de secciones ni por longitud.
- `BODY_TRUNCATED` solo ante firmas mecánicas: bloque de código sin cerrar, o fichero sin salto final cuya última línea es prosa sin puntuación de cierre.
- `NAME_FILENAME_MISMATCH` exento para `codex`.
- Clasificación por rango (event/process > resto > codex; a igual rango, prefijo más largo) con límite de directorio `/`, compartida por `scan_all` y `scan_changed`.

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
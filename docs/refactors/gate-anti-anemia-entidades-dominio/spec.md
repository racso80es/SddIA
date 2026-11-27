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
| AC-2 | Contrafactual testigo | `cargo test -p execute-process entity_no_anemia` |
| AC-3 | Peaje DCC/PPR si toca genoma | `gate-entity-no-anemia --if-touched` en índices (phase_capsules + PPR) |
| AC-4 | Forja create/update acción | `orchestration_logic` con ≥2 `##` + post-validación anti-anemia con rollback |

AC-1 sigue abierto tras la auditoría `REJECT` (`3899742f…`): la única vía honesta es la remediación real de AUT-133; mientras tanto el CI aplica el gate en modo ratchet (`--if-touched`).

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
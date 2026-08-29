---
feature_name: integridad-proceso-forge-ci
created: "2026-08-29"
process: bug-fix
branch_name: fix/integridad-proceso-forge-ci
persist_ref: docs/fixes/integridad-proceso-forge-ci
pbi_ref: docs/todos/pending/[FIX] Integridad de proceso — parse_frontmatter ciego, hash forge divergente y aduana CI opaca.md
document_id: PBI-FIX-INTEGRIDAD-PROCESO-FORGE-CI
uuid: 5a049a19-29ae-4c3b-adb0-a8b4e8d042fb
---

# Implementación — Integridad de proceso (forge, CI, DCC)

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/engine/execute-process/src/forges/common.rs` | L1 test CA1; `split_md_frontmatter` alineado con parser Core (`split("---")`); `patch_process_phases_update` acepta `workspace_template` |
| `SddIA/engine/execute-process/src/forges/factory.rs` | L2 test CA2; cableo `workspace_template` en update por fases |
| `.github/workflows/sddia-index-qa.yml` | L3: job `verify-tools-index` → `sddia-index-integrity` |
| `SddIA/engine/execute-process/src/engine/phase_capsules.rs` | L4: `capsule_index_integrity_audit_gate` + tests |
| `SddIA/engine/execute-process/src/engine/delivery_close.rs` | Dispatch fase «Aduana integridad índices»; `F-DCC-INDEX-INTEGRITY` |
| `SddIA/engine/execute-process/src/engine/residual_runner.rs` | Mismo dispatch |
| `SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md` | v1.3.0 vía `entity-manager`: fase «Aduana integridad índices»; L5 `workspace_template` sin `---` terminal; notas lab |
| `SddIA/library/codexes/codex-software-engineering/process/index.md` | Versión DCC 1.3.0 |

## L1 — Test parse_frontmatter

`parse_frontmatter_reads_uuid_when_workspace_template_ends_with_delimiter` — fixture con `workspace_template: …/---`.

## L2 — Test hash de fases

`process_forge_body_replacement_seals_phases_hash_not_artifact_hash` — `markdown_body_replacements` + `refresh_process_hash`; hash sellado = `sha256_phases_integrity(phases)` ≠ hash de artefacto completo.

## L3 — CI

Job renombrado; steps conservan nombre propio. **Antes del merge:** actualizar required check en branch protection de `main` (`sddia-index-integrity` sustituye `verify-tools-index`).

## L4 — DCC

Fase «Aduana integridad índices» entre Aduana EDA y Publicación remota. Handler invoca `verify-process-integrity` luego `verify-tools-index`; `status: blocked` sin `fail_soft`. Skip lab: `SDDIA_LAB_SKIP_INDEX_INTEGRITY`.

Forja: `entity-manager` (`.tmp/entity-manager-dcc-index-integrity.json` + body). Evento `Domain_Entity_Updated` `2094024e-8318-4c87-929c-1a57528e8dd5`. Hash fases: `sha256:83b193960647b429f9001469c73a616e8afa9cde31117c57583b0f9790d4ed01`.

## L5 — workspace_template

Sanado en la misma forja de fases (sin `---` pegado al path); frontmatter con delimitador de cierre explícito.

## Verificación local

```bash
cd SddIA && cargo test -p execute-process parse_frontmatter_reads_uuid
cd SddIA && cargo test -p execute-process process_forge_body_replacement
cd SddIA && cargo test -p execute-process index_integrity_gate
SddIA/target/debug/sddia-qa verify-process-integrity
SddIA/target/debug/sddia-qa verify-tools-index
```

## Pendiente pre-PR

- `execution.md` + Argos `validacion.md` + PBI a `done/` + `delivery-close-cycle`
- Branch protection: required check `sddia-index-integrity`
- Smoke CA4 manual (hash corrupto → DCC bloquea antes de push)

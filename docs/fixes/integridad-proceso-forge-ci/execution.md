---
feature_name: integridad-proceso-forge-ci
created: "2026-08-29"
process: bug-fix
branch_name: fix/integridad-proceso-forge-ci
persist_ref: docs/fixes/integridad-proceso-forge-ci
execution_id: "1dd48b02-251c-433a-85f8-bcfd7e93336e"
---

# Ejecución — Integridad de proceso (forge, CI, DCC)

## Fases

| Fase | Estado |
|------|--------|
| Inicialización | executed (`objectives.md`) |
| Diseño (Dedalo) | executed (`spec.md`, `plan.md`) |
| Ejecución (Tekton) | executed (`implementation.md`) |
| Verificación (Argos) | executed (`validacion.md`) |
| Cierre documental / delivery | pendiente (`delivery-close-cycle`) |

## Comandos ejecutados

```bash
cd SddIA && cargo build -p execute-process -p sddia-qa

./sddia-run.sh --process entity-manager --inputs-file .tmp/entity-manager-dcc-index-integrity.json
./sddia-run.sh --process entity-manager --inputs-file .tmp/entity-manager-dcc-index-integrity-body.json

cargo test -p execute-process parse_frontmatter_reads_uuid
cargo test -p execute-process process_forge_body_replacement
cargo test -p execute-process index_integrity_gate

SddIA/target/debug/sddia-qa verify-process-integrity
SddIA/target/debug/sddia-qa verify-tools-index
```

## entity-manager (L4 + L5)

- Proceso: `delivery-close-cycle` v1.3.0
- Fase nueva: «Aduana integridad índices»
- `workspace_template`: `.SddIA/workspaces/{process_name}/{execution_id}/` (sin colisión `---`)
- Hash fases: `sha256:83b193960647b429f9001469c73a616e8afa9cde31117c57583b0f9790d4ed01`

## Nota forja

Primer intento de `entity-manager` falló con `frontmatter ausente (sin --- de cierre)` en DCC legacy. Corregido `split_md_frontmatter` en `forges/common.rs` (paridad Core `split("---")`) antes de reintentar.

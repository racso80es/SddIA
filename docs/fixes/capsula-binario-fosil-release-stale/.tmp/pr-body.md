## Summary
- Patrón de Anclaje: genoma `source_sha256` + testigo ELF + aduana `SDDIA_CAPSULE_ANCHOR`
- `capsule_seal` CLI / entity-manager `seal-anchor`; porcelain UTF-8; fractura F-DIRTY-WORKTREE
- Rebuild release publisher; cola reanchor drenada; PBI R1 archivado

## Test plan
- [x] cargo test -p execute-process capsule_paths capsule_digest
- [x] cargo test -p iota-immutable-publisher
- [x] validacion.md APTO + pbi_archived: true
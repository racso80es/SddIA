## Summary

- Purga del PBI Kaizen duplicado en `docs/todos/pending/` (canónico ya en `done/` post PR #124).
- Archiva el PBI OPERATIVO de purga en `docs/todos/done/` con cascada `docs/fixes/pbi-stale-pending-purge-ppr-124/` APTO.

## Test plan

- [x] Path stale ausente en `pending/`
- [x] Canónico presente en `done/`
- [x] `validacion.md` con `global: APTO` y `pbi_archived: true`

## Summary
- Rehabilita `bug-fix` en Cerbero/Radamanto (A1 instancia; fuera del PR).
- Corrige ontología fósil `tool` → `process` (jurisprudencia #174); motor intacto.
- Cascada documental + PBI archivado en la misma rama.

## Test plan
- [ ] `bug-fix` ausente de `.SddIA/cerbero/revoked_entities.json` (`revoked`/`permanent`)
- [ ] Stats raíz `bug-fix`: `healthy`, `entity_type: process`
- [ ] Diff PR sin `.SddIA/cerbero/` ni `.SddIA/radamanto/` ni umbrales
- [ ] `validacion.md` APTO y PBI en `docs/todos/done/`
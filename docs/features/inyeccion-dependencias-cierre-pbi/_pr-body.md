## Summary
- Done global PBI-042 (R15): archivo del PBI padre tras MVP + Hitos 2–6 en main.
- Cascada documental `inyeccion-dependencias-cierre-pbi` + evolution `d4e8f1a3-…` (traza MVP→H6→R15).
- PBI movido a `docs/todos/done/` (`status: cerrado`, v1.2.1); `validacion` APTO + `pbi_archived: true`.
- Genoma DI intacto (sin gate/resolver/Cerbero/bindings/taxonomía); H7/EDA-only diferidos.

## Test plan
- [x] PBI exclusivo en `docs/todos/done/` (ausente en `pending/`)
- [x] Cascada clarify→validacion + evolution presentes
- [x] `validacion.md` `global: APTO` + `pbi_archived: true`
- [x] Diff sin mutación de genoma DI
- [ ] CI PR checks verdes

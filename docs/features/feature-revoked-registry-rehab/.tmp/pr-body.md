## Summary
- Fail-soft del padre `feature` cuando el hijo `delivery-close-cycle` falla con umbral físico (`pr_url` / `delivery_push`) y cola secundaria.
- Poda de telemetría hueca (`lab_hollow` / `cycle_phase` no terminal) antes de `samples` Radamanto.
- Cierre documental `PBI-FEATURE-185-REVOKED-REGISTRY` en la misma rama.
- Rehab de instancia Cerbero/stats (A1) fuera del diff git.

## Test plan
- [ ] `cargo test -p execute-process --lib -- feature_dcc_parent_fail_soft is_survival_hollow`
- [ ] Diff sin `.SddIA/cerbero/` ni `.SddIA/radamanto/` ni `radamanto.thresholds.json`
- [ ] `validacion.md` `global: APTO` y `pbi_archived: true`
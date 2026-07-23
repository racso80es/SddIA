## Summary
- Hito 3 (H9) PBI-043: laudos Racso — alta `qa:probe` + extensión DI `tool:` + provides en tools caos/audit; `audit:compliance` exclusiva para `telemetry-compliance-audit` (sin reuso qa:probe).
- Runtime: gate/resolver admiten providers `tool:`; preferencia delegates con `provides`.
- Ola N_ola=5 ED §3.3; skill `compliance-auditor`; taxonomía v1.0.4; bindings v1.3.0; orphan 0; DI 24/24.
- `llm:interact` catalogada (H10-A); §3.4 restante **defer** (H10-B). `pbi_archived: false`.

## Test plan
- [x] Taxonomía: qa:probe, audit:compliance (+ llm:interact catalog)
- [x] 4 tools provide qa:probe; telemetry-compliance-audit → audit:compliance
- [x] Inventario 34/8
- [x] orphan_count 0
- [x] cargo capability_di + cerbero_di
- [ ] CI PR checks verdes

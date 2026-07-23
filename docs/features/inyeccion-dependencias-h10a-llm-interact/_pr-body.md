## Summary
- H10-A PBI-043 (laudo Racso): `provides: llm:interact` en `skill:mayeuta-llm` v1.1.0; homologación DI `kalma2-interact` (Clasificación + Síntesis).
- Códice/bindings ya catalogados en H9; H10-B §3.4 **defer**. Inventario 35 with / 7 without; orphan 0.
- `pbi_archived: false`.

## Test plan
- [x] mayeuta-llm provides llm:interact
- [x] kalma2-interact fases con requires_capability llm:interact
- [x] orphan_count 0
- [x] Inventario 35/7 (residual = H10-B)
- [ ] CI PR checks verdes

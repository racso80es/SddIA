## Summary
- Cápsula WASI `sddia-evolution-register`: cotejo `diff`×`registry` inyectado (cero Git en dominio).
- CLI `sddia-qa gate-evolution` / `evolution-register`: captura árbol, inyección JSON, persistencia atómica.
- Aduana pre-commit (detonador inerte) + CI `gate-evolution --json --range`. Contrato evolution **1.1.1**. Fail-hard solo delta (universo 61 no se certifica).
- Cierre documental: PBI `70f78d23-…` en `docs/todos/done/` + `validacion.md` APTO en esta rama.

## Test plan
- [x] `cargo test -p sddia-evolution-register` (14 passed)
- [x] `sddia-qa gate-evolution --json` → EVOL_OK (L-SELF / sin staged material)
- [x] Fixture material unregistered / hash / not-indexed / alta dup / idempotente / baja / modificación
- [ ] CI job `wasi-runtime-smoke` → `gate-evolution --json --range`
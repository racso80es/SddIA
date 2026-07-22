## Summary
- Hito 1 (H7) PBI-043: homologar 8 ED núcleo FS (§3.1) con DI `fs:persist` (path ciego preferente / mixto documentado).
- Ola vía `entity-manager` update → v1.0.1: `route-domain-event`, `daemon-kill-switch`, `governance-daemon-manager`, `daemon-heartbeat-audit`, `fix-tool-process`, `telemetry-batch-stub`, `workspace-smoke`, `entity-manager`.
- Sellos `Domain_Entity_Updated` ×8; `orphan_count: 0`; regresión DI capability_di+cerbero_di 24/24.
- `validacion` APTO; `pbi_archived: false` (PBI-043 multi-hito permanece en pending; H8–H10 fuera).

## Test plan
- [x] 8/8 ED con `requires_capability` → `fs:persist` y sin `delegates_to skill:filesystem-manager` en fases anotadas
- [x] `sddia-qa audit-eda-coverage --scan --json` → orphan_count 0
- [x] `cargo test -p execute-process capability_di cerbero_di`
- [ ] CI PR checks verdes

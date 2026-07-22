## Summary
- Binding table SSOT: capability-bindings.md + capability_di.bindings in Cumulo 1.5.3
- Blind resolver: capability_di_resolver (resolve then gate then ignition) without hardcoded delegates_to
- Inject di_binding into capsule-json-io v2 envelope
- Expanded pilot: feature and bug-fix closure phases use only requires_capability

## Notes
- validacion.md global APTO (Hito 2 scope), pbi_archived false
- PBI-042 remains in pending (Hito 3 residual R5-R8)
- Library_Codex / taxonomy not used as DI router

## Test plan
- [x] cargo test -p execute-process capability_di (12 passed)
- [x] sddia-qa verify-process-integrity
- [x] sddia-qa audit-eda-coverage --scan (orphan_count=0)
- [ ] Residual PBI not archived to done
- [ ] Smoke blind path for documental closure (doc:closure -> filesystem-manager)

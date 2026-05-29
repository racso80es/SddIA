---
uuid: "b2c3d4e5-f6a7-4890-b123-456789abcdef"
name: core-full-stress
version: "1.0.0"
contract: suites-contract v1.0.0
context:
- chaos-engineering
hash_signature: sha256:9a2c9edf19e0daceffd73659a4f99fb2961c0742b60c0079207e9143a0d65c6c
execution_strategy: run_all
atomic_nodes:
- process_name: audit-thermodynamic-toll-failsoft
  expected_exit_code: 0
  timeout_ms: 120000
- process_name: audit-telemetry-compliance-breach
  expected_exit_code: 0
  timeout_ms: 120000
- process_name: audit-sandbox-isolation-rbac
  expected_exit_code: 0
  timeout_ms: 120000
---

# core-full-stress

**Códice de Asedio** — Suite referencia Fase 3 Caos. Orquesta los tres procesos audit atómicos de Fase 2 en campaña `run_all` antes del manifiesto de supervivencia Argos.

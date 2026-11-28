---
uuid: "b2c3d4e5-f6a7-4890-b123-456789abcdef"
name: core-full-stress
version: "1.0.0"
contract: suites-contract v1.0.0
context:
- chaos-engineering
hash_signature: "sha256:42e5ca2beaaf2da05905919753ae23d7e904c118e83b59c9316ecf93e4e73f42"
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

## Propósito

Suite referencia Caos Fase 3: campaña `run_all` sobre procesos audit Fase 2.

## Estrategia de ejecución

Nodos: `audit-thermodynamic-toll-failsoft`, `audit-telemetry-compliance-breach`, `audit-sandbox-isolation-rbac`; timeout 120s cada uno.

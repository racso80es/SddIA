---
uuid: "b2c3d4e5-f6a7-4890-b123-456789abcdef"
name: core-full-stress
version: "1.0.0"
contract: suites-contract v1.0.0
context:
- chaos-engineering
hash_signature: "sha256:748ccf5e3c2c84c6ce6db7694f55d64e223e7fddd28156587946cfb42fa14556"
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

## Propósito

**Códice de Asedio** — Suite referencia Fase 3 Caos. Orquesta los tres procesos audit atómicos de Fase 2 en campaña `run_all` antes del manifiesto de supervivencia Argos.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.

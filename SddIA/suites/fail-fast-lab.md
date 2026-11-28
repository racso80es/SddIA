---
uuid: "eee923b0-5eec-43fa-abc1-835246278f2d"
name: fail-fast-lab
version: "1.0.0"
contract: suites-contract v1.0.0
context:
- chaos-engineering
hash_signature: "sha256:dc1c4ca57be0d183dcdad5cbb269ff1295d2f91826204ac94676332891df8470"
execution_strategy: fail_fast
atomic_nodes:
- process_name: audit-sandbox-isolation-rbac
  expected_exit_code: 99
  timeout_ms: 120000
- process_name: audit-thermodynamic-toll-failsoft
  expected_exit_code: 0
  timeout_ms: 120000
---

# fail-fast-lab

## Propósito

Lab `fail_fast`: primer nodo audit con exit esperado 99 vs real 0 → aborta suite.

## Estrategia de ejecución

`execution_strategy: fail_fast`; nodos en `atomic_nodes` (execute-suite).

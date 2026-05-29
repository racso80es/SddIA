---
uuid: "c3d4e5f6-a7b8-4901-c123-456789abcdef"
name: fail-fast-lab
version: "1.0.0"
contract: suites-contract v1.0.0
context:
- chaos-engineering
hash_signature: sha256:7c4bc74058ccbe78e202d2c3cd44f68fbb8d5b934c1bcdd11b89f116b7d221b1
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

Suite **laboratorio** para validar estrategia `fail_fast`: el nodo 0 declara `expected_exit_code: 99` pero el proceso audit retorna `0` → veredicto `fail` → aborta antes del nodo 1.

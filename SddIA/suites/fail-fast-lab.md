---
uuid: "f073a542-e0ae-4d19-8a01-be18c59de080"
name: fail-fast-lab
version: "1.0.0"
contract: suites-contract v1.0.0
context:
- chaos-engineering
hash_signature: "sha256:47b85993a893fed18ab32cf87fb5bc5ab41a30c2d2c3ee1062a9af8fbf56e74c"
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

## Propósito

Suite **laboratorio** para validar estrategia `fail_fast`: el nodo 0 declara `expected_exit_code: 99` pero el proceso audit retorna `0` → veredicto `fail` → aborta antes del nodo 1.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.

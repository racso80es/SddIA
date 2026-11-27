---
uuid: "abdafa2f-bfea-4b30-ab2b-4fbafbdcb903"
name: "kalma2-bridge"
version: "1.0.0"
contract: "daemons-contract v1.0.0"
context: "system-operations"
hash_signature: "sha256:380aeb34979a9dbdebdeae11f08d77045588d311d38f35b093d32684c8e07aaa"
capabilities:
  - "kalma2-http-bridge"
  - "daemon-heartbeat"
execution:
  entrypoint: "SddIA/scripts/daemons/kalma2-bridge.sh"
  runtime: "native-rust"
  heartbeat_interval_seconds: 30
jurisdiction: "Órgano de interfaz HTTP. Circuito de vitalidad; no Ceguera Lógica plena."
telemetry_provided: true
telemetry_schema:
  - "uptime_seconds"
  - "pid"
  - "status"
---

# kalma2-bridge

Órgano de interfaz HTTP Kalma2. Circuito de vitalidad (lock, side-channel, Daemon_Heartbeat). No es sensor periférico; excepción a Ceguera Lógica plena del daemons-contract §2.

Forja: `daemon-creator` (porte nativo `run_daemon_forge`). UUID vía `action:crypto-broker`.

## Propósito

Órgano de interfaz HTTP Kalma2. Circuito de vitalidad (lock, side-channel, Daemon_Heartbeat). No es sensor periférico; excepción a Ceguera Lógica plena del daemons-contract §2.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.

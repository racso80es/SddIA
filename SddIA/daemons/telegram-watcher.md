---
uuid: "89a10029-d4bd-4abc-bb08-ff59f6faf17f"
name: "telegram-watcher"
version: "1.0.0"
contract: "daemons-contract v1.0.0"
context: "peripheral-sensing"
hash_signature: "sha256:d3eb6293295d3f417d1d73dd20205db13ac5aa33ec377f8fda35b22442c7ae05"
capabilities:
  - "telegram-long-poll"
execution:
  entrypoint: "SddIA/scripts/daemons/telegram-watcher.py"
  runtime: "python3"
  heartbeat_interval_seconds: 30
jurisdiction: "Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus"
telemetry_provided: true
telemetry_schema:
  - "uptime_seconds"
  - "pid"
  - "status"
---

# telegram-watcher

Centinela Capa 0: long polling Telegram → `telegram-gateway`. Emite `Daemon_Heartbeat` cada 30s.

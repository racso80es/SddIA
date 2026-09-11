---
uuid: "89a10029-d4bd-4abc-bb08-ff59f6faf17f"
name: "telegram-watcher"
version: "1.1.0"
contract: "daemons-contract v1.0.0"
context: "peripheral-sensing"
hash_signature: "sha256:07bf4191783afa4a818300273dd4e48a885b01e0faa71ea63fbd3062bcd2fe40"
capabilities:
  - "telegram-long-poll"
execution:
  entrypoint: "SddIA/daemons/telegram-watcher.sh"
  runtime: "native-rust"
  heartbeat_interval_seconds: 30
jurisdiction: "Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus"
telemetry_provided: true
telemetry_schema:
  - "uptime_seconds"
  - "pid"
  - "status"
---

# telegram-watcher

Centinela Capa 0: long polling Telegram → `telegram-gateway` vía `execute-process`. Intercepta `message`/`edited_message` y `callback_query`; cierra spinner con `answerCallbackQuery`. No interpreta preferencias. Binario Rust en `SddIA/target/{release|debug}/telegram-watcher`; launcher `SddIA/daemons/telegram-watcher.sh`. Emite `Daemon_Heartbeat` cada 30s.

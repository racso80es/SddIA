---
uuid: "89a10029-d4bd-4abc-bb08-ff59f6faf17f"
name: "telegram-watcher"
version: "1.1.0"
contract: "daemons-contract v1.0.0"
context: "peripheral-sensing"
hash_signature: "sha256:5a739009a4d2799a1c3ebcf945acf2a390a3c455766e453ec5676376897abf3a"
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

## Propósito

Centinela Capa 0: long polling Telegram → `telegram-gateway` vía `execute-process`. Intercepta `message`/`edited_message` y `callback_query`; cierra spinner con `answerCallbackQuery`. No interpreta preferencias. Binario Rust en `SddIA/target/{release|debug}/telegram-watcher`; launcher `SddIA/daemons/telegram-watcher.sh`. Emite `Daemon_Heartbeat` cada 30s.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.

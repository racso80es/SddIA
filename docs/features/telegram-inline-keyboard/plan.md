---
feature_name: telegram-inline-keyboard
created: "2026-09-11"
process: feature
phases:
  - factory-tool-update
  - capsule-send
  - watcher-callback
  - gateway-xor
  - entity-manager
  - evolution-eda
---

# Plan — telegram-inline-keyboard

1. `run_tool_forge` update + `markdown_body_replacements`.
2. Capsula `send-telegram-notification`.
3. `telegram-watcher` extract + answer + invoke.
4. Handler y tool `telegram-gateway` XOR.
5. EM create event; update tool/process/daemon.
6. `event-domain-subscriptions.json` clave vacía; evolution + eda-coverage.
7. Tests locales; DCC cuando CA locales verdes.

---
feature_name: telegram-watcher-heartbeat-fracture-67a56998121e
created: "2026-06-20"
process: bug-fix
branch: fix/telegram-watcher-heartbeat-fracture-67a56998121e
---

# Ejecución — telegram-watcher heartbeat fracture

## Comandos

```bash
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build -p telegram-watcher
SddIA/target/debug/telegram-watcher --once   # exit 2 sin TELEGRAM_* (esperado)
```

## Rama

`fix/telegram-watcher-heartbeat-fracture-67a56998121e`

## PBI

`docs/todos/pending/[FIX] telegram-watcher — fractura sistémica (67a56998121e).md`

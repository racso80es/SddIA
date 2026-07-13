---
feature_name: eda-outbound-native-remediation
created: "2026-07-13"
process: bug-fix
items_applied:
  - build-send-telegram-notification
  - build-iota-immutable-publisher
  - unit-tests-outbound
  - smoke-stdin-tools
---

# Ejecución — remediación outbound EDA

## Comandos

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo build -p sddia-io -p send-telegram-notification -p iota-immutable-publisher
cd SddIA && cargo test -p sddia-io -p send-telegram-notification -p iota-immutable-publisher

# Smoke lab (sin red externa)
export SDDIA_LAB_MOCK_OUTBOUND=1
printf '%s' '{"message":"smoke lab"}' | SddIA/target/debug/send-telegram-notification
printf '%s' '{"action":"publish_immutable_data","network":"testnet","payload":"{}"}' | SddIA/target/debug/iota-immutable-publisher
```

## Resultados

| Paso | Estado | Evidencia |
|------|--------|-----------|
| Build tools + sddia-io | ✅ | `Finished dev profile` |
| Tests unitarios (5) | ✅ | send-telegram 2, iota 2, outbound_lab 1 |
| Smoke Telegram lab | ✅ | `success:true`, `mode: lab-mock-outbound` |
| Smoke IOTA lab | ✅ | `success:true`, `transaction_digest: lab-sim-*` |
| Build release tools | ✅ | Requerido: SSOT prioriza `target/release/` sobre `debug` |
| Integración `route-domain-event` lab | ✅ | 3/3 suscriptores `success`, `sweep: purged` |
| Errores `config-missing` | ✅ | Telegram + IOTA sin credenciales |
| Re-auditoría `event-bus-audit` | ✅ | `pending: 0`, `stale: 0` |

## Pendiente (fases posteriores)

- Verificación Argos (`validacion.md`)
- Integración `route-domain-event` con fixture PR lab
- Re-auditoría `event-bus-audit` en entorno con centinelas activos
- Cierre documental PBI → `docs/todos/done/`

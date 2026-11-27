---
uuid: "773a11e7-3a42-4eba-a383-79dd6ef8c263"
name: "email-watcher"
version: "1.0.0"
contract: "daemons-contract v1.0.0"
context: "peripheral-sensing"
hash_signature: "sha256:f9e31a1859fbb9d52b85cd7a597f593579edff62d31376ac259a2832029b6d8a"
capabilities:
  - "imap-mailbox-poll"
  - "email-stimulus-injection"
execution:
  entrypoint: "SddIA/daemons/email-watcher.sh"
  runtime: "native-rust"
  heartbeat_interval_seconds: 30
jurisdiction: "Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus"
telemetry_provided: true
telemetry_schema:
  - "uptime_seconds"
  - "pid"
  - "status"
---

# email-watcher

Centinela Capa 0: sondeo IMAP **read-only** (`EXAMINE` + `BODY.PEEK`) → instancia `Email_Received` en `eda_fractal.domain`. No invoca orquestador. No interpreta veredicto. Watermark UID en estado de instancia (`.SddIA/daemons/state/email-watcher.json`) con `imap_identity_sha256` sobre `{host}|{port}|{mailbox}|{user}`. Ante cambio de cuenta IMAP o watermark por encima del techo del buzón, bootstrap automático (F-07, lote ≤50). Emite `Daemon_Heartbeat` cada 30s.

Forja de definición: in-ciclo (F-01: `entity-manager` no declara clase `daemon`; `daemon-creator` sin porte nativo). UUID vía `action:crypto-broker`.

## Propósito

Centinela Capa 0: sondeo IMAP **read-only** (`EXAMINE` + `BODY.PEEK`) → instancia `Email_Received` en `eda_fractal.domain`. No invoca orquestador. No interpreta veredicto. Watermark UID en estado de instancia (`.SddIA/daemons/state/email-watcher.json`) con `imap_identity_sha256` sobre `{host}|{port}|{mailbox}|{user}`. Ante cambio de cuenta IMAP o watermark por encima del techo del buzón, bootstrap automático (F-07, lote ≤50). Emite `Daemon_Heartbeat` cada 30s.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.

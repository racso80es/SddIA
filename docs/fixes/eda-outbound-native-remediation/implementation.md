---
feature_name: eda-outbound-native-remediation
created: "2026-07-13"
process: bug-fix
items:
  - sddia-io/outbound_lab.rs
  - tools/send-telegram-notification
  - tools/iota-immutable-publisher
---

# Implementación — remediación outbound EDA

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/sddia-io/src/outbound_lab.rs` | Helpers compartidos: flags lab, mock URLs, wallet IOTA, digest simulado |
| `SddIA/sddia-io/Cargo.toml` | Dependencia `uuid` |
| `SddIA/tools/send-telegram-notification/src/main.rs` | HTTP nativo (`ureq`) + modos lab/mock; stub explícito solo en `wasm32` |
| `SddIA/tools/send-telegram-notification/Cargo.toml` | Dependencia `ureq` |
| `SddIA/tools/iota-immutable-publisher/src/main.rs` | Simulación lab, mock HTTP, relay opcional; stub solo en `wasm32` |
| `SddIA/tools/iota-immutable-publisher/Cargo.toml` | Dependencia `ureq` |

## Contrato operativo

### Variables de entorno (sin secretos en código)

| Variable | Tool | Efecto |
|----------|------|--------|
| `SDDIA_LAB_MOCK_OUTBOUND` | ambos | Éxito local sin red |
| `SDDIA_LAB_SIMULATE_IOTA` | IOTA | Digest `lab-sim-*` |
| `SDDIA_LAB_MOCK_TELEGRAM_URL` | Telegram | POST a doble HTTP |
| `SDDIA_LAB_MOCK_IOTA_URL` | IOTA | POST a doble HTTP |
| `SDDIA_LAB_SKIP_OUTBOUND_TELEGRAM` | Telegram | Éxito `skipped-lab-no-credentials` |
| `TELEGRAM_BOT_TOKEN` / `TELEGRAM_ALLOWED_CHAT_ID` | Telegram | Envío real API |
| `IOTA_WALLET_SECRET` / `.SddIA/.dev/wallet.key` | IOTA | Prerequisito publicación física |
| `IOTA_PUBLISH_RELAY_URL` | IOTA | Relay HTTP instancia (sin wallet en JSON) |

### Errores clasificados

- `config-missing: TELEGRAM_BOT_TOKEN or TELEGRAM_ALLOWED_CHAT_ID`
- `config-missing: IOTA_WALLET_SECRET`
- `iota-publish-unavailable: ...` (sin relay ni modo lab)
- `telegram-http-failed` / `telegram-api-rejected`

## Decisiones

1. **Sin cambio en `route_domain_core`** — ya invoca con `prefer_wasm: false`.
2. **WASI aislado** — `#[cfg(target_arch = "wasm32")]` mantiene fallo explícito sin red.
3. **IOTA físico** — relay HTTP opcional por instancia; SDK completo fuera de alcance inmediato.
4. **Telegram** — paridad con contrato tool: fallback plain tras error 400 con `parse_mode`.

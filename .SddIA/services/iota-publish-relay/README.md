# iota-publish-relay (instancia)

Relay HTTP local que firma y publica en IOTA Testnet. La cápsula nativa `iota-immutable-publisher` lo invoca vía `IOTA_PUBLISH_RELAY_URL`.

## Contrato

`POST /v1/publish`

```json
{ "action": "publish_immutable_data", "network": "testnet", "payload": "<string>" }
```

Respuesta OK:

```json
{ "success": true, "result": { "transaction_digest": "...", "object_id": "..." } }
```

`GET /health` → `{ "ok": true }`.

## Bóveda (`.SddIA/.dev/.env`)

| Variable | Valor adecuado (físico) |
|----------|-------------------------|
| `SDDIA_LAB_SIMULATE_IOTA` | `0` |
| `SDDIA_LAB_MOCK_IOTA_URL` | **vacío** (si se setea, precede al relay y no ancla on-chain) |
| `IOTA_PUBLISH_RELAY_URL` | `http://127.0.0.1:8787/v1/publish` |
| `IOTA_WALLET_SECRET` | wallet Testnet |
| `IOTA_ANCHOR_PACKAGE_ID` | package Move ya publicado |

## Arranque

```bash
export PATH="$(pwd)/.tools/node-v22.16.0-linux-x64/bin:$PATH"
cd .SddIA/services/iota-publish-relay
npm install
npm start
```

---
feature_name: telegram-route-ecst-gate
branch: Fix_Events_Multiple
tracker_ref: AUT-60
global: APTO
pbi_archived: false
---

# Validación — AUT-60

## Evidencia

```text
cd SddIA && cargo build -p execute-process
cargo test -p execute-process --test eda_subscriber_ecst_redb route_domain_telegram -- --nocapture --test-threads=1
→ ok (2 passed)
```

## Criterios

| Ítem | Estado |
|------|--------|
| Catálogo ECST en repo temporal de integración | APTO |
| Payload Telegram con `chat_id` / `source` | APTO |
| Texto vacío → `skipped-empty-text` (no corte solo `ecst-gate`) | APTO |
| Texto presente → despacho más allá de aduana ECST | APTO |

Cierre Linear (`state_map.done`) y merge del PR en `Fix_Events_Multiple` pendientes de operador.

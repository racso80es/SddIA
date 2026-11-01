---
feature_name: telegram-route-ecst-gate
branch: Fix_Events_Multiple
tracker_ref: AUT-60
global: APTO
pbi_archived: false
validated_base_sha: 2d14642644f1acd7ebfa1fa32bac0f941f817736
validated_head_sha: c682cce544302c0fcabcb9acde43d094cdc9555c
validated_tree_sha: ba5f84ba576acde35df7112b943dadaf2314e117
diff_digest: sha256:ea4f358486c5b8f6856c1f2b323241030545cb39c5b7a4540e491c8f97717103
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
---
feature_name: telegram-route-ecst-gate
branch: Fix_Events_Multiple
tracker_ref: AUT-60
global: APTO
pbi_archived: false
validated_base_sha: 2d14642644f1acd7ebfa1fa32bac0f941f817736
validated_head_sha: e91a7c5519aa34176269af8a963310bf241cb17b
validated_tree_sha: eb3d8417a550799fd8a713af8053e89f3e8624cf
diff_digest: sha256:c0b4d4f5ea07ed2983bcd69e199a4ec738633dcd36ba72ab4163d223ace5ffb2
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
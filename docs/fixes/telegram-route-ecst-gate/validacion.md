---
feature_name: telegram-route-ecst-gate
branch: Fix_Events_Multiple
tracker_ref: AUT-60
global: APTO
pbi_archived: false
validated_base_sha: "2d14642644f1acd7ebfa1fa32bac0f941f817736"
validated_head_sha: "6ac2b651c54bbbab8f5b14cb1cc4eb6aaabb7fa4"
validated_tree_sha: "520520befed83c2b688eaaf303c1320b1414b6ab"
diff_digest: "sha256:0971d1c7bde471cbf30ba394620c0c3513f6cd6c3ee32f41c29969fe93134121"
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
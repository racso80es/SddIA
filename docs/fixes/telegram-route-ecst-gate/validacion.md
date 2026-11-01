---
feature_name: telegram-route-ecst-gate
branch: Fix_Events_Multiple
tracker_ref: AUT-60
global: APTO
pbi_archived: false
validated_base_sha: 2d14642644f1acd7ebfa1fa32bac0f941f817736
validated_head_sha: 91e5c2d9257adfb2ddd35f82eda3e42239d4c04f
validated_tree_sha: 2baf055435cc8bda43a8a8e7374f90b000e42519
diff_digest: sha256:4ee930499400b410983193f3b69382d074fc338896fe74c42a18218624c27688
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
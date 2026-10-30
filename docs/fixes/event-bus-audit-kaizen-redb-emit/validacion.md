---
fix_name: event-bus-audit-kaizen-redb-emit
global: APTO
branch: Kaicen_Auditoria_Eventos_261030
persist_ref: docs/fixes/event-bus-audit-kaizen-redb-emit
validated_base_sha: d906cc8e843effdb991cecd3190080a2c073d738
validated_head_sha: 9a0ce325c23c84cacf3918ad4ff75a028f7dc7a8
validated_tree_sha: 769dc4d62a4ade8eb37d467d2a89887a7b277637
diff_digest: sha256:22ea3f508dc69d29e31516341abe066752f14deb318a643f8985eb2a5f9868f1
---

# Validación — event-bus-audit Kaizen redb emit

## Pruebas

```bash
cd SddIA && cargo test -p event-bus-audit -- --test-threads=1
```

| Test | Resultado |
|------|-----------|
| `kaizen_emit_redb_uses_store_not_pending_file` | PASS |
| `kaizen_emit_filesystem_writes_pending_json` | PASS |
| suite completa (9 tests) | PASS |

## Criterios

- AC-1 redb: verificado por test EBA-REDB-01
- AC-2 filesystem: verificado por test EBA-FS-01
- `emit_kaizen_alert` sin cambio semántico
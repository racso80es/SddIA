---
fix_name: event-bus-audit-kaizen-redb-emit
global: APTO
branch: Kaicen_Auditoria_Eventos_261030
persist_ref: docs/fixes/event-bus-audit-kaizen-redb-emit
validated_base_sha: d906cc8e843effdb991cecd3190080a2c073d738
validated_head_sha: bbcf1a998b988683ad08d58571140cbbdb16df02
validated_tree_sha: f2895c56578418382da76e0e4e9703ebea94f1b3
diff_digest: sha256:ebd18660bbad45c412f43586f8794c02c81f21ee19f8cb7f53cc837036d105e6
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
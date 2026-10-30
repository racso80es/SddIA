---
fix_name: event-bus-audit-kaizen-redb-emit
global: APTO
branch: Kaicen_Auditoria_Eventos_261030
persist_ref: docs/fixes/event-bus-audit-kaizen-redb-emit
validated_base_sha: d906cc8e843effdb991cecd3190080a2c073d738
validated_head_sha: 978cae42a1dec809223b781b8d8e281b77804ae0
validated_tree_sha: bd0b5aaf506231d64635185b5821dbf81cfc2d24
diff_digest: sha256:2b120790a0d21b87f1655f909bfb288ef736d332f0ddf04e4b6ed2a2d8b0526c
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
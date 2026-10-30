---
fix_name: event-bus-audit-kaizen-redb-emit
global: APTO
branch: Kaicen_Auditoria_Eventos_261030
persist_ref: docs/fixes/event-bus-audit-kaizen-redb-emit
validated_base_sha: d906cc8e843effdb991cecd3190080a2c073d738
validated_head_sha: c2bfdc425e9037e03849a6be40d509ee055f0cbd
validated_tree_sha: 8486e4e6dbd03fd70343519940f5207d83864e36
diff_digest: sha256:232b37079b79b8c231714fde2c0bb474f4e71a956a3e7b38c4913a298e419799
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

---
feature_name: pbi-005-hito2-action-engine
created: "2026-05-20"
process: feature
items_applied:
  - feature-init-execute-process
  - bus-operator-forge
  - smoke-tests
---

# Ejecución — registro físico

## Inicialización feature

```powershell
python SddIA/scripts/qa/execute-process.py --input-file tmp/feature-pbi005-hito2-init.json
```

Salida: rama `feat/pbi-005-action-engine`, `docs/features/pbi-005-action-engine/objectives.md`.

## Pruebas de humo (ver validacion.md)

- `markdown-table-editor` → `parse` sobre `SddIA/skills/index.md`
- `execute-action` → `sync-entity-index` vía `bus-operator`
- `event-watcher.py --once` con evento sintético en `pending/`

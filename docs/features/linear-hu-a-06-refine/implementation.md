---
feature_name: linear-hu-a-06-refine
---

# Implementación — HU-A 06

- `refine-pbi`: valida `pbi_ref` bajo `todos_pending`, sella `status: refinado`, emite `PBI_Refined`.
- `refine-hu`: valida `hu_ref` bajo `historias/`, `create_issue` si hay tracker y falta `tracker_ref`; sella `status: refinada`; emite `HU_Refined`; `Tracker_Sync_Failed` con `operation: create_issue` en fail-soft.
- Cerbero: `resolve_requester_policies` admite `context` como lista comma-separated (forjas `process-creator`).

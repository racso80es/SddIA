---
feature_name: laboratorio-handlers-l2-l3
created: "2026-05-24"
process: feature
items:
  - execute_process_capsules.py
  - delivery-close-cycle.md
  - feature.md
---

# Implementación — Laboratorio handlers L.2 + L.3

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/execute_process_capsules.py` | `capsule_delivery_impact_assessment`, `capsule_feature_pbi_archive`, `capsule_feature_invoke_delivery_close`, routing L.2/L.3 |
| `SddIA/process/delivery-close-cycle.md` | § fase 2 lab + `SDDIA_LAB_SKIP_IMPACT_ASSESSMENT` |
| `SddIA/process/feature.md` | § fases 6–7 lab + variables skip |

## Cápsulas nuevas

### L.2 — `capsule_delivery_impact_assessment`

- Diff `git diff --name-only origin/<target>...<branch>`.
- Filtra paths `SddIA/`.
- Skip si `source_process != feature` o `SDDIA_LAB_SKIP_IMPACT_ASSESSMENT`.

### L.3 — `capsule_feature_pbi_archive`

- Lee `{persist_ref}/validacion.md` (`global: APTO`, `pbi_archived: true`).
- Resuelve PBI vía `related_todo` o frontmatter `objectives.md`.
- Move atómico `docs/todos/pending/` → `docs/todos/done/`.

### L.3 — `capsule_feature_invoke_delivery_close`

- Subproceso `delivery-close-cycle` con `source_process: feature`.
- Propaga `pr_url`, `event_id`, `target_path`.

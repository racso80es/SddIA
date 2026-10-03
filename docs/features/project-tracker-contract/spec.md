---
feature_name: project-tracker-contract
created: "2026-10-02"
process: feature
---

# Especificación — project-tracker-contract

## Contrato `project-config-contract` v1.2.0

Bloque opcional `tracker`:

| Campo | Tipo | Notas |
|-------|------|-------|
| `provider` | string | p.ej. `linear` |
| `team_id` | string | equipo Linear |
| `project_id` | string | opcional |

## Validación

`project_binding` valida forma y tipos; no lee secretos ni llama a Linear.

---
feature_name: interaction-audit-session-karma2
created: "2026-10-28"
process: feature
branch_name: feat/interaction-audit-session-karma2
persist_ref: docs/features/interaction-audit-session-karma2
pbi_ref: AUT-10
tracker_ref: AUT-10
global: APTO
---

# Validación — AUT-10

| Check | Veredicto | Notas |
| --- | --- | --- |
| Tabla karma2 en shell nuevo | APTO | test `shell_has_uuid_v4_and_karma2_header` |
| Dedup cuatro campos | APTO | test `dedup_row` |
| Enum `entity_type` | APTO | test `rejects_invalid_entity_type` |
| Peaje lifecycle | APTO | código en `thermodynamic` + `maybe_record_process_peaje` |
| Tests Rust | APTO | 5 passed |

## global

**APTO** — implementación lista para PR; cierre Linear/DCC fuera de este diff documental.

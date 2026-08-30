---
feature_name: mayeuta-heartbeat-kaizen-classifier
created: "2026-08-30"
process: bug-fix
branch_name: fix/mayeuta-heartbeat-kaizen-classifier
persist_ref: docs/fixes/mayeuta-heartbeat-kaizen-classifier
items:
  - enrich_fracture_pbi_kaizen/heartbeat_starvation_cube
  - enrich_fracture_pbi_kaizen/trap_tests
  - actions/enrich-fracture-pbi-kaizen-1.2.0
---

# Implementation — cubo latido Mayeuta

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `enrich_fracture_pbi_kaizen.rs` | `is_heartbeat_starvation_trace` + cubo `refactor_tool` antes del catch-all |
| `enrich_fracture_pbi_kaizen.rs` | tests CA-1…CA-6 (`heartbeat_starvation`, `heartbeat_not_from_action_name`, DNS/hook intactos) |
| `enrich-fracture-pbi-kaizen.md` | v1.2.0 vía `entity-manager` (`27dfcf84`); cubo + F-MAYEUTA-HB-TOKEN-TRAP |
| `actions/index.md` | fila 1.2.0 |

## Contrato (implementado)

- Traza canónica Argos → `refactor_tool` (inanición de latido, no muerte).
- `attempted_action=daemon-heartbeat-audit` **sin** patrón Argos → no cubo latido.
- Prohibido token `heartbeat`/`daemon`/`audit` en blob concatenado.
- Hook / DNS / bypass intactos.

## Fuera de alcance (respetado)

- F2–F5. Keepalive `email-watcher`. Umbrales Argos. Plantilla Cúmulo.

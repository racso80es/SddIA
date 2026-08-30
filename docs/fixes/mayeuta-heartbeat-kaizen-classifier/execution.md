---
feature_name: mayeuta-heartbeat-kaizen-classifier
created: "2026-08-30"
process: bug-fix
branch_name: fix/mayeuta-heartbeat-kaizen-classifier
persist_ref: docs/fixes/mayeuta-heartbeat-kaizen-classifier
execution_id: "507e8ff0-388a-4040-8c52-c23b87af1dfd"
items_applied:
  - heartbeat_starvation_cube
  - trap_tests
  - entity_manager_action_1_2_0
---

# Ejecución — mayeuta-heartbeat-kaizen-classifier

## Fases aplicadas

| Fase | Estado | Evidencia |
|------|--------|-----------|
| 1 — Cubo latido | done | `is_heartbeat_starvation_trace` |
| 2 — Tests trampa | done | `analyze_fracture_kaizen_heartbeat_*` |
| 3 — Genoma | done | `entity-manager` `27dfcf84` → v1.2.0 `sha256:eabe4ede…` |
| 4 — Verificación | done | `cargo test -p execute-process -- analyze_fracture_kaizen` |

## Comandos

```bash
cd SddIA && cargo test -p execute-process -- analyze_fracture_kaizen
```

## Verificación

```
test engine::enrich_fracture_pbi_kaizen::tests::analyze_fracture_kaizen_bypass_new_norm ... ok
test engine::enrich_fracture_pbi_kaizen::tests::analyze_fracture_kaizen_dns_not_hook_recursion ... ok
test engine::enrich_fracture_pbi_kaizen::tests::analyze_fracture_kaizen_recursion_verdict ... ok
test engine::enrich_fracture_pbi_kaizen::tests::analyze_fracture_kaizen_heartbeat_not_from_action_name ... ok
test engine::enrich_fracture_pbi_kaizen::tests::analyze_fracture_kaizen_heartbeat_starvation ... ok
```

5 passed; 0 failed.

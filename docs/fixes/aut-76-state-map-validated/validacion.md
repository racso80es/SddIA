---
feature_name: aut-76-state-map-validated
created: 2026-11-06
process: bug-fix
branch: fix/aut-76-state-map-validated
branch_name: fix/aut-76-state-map-validated
persist_ref: docs/fixes/aut-76-state-map-validated
pbi_ref: AUT-76
document_id: PBI-222b4def
uuid: 222b4def-325a-4645-80d3-fbb15bf7d34b
execution_id: 6c28414f-8a36-4f66-b9e3-4969dc642978
global: APTO
checks:
  ca1_bind_validated: APTO
  ca2_unknown_key_rejected: APTO
  ca3_state_rank: APTO
  ca4_contract_1_5_0: APTO
  ca5_audit_section_3_4: APTO
  ca6_refine_item_da: APTO
  ca7_unit_tests: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/project_binding.rs
- SddIA/engine/execute-process/src/engine/handlers/tracker_sync_replay.rs
- SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md
- docs/audits/ciclo-vida-software-sddia.md
- SddIA/evolution/6f997575-80ab-431f-ae2a-9fa296eb69f6.md
- SddIA/evolution/Evolution_log.md
- docs/fixes/aut-76-state-map-validated/
validated_base_sha: 4e3fb78a897dc111e2f4e7a856ef371bfd054838
validated_head_sha: 3ff70d8a68c9e6298f52cebfb2a80183fd45b094
validated_tree_sha: e61ac87e84ffd0270d5ddf6795678e6de06640d1
diff_digest: sha256:972b638c258a1c880267d1d39095add82414b751bef4ebd59fa7e9cbd4573dab
---

# Validación — AUT-76

## Veredicto

**global: APTO** — rama `fix/aut-76-state-map-validated`.

## Evidencia

| CA | Resultado |
| --- | --- |
| 1 bind / DA con `validated` | `contract_accepts_state_map_validated_and_rejects_unknown_key` OK; `refine-item` AUT-76 `success: true` |
| 2 clave ajena | mismo test: `clave desconocida 'bogus'` |
| 3 `state_rank` | `validated=4` < `in_review=5` < `done=6`; anti-retroceso OK |
| 4 contrato | `project-config-contract` 1.5.0 |
| 5 auditoría §3.4 | fila `validated` rango 4 |
| 6 refine-item | acuse `d8795abe-…` sin `clave desconocida 'validated'` |
| 7 tests | tres filtros `--lib` 1/1 OK |

Mayeuta/DoR de AUT-76 fuera de alcance (`awaiting_agents`).

## Cierre

Un PR vía `delivery-close-cycle`. Sin ficheros en `docs/todos/`. Linear: AUT-76.
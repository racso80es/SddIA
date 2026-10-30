---
feature_name: forge-pbi-lock-after-create
created: 2026-10-30
process: bug-fix
branch_name: fix/forge-pbi-lock-after-create
persist_ref: docs/fixes/forge-pbi-lock-after-create
pbi_ref: AUT-29
tracker_ref: AUT-29
execution_id: b7f670f6-35d3-411b-9e70-9fb68fb4fc61
global: APTO
pbi_archived: false
branch: fix/forge-pbi-lock-after-create
checks:
  CA-LOCK: APTO
  CA-IDEMPOTENT: APTO
  CA-DOMAIN: APTO
  CA-TITLE: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/handlers/forge_pbi.rs
- docs/fixes/forge-pbi-lock-after-create/
validated_base_sha: f5b082f400dfbe76536ab5c339747daa1f6de8df
validated_head_sha: c318f0cfdb4d2b4f423cde9f20cc2da2f8d6152e
validated_tree_sha: 501494444d583e2c3c592a89afaba7f0312526af
diff_digest: sha256:8b7bdfd5b4513742c7d925688b31e7c48aa14a39d2b9b5d0fd8a26e01f00243a
---

# Validación — AUT-29

| Check | Resultado |
|-------|-----------|
| `cargo test -p execute-process forge_pbi --lib` | 13/13 OK |
| CA-LOCK | Lock simulado: éxito, `tracker_ref`, `document_id`, `pbi_forged_pending: true` |
| CA-IDEMPOTENT | Segundo `forge-pbi` del mismo `raw_idea`: mismo `tracker_ref`, un solo `create` |
| CA-DOMAIN | `core_project_slug_allowed_with_instance_software_forge_profile` OK |
| `gate-evolution` | alta `b919fa85-c552-497f-9179-b03e17772a92` |

Cierre Linear vía merge de PR (`done_gate: linear`). `pbi_archived` queda en false hasta ese merge.
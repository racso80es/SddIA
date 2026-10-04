---


feature_name: linear-hu-a-07-forge-pbi
---
# Implementación

- **Proceso:** `forge-pbi` 1.1.3; inputs `historia_ref`, `tipo`; fase tool documentada; context incluye `tracker-operations`.
- **Handler:** `tracker_registration_phase` → `linear-tracker-adapter` `create_issue`; `Tracker_Sync_Failed` si HU sin `tracker_ref` o error Linear; sin tracker en manifiesto → no-op.
- **Forja:** `patch_process_phases_update` propaga `process_context` en updates EM.
- **Verificación:** `cargo test -p execute-process --lib forge_pbi` y `forge_pbi_allows_create_issue_gate`.

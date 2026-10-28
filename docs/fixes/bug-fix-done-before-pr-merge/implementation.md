---
feature_name: bug-fix-done-before-pr-merge
created: "2026-10-28"
process: bug-fix
tracker_ref: AUT-12
execution_id: "dea619eb-78cc-498c-9747-8cd6e34bb14b"
---

# Implementación

## T1 — Core-self y `delivery_mode`

En `bind()`, si `project_slug == core` y `repo_is_core`, la precedencia de `delivery_mode` queda fijada al manifiesto (`branch_pr`). Evita atajo `trunk_direct` en inputs durante DCC.

## T2 — Sello `Delivery_Committed`

`tracker_stamp` exige `payload.delivery_mode == "trunk_direct"` antes de `stamp_done_with_hu_roll_up`. En `branch_pr`, el evento no debe sellar Done (aunque se materialice por error).

## T3 — Guardia `TRACKER_DONE_GUARD`

Antes de invocar la cápsula Linear, `invoke_tool_for_process` rechaza `update_issue_state` con `state_name: Done` si el proceso solicitante no es `tracker-stamp` ni `tracker-sync-replay`.

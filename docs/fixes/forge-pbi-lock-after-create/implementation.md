---
feature_name: forge-pbi-lock-after-create
created: "2026-10-30"
process: bug-fix
branch_name: fix/forge-pbi-scan-sin-ledger
persist_ref: docs/fixes/forge-pbi-lock-after-create
pbi_ref: AUT-29
tracker_ref: AUT-29
execution_id: "b7f670f6-35d3-411b-9e70-9fb68fb4fc61"
---

# Implementación — AUT-29

`forge_pbi::run` calcula `forge_key` y busca un alta previa: primero ledger `.SddIA/state/forge-pbi/{forge_key}.json`; si falta y hay `parent_ref`, escanea hijas del padre (`fetch_issue`). Si la encuentra, no llama a `create_issue` y reutiliza `document_id`, `uuid` y `tracker_ref`.

Tras un alta nueva el ledger queda con `event_sealed: false` antes de `PBI_Forged`. `emit_pbi_forged` trata el lock del store como `Pending`: éxito, `pbi_forged_pending: true`, sin `event_path`. El reintento del mismo `raw_idea` (vía ledger aunque no haya `parent_ref`) sella el evento y marca `event_sealed`. Si el ledger ya estaba sellado, no reemite.

`update_issue_description` solo corre en el alta nueva. El título no cambia.

Lab: con `SDDIA_LAB_MOCK_OUTBOUND=1`, `invoke_tool_for_process` exige cápsula **nativa** de `linear-tracker-adapter` (error si falta o hash stale; sin fallback WASM). Motivo: WASM desfasado ignora `.SddIA/lab-linear-store.json` en `fetch_issue` y rompe el scan por `forge_key` (AUT-29 corte 2).

---


feature_name: linear-hu-a-08-stamp
---
# Implementación

- `PBI_Refined` / `HU_Refined` → `todo` + comentario; sin `todo` en `state_map` → warn no-op.
- `Work_Initiated`: HU `backlog|todo` → `in_progress`; PBI idempotente si ya `in_progress`.
- `PBI_Cancelled` → `cancelled` + archiva markdown a `todos_done` (`status: cancelado`).
- `Delivery_Committed` → mismo sello `done` que `PullRequest_Merged`.
- Replay: `backlog < todo < in_progress < in_review < done`.

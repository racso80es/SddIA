---
feature_name: linear-hu-a-04-events
branch: feat/linear-hu-a-04-events
branch_name: feat/linear-hu-a-04-events
global: APTO
pbi_archived: true
pbi_document_id: PBI-LINEAR-A-04-EVENTS
persist_ref: docs/features/linear-hu-a-04-events
---

| Check | Estado |
|-------|--------|
| AC-EV-1 payload mínimo + rechazo `team_key` | APTO (`cargo test linear_direct_cycle_events`) |
| AC-EV-2 `Delivery_Committed` 1.1.0 legacy + `tracker_ref` | APTO (mismo test) |
| Forja vía entity-manager (4 clases) | APTO |
| Códice duplicado `delivery-committed` retirado | APTO |

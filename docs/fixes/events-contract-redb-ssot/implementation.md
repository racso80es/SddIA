---
feature_name: events-contract-redb-ssot
created: "2026-10-29"
process: bug-fix
branch_name: fix/events-contract-redb-ssot
pbi_ref: AUT-45
execution_id: "a629489e-2c6c-41fb-89db-26484a96fbd9"
---

# Implementación

- Contrato §2 y §4 alineados con `StoreMode::Redb` por defecto.
- `delivery_terminal_in_store` en `gate.rs`.
- `terminal_witness_exists` idempotente en modo redb.

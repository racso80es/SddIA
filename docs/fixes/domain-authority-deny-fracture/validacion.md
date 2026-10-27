---
feature_name: domain-authority-deny-fracture
created: "2026-10-27"
process: bug-fix
branch_name: fix/domain-authority-deny-fracture
persist_ref: docs/fixes/domain-authority-deny-fracture
tracker_ref: AUT-8
document_id: PBI-50be8903
pbi_ref: AUT-8
execution_id: "383795cc-6dee-478f-bcb1-6cbbc670f10c"
global: APTO
pbi_archived: false
branch: fix/domain-authority-deny-fracture
pr_url: https://github.com/racso80es/SddIA/pull/402
---

# Validación — domain-authority-deny-fracture

| Check | Resultado |
|-------|-----------|
| `cargo test -p execute-process domain_authority:: --lib` | 9/9 OK |
| AC AUT-8 | `System_Fracture_Detected` en deny; `F-DOMAIN-PROJECT-GIT-INVALID` / `F-DOMAIN-AUTHORITY-DENIED` |
| Manifiesto | `tracker.project_id` UUID proyecto SddIA (Linear) |
| `gate-evolution --range` | Tras evolution `6d0e423c-f418-416c-9878-7d19eaf2ee7e` |
| PR | https://github.com/racso80es/SddIA/pull/402 |

Cierre Linear vía merge de PR (`done_gate: linear`).

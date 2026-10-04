---
feature_name: linear-hu-a-09-e2e-lab
document_id: PBI-LINEAR-A-09-E2E
branch: feat/linear-hu-a-09-e2e-lab
---
# Spec — HU-A 09

| AC | Comportamiento verificado |
|----|---------------------------|
| AC-7 | `branch_pr`: refine-hu → forge-pbi → refine-pbi → sellos hasta merge; comentarios en tracker. |
| AC-8 | `trunk_direct`: cadena hasta `Delivery_Committed`. |
| AC-9 | Sin `LINEAR_API_TOKEN` ni cuerpo GraphQL en `.events/*`. |
| AC-10 | Manifiesto sin `tracker`: handlers verdes sin `create_issue`. |

Arnés: `execute-process::linear_direct_cycle_e2e` + store lab en `.SddIA/lab-linear-store.json` (`SDDIA_REPO_ROOT`).

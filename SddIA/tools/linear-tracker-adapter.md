---
uuid: "8f3c2a1b-9d4e-4f5a-b6c7-1234567890ab"
name: "linear-tracker-adapter"
version: "1.1.0"
contract: "tools-contract v1.6.0"
contract_ref: "SddIA/tools/tools-contract.md"
domain_origin: "SddIA"
context: "tracker-operations"
capabilities:
  - "linear_tracker_adapter"
  - "capsule-json-io"
io_mode: "capsule-json-io"
implementation_path_ref: "SddIA/tools/linear-tracker-adapter"
hash_signature: "sha256:455541dc257756b2ec330b27d84e87c6f92c7754549729efa25450e7890eb0eb"
outputs:
  - success: boolean
  - exitCode: integer
  - message: string
  - result: object
provides:
  - id: tracker:lifecycle
    contract: tracker.lifecycle
    version: 1.0.0
---

# linear-tracker-adapter

Proveedor canónico de `tracker:lifecycle`. Cápsula ciega GraphQL → Linear. Operaciones: `fetch_issue`, `list_issues`, `update_issue_state`, `create_issue`, `create_comment`, `update_issue_description`. `create_issue`: `team_key`, `title`, `description`, `labels[]`, opcionales `parent_ref`, `project_id`, `state_name`, `priority` → `issue_ref`, `id`, `url`. Errores: `LINEAR_LABEL_UNKNOWN`, `LINEAR_PARENT_NOT_FOUND` (+ códigos existentes). Token `LINEAR_API_TOKEN`. Endpoint `SDDIA_LINEAR_API_URL` (default `https://api.linear.app/graphql`). Lab: `SDDIA_LAB_MOCK_OUTBOUND`, `SDDIA_LAB_MOCK_LINEAR_URL`. Desviación §8 tools-contract: binario nativo (`ureq`).

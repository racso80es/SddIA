---
uuid: "8f3c2a1b-9d4e-4f5a-b6c7-1234567890ab"
name: "linear-tracker-adapter"
version: "1.0.0"
contract: "tools-contract v1.6.0"
contract_ref: "SddIA/tools/tools-contract.md"
domain_origin: "SddIA"
context: "tracker-operations"
capabilities:
  - "linear_tracker_adapter"
  - "capsule-json-io"
io_mode: "capsule-json-io"
implementation_path_ref: "SddIA/tools/linear-tracker-adapter"
hash_signature: "sha256:0000000000000000000000000000000000000000000000000000000000000000"
---

# linear-tracker-adapter

Cápsula ciega GraphQL → Linear. Operaciones: `fetch_issue`, `list_issues`, `update_issue_state`, `create_comment`, `update_issue_description`. Token `LINEAR_API_TOKEN`. Endpoint `SDDIA_LINEAR_API_URL` (default `https://api.linear.app/graphql`). Lab: `SDDIA_LAB_MOCK_OUTBOUND`, `SDDIA_LAB_MOCK_LINEAR_URL`. Desviación §8 tools-contract: binario nativo (`ureq`).

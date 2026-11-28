---
uuid: "8f3c2a1b-9d4e-4f5a-b6c7-1234567890ab"
name: "linear-tracker-adapter"
version: "1.1.1"
contract: "tools-contract v1.6.0"
contract_ref: "SddIA/tools/tools-contract.md"
domain_origin: "SddIA"
context: "tracker-operations"
capabilities:
  - "linear_tracker_adapter"
  - "capsule-json-io"
io_mode: "capsule-json-io"
implementation_path_ref: "SddIA/tools/linear-tracker-adapter"
hash_signature: "sha256:ab0a97e997668f0c2404b8eb8468441202d9bfcd0852281dc90da8948b3495a8"
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

## Propósito

Adaptador Linear GraphQL (`tracker:lifecycle`); operaciones fetch/create/update sin tokens en ECST.

## Operación

Binario nativo `ureq`; token `LINEAR_API_TOKEN`; mocks `SDDIA_LAB_MOCK_LINEAR_URL`. `create_issue`: `team_key`, `title`, `description`, `labels[]`, opcionales `parent_ref`, `project_id`, `state_name`, `priority` → `issue_ref`, `id`, `url`. Mutaciones `issueUpdate`: variables `$id` y `stateId` como `String`/`String!` según esquema Linear actual (paridad con `UpdateDesc`). Errores: `LINEAR_LABEL_UNKNOWN`, `LINEAR_PARENT_NOT_FOUND` (+ códigos existentes). Token `LINEAR_API_TOKEN`. Endpoint `SDDIA_LINEAR_API_URL` (default `https://api.linear.app/graphql`). Lab: `SDDIA_LAB_MOCK_OUTBOUND`, `SDDIA_LAB_MOCK_LINEAR_URL`. Desviación §8 tools-contract: binario nativo (`ureq`).

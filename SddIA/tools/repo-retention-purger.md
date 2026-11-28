---
uuid: "e9f12a4f-f6b1-4ad3-af03-634c8cc9707c"
name: "repo-retention-purger"
version: "1.0.0"
contract: "tools-contract v1.5.0"
domain_origin: "SddIA"
context: "filesystem-ops"
capabilities:
  - "repo_retention_purger"
hash_signature: "sha256:997e604bbdb1c3586227bda7c6310d91425407775cf1f61ea181ef6d8ef17577"
implementation_path_ref: "SddIA/tools/repo-retention-purger"
---

# repo-retention-purger

## Propósito

Planifica y aplica purga de retención bajo el repo con jail de candidatos (`repo-retention-purger` crate).

## Operación

Modo `simulate` → plan JSON; apply exige `plan_digest` acusado. Consumida por `action:purge-repo-retention`.

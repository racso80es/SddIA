---
uuid: "e4b84b41-c804-4c34-8fea-8b0f6bbcb573"
name: "purge-repo-retention"
version: "1.0.1"
contract: "actions-contract v1.2.0"
context: "filesystem-ops"
capabilities:
  - "purge_repo_retention"
hash_signature: "sha256:156190d299488776eae93b782a6b0fcecf2e3813ccf76e8cb32e8c6755c95b43"
---

## Propósito

Retención repo: dry-run + purga `repo-retention-purger` con jail de candidatos.

## Orquestación

`purge_repo_retention.rs`: simulate → `assert_candidate_jail` → apply con `plan_digest`.

## Límites

`filesystem-ops`.

---
feature_name: ppr-rehab-revoked-entities
created: "2026-07-22"
process: bug-fix
branch_name: fix/ppr-rehab-revoked-entities
persist_ref: docs/fixes/ppr-rehab-revoked-entities
agents: tekton
phase: Ejecución
uuid: 23a81b0e-3930-4589-b5db-25ddd8eb5717
---

# Ejecución

## Instancia (no versionada)

```text
revoked_entities.revoked.pull-request-review → REMOVED
  was: {entity_type: tool, reason: latency_threshold, since: 2026-07-21T06:26:40Z}
entities.pull-request-review.status → healthy
samples: outlier >=300000ms podado
```

## Código

```bash
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib pull_request_review_is_latency_exempt
```

## Witness RBAC_PROCESS_REGISTRY

```bash
python3 -c "import json; r=json.load(open('.SddIA/cerbero/revoked_entities.json')); assert 'pull-request-review' not in r.get('revoked',{})"
```

## Veredicto

**ok**

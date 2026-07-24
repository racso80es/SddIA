---
feature_name: delivery-close-cycle-revoked-signer
created: "2026-07-24"
process: feature
branch_name: feat/delivery-close-cycle-revoked-signer
persist_ref: docs/features/delivery-close-cycle-revoked-signer
document_id: PBI-PPR-136-DCC-REVOKED-SIGNER
execution_id: 00b9e53d-d231-45f5-9685-4d2b86b7ab63
phase: tekton
agents: tekton
items_applied:
  - E2-signer-code
  - E2-action-doc
  - E1-instance-rehab
  - E2-smoke
---

# Execution

## E2 — signer

1. Parche `SddIA/engine/execute-process/src/engine/actions.rs`.
2. `cargo build -p execute-process` → `SddIA/target/debug/execute-process` (rebuild forzado; binario previo root-owned stale).
3. Action MD v1.1.1 (manual tras fallo EM).
4. Smoke:

```bash
./sddia-run.sh --action emit-pr-presented-event --inputs \
  '{"branch":"feat/delivery-close-cycle-revoked-signer","status":"presented","pr_url":"https://example.invalid/pr/2"}'
```

**Resultado:** `payload.signer_identity_rbac == "Vertice_Biologico_Relay"` · event limpio del bus.

## E1 — rehabilitación instancia

```text
revoked.delivery-close-cycle → REMOVED (era abrupt_success_rate_drop since 2026-07-23T10:05:15Z)
remaining revoked: bug-fix, emit-pr-audited-event, feature
stats.delivery-close-cycle.status → healthy
```

Assert:

```python
assert 'delivery-close-cycle' not in revoked['revoked']
assert 'feature' in revoked['revoked'] and 'bug-fix' in revoked['revoked']
```

## Entity-manager (fallido → laudo)

- `execution_id` EM: `14626639-8b6b-487c-9120-e54a1de90e51`
- Error sello: `update: hash_signature_old (sha256:) es obligatorio`
- Side-effect: artefacto truncado + UUID nuevo → `git checkout HEAD --` + rewrite 1.1.1

## Residual

Instancia Cerbero/Radamanto gitignored — no viaja en PR; aduana host/CI debe reflejar rehab local o reaplicar runbook.

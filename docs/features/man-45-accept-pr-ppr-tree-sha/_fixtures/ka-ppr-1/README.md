# KA-PPR-1 — sin atestación aprobada

**Esperado:** `accept-pr` → `abortado` con `friction_id: qa_attestation_required`.

No hay `qa-attestations/{slug}.json`. Inputs mínimos:

```json
{
  "source_branch": "feat/ka-ppr-1",
  "delta_class": "active"
}
```

Cubierto por test `ka_ppr_1_missing_attestation_aborts` y `ka_ppr_1_genomic_audit_aborts_without_attestation`.

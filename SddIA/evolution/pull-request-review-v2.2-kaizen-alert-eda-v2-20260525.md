---
evolution_id: pull-request-review-v2.2-kaizen-alert-eda-v2
date: "2026-05-25"
process: pull-request-review
from_version: "2.1.0"
to_version: "2.2.0"
feature_ref: docs/features/kaizen-alert-required-eda-v2
---

# Evolución — pull-request-review v2.2.0 (Kaizen_Alert_Required EDA v2)

## Cambio

- **DIA-2/DIA-3:** persistencia Kaizen documental exclusivamente vía evento `Kaizen_Alert_Required` en `eda_bus.pending`.
- Poda puente síncrono v1 (`kaizen_items` / escritura directa `PENDING_AUDIT_DOC_*` en cápsula Kaizen).
- Cosecha Kaizen reservada a deuda genérica no documental.

## Hash

`hash_signature`: recalculado vía `recalc-process-hash-signatures.py --write` (2026-05-25).

---
feature_name: email-triage-heuristic-cold-start
created: "2026-09-06"
process: feature
branch_name: feat/email-triage-heuristic-cold-start
persist_ref: docs/features/email-triage-heuristic-cold-start
execution_id: "5b530130-8225-4904-98f0-a894523f9c7e"
items_applied:
  - crate-hash-and-handler
  - tests-slice1
  - entity-manager-process
  - entity-manager-norm
  - entity-manager-event
  - evolution-register
---

# Ejecución — email-triage-heuristic-cold-start

## Init

`SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 ./sddia-run.sh --process feature --inputs-file .tmp/feature-email-triage-heuristic-cold-start.json`

`execution_id` `5b530130-8225-4904-98f0-a894523f9c7e`. workspace-init **executed**. Mayeuta simulated; Dedalo…DCC phase-barrier skipped. Relevo IDE. Commit planificación `57da1be`.

## Código (L1)

`canonical_subject_key_from_addr` en `user-preference-core`. Handler: Triaje-P → P-EXEMPT-C → C → P-MUTE-SENDER → Clasificacion.

## Tests (L2)

```text
cd SddIA && cargo test -p user-preference-core --lib -- canonical_subject
# 1 passed

cd SddIA && cargo test -p execute-process --lib -- email_triage
# 24 passed (incluye cold-start, mute, exempt, inferred, proposed, conjugación, IMAP)

cd SddIA && cargo test -p execute-process --lib -- email_triage_pref_query
# 1 passed
```

## Genoma (L3)

| Entidad | EM execution_id | Sello Domain_Entity_Updated | hash_new |
|---------|-----------------|------------------------------|----------|
| process `email-triage-gateway` 1.1.0 | `af25eac2-cecd-43eb-8602-355bd9e92dff` | `35c8a430-7965-4191-abc6-3c0cd2198fc3` | `sha256:794696a90e36910a64bc35863695b77ad8a54f0381b552dc4a0333b571a35366` |
| process body G5 | `a576c166-f820-45d9-a905-b35965499ad7` | idempotente mismo sello (hash de fases) | mismo |
| norm `email-triage-matrix` 1.1.0 | `82ef7286-6cd0-4bd6-bfa7-49f9e7b794a5` | `01c9f81d-df5b-4bbd-9565-f6852bd65aa2` | `sha256:82acf00d3e934b68fb26da5b3c0e9d5aef8a008ab2f8fb77b94f2cfaaf39dbc3` |
| event `email-triaged` | `f7acf565-d539-47c1-98ec-c9f4fa4693c3` | `8830b512-008d-4c9f-baf8-f7561c07384f` | `sha256:c5a6a60c3a66b6cc2081e6bd00402df83e8aa54d25fd60d633733102ee4cde8b` |

UUID inmutables. Body process documenta `preference`.

## Evolution

`sddia-qa evolution-register` → `95441293-1049-4016-8112-a322919d34e8` (`EVOL_OK`, `alta`). Residual índices/DI → `d8d74352-2e54-46b3-bc67-08f8b8369f60`.

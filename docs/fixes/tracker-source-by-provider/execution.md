---
feature_name: tracker-source-by-provider
created: "2026-10-25"
process: bug-fix
branch: fix/tracker-source-by-provider
persist_ref: docs/fixes/tracker-source-by-provider
tracker_ref: MAN-21
execution_id: "fd88acb8-0104-4101-875d-aef751fa47e6"
items_applied:
  - resolver-invoke
  - file-impl-historias
  - handlers-verb
---

# Ejecución — MAN-21

## Init

```bash
SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  ./sddia-run.sh --process bug-fix --inputs-file /tmp/man21-bug-fix-init.json
```

`execution_id`: `fd88acb8-0104-4101-875d-aef751fa47e6`. Diseño `simulated`. Rama `fix/tracker-source-by-provider`.

## Tests

```text
cd SddIA && cargo test -p execute-process --lib -- \
  resolver_file same_verb refine:: forge_pbi:: materialize_fracture \
  materialize_kaizen materialize_ci tracker_markdown_apply file_provider_skips
```

FIX-CA2: `same_verb_input_on_file_and_linear`.

## Cierre de entrega

`delivery-close-cycle` `execution_id` `6ac3c0b3-9c0f-4def-b73d-80dd7b263139`. Snapshot `557d6332`. PR https://github.com/racso80es/SddIA/pull/382. `PullRequest_Presented` `9824b73b-98f4-4d74-82b6-dd390f5b2dbc`.

La fase **Aduana evolution** bloqueó (`EVOL_MATERIAL_UNREGISTERED`) por `relacionado` incompleto. `sddia-qa evolution-register` `modificacion` sobre `5350b4ce-e44c-4a90-90a0-746a5a40a7d9` (prefijos `execute-process`, adapters). Commit `e5fab9d0`; `gate-evolution --range` → delta cubierto.

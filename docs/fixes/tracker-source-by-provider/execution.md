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

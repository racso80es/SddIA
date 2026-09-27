---
feature_name: sddia-installer-v3-request-stdin-env
created: "2026-09-27"
process: refactorization
pbi_ref: docs/todos/pending/[ARQUITECTURA] Installer v3 — entrada stdin y SDDIA_CAPSULE_REQUEST.md
document_id: PBI-ARQUITECTURA-INSTALLER-V3-REQUEST-STDIN-ENV
pbi_uuid: "f7b237b0-dd48-4b85-9834-d85067123115"
---

# Clarificación

| ID | Laudo |
|----|-------|
| L-PREC | `--request-file` > stdin JSON > `SDDIA_CAPSULE_REQUEST` > argv |
| L-TTY | No leer stdin si `-t 0` |
| L-ERR | `REQUEST_INVALID` siempre con envelope `static-envelope` |

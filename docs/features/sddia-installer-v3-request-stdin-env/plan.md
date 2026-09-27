---
feature_name: sddia-installer-v3-request-stdin-env
branch_name: refactor/sddia-installer-v3-request-stdin-env
persist_ref: docs/features/sddia-installer-v3-request-stdin-env
---

# Plan

1. `_resolve_and_apply_request` en motor.
2. `_emit_request_invalid` vía `static-envelope`.
3. Smoke AC-R1..R6.
4. Nota `SDDIA_SKIP_STDIN` en norma installer.
5. Evolution + cierre documental tras CI.

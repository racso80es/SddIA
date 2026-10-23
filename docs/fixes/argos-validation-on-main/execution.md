---
feature_name: argos-validation-on-main
created: "2026-10-23"
process: bug-fix
branch_name: fix/argos-validation-on-main
persist_ref: docs/fixes/argos-validation-on-main
execution_id: "a0509a75-3e2f-4a4d-ae64-18379411a5cb"
---

# Ejecución

## Init

```bash
SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 \
  SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  ./sddia-run.sh --process bug-fix --inputs-file .tmp/bug-fix-argos-validation-on-main-init.json
```

Primer intento `8035fca6-058c-44f4-8f09-30432a664c5d` abortó en checkout: `docs/features/tracker-lifecycle-sink/validacion.md` sucio sobre `main`. Se restauró ese fichero a HEAD. Reentrada `a0509a75-3e2f-4a4d-ae64-18379411a5cb` en `fix/argos-validation-on-main`. Diseño `simulated`.

## Verificación local

```bash
cd SddIA && cargo test -p execute-process --lib -- verification_handoff verification_missing runtime_evidence_forwards branch_name_coalesces --test-threads=1
```

Resultado: 5 passed (`verification_handoff_without_branch`, `verification_handoff_without_execution_id`, `verification_missing_local_ref_fails_before_spawn`, `runtime_evidence_forwards_native_state_flags`, `branch_name_coalesces_from_pr_branch`).

---
feature_name: sddia-installer-v3-smoke-lab-ac4-8
---

# Spec

| AC | Test |
|----|------|
| AC-4 | `deploy --skip-build` lab: stdout = 1 JSON; ruido hijo en `log_ref` |
| AC-5 | Perfil inválido → exit 6, STEP_FAILED, step=build_bundle, child_exit=1 |
| AC-6 | Parser `@sddia-progress` begin/end, index/total |
| AC-7 | `correlation_id` → `.events/progress/{cid}/`, source_agent |
| AC-8 | Sin valores R-VAULT-2 en envelope, progreso, log |

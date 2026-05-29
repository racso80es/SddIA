---
feature_name: inmunidad-caos-fase2
created: "2026-05-29"
process: feature
items_applied:
  - "2.0 RBAC agents"
  - "2.A–2.C procesos audit"
  - "2.D run_chaos_audit_process"
  - "2.E tests + índice"
---

# Ejecución — Fase 2

## Secuencia aplicada

1. Ampliación `allowed_policies` Tekton (`chaos-engineering`) y Argos (`event-routing`).
2. Forja tres procesos bajo `SddIA/process/` con `workspace_template` y `phase_invocations`.
3. Handler `run_chaos_audit_process` en `execute_process_capsules.py` — orden: estímulo → peaje (si aplica) → compliance fan-out → Argos.
4. Extensión `resolve_ed_telemetry_contract` para familia **tool** (prerequisito breach `schema-corruptor`).
5. Tests `test_chaos_audit_processes.py` — 5/5 verdes.

## Smoke local

```powershell
cd SddIA/scripts/qa
python -m unittest test_chaos_audit_processes.py -v
```

Plantillas smoke en `persist_ref/_smoke-audit-*.json` para `execute-process`.

## Notas Tekton

- `audit-thermodynamic-toll-failsoft` usa flag lab `chaos_simulate_telemetry_io_fail` en Peaje (paridad test D3.13).
- `audit-sandbox-isolation-rbac` omite Peaje — vector aislado a envelope tool.
- PBI maestro permanece en `pending/` (`pbi_archived: false`).

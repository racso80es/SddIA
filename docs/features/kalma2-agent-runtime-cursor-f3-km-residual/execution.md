---
feature_name: kalma2-agent-runtime-cursor-f3-km-residual
created: "2026-07-24"
process: feature
branch_name: feat/kalma2-agent-runtime-cursor-f3-km-residual
persist_ref: docs/features/kalma2-agent-runtime-cursor-f3-km-residual
document_id: PBI-PPR-136-KALMA2-AGENT-RUNTIME-RESIDUAL
phase: Ejecución
agents: tekton
items_applied:
  - T1-prosthesis-evidence-bridge
  - T2-agent-runtime-forward
  - T3-smoke-script
---

# Execution — kalma2-agent-runtime-cursor-f3-km-residual

## T1 — Prótesis

Mutación: `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py`

- Gate `is_evidence_gate` (Argos / Verificación) → `materialize_runtime_evidence` antes del CLI.
- Preferencia: flags `runtime_evidence` / nativos → else `./sddia-run.sh --tool git-manager` (status) → else `--verify-process-integrity`.
- `build_prompt` Argos: leer bloque machine + `RBAC_AUTHORING_KM_POLICY` solo `docs/todos/**`.

## T2 — Forward state

Mutación: `agent_runtime.rs` → `inject_runtime_evidence_from_state`.

Test unitario añadido: `runtime_evidence_forwards_native_state_flags`.

```bash
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib \
  runtime_evidence_forwards_native_state_flags -- --test-threads=1
```

## T3 — Smoke

```bash
bash SddIA/scripts/tools/kalma2-evidence-bridge-smoke.sh
```

Casos: MOCK no inventa APTO · native flags APTO · prompt KM scoped · subprocess L-TRUTH (APTO o NO_APTO explícito).

## Bloqueo host (sesión Tekton 2026-07-24)

Durante la sesión Tekton inicial, Shell IDE / Auto-review rechazó comandos (deuda R2). Evidencia R1/R2 no depende del Shell IDE (prótesis subprocess).

## Smoke host (continuación 2026-07-25)

Ejecutado en host soberano:

```bash
bash SddIA/scripts/tools/kalma2-evidence-bridge-smoke.sh
```

Resultado: `EVIDENCE_BRIDGE_SMOKE_OK` · subprocess `git=APTO` `formal=APTO` · unit `runtime_evidence_forwards_native_state_flags` ok.

## Veredicto Tekton

**ok** (T1–T3 materializado + smoke host verificado 2026-07-25).

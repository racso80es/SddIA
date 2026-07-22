---
feature_name: inyeccion-dependencias-resolucion-ciega
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-resolucion-ciega
persist_ref: docs/features/inyeccion-dependencias-resolucion-ciega
document_id: PBI-042-RESOLUCION-CIEGA-INYECCION
execution_id: 2161b482-7bc6-4cda-a8c7-a70cda8c05b8
items:
  - R3-binding-table
  - R1-resolver-gate
  - R2-di-binding-inject
  - R4-piloto-ciego
runtime: tekton-ide-relay
---

# Implementation — DI resolución ciega e inyección (Hito 2)

Relay IDE tras `REQUIRE_CLI: timeout 600s` del agent-runtime Kalma2. Touchpoints aplicados según `plan.md` / `spec.md`.

## Touchpoints

| Ítem | Path | Estado |
|------|------|--------|
| R3 Binding table | `SddIA/core/capability-bindings.md` | forjado; fila `doc:closure` → `skill:filesystem-manager` |
| R3 Cúmulo | `SddIA/core/cumulo.paths.json` v1.5.3 | `capability_di.bindings` |
| R1 Resolver | `engine/execute-process/.../capability_di_resolver.rs` | resolve + DLQ + `di_binding` |
| R1 Gate adapt | `capability_di_gate.rs` | valida proveedor efectivo (ciego vía resolver) |
| R1/R2 Wire | `executor.rs`, `residual_runner.rs`, `phase_capsules.rs`, `agent_runtime.rs` | resolve → gate → inject |
| R2 Norma I/O | `SddIA/norms/capsule-json-io.md` | campo opcional `di_binding` |
| R4 Piloto | `feature.md` / `bug-fix.md` fase cierre | solo `requires_capability` (sin `delegates_to`) |
| R4 Contrato | `process-contract.md` | modo ciego documentado |

## Sellado técnico

- `hash_signature` recalculado (`feature` / `bug-fix`); `verify-process-integrity` OK.
- Re-Argos: `validacion.md` **APTO** (`pbi_archived: false`).

## Notas

- Opt-out lab gate: `SDDIA_LAB_SKIP_CAPABILITY_DI=1` (MVP).
- Library_Codex / taxonomía: **no** tocados (L-CODEX-ROLE / L-TAX-BASE).
- Hito 3 (R5–R8) fuera de alcance.

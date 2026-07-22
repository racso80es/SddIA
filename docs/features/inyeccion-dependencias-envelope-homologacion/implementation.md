---
feature_name: inyeccion-dependencias-envelope-homologacion
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-envelope-homologacion
persist_ref: docs/features/inyeccion-dependencias-envelope-homologacion
document_id: PBI-042-ENVELOPE-HOMOLOGACION
execution_id: e7a4b2c3-8f1d-4e6a-9b2c-1d3e5f7a9b0c
items:
  - R9-cerbero-di-envelope
  - R10-homologacion-catalogo
runtime: tekton-kalma2-cursor
---

# Implementation — DI envelope Cerbero + homologación catálogo (Hito 4)

Materialización Tekton según `plan.md` / `spec.md` (post-Hito 3 PR #128 `51fd434`).

## Touchpoints

| Ítem | Path | Estado |
|------|------|--------|
| R9 Schema envelope | `SddIA/library/norms/capability-contracts/di.binding.schema.json` | nuevo — meta-contrato `di_binding` |
| R9 Cerbero envelope | `engine/execute-process/.../cerbero_di_envelope.rs` | nuevo — `validate_packaged_bindings` + DLQ |
| R9 Wire sync | `executor.rs`, `residual_runner.rs` | post-RBAC pre-inject |
| R9 Wire EDA | `capability_di_reactor.rs` | envelope en `run_sync_chain`; `cerbero_envelope_di_code` |
| R9 Export | `mod.rs` | `cerbero_di_envelope` |
| R10 refactorization | `SddIA/process/refactorization.md` v1.2.1 | fase «Cierre documental en rama» ciego `doc:closure` |
| R10 delivery-close-cycle | `SddIA/process/delivery-close-cycle.md` v1.1.1 | `proc:git-sync` en Publicación remota |
| R10 accept-pr | `SddIA/process/accept-pr.md` v1.0.1 | `proc:git-sync` en Fusión Soberana |
| R10 pull-request-review | `SddIA/process/pull-request-review.md` v2.2.1 | `proc:git-sync` en Preparación de rama |
| Docs I/O | `SddIA/norms/capsule-json-io.md` | nota R9 revalidación Cerbero |
| Evolution | `SddIA/evolution/e7a4b2c3-...md` | registro Hito 4 |

## Criterios demostrables

| AC | Verificación |
|----|--------------|
| AC-R9 | `cargo test cerbero_di_envelope` — tamper contract → `CERBERO_DI_BINDING_INCOHERENT`; campo ausente → `CERBERO_ENVELOPE_SCHEMA_MISMATCH` |
| AC-R10 | 8 ED homologadas (4 baseline + 4 nuevas §4.6 spec); sin altas taxonomía |
| AC-REG-H3 | `cargo test cerbero_di di_reactor di_output` |
| AC-REG-H2/MVP | `cargo test capability_di` |

## Notas

- Cadena DI: `resolve → gate → cerbero_rbac → cerbero_envelope → inject → output_validator`.
- Skip lab unificado: `SDDIA_LAB_SKIP_CAPABILITY_DI=1`.
- `hash_signature` procesos R10: **recalculado** (`sddia-qa recalc-process-hash-signatures --write`); `verify-process-integrity: OK`.
- Fixture envelope: incluye `SddIA/skills/filesystem-manager.md` para regresión AC-R5 (`CERBERO_RBAC_DENIED`, no `CONFIG_ERROR`).

---
feature_name: inyeccion-dependencias-gobernanza-asincronia
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-gobernanza-asincronia
persist_ref: docs/features/inyeccion-dependencias-gobernanza-asincronia
document_id: PBI-042-GOBERNANZA-ASINCRONIA
execution_id: f8b2c4d1-6e3a-4f7b-9c2d-1a0e5f8b3c7d
items:
  - R5-cerbero-di-rbac
  - R6-eda-pilot-reactor
  - R7-proc-git-sync-codex
  - R8-output-schema-validator
runtime: tekton-ide-relay
---

# Implementation — DI gobernanza y asincronía (Hito 3)

Relay Tekton según `plan.md` / `spec.md` (post-Hito 2 PR #127 `60c4635`).

## Touchpoints

| Ítem | Path | Estado |
|------|------|--------|
| R5 Cerbero RBAC | `engine/execute-process/.../cerbero_di_rbac.rs` | nuevo — `validate_di_rbac` post-gate |
| R5 Wire | `executor.rs`, `residual_runner.rs` | orden `resolve → gate → Cerbero → inject` |
| R6 Reactor EDA | `capability_di_reactor.rs` | `CapabilityDi_Requested` / `Resolved` + `drain_di_reactor_once` |
| R6 Wire | `executor.rs`, `residual_runner.rs` | rama `SDDIA_DI_EDA_PILOT=1` o `di_composition: eda_pilot` |
| R6 Suscripción | `SddIA/core/event-domain-subscriptions.json` | piloto `CapabilityDi_Requested` |
| R7 Códice | `capability-taxonomy.md` v1.0.1 | `proc:git-sync` |
| R7 Contrato | `proc.git_sync.schema.json` | schema salida git-manager |
| R7 Binding | `capability-bindings.md` | fila `proc:git-sync` → `skill:git-manager` |
| R7 Provides | `git-manager.md` | `provides` proc:git-sync |
| R7 Evolution | `SddIA/evolution/f8b2c4d1-...md` | registro Hito 3 |
| R8 Validador | `capability_di_output_validator.rs` | `jsonschema` + DLQ `CONTRACT_OUTPUT_SCHEMA_MISMATCH` |
| R8 Hook | `phase_capsules.rs` | post-`invoke_tool` en `try_invoke_delegates` |
| R8 Norma I/O | `SddIA/norms/capsule-json-io.md` | nota validación post-ejecución |
| Deps | `Cargo.toml` | `jsonschema = "0.26"` |
| Exports | `mod.rs` | módulos Hito 3 |

## Criterios demostrables

| AC | Verificación |
|----|--------------|
| AC-R5 | `cargo test cerbero_di` — políticas restrictivas, gate mock APTO implícito, `CERBERO_RBAC_DENIED` |
| AC-R6 | `cargo test di_reactor` — pending sin await; drain → `CapabilityDi_Resolved` + `ecst_ack` |
| AC-R7 | diff taxonomía + binding + evolution UUID |
| AC-R8 | `cargo test di_output` — payload sin `required` → `CONTRACT_OUTPUT_SCHEMA_MISMATCH` |
| AC-REG-H2/MVP | `cargo test capability_di` — suite gate/resolver existente |

## Notas

- Path sync **default** sin flag EDA (regresión H2/MVP).
- Skip lab unificado: `SDDIA_LAB_SKIP_CAPABILITY_DI=1` (resolve, gate, Cerbero, output validator).
- Cerbero Hito 3: solo RBAC; revalidación schema `di_binding` diferida post-Hito 3.

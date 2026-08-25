---
feature_name: kaizen-paciente0-redeploy-fricciones
created: "2026-08-25"
process: feature
items: "T1-T7"
branch_name: feat/kaizen-paciente0-redeploy-fricciones
persist_ref: docs/features/kaizen-paciente0-redeploy-fricciones
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
execution_id: "c95fa63f-be71-481b-a927-475e7c885fd0"
---

# Implementation — kaizen-paciente0-redeploy-fricciones

## T1 — Bundle gate (F-DEP-03 / L-BUNDLE-STALE v2)

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/scripts/build-release-bundle.sh` | Cicatriz SHA-256 (cierre crate + `path =` + workspace lock); testigo `<bin>.sha256`; `--skip-build` fail-closed; gate `strings` / `execute-process.py`; ONBOARDING inventario + LLM |
| `SddIA/norms/sddia-distribution-protocol.md` v1.2.0 | Core `directories.norms`: `norm-creator` no escribe aquí. Mutación bajo feature (mismo laudo que Filtro C) |

## T2 — instance-creator

| Artefacto | Cambio |
|-----------|--------|
| `handlers/instance_creator.rs` | `CORE_ROOT=instance_root`; starter-kit `local.paths.json`; `route-domain-event` si `!skip_ignition` |
| `instance-creator.md` v1.1.1 | `entity-manager` UPDATE `event_id=c1160a5e-…` uuid `dead5ca7-…`; prosa cita protocolo v1.2.0 |

## T3 — start-sddia bundle-safe

| Artefacto | Cambio |
|-----------|--------|
| `start-sddia.sh` | `MANIFEST.json` o sin `Cargo.toml` → resolve ELF, no cargo |

## T4 / T5 — triaje

| Artefacto | Cambio |
|-----------|--------|
| `email_triage.rs` | `maybe_elevate_from_subject`; `subject_elevation`; `mark_classification_degraded` (T-INFER sin env); `SDDIA_LLM_REQUIRE_INFER` |
| `email-triage-matrix.md` v1.0.1 | §1 desempate post-LLM. **No** `norm-creator` UPDATE: reescribiría el cuerpo. Índice 1.0.1 |

## T6 — empiría Paciente 0

Auditoría: `docs/audits/kaizen-paciente0-redeploy-20260825.md`. Bundle `20260825T111733Z`. Creator `9528fb5f-…`. G5 `413e6edf-…` `actionable` + agenda + WUI. Telegram eferente: `message_id=9`.

## T7 — Argos

`validacion.md` APTO (`pbi_archived: true`). PBI en `docs/todos/done/`. Evolution `916bf0f9-…`. T-INFER unitario. Cita `instance-creator` v1.2.0. **O11** `delivery-close-cycle` en este estímulo.

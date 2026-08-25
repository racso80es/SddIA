---
feature_name: kaizen-paciente0-redeploy-20260825
created: "2026-08-25"
process: feature
phases: "T0-docs,T1-resolve,T2-stub,T3-pin-bundle,T4-smoke-ecst,T5-contrato-norma,T6-redeploy,T7-argos-cierre"
branch_name: feat/kaizen-paciente0-redeploy-20260825
persist_ref: docs/features/kaizen-paciente0-redeploy-20260825
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
execution_id: "7fd0a353-d2fe-4895-8abe-d7f5b34f652c"
laudo: absorber-fricciones-post-absorcion-un-pr
---

# Blueprint — kaizen-paciente0-redeploy-20260825

## Estrategia

Absorber fricciones 2026-08-25 en Core. Orden PBI §5 = riesgo: **resolver orquestador antes de redeploy**. Un persist_ref, un PR. F-SYS-01 no entra (L-SYS).

Laudos: `spec.md` §1.

`delegates_to` ejecución: `agent:tekton`. Genoma process: `action:execute-process` (`entity-manager`). Verificación: `agent:argos`. Cierre: `action:execute-process` (`delivery-close-cycle`).

## Fases

### T0 — Dedalo

- [x] `clarify.md` / `objectives.md` / `spec.md` / `plan.md`
- Gate: laudo `absorber-fricciones-post-absorcion-un-pr`; L-SYS fuera

### T1 — F-DEP-07 resolver

- [x] `_sddia_resolve_orchestrator`: L-RESOLVE (mtime debug vs release)
- Gate: O1

### T2 — F-DEP-08 overlay stub

- [x] `materialize_local_paths`: sustituir missing/empty/`{}`
- [x] Test `replaces_empty_local_paths_stub`
- Gate: O2

### T3 — F-DEP-09 pin ignición

- [x] `_ensure_orchestrator` rama bundle: discard pin fuera de `REPO_ROOT`
- Gate: O4 (lab: `Cargo.toml` intacto)

### T4 — F-SMOKE-01

- [x] `run_smoke` nativo: no escribir `Local_QA_Requested`
- [x] Check overlay no-`{}`; ajustar `smoke_native_without_skip`
- Gate: O5

### T5 — Contrato + norma

- [x] `entity-manager` UPDATE `instance-creator` v1.2.0
- [x] `sddia-distribution-protocol` 1.2.1 (feature, no norm-creator)
- Gate: L-CREATOR-MD, L-DIST

### T6 — Redeploy smoke Paciente 0

- [x] Bundle fresco; **un** `instance-creator` sin unlink/pin; ExecStart instancia; overlay no vacío
- [x] Ignición: orquestador bajo instancia pese a pin forja en shell
- [x] `docs/audits/kaizen-paciente0-redeploy-20260825-residual.md`
- Gate: O3, O4 empírico, O8. **Sin G5**

### T7 — Argos + cierre

- [x] `implementation.md` / `execution.md` / `validacion.md` APTO
- [x] PBI → `docs/todos/done/`; `pbi_archived: true`
- [ ] `delivery-close-cycle`
- Gate: O9

## Dependencias

| Antes | Después |
|-------|---------|
| T1–T5 | T6 |
| T6 | T7 |

T1–T5 misma sesión Tekton. T6 exige instancia `SddIA_AP` y bóveda operador.

## Fuera

- F-SYS-01 / `DT-SYSTEMD-USER-ENABLE`
- G5, F-TRIAGE-*, wizard, auto-merge bóvedas

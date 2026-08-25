---
feature_name: kaizen-paciente0-redeploy-fricciones
created: "2026-08-25"
process: feature
phases: "T0-docs,T1-bundle-gate,T2-instance-creator,T3-start-sddia,T4-triage-guard,T5-llm-infer,T6-redeploy-g5,T7-argos-cierre"
branch_name: feat/kaizen-paciente0-redeploy-fricciones
persist_ref: docs/features/kaizen-paciente0-redeploy-fricciones
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
execution_id: "c95fa63f-be71-481b-a927-475e7c885fd0"
laudo: absorber-parches-core-un-pr
---

# Blueprint — kaizen-paciente0-redeploy-fricciones

## Estrategia

Absorber parches ad-hoc del redeploy 2026-08-24 en Core. Orden PBI §7 = riesgo: **ELF fresco antes de redeploy**. Un persist_ref, un PR. Genoma solo `entity-manager`.

Laudos: `spec.md` §1 (`L-CORE-ROOT` … `L-FORGE`).

`delegates_to` de ejecución: `agent:tekton`. Mutación indexada: `action:execute-process` (`entity-manager`). Verificación: `agent:argos`. Cierre: `action:execute-process` (`delivery-close-cycle`).

## Fases

### T0 — Dedalo (este documento)

- [x] `clarify.md` / `objectives.md` / `spec.md` / `plan.md`
- Gate: laudo `absorber-parches-core-un-pr`; F-TRIAGE-03 fuera

### T1 — F-DEP-03 gate bundle

- [x] `build-release-bundle.sh`: gate `strings` sin `execute-process.py` en centinelas
- [x] Cicatriz SHA-256: helper `find`+`LC_ALL=C sort`+`sha256sum` sobre **cierre de compilación** (crate + `path =` + `Cargo.lock` workspace); testigo `<bin>.sha256` junto al ELF tras build
- [x] `--skip-build`: comparar digest actual vs testigo; mismatch o testigo ausente → abort (L-BUNDLE-STALE v2). **No** `mtime` / `find -newer`
- [x] ONBOARDING: inventario mínimo + LLM recomendado (`L-ONBOARD`)
- [x] UPDATE `sddia-distribution-protocol` v1.2.0 bajo feature (Core `directories.norms`; `norm-creator` no aplica)
- Gate: O1; `--skip-build` sin testigo → exit 1

### T2 — F-DEP-01 + F-DEP-04 + F-DEP-06 creator

- [x] `instance_creator.rs`: `@@SDDIA_CORE_ROOT@@` ← `instance_root`
- [x] Topología: copiar starter-kit `local.paths.json` (no `{}`)
- [x] Smoke: `route-domain*` solo si ignición no skipped
- [x] Tests T-CORE, T-PATHS
- [x] UPDATE `instance-creator.md` vía `entity-manager` (`c1160a5e-…`)
- Gate: O2, O3, O5 (contrato); ignición real se prueba en T6

### T3 — F-DEP-02 start-sddia bundle-safe

- [x] `_ensure_orchestrator` en `start-sddia.sh`: MANIFEST o sin Cargo.toml → resolve ELF, no cargo
- Gate: O4; lab con Cargo.toml intacto

### T4 — F-TRIAGE-01 guard asunto

- [x] `classify_llm`: extracción completa eleva (L-GUARD) + `subject_elevation`
- [x] Prompt candidato reunión (L-PROMPT)
- [x] Test T-GUARD + T-NOISE (patrón UID 104579)
- [x] UPDATE `email-triage-matrix` §1 a mano v1.0.1 (`norm-creator` UPDATE clobberea el cuerpo)
- Gate: O6, O7

### T5 — F-TRIAGE-02 inferencia

- [x] `classification-degraded` + `SDDIA_LLM_REQUIRE_INFER` (L-INFER) en `classify_llm` (invoke fail / tokens 0)
- [x] Test T-INFER aislado (`mark_classification_degraded`; sin mutar env global)
- Gate: O8

### T6 — Redeploy Paciente 0 + G5 + auditoría

- [x] Bundle fresco **sin** `--skip-build` stale; `instance-creator`; ignición **sin** parche `{instancia}/start-sddia.sh`
- [x] Verificar systemd ExecStart bajo `SddIA_AP`; WUI; route-domain (`Email_Triaged` enrutado)
- [x] Correo reunión (estímulo lab) → `actionable` + agenda + WUI inbox
- [x] `docs/audits/kaizen-paciente0-redeploy-20260825.md`
- Gate: cierre PBI §6 despliegue + triaje (Telegram poke `message_id=9`)

### T7 — Argos + cierre documental

- [x] `implementation.md` / `execution.md` / `validacion.md` APTO
- [x] PBI → `docs/todos/done/`; `pbi_archived: true`
- [x] `delivery-close-cycle` (este estímulo; sin `SDDIA_LAB_SKIP_DELIVERY_CLOSE`)
- Gate: O11

## Dependencias

| Antes | Después |
|-------|---------|
| T1 | T6 (bundle usable) |
| T2 | T6 (creator correcto) |
| T3 | T6 (ignición bundle) |
| T4 | T5 (guard antes de REQUIRE_INFER) |
| T4–T5 | T6 G5 correo |
| T1–T6 | T7 |

T1–T5 pueden ejecutarse en serie en la misma sesión Tekton; T6 exige instancia `SddIA_AP` y bóveda operador (no secretos en git).

## Fuera de este blueprint

- F-TRIAGE-03
- Poda `AGENT_RUNTIME_*` en `Proyectos/.dev` (D11)
- Wizard UX

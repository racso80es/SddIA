---
feature_name: kaizen-consumer-ignition-filtro-c
created: "2026-08-20"
process: feature
phases: "T0-docs,T1-filtro-c-r07-f07,T2-bundle-f06,T3-norm-onboarding,T4-instance-creator-smoke,T5-f08-f09,T6-optional-paciente0,T7-argos"
branch_name: feat/kaizen-consumer-ignition-filtro-c
persist_ref: docs/features/kaizen-consumer-ignition-filtro-c
document_id: PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C
---

# Blueprint — kaizen-consumer-ignition-filtro-c

## Estrategia

Perfil consumidor como **proyección** (no castración) + ignición hermética + tripartita Vía C. Orden PBI §5 respetado. Genoma solo vía `entity-manager`.

Laudos: `spec.md` §1 (`L-PROFILE` … `L-FORGE`).

## Fases

### T0 — Dedalo (este documento)

- [x] `clarify.md` / `objectives.md` / `spec.md` / `plan.md`
- Gate: laudo `perfil-consumidor-tripartita-via-c`; sin dualidad de norma

### T1 — Runtime Filtro C + R-07 + F-07

- [x] `start-sddia.sh`: perfil consumer excluye `github-bridge-watcher`; R-07 no spawn email/telegram si jurisdicción systemd
- [x] Kalma2: forge hidden/disabled en consumer
- [x] Gate Fracture consumer (skip forja; no vaciar JSON Core)
- [x] `email-watcher`: bootstrap `last_uid=0` → últimos N UIDs
- Gate: tests/smoke local; O1–O3

### T2 — Bundle F-06

- [x] Implementar `build-release-bundle` (script/cápsula; forjar tool si indexación exige)
- [x] Resolver grafo de códice → incluir eferentes (`send-telegram-notification`)
- [x] Generar `ONBOARDING.md` alineado al artefacto
- Gate: paquete sin fuentes ingeniería; O4

### T3 — Norma + ONBOARDING canónico

- [x] Evolucionar `sddia-distribution-protocol` (Vía C + bundle + creator + `%f` + uuid v4) — laudo locus Core vs library_norms
- Gate: norma indexada; O5 (capa norma)

### T4 — `instance-creator` + smoke

- [x] CREATE proceso `instance-creator` vía `entity-manager`
- [x] Materializar fases: topología, vault, systemd `%f`, ignición (diferida), smoke (orquestable)
- [x] Post-ignición: smoke real `success: true` sin `skip_smoke` (`native-topology+local-qa` + `Local_QA_Requested`)
- Gate: O5 (motor) + O6

### T5 — F-08 / F-09

- [x] Plantilla `sddia-daemon@.service.template` `%f`; constitución consumidor sin L2 Windows
- [x] Dual instancia lab (WD distintos)
- [x] Migración unidad host documentada en `systemd-f08-migration.md` (aplicación enable = operador)
- Gate: O7 + O8

### T6 — Paciente 0 (opcional)

- [x] Re-despliegue lab desde preprod vault vía `instance-creator` (`dist/paciente0-redeploy`, `skip_ignition`)
- Gate: demostración APTO

### T7 — Argos + cierre documental

- [x] `implementation.md` / `execution.md` / `validacion.md` APTO
- [x] PBI → `docs/todos/done/` en rama; `pbi_archived: true`
- [ ] `delivery-close-cycle` (un PR)

## Dependencias

| Antes | Después |
|-------|---------|
| T1 | T2–T5 (runtime estable) |
| T2 | T3 (ONBOARDING como proyección del bundle) |
| T3 | T4 (norma describe el motor) |
| T4 | T5–T6 |
| T1–T5 | T7 |

## Notas Tekton

- Prefijo RAW si creators: verificar topología `persist_ref` (activa).
- Tras acuse CLI: fire-and-forget (DA-5).
- Evolution al cerrar: uuid PBI `1c70e777-9b7f-4ad3-ada5-225ab6d141c6`.

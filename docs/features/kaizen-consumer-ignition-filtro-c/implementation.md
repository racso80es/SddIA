---
feature_name: kaizen-consumer-ignition-filtro-c
created: "2026-08-20"
process: feature
items: "T1-T5-parcial"
branch_name: feat/kaizen-consumer-ignition-filtro-c
persist_ref: docs/features/kaizen-consumer-ignition-filtro-c
document_id: PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C
---

# Implementation — kaizen-consumer-ignition-filtro-c

## T1 — Runtime Filtro C + R-07 + F-07

| Touchpoint | Cambio |
|------------|--------|
| `start-sddia.sh` / `start-sddia.md` v1.3.0 | Perfil + R-07 |
| Kalma2 + bridge | Forge hidden / 403 / `/api/runtime-profile` |
| `route_domain_core` | `skipped-consumer-profile` |
| `email-watcher` | Bootstrap últimos N UIDs |

## T2 — Bundle F-06

| Artefacto | Evidencia |
|-----------|-----------|
| `SddIA/scripts/build-release-bundle.sh` | Genera paquete + `MANIFEST.json` + `ONBOARDING.md` |
| Lab | `dist/sddia-release-consumer-lab` — 0 `.rs`; `send-telegram-notification` presente; sin github-bridge launcher |

## T3 — Norma

| Artefacto | Nota |
|-----------|------|
| `SddIA/norms/sddia-distribution-protocol.md` v1.1.0 | UUID v4 `c17189c7-…`; bundle + instance-creator + `%f` + ONBOARDING |
| Laudo locus | `norm-creator` solo escribe `library/norms` (`factory.rs`); Core `directories.norms` mutado bajo feature activa |

## T4 — instance-creator

| Artefacto | Evidencia |
|-----------|-----------|
| Forja | `entity-manager` CREATE → uuid `dead5ca7-…` event `f15b3b01-…` |
| Handler | `handlers/instance_creator.rs` nativo |
| Smoke lab | `./sddia-run.sh --process instance-creator` → `success:true` (skip smoke/ignition) |

## T5 — F-08 / F-09 (parcial)

| Artefacto | Estado |
|-----------|--------|
| `templates/systemd/sddia-daemon@.service.template` | Añadido (`%f`) |
| `templates/constitution-consumer/CONSTITUTION.md` | Sin L2 Windows/pwsh |
| Dual instancia lab | `dist/lab-instance-{a,b}` WD distintos |
| Migración `~/.config/systemd/user/sddia-daemon@.service` host | Doc en norma; enable real pendiente operador |

## Pendiente cierre

- [x] Smoke real + T6 vault + validacion APTO + PBI archivado
- [ ] `delivery-close-cycle` (PR)
- Enable systemd user en host (operador; ver `systemd-f08-migration.md`)

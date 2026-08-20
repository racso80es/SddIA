---
document_id: PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C
uuid: "1c70e777-9b7f-4ad3-ada5-225ab6d141c6"
title: "[KAIZEN] Perfil de ignición consumidor — poda Filtro C (github-bridge, Forjar Proceso, suscriptores Fracture)"
format: markdown
version: "0.1.0"
status: pending
type: kaizen
priority: media
created: "2026-08-20"
updated: "2026-08-20"
derived_from: PBI-LAB-PACIENTE0-SDDIA-AP
tech_debt_ids:
  - DT-START-SDDIA-CONSUMER-PROFILE
friction_ids:
  - F-04
---

# [KAIZEN] Perfil ignición consumidor — Filtro C

## Origen

Ensayo `PBI-LAB-PACIENTE0-SDDIA-AP` (§11.4 F-04, §11.5): instancia consumidor dispara suscriptores de forja ante `System_Fracture_Detected` (`enrich-fracture-pbi-kaizen`, `materialize-fracture-pbi`). Además `start-sddia.sh` intenta `github-bridge-watcher` y la WUI expone «Forjar Proceso».

## Objetivo

Perfil de producto **consumidor** que garantice Filtro C sin poda operativa manual:

1. `start-sddia.sh` / plantilla systemd: no lanzar `github-bridge-watcher` en perfil consumidor.
2. WUI Kalma2: ocultar o deshabilitar «Forjar Proceso» en build/perfil consumidor.
3. Suscripciones de dominio: no enrutar `System_Fracture_Detected` a procesos de ingeniería en instancias sin códice de software-engineering (o flag de perfil).

## Fuera de alcance

- Wizard de configuración (`DT-CONFIG-UX-ONBOARDING`, aplazada).
- Cobertura systemd completa (`DT-SYSTEMD-FULL-COVERAGE`).

## Criterios de aceptación (borrador)

- [ ] Instancia tipo `SddIA_AP` no emite ni consume procesos `feature`/`bug-fix`/enrich-fracture de forja.
- [ ] Gate de prueba: inducir fracture sintética → cero suscriptores de ingeniería en DL/processed.
- [ ] Documentación de perfil en starter-kit / constitución local.

## Notas de forja

Mutación de genoma vía proceso `feature`/`kaizen` (no edición manual). Vincular UUID de este PBI en evolution al cerrar.

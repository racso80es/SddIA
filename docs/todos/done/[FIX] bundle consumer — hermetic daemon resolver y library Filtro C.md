---
document_id: PBI-FIX-BUNDLE-CONSUMER-HERMETIC-LIBRARY
uuid: "c1d2e3f4-a5b6-4789-0abc-def123456789"
type: process
version: "1.0.0"
title: "bundle consumer — hermetic daemon resolver y library Filtro C"
status: done
friction_ids:
  - F-BUNDLE-HERMETIC-DAEMON-RESOLVER
  - F-BUNDLE-LIBRARY-FILTRO-C
derived_from: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260920
audit_ref: docs/audits/paciente0-deploy-20260920T093200Z.md
persist_ref: docs/fixes/bundle-consumer-hermetic-daemon-library
---

# FIX — bundle consumer hermetic + Filtro C library

## Síntoma

Ola 9 Paciente 0: centinelas no arrancan en bundle `MANIFEST.json` (resolutor exige crate Rust). G4 KO por `codex-software-engineering` en `library/codexes/`.

## Criterios Done

- `_sddia_resolve_daemon_binary` sirve ELF empaquetado si `MANIFEST.json` en raíz instancia.
- `build-release-bundle --profile consumer --codex <slug>` no incluye códices ajenos al slug (+ index/contrato).
- Smokes QA verdes.

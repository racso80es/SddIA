---
feature_name: core-software-forge-profile
created: "2026-10-29"
process: refactorization
tracker_ref: AUT-33
---

# Clarificación — AUT-33

## Decisiones

- Alcance mínimo: materializar `.SddIA/active-domain-profile.json` en Core-self con `software_forge: true` (AUT-33 propuesta 1).
- Derivar `software_forge` desde el índice de proyecto (propuesta 2) queda fuera de este corte; el perfil de instancia es SSOT ya documentado en Workspace 1×N.
- `instance-creator` mantiene opt-in `software_forge: true` en inputs; no cambiar default global de engineering (propuesta 3 = documentación + perfil en repo Core-self).
- Criterio de aceptación: `refine-item` / `forge-pbi` con `project_slug: core` sin `execution_profile` explícito pasan `domain-authority`.

## Huecos

Ninguno.

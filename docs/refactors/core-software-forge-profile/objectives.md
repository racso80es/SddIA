---
feature_name: core-software-forge-profile
created: "2026-10-29"
process: refactorization
branch_name: refactor/core-software-forge-profile
persist_ref: docs/refactors/core-software-forge-profile
execution_id: "c214dcd6-4e70-4c23-bfc7-8931780582df"
tracker_ref: AUT-33
document_id: PBI-497d7023
---

# Objetivos — core-software-forge-profile

## Misión

Eliminar la fricción `DOMAIN_AUTHORITY_DENIED` al invocar procesos de membresía software con `project_slug: core`, sin exigir `execution_profile.software_forge` en cada llamada.

## Alcance

1. Perfil de instancia `.SddIA/active-domain-profile.json` en el repositorio Core-self (`software_forge: true`, `codex_slug: codex-software-engineering`, `git_required: true`).
2. Test unitario en `domain_authority` que simula binding `core` + perfil de instancia y permite `refine-item`.
3. Sin regresión: perfil `codex-kalma2-assistant` sigue denegado (test existente).

## Fuera de alcance

- Derivar `software_forge` automáticamente desde `.SddIA/projects/{slug}.md`.
- Cambiar default de `instance-creator` a `software_forge: true` para todo engineering.

## Criterios de aceptación (AUT-33)

- `refine-item` y `forge-pbi` con `project_slug=core` pasan el gate sin `execution_profile` explícito.
- Test unitario en `domain_authority` cubre el caso.
- `codex-kalma2-assistant` sigue denegado.

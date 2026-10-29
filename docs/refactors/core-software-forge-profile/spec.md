---
feature_name: core-software-forge-profile
created: "2026-10-29"
process: refactorization
execution_id: "c214dcd6-4e70-4c23-bfc7-8931780582df"
tracker_ref: AUT-33
---

# Especificación — perfil software_forge Core-self

## Contexto

`resolve_execution_profile` lee `.SddIA/active-domain-profile.json` con precedencia sobre el default (`software_forge: false`). Core-self no tenía el fichero; `project_slug: core` exige `external_project_forge_allowed` → fallo sin perfil.

## Cambio

| Artefacto | Acción |
|-----------|--------|
| `.SddIA/active-domain-profile.json` | Crear JSON con `codex_slug`, `git_required`, `software_forge: true` |
| `domain_authority.rs` (tests) | Añadir `core_project_slug_allowed_with_instance_software_forge_profile` |

## Verificación

```bash
cd SddIA && cargo test -p execute-process --lib domain_authority::tests::core_project_slug_allowed -- --nocapture
./sddia-run.sh --process refine-item --inputs '{"item_ref":"AUT-33","project_slug":"core"}'
```

Segundo comando debe pasar Domain Authority (Mayeuta aparte).

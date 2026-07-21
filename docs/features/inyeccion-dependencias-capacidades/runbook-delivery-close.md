---
feature_name: inyeccion-dependencias-capacidades
created: "2026-07-21"
process: feature
purpose: Runbook cierre delivery-close-cycle — gh API Forbidden en sesion
branch_name: feat/inyeccion-dependencias-capacidades
persist_ref: docs/features/inyeccion-dependencias-capacidades
commit: 38f5809dd57b82eebc143f1f4a767b66b3102ec2
---

# Runbook — cierre de entrega (gh GraphQL Forbidden)

## Estado

| Paso | Resultado |
|------|-----------|
| Commit local | OK `38f5809` |
| Snapshot / EDA genomic | OK (`orphan_count: 0`) |
| Push `origin` | OK (rama remota actualizada) |
| `gh pr create` via shell-executor | **FAIL** — `Post https://api.github.com/graphql: Forbidden` |
| `PullRequest_Presented` | No emitido (falta `pr_url`) |
| Higiene local | skip (`SDDIA_LAB_SKIP_HIGIENE=1`) |
| PBI archive | skip intencional (`pbi_archived: false`, residual R1–R8) |

## Accion operador

```bash
# Desde entorno con acceso a api.github.com (fuera del tunnel 403):
gh pr create --repo racso80es/SddIA \
  --base main \
  --head feat/inyeccion-dependencias-capacidades \
  --title "feat(core): MVP DI por capacidades - taxonomia, metadatos y aduana temprana" \
  --body-file docs/features/inyeccion-dependencias-capacidades/_pr-body.md

# Luego sello ECST + handoff:
export SDDIA_LAB_SKIP_HIGIENE=1
./sddia-run.sh --process delivery-close-cycle --inputs '{
  "source_process": "feature",
  "persist_ref": "docs/features/inyeccion-dependencias-capacidades",
  "branch_name": "feat/inyeccion-dependencias-capacidades",
  "target_branch": "main",
  "pr_url": "https://github.com/racso80es/SddIA/pull/NNN"
}'
```

UI fallback: https://github.com/racso80es/SddIA/pull/new/feat/inyeccion-dependencias-capacidades

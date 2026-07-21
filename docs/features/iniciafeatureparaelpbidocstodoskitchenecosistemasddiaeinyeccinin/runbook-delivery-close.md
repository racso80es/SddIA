---
feature_name: iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
created: "2026-07-21"
process: feature
purpose: Runbook cierre delivery-close-cycle bloqueado por auth GitHub
correlation_id: 4dd6f7a2-7bbf-4744-8a4c-7ac315ed9a51
branch_name: feat/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
---

# Runbook — cierre de entrega (auth pendiente)

## Estado

| Paso | Resultado |
|------|-----------|
| Snapshot / EDA genomic | OK (`orphan_count: 0`) |
| Push `origin` | **FAIL** — HTTPS sin credenciales; `gh` token inválido |
| `gh pr create` | **FAIL** — `hosts.yml` token invalid |
| `PullRequest_Presented` | No emitido (`pr_url` obligatorio) |
| Higiene local | skip (`SDDIA_LAB_SKIP_HIGIENE=1`) |

## Precondiciones operador

```bash
gh auth login -h github.com
# o exportar GH_TOKEN / GITHUB_TOKEN en bóveda .SddIA/.dev/.env
git push -u origin HEAD
```

## Reintento canónico

```bash
export SDDIA_LAB_SKIP_HIGIENE=1
./sddia-run.sh --process delivery-close-cycle --inputs '{
  "source_process": "feature",
  "persist_ref": "docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin",
  "branch_name": "feat/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin",
  "target_branch": "main",
  "correlation_id": "4dd6f7a2-7bbf-4744-8a4c-7ac315ed9a51",
  "pr_title": "feat(core): Fractura Core F1 Shared Kernel products SSOT y cascaras Forge Portal",
  "pr_body": "F1 boundary Shared Kernel + products SSOT + forge/portal. validacion APTO. Ver persist_ref."
}'
```

**Nota:** evitar metacaracteres shell en `pr_body` (el stub `shell-executor` rechaza backticks/`$`/etc.).

## Tras PR

1. Anotar `pr_url` en `validacion.md`.
2. Confirmar JSON `PullRequest_Presented` en `.events/pending/` o processed.
3. Ejecutar / esperar `pull-request-review` (Argos).

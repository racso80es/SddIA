---
document_id: PBI-KAIZEN-DELIVERY-CLOSE-SNAPSHOT-PR-BODY
title: "[Kaizen] delivery-close — snapshot vacío y pr_body con newlines en shell-executor"
format: markdown
version: "1.0.0"
created: "2026-07-22"
status: abierto
priority: alta
process: bug-fix
uuid: 09c707bb-03fd-445a-9aa6-bf165b94b7e5
source_feature: docs/features/inyeccion-dependencias-resolucion-ciega
source_pr: https://github.com/racso80es/SddIA/pull/127
source_execution_id: 067337ee-4ed1-44f5-b5be-40e8d7f6deb5
related:
  - docs/features/inyeccion-dependencias-resolucion-ciega/finalize-process.md
  - docs/features/inyeccion-dependencias-resolucion-ciega/_agent_handoff.md
  - SddIA/process/delivery-close-cycle.md
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/skills/shell-executor/src/main.rs
  - SddIA/skills/git-manager/src/main.rs
  - docs/todos/done/Kaizen_delivery-close_shell-executor-wasm-fallback.md
incident_ref: "delivery-close-cycle falló: snapshot vacío (WIP no consolidado) + Apertura en forja arguments[N] forbidden shell metacharacters (\\n en pr_body)"
---

# [Kaizen] delivery-close — snapshot vacío y pr_body con newlines en shell-executor

## 0. Mandato

Abrir como **`bug-fix`** (o `feature` Kaizen si el alcance exige norma + contrato). Objetivo: convertir el incidente del cierre Hito 2 DI (PR #127) en deuda accionable del **proceso `delivery-close-cycle`**, no reabrir el alcance de producto PBI-042.

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **K1** | Snapshot final consolida WIP real | Con cambios unstaged/untracked en la rama, `delivery-snapshot-final` hace commit (vía `git-manager`) o **aborta** con error explícito; nunca reporta `executed` con `snapshot_commit_hash` = HEAD previo y working tree sucio |
| **K2** | Apertura PR sin `\n` en argv | `delivery-gh-pr` no pasa `pr_body` multilínea como argumento de `shell-executor`; usa `--body-file` (path bajo `persist_ref` / `.tmp`) o equivalente sin metacaracteres prohibidos (`\n\r;\|><\`&&$()`) |
| **K3** | Diagnóstico auditable | Si falla Apertura en forja, el envelope/fase incluye causa tipada (`PR_BODY_METACHAR` / `SNAPSHOT_DIRTY_SKIPPED`) además del string genérico |
| **K4** | Paridad lab | Smoke o test de integración: body multilínea + working tree con untracked → close exit 0 o fail determinista (no push de rama vacía de contenido) |

## 1. Incidente

| Campo | Valor |
|-------|--------|
| Fecha | 2026-07-22 |
| Feature origen | `inyeccion-dependencias-resolucion-ciega` (Hito 2 PBI-042) |
| execution_id | `067337ee-4ed1-44f5-b5be-40e8d7f6deb5` |
| Síntoma A | Fase **Snapshot final** `executed` con `snapshot_commit_hash=98f97b7` (= HEAD previo); working tree seguía con 26 paths pendientes |
| Síntoma B | Fase **Apertura en forja** `failed`: `arguments[9] contains forbidden shell metacharacters` |
| Efecto | Push de rama sin el WIP; sello `PullRequest_Presented` bloqueado; bypass manual (commit + `--body-file` + re-close con `pr_url`) |
| Mitigación sesión | Commit/push vía `git-manager`; `gh pr create --body-file`; re-`delivery-close-cycle` con `pr_url` → PR #127 + event `a7d49178-…` |

### Evidencia (extracto)

```text
Snapshot final: status=executed, commit_hash=98f97b7…  # sin consolidar WIP
Publicación remota: executed (rama nueva en origin = tip sin Hito 2)
Apertura en forja: failed — arguments[9] contains forbidden shell metacharacters
Sello Presentación ECST: failed — pr_url obligatorio
```

Causa inmediata B: `shell-executor` rechaza tokens con `\n` (`assert_safe_token`). El handler inyectaba `pr_body` Markdown multilínea como elemento de `arguments[]`.

## 2. Hipótesis de causa raíz

| # | Hipótesis | Notas |
|---|-----------|-------|
| H1 | `delivery-snapshot-final` no enumera untracked/dirs o no falla si `git-manager commit` no-op | Empuja tip vacío de contenido nuevo |
| H2 | Contrato `delivery-gh-pr` asume `pr create --body "<markdown>"` incompatible con aduana de metacaracteres | Debe ser `--body-file` |
| H3 | Falta preflight: dirty tree + body multilínea antes de push | Evitaría publicar rama incompleta |

Relacionado (no duplicar): Kaizen ya cerrado `PBI-KAIZEN-DELIVERY-CLOSE-SHELL-EXECUTOR-WASM-FALLBACK` (fallback WASI). Este PBI ataca **snapshot + argv body**, no wasmtime.

## 3. Fuera de alcance

- Residual Hito 3 PBI-042 (R5–R8).
- Cambiar la allowlist de metacaracteres de `shell-executor` para admitir `\n` en argv (incorrecto; preferir `--body-file`).
- Reescribir el producto DI del PR #127.

## 4. Arranque sugerido

```json
{
  "fix_name": "kaizen-delivery-close-snapshot-pr-body",
  "branch_name": "fix/kaizen-delivery-close-snapshot-pr-body",
  "persist_ref": "docs/fixes/kaizen-delivery-close-snapshot-pr-body",
  "base_branch": "main",
  "refined_requirements": "Kaizen: delivery-close-cycle debe (1) consolidar o abortar si hay WIP dirty en Snapshot final; (2) abrir PR con --body-file sin pasar pr_body multilinea por argv de shell-executor; (3) tipar errores PR_BODY_METACHAR / SNAPSHOT_DIRTY_SKIPPED. Semilla: docs/todos/pending/[Kaizen] delivery-close — snapshot vacío y pr_body newlines en shell-executor.md. Origen: PR #127 / execution 067337ee.",
  "pbi_ref": "docs/todos/pending/[Kaizen] delivery-close — snapshot vacío y pr_body newlines en shell-executor.md",
  "document_id": "PBI-KAIZEN-DELIVERY-CLOSE-SNAPSHOT-PR-BODY"
}
```

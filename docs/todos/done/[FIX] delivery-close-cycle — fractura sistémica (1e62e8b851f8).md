---
document_id: PBI-FIX-FRACTURE-1e62e8b851f8
uuid: "f239818c-57e0-4e94-b331-3267bfad4729"
title: "[FIX] delivery-close-cycle — snapshot gitignore `.dev` y branch_name=HEAD"
format: markdown
version: "1.1.0"
created: "2026-09-04"
updated: "2026-09-05"
status: cerrado
refinement_status: implemented
priority: alta
process: bug-fix
fracture_hash: 1e62e8b851f8
fracture_process: delivery-close-cycle
fix_ref: docs/fixes/dcc-snapshot-gitignore-1e62e8b851f8
incident_ref: "System_Fracture_Detected — 1e62e8b851f8"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/skills/git-manager/src/main.rs
  - SddIA/scripts/qa/git-hooks/hook_common.sh
  - .gitignore
  - docs/todos/done/[KAIZEN] Post-ciclo LanceDB — PAT-workflow, Kintsugi saltado, Mayeuta ciego y correlato evolution.md
---

# [FIX] delivery-close-cycle — 1e62e8b851f8

Laudo humano (ola 2 Telegram / PR #262): refinar e implementar. Mayeuta v1.0.0 **no** es SSOT de este specimen.

## 0. Filtro A

| Fuente | Texto | Veredicto |
|--------|-------|-----------|
| Cúmulo `error_trace` | `[SNAPSHOT_DIRTY_SKIPPED] git add failed` + `.gitignore` + `SddIA/scripts/starter-kit/.SddIA/.dev` | **Causa raíz #1.** `.gitignore` `**/.dev/*` (comentario explícito: `git add -A -- path` aborta el snapshot). |
| Mayeuta v1.0.0 | «Rama ausente en origin / Head sha / Halt si Publicación remota failed» | **Cubo ajeno** (`01c9040df256` / LanceDB). La traza Cúmulo **no** contiene `Head sha can't be blank`. El halt post-push **ya existe** (`dcc_push_terminal_halt`). No reimplementar. |
| Sesión PR #262 | DCC anidado pre-push con `branch_name=HEAD` → GraphQL `Head ref must be a branch` | **Causa raíz #2 concurrente.** `ref_to_branch(HEAD)` devolvía `HEAD`; `gh pr create --head HEAD`. |

Prohibido `SDDIA_SKIP_HOOKS=1` como «fix». Prohibido `-f` para forzar bóvedas `.dev`.

## 1. Incidente

| Campo | Valor |
|-------|--------|
| Proceso | `delivery-close-cycle` |
| Emisor | `execute-process` |
| Acción | `Snapshot final` |
| Hash | `1e62e8b851f8` |

```
[SNAPSHOT_DIRTY_SKIPPED] git add failed: Las siguientes rutas son ignoradas por uno de tus archivos .gitignore:
SddIA/scripts/starter-kit/.SddIA/.dev
```

## 2. Corrección

1. Snapshot omite `**/.dev` (salvo `.env.example`) y `starter-kit/.SddIA` untracked; porcelain residual no cuenta como sucio.
2. `git-manager` `commit`: si `git add` reporta path ignorado, **salta** ese path; no tumba el lote. Si todos ignorados → error explícito.
3. DCC resuelve `branch_name=HEAD` vía `branch_list` (`*` actual); detached → abort. `delivery-gh-pr` bloquea `HEAD` (`BRANCH_SYMBOLIC_HEAD`).
4. Hook `ref_to_branch`: `HEAD` → `git symbolic-ref --short HEAD`.
5. Mayeuta: cubo `F-DCC-SNAPSHOT-GITIGNORE`; no clasificar esta traza como «Rama ausente».

## 3. Criterios

| ID | Criterio |
|----|----------|
| FIX-1 | Untracked `.dev` / starter-kit `.SddIA` no entra en `files` del snapshot; tests `filter_snapshot_skips_starter_kit_dev_vault`. |
| FIX-2 | `git_add_rejected_as_ignored` cubre stderr ES/EN. |
| FIX-3 | `current_branch_from_list` lee `* feat/…`; detached → None. |
| FIX-4 | Specimen gitignore → Mayeuta `process_fix` **sin** «Rama ausente». |
| FIX-5 | IOTA / Telegram ola 2 no se revierten. |

## 4. Fuera

Halt post-push (ya en genoma). Recursión hook / `SDDIA_HOOK_DELIVERY_CLOSE`. Fine-tune LLM. Nuevo op `diff` de git-manager (`PBI-KAIZEN-GIT-DIFF-LLM-SYNTHESIS`).

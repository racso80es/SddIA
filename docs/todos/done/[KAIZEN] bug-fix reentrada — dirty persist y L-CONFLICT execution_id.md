---
document_id: PBI-KAIZEN-BUGFIX-REENTRY-DIRTY-LCONFLICT
uuid: "8a1f2c44-0e6b-4d91-b3a7-5c8e9d0f1a22"
title: "[KAIZEN] bug-fix reentrada — dirty persist y L-CONFLICT execution_id"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: kaizen
process: feature
dispatch: false
feature_name: kaizen-bugfix-reentry-dirty-lconflict
historia_ref: "docs/todos/historias/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md"
historia_document_id: HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN
historia_uuid: "d05b8d36-b0b2-494a-893c-52256447266d"
created: "2026-10-02"
author: tekton
incident_ref: "Kalma2 cid 798044c8 / 8bf81715 — workspace-init checkout abort + persist-execution-id-conflict en BX docs/fixes/e2e-workspace-1xn-ac9"
---

# Reentrada de bug-fix sobre el mismo persist_ref

Historia madre §12. El epic 1×N está cerrado. Este PBI quita el bloqueo de re-disparo observado en el piloto.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| Hay que borrar `execution_id` a mano antes de reintentar | El guard L-CONFLICT compara frontmatter con el `execution_id` vivo y falla el ciclo entero. |
| `workspace-init` puede hacer checkout a `main` con el fix ya sucio | El checkout aborta si hay cambios locales en el `persist_ref` (handoff, validacion). |

## 1. Intención

Un segundo `bug-fix` con el mismo `fix_name` y `project_slug` sobre una rama `fix/…` ya existente no muere en init ni en Dedalo por suciedad **dentro** del `persist_ref` ni por `execution_id` de un ciclo anterior.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R1 | Si el repo del piloto ya está en `branch_name` y el dirty está solo bajo `persist_ref`, init no hace checkout a `base_branch` que pise esos ficheros. |
| R2 | L-CONFLICT: reentrada del mismo `persist_ref` refresca o ignora `execution_id` de ciclos previos en los `.md` del fix (handoff excluido sigue excluido). No `failed` solo por id distinto. |
| R3 | Dirty **fuera** de `persist_ref` / `pbi_ref` sigue abortando (`dirty-worktree`), sin relajar el guard actual. |
| R4 | Tests de `workspace_init` y `check_persist_execution_id_conflict` cubren R1–R3. |

## 3. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-1 | Tests R4 verdes. |
| AC-2 | Re-disparo lab del fix `e2e-workspace-1xn-ac9` (o fixture equivalente) con handoff modificado y `execution_id` viejo en `spec.md` llega a Dedalo (`executed` o `blocked` de agente), no a `persist-execution-id-conflict` ni a checkout abort. |

## 4. Fuera de alcance

- Auto-commit de la suciedad.
- Stash global del árbol.
- Redeploy de Paciente 0.

---
document_id: PBI-KAIZEN-TQM-SLUG-PR-REF
uuid: "6c4e8a21-9b3d-4f17-a8e2-1d5f0c7b9e34"
title: "[KAIZEN] TQM — slug y pbi_ref no deben nacer de «PR #N»"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: kaizen
process: feature
dispatch: false
feature_name: kaizen-tqm-slug-pr-ref
historia_ref: "docs/todos/historias/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md"
historia_document_id: HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN
historia_uuid: "d05b8d36-b0b2-494a-893c-52256447266d"
created: "2026-10-02"
author: tekton
incident_ref: "Kalma2 cid 98be62a3 — bug-fix persistió en Core docs/fixes/pr2 (pbi_ref literal PR #2)"
---

# TQM no deriva ciclo desde «PR #N»

Historia madre §12. El epic 1×N está cerrado. Este PBI endurece el despacho Kalma2.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| «PR #2» es un `pbi_ref` válido | No hay fichero. `resolve_pbi_ref` / `derive_slug` lo convierten en slug `pr2` y `persist_ref` bajo el Core. |
| Con `project_slug` el piloto absorbe cualquier texto | El slug de **proyecto** y el slug de **ciclo** (`fix_name`) son distintos. Sin `fix_name` explícito, el texto «PR #N» gana el ciclo. |

## 1. Intención

`task-queue-manager` rechaza o ignora anclas `PR #N` / `PR#N` como `pbi_ref` y como fuente de `fix_name` / `feature_name`. El ciclo solo nace de `fix_name` / `feature_name` / `refactor_name` explícitos o de un path `docs/todos/…`.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R1 | Token que case `PR #?<dígitos>` no es `pbi_ref` ni slug de ciclo. |
| R2 | Si el prompt trae `fix_name <slug>` y además «PR #N», gana `fix_name` (ya parcial) y «PR #N» no sustituye `pbi_ref`. |
| R3 | Sin slug explícito ni path de PBI: acuse de error tipado, sin crear `docs/fixes/prN` en el Core. |
| R4 | Test unitario que reproduzca el prompt del incidente `98be62a3` y espere fallo o `fix_name` ausente, nunca `pr2`. |

## 3. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-1 | Test en `task_queue_manager.rs` verde para R4. |
| AC-2 | `project_slug=barcelonaxplorer` + texto sin `fix_name` y con «PR #2» no escribe bajo el árbol Core `docs/fixes/pr2`. |

## 4. Fuera de alcance

- Cambiar la UI de Kalma2.
- Reabrir AC-9.

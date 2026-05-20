---
feature_name: pr-presented-orchestration
process: feature
created: "2026-05-20"
persist_ref: docs/features/pr-presented-orchestration
branch_name: feat/pr-presented-orchestration
related_todo: docs/todos/[ARQUITECTURA] Acción request-change-incorporation — PR y evento PullRequest_Presented.md
---

# Objetivos — Orquestación fractal PR presentado

## Meta

Cerrar el hueco **PullRequest_Presented** en el ciclo de entrega sin violar SRP: el **proceso** `delivery-close-cycle` orquesta el «hacer físico» (GitHub) y la **acción** `emit-pr-presented-event` registra el hecho en el bus EDA, en simetría con `accept-pr` → `emit-pr-merged-event`.

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| O1 | **Abortar** la forja de `request-change-incorporation` | No existe `SddIA/actions/request-change-incorporation.md`; TODO y backlog alineados |
| O2 | **Pureza EDA** de `emit-pr-presented-event` | Acción sin `gh`, sin `git push`; solo minteo + `pending/` |
| O3 | **Contrato** `delivery-close-cycle` v1.1+ | Fase PR = `shell-executor` + `gh`; fase sello = `emit-pr-presented-event`; sin `PullRequest_Merged` en este proceso |
| O4 | **Norma** `pull-request-orchestration.md` | Presentación vía proceso; fusión vía `accept-pr` exclusivamente |
| O5 | **Laboratorio** | Handler de proceso ejecuta la cadena A→B; smoke: `pr_url` + JSON en `docs/events/pending/` |
| O6 | **Runbooks** | Guías en `docs/features/*/execution.md` sin `gh pr create` suelto (salvo excepción normativa) |

## No objetivos (esta feature)

- Hooks Git Hito 3 (PBI-005) — solo documentar precedencia.
- Evolución de payload ECST con `pr_url` — decisión en `clarify.md` (D6).
- Retirada de shims CLI Ola C — TODO aparte.

## Estado

| Fase feature | Estado |
|--------------|--------|
| Clarificación | ✅ `clarify.md` |
| Especificación | ✅ `spec.md` |
| Implementación | ✅ genoma + handlers lab (7 fases) |
| Gobernanza Fase 3 | ✅ hash, perfiles lab, PBI-005 CA-3 parcial |
| Validación | ⏳ PR GitHub + watcher IOTA en CI/local |

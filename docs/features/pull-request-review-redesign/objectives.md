---
feature_name: pull-request-review-redesign
process: feature
created: "2026-05-22"
persist_ref: docs/features/pull-request-review-redesign
branch_name: feat/pull-request-review-redesign
related_todo: docs/todos/ARQUITECTURA_Rediseno_Proceso_pull-request-review.md
---

# Objetivos — Rediseño Aduana `pull-request-review`

## Meta

Materializar el proceso **`pull-request-review`** como **Aduana de Fricción** reactiva al estímulo **`PullRequest_Presented`**: bloqueo determinista ante entropía degenerativa, feedback correctivo vía **Argos**, certificación RBAC vía **Cerbero**, absorción Kaizen vía **Cúmulo**, sin tubería CI/CD ciega ni heurísticas en scripts locales.

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| O1 | **Intercepción EDA** | Suscripción `PullRequest_Presented` → proceso `pull-request-review` (vía `event-subscriptions.json` + `route-domain-event` / watcher) |
| O2 | **Filtro documental (Fase 1)** | Validación frontmatter YAML + coherencia de `spec.md`, `plan.md`, `implementation.md`, `objectives.md` bajo `persist_ref` de la feature del PR |
| O3 | **Filtro técnico (Fase 1)** | Invocación de herramientas de test/auditoría estática vía cápsulas autorizadas; contratos `capsule-json-io` respetados |
| O4 | **Filtro RBAC (Fase 1)** | Sub-proceso **Cerbero** certifica permisos del firmante sobre el área del genoma afectada |
| O5 | **Bloqueo duro (Fase 2)** | Violación Fase 1 → `delivery_state: failed`; Argos publica feedback atómico (línea/diff ↔ norma) |
| O6 | **Kaizen no bloqueante (Fase 3)** | Deuda menor → **Cúmulo** persiste TODO en `docs/todos/` (`[ARQUITECTURA]` / `[OPERATIVO]`) sin abortar flujo |
| O7 | **Cierre de ciclo (Fase 4)** | Éxito rotundo → handoff a **`accept-pr`** (fusión soberana); **no** duplicar merge directo fuera de `accept-pr` |
| O8 | **Limpieza genoma legacy** | Reescribir `SddIA/process/pull-request-review.md` v2.x; retirar fase **Dedalo** y referencias obsoletas a `validate-pull-requests` |
| O9 | **Handler laboratorio** | `execute_process_capsules.py` ejecuta cadena mínima Fase 1–2; smoke JSON + `validacion.md` |

## No objetivos (esta feature)

- Sustituir **`accept-pr`** ni relajar `pull-request-orchestration.md` §4.
- Hooks Git Hito 3 (`pre-push` / `post-merge`) — precedencia documentada únicamente.
- Anclaje DLT IOTA en `PullRequest_Presented` — permanece en suscriptor **Cúmulo** existente.
- Retirada de shims CLI en laboratorios `SddIA_1`…`SddIA_4` — backlog aparte.

## Estado

| Fase feature | Estado |
|--------------|--------|
| Clarificación | ✅ `clarify.md` |
| Especificación | ✅ `spec.md` |
| Plan | ✅ `plan.md` |
| Implementación | ✅ genoma + handlers + bus |
| Validación | ✅ PR #15 MERGED + ciclo Presented→Merged |

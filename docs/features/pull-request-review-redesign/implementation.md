---
feature_name: pull-request-review-redesign
created: "2026-05-22"
process: feature
status: in_progress
---

# Implementación — Aduana `pull-request-review` v2

## Genoma

| Artefacto | Versión | Estado |
|-----------|---------|--------|
| `SddIA/process/pull-request-review.md` | 2.0.0 | ✅ 7 fases Argos/Cerbero/Cúmulo |
| `SddIA/process/index.md` | — | ✅ |
| `SddIA/core/event-subscriptions.json` | — | ✅ suscriptor Argos + IOTA |
| `SddIA/events/pull-request-presented.md` | — | ✅ texto suscripciones |

## Handlers laboratorio

| Módulo | Handler | Fases |
|--------|---------|-------|
| `execute_process_capsules.py` | `execute_pull_request_review_phase` | Preparación → Handoff |
| `event-watcher.py` | `_dispatch_subscriber` + `process` | Invoca aduana desde bus |

## Variables lab

| Variable | Efecto |
|----------|--------|
| `SDDIA_LAB_SKIP_GIT_CHECKOUT` | Omite checkout en Preparación |
| `SDDIA_LAB_PR_REVIEW_DOC_FAIL` | Fuerza fallo triaje documental |
| `SDDIA_LAB_PR_REVIEW_TECH_FAIL` | Fuerza fallo triaje técnico |
| `SDDIA_LAB_PR_REVIEW_RBAC_FAIL` | Fuerza fallo Cerbero |
| `SDDIA_LAB_PR_REVIEW_KAIZEN` | Genera TODO Kaizen |
| `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF` | Omite encadenar `accept-pr` (default watcher) |
| `SDDIA_LAB_SIMULATE_IOTA` | IOTA simulado en watcher |

## Pendiente

- Purge refs `validate-pull-requests` en labs `SddIA_1`…`SddIA_4`
- Comentarios atómicos Argos en forja (IDE runtime completo)

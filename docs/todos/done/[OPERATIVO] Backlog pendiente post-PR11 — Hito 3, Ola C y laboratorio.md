---
document_id: TODO-BACKLOG-PENDIENTE-POST-PR11
title: "[OPERATIVO] Backlog pendiente post-PR11 — Ola C, laboratorio e higiene"
format: markdown
version: "1.7.0"
created: "2026-05-20"
updated: "2026-05-25"
status: cerrado
priority: alta
closed_by: docs/features/l1-o5-runbooks-paridad
supersedes: null
related:
  - docs/todos/done/[OPERATIVO] Planificación de Backlog_ Resolución de Pasivos y Automatización Core (Ola A).md
  - docs/features/pbi-005-hito3-ola-b
  - docs/features/pbi-005-hito3-git-hooks
  - docs/features/vanguardia-soberania-local
  - docs/features/laboratorio-handlers-l2-l3
  - docs/features/e1-iota-ci
  - docs/features/ola-c-v3-coreografia
  - docs/features/refactor-topologia-eventos-ola-c-v3
  - docs/features/l1-o5-runbooks-paridad
  - docs/todos/done/[ARQUITECTURA] Deuda Ola C — Retirar compatibilidad CLI execute-process y execute-action.md
  - docs/todos/done/[ARQUITECTURA] Especificación Técnica Avanzada_ El Genoma de Eventos y Coreografía Asíncrona (Ola C) V3.md
  - docs/todos/done/[FIX] accept-pr — higiene silenciosa delete_branch tras merge.md
  - docs/todos/done/[FIX] delivery-close-cycle — hooks EDA, evento Presented y gobernanza operador IA.md
  - docs/todos/pending/norma-paridad-documental.md
  - docs/features/pull-request-review-redesign
  - docs/features/pull-request-automation-dlt
  - SddIA/process/accept-pr.md
  - SddIA/actions/emit-domain-mutation.md
  - SddIA/process/delivery-close-cycle.md
  - SddIA/scripts/qa/ecst_validation.py
---

# Backlog pendiente (consolidado) — CERRADO

> **Contexto (2026-05-20):** **PBI-005 cerrado al 100 %** en `main` (PR #13, merge `ed543c8`, CA-3 completo). Orquestación fractal PR (PR #11), aduana `pre-commit` (PR #12) y hooks ciclo PR Ola B (PR #13) en producción. Este manifiesto agrupaba la deuda **posterior al PBI** — no reabrir Hitos 1–3.

> **Cierre (2026-05-25):** **L1-O5 runbooks** entregado en feature [`l1-o5-runbooks-paridad`](../../features/l1-o5-runbooks-paridad/) — runbook SSOT `runbook-accept-pr.md`, gate `verify-runbook-paridad.py`, manifiesto archivado.

---

## Análisis de estado final (2026-05-25)

| Bloque | Estado |
|--------|--------|
| **PBI-005 / Ola C shims** | ✅ Cerrado (OC.5 residual no bloqueante) |
| **Cadena PR reactiva** | ✅ PR #11 + #15 + #36 + #37 |
| **L.1 `accept-pr`** | ✅ 100 % — código + runbooks |
| **E.2 `emit-domain-mutation`** | ✅ 100 % |
| **L.2–L.3 laboratorio** | ✅ Entregado |
| **E.1 IOTA CI** | ✅ PR #40 |
| **Ola C V3 coreografía (P4)** | ✅ PR #41 |
| **L1-O5 runbooks** | ✅ `l1-o5-runbooks-paridad` |

---

## Definición de hecho (completa)

- [x] **PBI-005** y **CA-3** al 100 % (`main`, PR #13).
- [x] **OC.1–OC.4** completos (OC.5 residual no bloqueante).
- [x] **L.1** cápsula estricta + higiene ramas auditable (vanguardia P1, PR #37).
- [x] **E.2** aduana ECST en `emit-domain-mutation` pre-`pending/` (vanguardia P1, PR #37).
- [x] **L.2** gate Impacto SddIA en `delivery-close-cycle` (lab).
- [x] **L.3** handlers fases 6–7 en `feature` + `execution_report` honesto fases 2–5.
- [x] **E.1** IOTA físico en CI (`run-iota-ci-smoke` + workflow, PR #40).
- [x] **P4** Ola C V3 coreografía — runtime + cierre documental (`ola-c-v3-coreografia`, PR #41).
- [x] **L1-O5** runbooks sin `git-manager` suelto — [`runbook-accept-pr.md`](../../features/l1-o5-runbooks-paridad/runbook-accept-pr.md).
- [x] Manifiesto archivado en `docs/todos/done/`.

---

## Referencias rápidas

| Tema | Ruta |
|------|------|
| Runbook SSOT merge | `docs/features/l1-o5-runbooks-paridad/runbook-accept-pr.md` |
| Feature cierre L1-O5 | `docs/features/l1-o5-runbooks-paridad/` |
| Feature vanguardia | `docs/features/vanguardia-soberania-local/` |
| Fusión PR (genoma) | `SddIA/process/accept-pr.md` |
| Norma PR | `SddIA/norms/pull-request-orchestration.md` |

---

## Histórico (inmutable)

El contenido operativo detallado de prioridades P1–P5 permanece en el historial git de este archivo (v1.6.0). No reabrir tracks cerrados salvo regresión demostrada.

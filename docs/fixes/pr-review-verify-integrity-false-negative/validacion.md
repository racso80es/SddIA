---
feature_name: pr-review-verify-integrity-false-negative
created: "2026-05-22"
process: bug-fix
branch: main
global: APTO
merged_pr: 32
merge_commit: e7b0c7de989ffef7a9598d0dcaf0e308c09f0141
closed: "2026-05-22"
pbi_archived: true
checks:
  CA-O1-sync-worktree: pass
  CA-O2-verify-integrity-aduana: pass
  CA-O3-smoke-pr-review: pass
  CA-O4-delivery-close-presented: pass
  CA-O5-retroactive-eda-batch: pass
  CA-O6-watcher-route-pr32: pass
  eda-orphan-scan: pass
git_changes:
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/scripts/qa/verify-process-integrity.py
  - SddIA/scripts/qa/smoke-pr-review-verify-integrity.py
  - docs/fixes/pr-review-verify-integrity-false-negative/
---

# Validación — pr-review-verify-integrity-false-negative

**Veredicto global: APTO**

## Merge

| Campo | Valor |
|-------|--------|
| PR | https://github.com/racso80es/SddIA/pull/32 |
| `merge_commit` | `e7b0c7de989ffef7a9598d0dcaf0e308c09f0141` |
| CI | `verify-tools-index` + `verify-process-integrity` (workflow sddia-index-qa) ✅ |

## Corrección técnica

| ID | Criterio | Evidencia |
|----|----------|-----------|
| CA-O1 | Sync `origin/<branch>` antes de verify | `_sync_pr_review_worktree` + smoke |
| CA-O2 | Triaje técnico sin falso negativo | `pull-request-review` smoke pre-merge |
| CA-O3 | `SDDIA_REPO_ROOT` en subproceso | `capsule_pr_review_technical` |

## Auditoría EDA — fixes recientes sin eventos

### Diagnóstico

| Factor | Efecto |
|--------|--------|
| `/.events/` en `.gitignore` | El bus no viaja con el repo; solo evidencia local |
| `should_skip_pre_push_present` | Si `gh pr` está `OPEN`/`MERGED` o ya hay Presented en bus, **no** se ejecuta `delivery-close-cycle` en `pre-push` |
| PR creadas con `gh pr create` antes del primer push con hook | Push posteriores omiten emisión de Presented |
| Purga sweeper tras `route-domain-event` | Padre sale de `pending/`; `accept-pr` puede marcar **merge huérfano** si no encuentra Presented en scan |

### PR #32 (este fix)

| Evento | event_id | Notas |
|--------|----------|-------|
| `PullRequest_Presented` (delivery-close) | `322e52b3-9535-4214-8f80-10b6cb7e5ea1` | Emitido en Fase Sello; watcher purgó pending |
| `PullRequest_Presented` (retroactivo) | `f7bc751c-3193-4c97-8598-90ad58a277e3` | `emitter_agent: retroactive-fix`; enrutado OK |
| `PullRequest_Merged` (post-merge hook) | `f23bcfe0-1f3a-4cb0-be5d-7ed7e447eb45` | `traceability_anomaly: merge_huérfano` — scan no vio Presented tras purga |

### Retroactivo lote PR #29–#31 (2026-05-22)

| PR | Presented | Merged | Watcher |
|----|-----------|--------|---------|
| #29 event-pending-sweeper | `39ab683b-4100-49be-a421-b5f56859351f` | `72937f9e-ec79-4c1a-a369-35e6d4bd309f` | OK |
| #30 kaizen-cierre-documental | `19d44586-04ad-4c84-a025-f230139d0a4b` | `c5e37f58-9928-4955-8d85-1c8fde6f36c1` | Presented: route falló (rama ya fusionada) |
| #31 docs cerrar PBI | `fe567363-cf3b-4490-945e-4f5e7a6ff458` | `6c811077-f9fc-418f-b1de-d451bf47fd0c` | Presented: route falló (rama docs) |
| #32 (correlación) | `f7bc751c-3193-4c97-8598-90ad58a277e3` | (ver arriba) | OK |

Payloads en `tmp/retroactive-eda-pr*.json`.

## PR #23 / dead-letter `c2573529`

Evento histórico no presente en bus local actual. Con el fix desplegado, nuevas aduanas sobre ramas `fix/*` deben pasar triaje técnico; re-emisión manual opcional si se recupera JSON desde backup.

## Kaizen operativo

- Persistir snapshot de eventos retroactivos en `docs/fixes/.../eda-retroactive-manifest.json` (opcional).
- Mejorar `scan_presented_for_branch` para incluir testigos en `processed/subscribers/` tras purga del padre.

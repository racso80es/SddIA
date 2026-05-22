---
document_id: PBI-FIX-PR-REVIEW-VERIFY-INTEGRITY-FN
title: "[FIX] pull-request-review — falso negativo verify-process-integrity en aduana"
format: markdown
version: "1.0.0"
created: "2026-05-22"
status: "cerrado"
priority: alta
process: bug-fix
merged_pr: 32
merge_commit: e7b0c7de989ffef7a9598d0dcaf0e308c09f0141
closed: "2026-05-22"
incident_ref: "PR #23 — aduana rechazó con hash mismatch; verify directo OK"
feature_ref_target: docs/fixes/pr-review-verify-integrity-false-negative
related:
  - SddIA/process/pull-request-review.md
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/scripts/qa/verify-process-integrity.py
  - SddIA/scripts/qa/recalc-process-hash-signatures.py
  - SddIA/scripts/daemons/event-watcher.py
  - docs/fixes/delivery-close-hook-eda-governance/validacion.md
  - docs/todos/done/[FIX] delivery-close-cycle — hooks EDA, evento Presented y gobernanza operador IA.md
  - docs/fixes/pr-review-fetch-prune/
---

# [FIX] pull-request-review — falso negativo verify-process-integrity en aduana

## 0. Mandato del PBI

Debe iniciarse como **`bug-fix`** bajo `docs/fixes/pr-review-verify-integrity-false-negative/`.

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | **Reproducir** el falso negativo de forma determinista | Test/smoke documentado: mismatch en subproceso aduana vs OK en invocación directa |
| **O2** | **Corregir** la causa raíz | `pull-request-review` Fase Triaje técnico → exit 0 cuando `verify-process-integrity` pasa en la rama bajo revisión |
| **O3** | **Restaurar** trazabilidad EDA PR #23 | Re-enrutar o re-emitir `PullRequest_Presented` `c2573529-…` desde `dead-letter/` → `processed/` tras aduana OK |
| **O4** | **Blindaje regresión** | Smoke aduana en CI/lab no depende de `accept-pr` directo ante fallo Argos |

---

## 1. Incidente (2026-05-22)

| Campo | Valor |
|-------|--------|
| Contexto | Cierre fix `delivery-close-hook-eda-governance` — PR #23 |
| Rama | `fix/delivery-close-hook-eda-governance` |
| Evento | `PullRequest_Presented` `c2573529-ca49-4716-bbf9-ae77135be8fe` |
| Síntoma | Aduana **rechazada** — `delivery_state.argos: failed` → evento en `dead-letter/` |
| Workaround | `accept-pr` directo (merge OK; `presented_found: true` en bus) |
| Verificación directa | `python SddIA/scripts/qa/verify-process-integrity.py` → **OK** (misma sesión, misma rama) |

### Traza de error (Triaje técnico)

```
verify-process-integrity falló en triaje técnico
refactorization.md: hash_signature mismatch (file c11858282c97b420… vs computed 5417bf69d8e10115…)
sddia-difusion.md: hash_signature mismatch (file c19c0b7f31b41bb2… vs computed ddcbbe126f7f2753…)
skill-creator.md: hash_signature mismatch (file 47937582521c663a… vs computed b440cb791d2d5ea6…)
task-queue-manager.md: hash_signature mismatch (file 5c13fa2e977fda21… vs computed 8242f460c0de8738…)
tool-creator.md: hash_signature mismatch (file 223830a95dd61b1e… vs computed d93db300fd0598a1…)
```

**Patrón:** el hash **en fichero** corresponde al post-`recalc-process-hash-signatures --write` (rama fix); el hash **computado** en subproceso coincide con valores **pre-recalc** (rama `main`).

---

## 2. Diagnóstico técnico (hipótesis)

| # | Hipótesis | Evidencia |
|---|-----------|-----------|
| H1 | **Subproceso verify lee worktree desincronizado** tras `fetch`/`checkout` en Fase 1 | Mismatch file=new / computed=old |
| H2 | **`cwd` o `REPO` root distinto** en `subprocess.run` de `capsule_pr_review_technical` | Ver `execute_process_capsules.py` ~558–572 |
| H3 | **Caché / estado git** — checkout no materializa archivos antes del verify | Fetch+checkout manual posterior → verify OK |
| H4 | **Watcher invoca aduana** con worktree en `main` mientras payload apunta a rama feature | `event-watcher.py` dispatch `pull-request-review` |

### Cadena afectada

```
PullRequest_Presented (pending/)
  → event-watcher / route-domain-event
  → pull-request-review
      → Fase 1: capsule_pr_review_branch_prep (fetch + checkout)
      → Fase 3: capsule_pr_review_technical (subprocess verify)  ← FALLO
  → dead-letter/ (argos failed)
```

---

## 3. Alcance del fix (Tekton)

### Hito 1 — Reproducción

- [ ] Script smoke mínimo: `fetch` + `checkout` + verify vía mismo patrón subprocess que aduana.
- [ ] Documentar en `docs/fixes/.../clarify.md` causa raíz confirmada.

### Hito 2 — Corrección

- [ ] Ajustar `capsule_pr_review_technical` y/o `capsule_pr_review_branch_prep` para garantizar worktree coherente antes de verify.
- [ ] Opciones a evaluar: verify post-checkout inline; pasar `ref` explícito; invocar verify como función importada en lugar de subprocess; `git reset --hard origin/<branch>` tras checkout.

### Hito 3 — Retroactivo EDA PR #23

- [ ] Re-procesar `c2573529-ca49-4716-bbf9-ae77135be8fe` desde `dead-letter/` tras fix, o re-emitir Presented correlacionado.
- [ ] Evidencia en `validacion.md`.

### Hito 4 — Regresión

- [ ] Smoke: rama `fix/*` con recalc reciente → `pull-request-review` → veredicto `aprobado` sin bypass.

---

## 4. Proceso de inicio

```json
{
  "process": "bug-fix",
  "fix_name": "pr-review-verify-integrity-false-negative",
  "branch_name": "fix/pr-review-verify-integrity-false-negative",
  "persist_ref": "docs/fixes/pr-review-verify-integrity-false-negative",
  "bug_summary": "Corregir falso negativo de verify-process-integrity en triaje técnico de pull-request-review; restaurar aduana EDA y dead-letter PR #23.",
  "base_branch": "main"
}
```

---

## 5. Criterio de cierre del PBI

- [ ] Argos **APTO** en `docs/fixes/pr-review-verify-integrity-false-negative/validacion.md`.
- [ ] `pull-request-review` exit 0 en rama de prueba post-fix.
- [ ] Evento `c2573529-…` en `processed/` o retroactivo documentado.
- [ ] Este TODO movido a `docs/todos/done/`.

---

## 6. Referencias

| Artefacto | Ruta |
|-----------|------|
| Handler triaje técnico | `SddIA/scripts/qa/execute_process_capsules.py` → `capsule_pr_review_technical` |
| Verify integridad | `SddIA/scripts/qa/verify-process-integrity.py` |
| Recalc hashes | `SddIA/scripts/qa/recalc-process-hash-signatures.py` |
| Watcher dispatch | `SddIA/scripts/daemons/event-watcher.py` |
| Fix precedente | `docs/fixes/delivery-close-hook-eda-governance/` |
| Dead-letter | `docs/events/dead-letter/c2573529-ca49-4716-bbf9-ae77135be8fe.json` |

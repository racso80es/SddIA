---
document_id: PBI-FIX-DELIVERY-CLOSE-EDA
title: "[FIX] delivery-close-cycle — hooks EDA, evento PullRequest_Presented y gobernanza operador IA"
format: markdown
version: "1.0.0"
created: "2026-05-22"
status: "abierto"
priority: alta
process: bug-fix
incident_ref: "PR #20 — ampliacion-configuracion-entornos (merge f0ef7bf sin bus EDA)"
feature_ref_target: docs/fixes/delivery-close-hook-eda-governance
related:
  - SddIA/process/delivery-close-cycle.md
  - SddIA/process/accept-pr.md
  - SddIA/process/bug-fix.md
  - SddIA/norms/pull-request-orchestration.md
  - SddIA/scripts/qa/git-hooks/pre_push_gate.py
  - SddIA/scripts/qa/git-hooks/hook_common.py
  - SddIA/actions/emit-pr-presented-event.md
  - docs/features/pr-presented-orchestration/
  - docs/features/ampliacion-configuracion-entornos/validacion.md
  - docs/todos/pending/AmpliacionConfiguracionEntornos.md
---

# [FIX] delivery-close-cycle — hooks EDA, evento Presented y gobernanza operador IA

## 0. Mandato del PBI (objetivos)

Este documento **es** el PBI en `docs/todos/pending`. Debe iniciarse como **`bug-fix`** bajo `docs/fixes/delivery-close-hook-eda-governance/`.

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | **Corregir** `delivery-close-cycle` invocado desde hook `pre-push` | Push de feature no entra en bucle recursivo; proceso termina en ≤1 ciclo con sello `PullRequest_Presented` |
| **O2** | **Restaurar** trazabilidad EDA del incidente PR #20 | `PullRequest_Presented` + `PullRequest_Merged` correlacionados con `feat/ampliacion-configuracion-entornos` / PR #20 en bus (`processed/`) |
| **O3** | **Gobernanza operador IA** ante fallo del flujo SddIA | Regla explícita: prohibido `gh pr create` / `gh pr merge` / `SDDIA_SKIP_HOOKS` como cierre sin PBI previo en `docs/todos/pending/` |
| **O4** | **Runbook de escalado** | Checklist: fallo hook → PBI → fix → retroactivo EDA → cierre canónico |
| **O5** | **Idempotencia Ola B** | Re-push con PR abierto o evento Presented existente no duplica sello ni re-dispara ciclo infinito |

---

## 1. Incidente (2026-05-22)

| Campo | Valor |
|-------|--------|
| Feature | `ampliacion-configuracion-entornos` (Hito 0 Jerarquía de Bóvedas) |
| PR | https://github.com/racso80es/SddIA/pull/20 — **MERGED** `f0ef7bf` |
| Síntoma | **Sin** `PullRequest_Presented` ni `PullRequest_Merged` en bus local |
| Causa raíz (probable) | `git push` con `SDDIA_SKIP_HOOKS=1` (hook pre-push bloqueado/recursivo) + `gh pr create` + `gh pr merge` fuera de `delivery-close-cycle` / `accept-pr` |
| Evidencia hook | `pre-push` → `invoke_process("delivery-close-cycle")` → push interno → **re-entrada pre-push** → error JSON anidado (~14 min) |
| Violación normativa | `pull-request-orchestration.md` §3 y §4 |

---

## 2. Diagnóstico técnico

### 2.1 Cadena canónica (referencia)

```
git push (feature)
  → pre-push hook
  → delivery-close-cycle
      → git-manager push
      → shell-executor gh pr create|view
      → emit-pr-presented-event → docs/events/pending/
  → accept-pr (post-revisión)
      → git-manager merge
      → emit-pr-merged-event
```

### 2.2 Fallos detectados

| # | Fallo | Impacto |
|---|--------|---------|
| F1 | **Recursión pre-push ↔ delivery-close-cycle** | Push abortado o timeout; operador usa `SDDIA_SKIP_HOOKS=1` |
| F2 | **Atajos GitHub CLI** | PR mergeado sin bus EDA |
| F3 | **IA no escaló a PBI** | Se priorizó entrega sobre gobernanza SddIA |
| F4 | **`resolve_persist_ref` solo `docs/features/`** | Ramas `fix/*` no resuelven `persist_ref` en hook |

---

## 3. Alcance del fix (Tekton)

### Hito 1 — Anti-recursión hook + delivery-close-cycle

- [ ] Variable de guarda en hook/payload: `SDDIA_HOOK_DELIVERY_CLOSE=1` o `source: git-hook-pre-push` → **omitir** push si ya dentro de ciclo hook.
- [ ] En `delivery-close-cycle` handler lab: si `git-hook-pre-push`, push con `SDDIA_SKIP_HOOKS=1` **solo** en subproceso hijo (documentado), no global.
- [ ] Test smoke: un push de rama `fix/smoke-hook` genera **un** `PullRequest_Presented` sin bucle.

### Hito 2 — Retroactivo PR #20

- [ ] `emit-pr-presented-event` con `branch`, `pr_url`, `emitter_agent: delivery-close-cycle-retroactive`.
- [ ] `accept-pr` o emisión `PullRequest_Merged` con `merge_commit_hash: f0ef7bf4bb9e28e67091d70a6fba6f8fadcbf280`.
- [ ] `event-watcher --once` → `processed/` + evidencia en `docs/fixes/.../validacion.md`.

### Hito 3 — Gobernanza operador IA

- [ ] Regla en `SddIA/norms/` o `SddIA/agents/` (Tekton/Mayeuta): **si el flujo SddIA falla**, la IA **debe** crear PBI en `docs/todos/pending/` antes de bypass manual.
- [ ] Plantilla mínima PBI: incidente, objetivos O*, proceso (`bug-fix`|`feature`), `related`, criterio de cierre.
- [ ] Prohibición explícita en runbook: `gh pr merge` hacia `main` sin `accept-pr` salvo PBI que lo documente como deuda temporal.
- [ ] Actualizar skill/regla Cursor del operador (si aplica) con enlace a este PBI como precedente.

### Hito 4 — Robustez Ola B

- [ ] Verificar `should_skip_pre_push_present` cuando PR ya MERGED (evitar re-presentación).
- [ ] Extender `resolve_persist_ref` → `docs/fixes/{slug}` para ramas `fix/*`.
- [ ] Documentar en README § Jerarquía de Bóvedas: `PYTHONUTF8=1` en bóveda global **no** sustituye flujo EDA.

---

## 4. Proceso de inicio

```json
{
  "process": "bug-fix",
  "fix_name": "delivery-close-hook-eda-governance",
  "branch_name": "fix/delivery-close-hook-eda-governance",
  "persist_ref": "docs/fixes/delivery-close-hook-eda-governance",
  "bug_summary": "Corregir recursión pre-push/delivery-close-cycle, retroactivo EDA PR #20, y regla operador IA: fallo SddIA → PBI en docs/todos/pending.",
  "base_branch": "main"
}
```

---

## 5. Criterio de cierre del PBI

- [ ] Argos **APTO** en `docs/fixes/delivery-close-hook-eda-governance/validacion.md`.
- [ ] Smoke push hook → `PullRequest_Presented` en `pending/` → `processed/`.
- [ ] PR #20 con eventos retroactivos registrados.
- [ ] Norma/regla IA publicada y referenciada desde README o `pull-request-orchestration.md`.
- [ ] Este TODO movido a `docs/todos/done/`.

---

## 6. Protocolo operador (mientras el fix está abierto)

1. **No** usar `SDDIA_SKIP_HOOKS=1` salvo emergencia documentada en issue/PBI.
2. Si hook falla → **parar** → crear/actualizar PBI en `docs/todos/pending/` (como este).
3. Cierre PR: `delivery-close-cycle` (presentación) + `accept-pr` (fusión).
4. Bypass temporal solo con checklist O2/O5 explícito en el PBI activo.

---

## 7. Referencias

| Artefacto | Ruta |
|-----------|------|
| Norma PR | `SddIA/norms/pull-request-orchestration.md` |
| Proceso cierre | `SddIA/process/delivery-close-cycle.md` |
| Proceso merge | `SddIA/process/accept-pr.md` |
| Hook pre-push | `SddIA/scripts/qa/git-hooks/pre_push_gate.py` |
| Feature incidente | `docs/features/ampliacion-configuracion-entornos/` |

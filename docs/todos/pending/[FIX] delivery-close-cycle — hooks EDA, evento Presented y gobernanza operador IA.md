---
document_id: PBI-FIX-DELIVERY-CLOSE-EDA
title: "[FIX] delivery-close-cycle — hooks EDA, evento PullRequest_Presented y gobernanza operador IA"
format: markdown
version: "1.0.0"
created: "2026-05-22"
status: "en-progreso"
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
  - docs/fixes/delivery-close-hook-eda-governance/
  - SddIA/events/system-fracture-detected.md
  - SddIA/actions/materialize-fracture-pbi.md
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - SddIA/norms/obediencia-procesos.md
---

# [FIX] delivery-close-cycle — hooks EDA, evento Presented y gobernanza operador IA

## 0. Mandato del PBI (objetivos)

Este documento **es** el PBI en `docs/todos/pending`. Debe iniciarse como **`bug-fix`** bajo `docs/fixes/delivery-close-hook-eda-governance/`.

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | **Corregir** `delivery-close-cycle` invocado desde hook `pre-push` | Push de feature no entra en bucle recursivo; proceso termina en ≤1 ciclo con sello `PullRequest_Presented` |
| **O2** | **Restaurar** trazabilidad EDA del incidente PR #20 | `PullRequest_Presented` + `PullRequest_Merged` correlacionados con `feat/ampliacion-configuracion-entornos` / PR #20 en bus (`processed/`) |
| **O3** | **Gobernanza operador IA** ante fallo del flujo SddIA | Regla explícita: prohibido `gh pr create` / `gh pr merge` / `SDDIA_SKIP_HOOKS` como cierre sin PBI previo en `docs/todos/pending/` |
| **O4** | **Kintsugi EDA + Autoconocimiento** | `System_Fracture_Detected`; fan-out Cúmulo (Qué) + Mayeuta (Por Qué); backfill Fase C |
| **O5** | **Idempotencia Ola B** | Re-push con PR abierto/MERGED o evento Presented existente no duplica sello ni re-dispara ciclo infinito |
| **O6** | **Runbook de escalado** | Checklist: fallo hook → fractura EDA → PBI Cúmulo → fix → retroactivo EDA → cierre canónico |

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

- [x] Variable de guarda `SDDIA_HOOK_DELIVERY_CLOSE=1` en subproceso hook + skip temprano en `pre_push_gate`.
- [x] En `delivery-close-cycle` handler lab: si `git-hook-pre-push`, push con `SDDIA_SKIP_HOOKS=1` **solo** en subproceso hijo (documentado en `delivery-close-cycle.md`).
- [x] Smoke lab: guarda hook verificada; sello `PullRequest_Presented` sin recursión en `tmp/`.

### Hito 2 — Retroactivo PR #20

- [x] `PullRequest_Presented` `868d1b8f-0171-4f8f-ab72-19382941523d` (`emitter_agent: retroactive-fix`).
- [x] `PullRequest_Merged` `75b8e950-9366-4ce5-bf22-b4b56430736e` (`merge_commit_hash: f0ef7bf…`).
- [x] `event-watcher --once` → `processed/` + evidencia en `docs/fixes/delivery-close-hook-eda-governance/validacion.md`.

### Hito 3 — Gobernanza Universal (Prohibición de Bypass Físico)

- [x] `SddIA/norms/obediencia-procesos.md` v1.1 — Ley de Jurisdicción Delegada.
- [x] Escalado vía `System_Fracture_Detected` o PBI antes de bypass manual.
- [x] `pull-request-orchestration.md` §7 — cross-ref Kintsugi.
- [ ] Actualizar skill/regla Cursor del operador con enlace a este PBI *(deuda post-merge)*.

### Hito 4 — Evento Nativo de Fractura (Kintsugi EDA y Autoconocimiento)

- [x] Contrato `SddIA/events/system-fracture-detected.md` — payload: `process_name`, `error_trace`, `agent_emitter`, `attempted_action`.
- [x] `event-subscriptions.json` — suscripción **dual** (orden fan-out):
  - **Cúmulo** (`materialize-fracture-pbi`) — Gestor de Deuda Técnica: el **Qué** ha fallado.
  - **Mayeuta** (`enrich-fracture-pbi-kaizen`) — Auditor Kaizen: el **Por Qué** + propuesta evolutiva.
- [x] Reacción Cúmulo: PBI mecánico en `docs/todos/pending/` categorizado `bug-fix`.
- [x] Reacción Mayeuta: sección **Conclusión Analítica y Propuesta Evolutiva** en el PBI de Cúmulo.
- [x] Backfill Fase C entidades Kintsugi (`orphan_count_after: 0`).

### Hito 5 — Robustez Ola B

- [x] `should_skip_pre_push_present` skip si PR `OPEN` o `MERGED`.
- [x] `resolve_persist_ref` → `docs/fixes/{slug}` para ramas `fix/*`.
- [x] README § Jerarquía de Bóvedas: `PYTHONUTF8=1` no sustituye flujo EDA.

---

## 4. Estado de ejecución (2026-05-22)

| Campo | Valor |
|-------|--------|
| Rama | `fix/delivery-close-hook-eda-governance` |
| persist_ref | `docs/fixes/delivery-close-hook-eda-governance/` |
| Argos | **APTO** — ver `validacion.md` |
| Smoke push hook | `PullRequest_Presented` `01656c3a-…` emitido en push real (2026-05-22) |
| Pendiente cierre | Push remoto → PR → `accept-pr` → mover PBI a `done/` |

---

## 5. Proceso de inicio

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

## 6. Criterio de cierre del PBI

- [x] Argos **APTO** en `docs/fixes/delivery-close-hook-eda-governance/validacion.md`.
- [x] Smoke push hook remoto → `PullRequest_Presented` `01656c3a-c3e4-4564-8937-05d300013b68` (push `20c5ee5`, anti-recursión OK).
- [x] PR #20 con eventos retroactivos registrados.
- [x] Norma IA publicada (`obediencia-procesos.md` v1.1 + `pull-request-orchestration.md` §7).
- [ ] Este TODO movido a `docs/todos/done/` *(tras merge del fix)*.

---

## 7. Protocolo Operador (Modo "Kintsugi Ontológico")

Marco innegociable ante fallos sistémicos — vigente mientras el fix está abierto y como precedente permanente:

1. **Intercepción y Emisión:** Si la IA o el flujo encuentra un bloqueo (ej. hook en pánico), la ejecución se detiene de inmediato.
2. **El Grito del Sistema:** El proceso fallido invoca `route-domain-event` emitiendo `System_Fracture_Detected.json` en el bus.
3. **Delegación de Deuda (Cúmulo):** Materializa el PBI — el **Qué** ha fallado.
4. **Autoconocimiento (Mayeuta):** Enriquece el PBI con causa raíz y **Conclusión Analítica y Propuesta Evolutiva** — el **Por Qué**.
5. **Laudo Humano:** La IA se detiene y notifica al Vértice Biológico: *"El proceso ha colapsado. Evento de fractura emitido. Cúmulo ha documentado la deuda. Mayeuta ha enriquecido el diagnóstico. A la espera de instrucciones."*
6. **No hay Bypass Silencioso:** El flujo no avanza hasta reparación o salto táctico explícito.

**Reglas tácticas adicionales:**

- **No** usar `SDDIA_SKIP_HOOKS=1` salvo emergencia documentada en PBI activo.
- Cierre PR canónico: `delivery-close-cycle` (presentación) + `accept-pr` (fusión).
- Bypass temporal solo con checklist O2/O5/O6 explícito en el PBI activo.

---

## 8. Referencias

| Artefacto | Ruta |
|-----------|------|
| Norma PR | `SddIA/norms/pull-request-orchestration.md` |
| Proceso cierre | `SddIA/process/delivery-close-cycle.md` |
| Proceso merge | `SddIA/process/accept-pr.md` |
| Hook pre-push | `SddIA/scripts/qa/git-hooks/pre_push_gate.py` |
| Evento fractura | `SddIA/events/system-fracture-detected.md` |
| Acciones Kintsugi | `materialize-fracture-pbi`, `enrich-fracture-pbi-kaizen` |
| Feature incidente | `docs/features/ampliacion-configuracion-entornos/` |

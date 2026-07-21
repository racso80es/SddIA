---
document_id: PBI-KAIZEN-KALMA2-FEATURE-CYCLE-OBS
title: "[Kaizen] ciclo Kalma2-feature — correlación EDA, estados terminales y aduana PPR"
format: markdown
version: "1.0.0"
created: "2026-07-21"
status: abierto
priority: alta
process: feature
uuid: 0cfe3c43-b83b-4b89-9597-173a667fb9f5
source_audit: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin/auditoria-pull-request-review.md
source_feature: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
source_correlation_id: 4dd6f7a2-7bbf-4744-8a4c-7ac315ed9a51
related:
  - docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin/auditoria-pull-request-review.md
  - docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin/validacion.md
  - docs/todos/done/[OPERATIVO] PBI: Integración Real de Kalma2 con el Motor de Eventos SddIA.md
  - docs/todos/pending/[FIX] kalma2-bridge — fractura sistémica (cbe0c30b3695).md
  - SddIA/process/feature.md
  - SddIA/process/pull-request-review.md
  - SddIA/process/task-queue-manager.md
  - SddIA/norms/capsule-json-io.md
  - SddIA/library/norms/features-documentation-pattern.md
incident_ref: "Kalma2 Forjar Proceso feature 4dd6f7a2 — timeout UI 120s; rastro EDA ausente; PPR no aplicable; F1-E blocked"
---

# [Kaizen] ciclo Kalma2-feature — correlación EDA, estados terminales y aduana PPR

## 0. Mandato

Abrir como **`feature`** Kaizen (no absorbe el residual F1 de Fractura Core). Objetivo: convertir el aprendizaje de la auditoría del ciclo `4dd6f7a2-…` en deuda accionable de **proceso / observabilidad**, no en reescritura del producto GesFer.

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Correlación durable Kalma2 → bus → workspace → docs | Dado un `correlation_id` emitido por Kalma2, existe rastro recuperable en `.events/` **o** registro durable SSOT (workspace/testigo) hasta cierre o dead-letter |
| **O2** | Estados terminales hacia el front | Kalma2 recibe `initialized` / `completed` / `failed` (o equivalente ECST) sin depender de timeout ciego de 120 s cuando el proceso sigue vivo |
| **O3** | Inventario de entrega reproducible | Todo feature que añada member Cargo exige `Cargo.lock` actualizado + evidencia `cargo check --locked` en `execution.md`/`validacion.md` |
| **O4** | Aduana PPR sin DL por input prematuro | `pull-request-review` no cae a dead-letter por `pr_url` ausente cuando el ECST aún no lo aporta; contrato de inputs alineado con `PullRequest_Presented` real |
| **O5** | Separación residual feature vs Kaizen de proceso | Norma/checklist: residual de alcance (commits, AC) permanece en el `persist_ref` de la feature; fallos de lazo/observabilidad generan PBI Kaizen (este patrón) |

## 1. Incidente / aprendizaje (fuente)

| Campo | Valor |
|-------|--------|
| Estímulo | Kalma2 «Forjar Proceso» → `feature` sobre PBI kitchen GesFer Paciente 0 |
| Correlación UI | `4dd6f7a2-7bbf-4744-8a4c-7ac315ed9a51` |
| Persistencia feature | `docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin/` |
| Auditoría | [`auditoria-pull-request-review.md`](../../features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin/auditoria-pull-request-review.md) |
| Síntoma operador | Timeout UI 120 s; Centinelas vivos; cambios en disco sin commit/PR |
| Veredicto auditoría | Implementación `APTO_CON_RESERVAS`; objetivos `PARCIAL`; cierre `NO_APTO`; PPR **no aplicable** |

### Hallazgos que motivan este Kaizen (no el residual F1)

1. **Rastro EDA incompleto:** el `correlation_id` no aparece en `pending|processing|processed|dead-letter`; no hay workspace `feature/<correlation_id>`.
2. **Lazo UI abierto:** el front espera estados terminales que el ciclo no emite/propaga a tiempo; el proceso puede seguir forjando en filesystem mientras Kalma2 ya marcó `timeout`.
3. **Aduana ciega:** sin commits/`PullRequest_Presented`, `pull-request-review` no puede auditar; DL histórico por `INPUT_VALIDATION` falta `pr_url` confirma fricción de contrato.
4. **Reproducibilidad:** member `sddia-core` sin lockfile versionado → `cargo check --locked` falla; no estaba en inventario documental.
5. **Contaminación WT / allowlist de sesión:** F1-E bloqueado por allowlist Shell; commits y smoke npm no ejecutables en la misma sesión — deuda de runtime de agentes, no de spec F1.

> El residual F1 (commit aislado, actualizar lock, re-Argos, `delivery-close-cycle`) **pertenece a la feature actual**. Este PBI Kaizen ataca el **sistema nervioso** que permitió el ciclo incompleto sin rastro auditable.

## 2. Fuera de alcance

- Completar Fractura Core F1 / empaquetar Nodos de Control.
- Fases 2–4 del PBI kitchen GesFer.
- Sustituir el FIX pendiente `kalma2-bridge` (prótesis LLM); este Kaizen **coordina** con él, no lo absorbe.
- Merge/cierre de la feature `iniciafeature…` (sigue su propio Done gate).

## 3. Diseño objetivo (laudo)

```text
Kalma2 Forjar Proceso
  → event_id/correlation_id durable en bus o testigo
  → task-queue-manager / feature emiten initialized
  → fases Mayeuta…Argos escriben bajo persist_ref + workspace correlacionado
  → completed|failed (o Process_Execution_Completed) llega al sondeo UI
  → solo entonces delivery-close-cycle → PullRequest_Presented (pr_url cuando exista)
  → pull-request-review opera sobre PR real sin DL por input prematuro
```

## 4. Proceso de inicio

```json
{
  "process": "feature",
  "feature_name": "kaizen-kalma2-feature-cycle-observability",
  "branch_name": "feat/kaizen-kalma2-feature-cycle-observability",
  "persist_ref": "docs/features/kaizen-kalma2-feature-cycle-observability",
  "refined_requirements": "Kaizen: correlación durable Kalma2↔EDA↔workspace; estados terminales al front sin timeout ciego; inventario lockfile/--locked; contrato PPR alineado a PullRequest_Presented; separar residual de feature vs deuda de proceso.",
  "pbi_ref": "docs/todos/pending/[Kaizen] ciclo Kalma2-feature — correlación EDA, estados terminales y aduana PPR.md",
  "base_branch": "main"
}
```

## 5. Dependencias / coordinación

| Relación | Artefacto | Nota |
|----------|-----------|------|
| Soft-dep | `[FIX] kalma2-bridge — fractura sistémica (cbe0c30b3695)` | Colapso prótesis LLM; puede bloquear chat SSE mientras se audita el lazo de proceso |
| Soft-dep | Feature `kalma2-event-bus-integration` (done) | Declaró lazo UI↔EDA; la auditoría demuestra hueco residual en producción local |
| Ortogonal | Feature Fractura Core F1 (`4dd6f7a2`) | Residual de producto; no fusionar scopes |

## 6. Criterio de cierre del PBI

- [ ] Feature Kaizen con cascada bajo `docs/features/kaizen-kalma2-feature-cycle-observability/`.
- [ ] Smoke: disparo Kalma2/`Kalma2_Process_Requested` → `correlation_id` recuperable + al menos un estado terminal observado por el cliente (o contrato de sondeo documentado y verificado).
- [ ] `pull-request-review` no genera DL por `pr_url` ausente en el camino feliz de `delivery-close-cycle` (contrato + test/lab).
- [ ] Norma o checklist de entrega: member Cargo ⇒ lockfile + `--locked` en evidencia Argos.
- [ ] `validacion.md` APTO + este PBI en `docs/todos/done/` en el **mismo** PR (patrón v1.2.x).

## 7. Referencias de evidencia

- Auditoría: `docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin/auditoria-pull-request-review.md`
- Validación feature origen: `…/validacion.md` (`global: NO_APTO`)
- Evolution: `SddIA/evolution/4dd6f7a2-7bbf-4744-8a4c-7ac315ed9a51.md`
- Proceso aduana: `SddIA/process/pull-request-review.md` v2.2.0
- Integración previa: `docs/todos/done/[OPERATIVO] PBI: Integración Real de Kalma2 con el Motor de Eventos SddIA.md`

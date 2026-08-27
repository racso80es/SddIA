---
feature_name: emit-pr-audited-revoked-registry-rehab-ppr202
created: "2026-08-27"
purpose: Estabilización Mayeuta — PBI-PPR-202-EMIT-PR-AUDITED-REVOKED-REGISTRY (rehab emit-pr-audited-event lateral Cerbero)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/emit-pr-audited-revoked-registry-rehab-ppr202
persist_ref: docs/features/emit-pr-audited-revoked-registry-rehab-ppr202
pbi_ref: docs/todos/done/[ARQUITECTURA] emit-pr-audited-event — rehabilitación revoked_entities (PPR #202).md
document_id: PBI-PPR-202-EMIT-PR-AUDITED-REVOKED-REGISTRY
uuid: c2e8f4a1-7b3d-4e9c-a5f6-8d1e2f3a4b5c
source_correlation_id: "1498e461-3235-483a-b210-907cca744cdd"
source_pr_url: https://github.com/racso80es/SddIA/pull/202
feature_ref: docs/features/accept-pr-revoked-registry-rehab-ppr200
incident_ref: "REVOKED_ENTITY_ALERT_EMIT_PR_AUDITED — emit-pr-audited-event ∈ revoked as tool (abrupt_success_rate_drop since 2026-06-12T10:10:06+00:00)"
olas:
  - A1
---

# Clarificación — emit-pr-audited-revoked-registry-rehab-ppr202

Transcript Mayeuta. Estabiliza el **qué** y el **por qué**. Sin diseño de cápsulas ni mutación de genoma.

## D0 — Semilla y evidencia

| Vector | Hecho |
|--------|--------|
| PBI canónico | `docs/todos/done/[ARQUITECTURA] emit-pr-audited-event — rehabilitación revoked_entities (PPR #202).md` (`document_id: PBI-PPR-202-EMIT-PR-AUDITED-REVOKED-REGISTRY`; `uuid: c2e8f4a1-…`; `status: done`) |
| Ciclo | `refactorization` · rama `refactor/emit-pr-audited-revoked-registry-rehab-ppr202` · un `persist_ref` · un PR |
| Semilla operador | Rehabilitar `emit-pr-audited-event` en Cerbero/Radamanto tras revocación lateral `abrupt_success_rate_drop` (since `2026-06-12T10:10:06+00:00`) |
| Padre | PPR #202 Cosecha Kaizen (`persist_ref`: `docs/features/accept-pr-revoked-registry-rehab-ppr200`; CID `1498e461…`) — elevó alerta F5 a seed canónica |
| Check origen | `REVOKED_ENTITY_ALERT_EMIT_PR_AUDITED` (F5 Argos · alerta no bloqueante) + Cosecha #202 |
| Sighting dedup | CID `3dcf4dfb-cd9c-4733-9925-b80f3f5806f4` · 0 create · affirm seed |

### Estado empírico (corte estabilización 2026-08-27)

| Clave | Cerbero | Radamanto | Nota |
|-------|---------|-----------|------|
| `emit-pr-audited-event` | **`revoked.emit-pr-audited-event`** · `entity_type: tool` (fósil) · `reason: abrupt_success_rate_drop` · `since: 2026-06-12T10:10:06+00:00` | bucket raíz **ausente** (sin stats previos) | Acción Core revocada como `tool` fósil — jurisprudencia #174/#194 |
| Laterales | `revoked.refactorization` | fuera de alcance | Prohibido rehabilitar este ciclo |

Dictamen (vinculante):

1. Entidad = acción Core (`SddIA/actions/emit-pr-audited-event.md`); registro Cerbero usa `entity_type: tool` fósil — conservar en stats (**L-ONTOLOGY**).
2. Revocación lateral sin PBI canónico hasta Cosecha #202; diseño aduana documentó «Sin seed» (ABSTRACT-03 D7).
3. No hay fases de proceso ni agregador terminal: **solo A1** Yunque Rúnico; sin A2 motor.

## D1 — Misión (qué / por qué)

| Decisión | Laudo |
|----------|--------|
| Objetivo | Rehabilitar `emit-pr-audited-event` en Cerbero/Radamanto para restaurar emisión `PullRequest_Audited` en aduana `pull-request-review`. |
| Por qué ahora | Alerta F5 lateral elevada a seed en Cosecha #202; deuda genérica desde `2026-06-12`. |
| Efecto observable | `emit-pr-audited-event` ∉ `revoked` ni `permanent`; stats raíz `healthy` con laudo #202; acción invocable vía handler nativo. |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un `persist_ref`, un PR. |
| **L-WAVES** | Solo **A1** instancia. Sin A2 motor (acción atómica; no lifecycle). |
| **L-REHAB-INST** | A1 = instancia `.SddIA/`. Evidencia en `execution.md`. Prohibido versionar instancia en el diff del PR. |
| **L-CERBERO** | Eliminar nodo `revoked.emit-pr-audited-event`. Verificar `permanent.emit-pr-audited-event` ausente. |
| **L-STATS** | Crear bucket raíz `emit-pr-audited-event` con reset absoluto. |
| **L-RESET-ABS** | `status: healthy`; `recovery_attempts: 0`; `consecutive_success_count: 0`; `degraded_at: null`; `rehab_laudo: PBI-PPR-202-EMIT-PR-AUDITED-REVOKED-REGISTRY`; `rehabilitated_at` ISO de A1; `samples: []`. |
| **L-ONTOLOGY** | Conservar `entity_type: tool` (fósil Cerbero; acción Core). |
| **L-OUT** | Fuera: rehab `refactorization`; reabrir alcance accept-pr #200; mutar umbrales; versionar instancia en el PR. |
| **L-DOC** | Cascada patrón + `validacion.md` APTO + PBI en `done/`. |

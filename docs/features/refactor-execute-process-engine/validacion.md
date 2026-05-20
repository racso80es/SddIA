---
feature_name: refactor-execute-process-engine
created: "2026-05-20"
process: feature
branch: main
global: APTO
checks:
  - id: feature-init
    result: pass
    evidence: process feature → rama feat/refactor-execute-process-engine + objectives.md
  - id: dynamic-interpreter-smoke
    result: pass
    evidence: --process feature --inputs-file; fase 1 executed workspace-init; 2-6 simulated
  - id: input-validation
    result: pass
    evidence: delivery-close-cycle sin source_process → INPUT_VALIDATION abort
  - id: action-registry
    result: pass
    evidence: emit-pr-presented-event, emit-pr-merged-event, emit-domain-mutation en execute-action.py
  - id: capsule-routing
    result: pass
    evidence: CAPSULE_ACTION_REGISTRY delega action:* a execute-action
  - id: eda-iota-physical
    result: pass
    evidence: watcher --once sin SDDIA_LAB_SIMULATE_IOTA; cumulo success en Presented y Merged
  - id: legacy-shim-warning
    result: pass
    evidence: --input-file y --action emiten WARNING stderr amarillo
git_changes:
  - SddIA/scripts/qa/execute-process.py
  - SddIA/scripts/qa/execute_process_core.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/scripts/qa/execute-action.py
  - SddIA/actions/emit-pr-presented-event.md
  - SddIA/actions/index.md
  - SddIA/core/event-subscriptions.json
---

# Validación — refactor execute-process engine (Argos)

**Veredicto global: APTO**

## 1. Intérprete dinámico

| Criterio | Resultado |
|----------|-----------|
| Sin ramas `if canonical == "<proceso>"` en núcleo | ✅ |
| Carga `SddIA/process/<nombre>.md` + frontmatter | ✅ |
| Validación inputs obligatorios | ✅ error JSON semántico |
| `execution_report` honesto (`executed` / `simulated` / `skipped`) | ✅ |

## 2. Inicialización de contexto (genérica)

| Criterio | Resultado |
|----------|-----------|
| Fase con `skill:git-manager` + `feature_name` | ✅ handler `workspace-init` |
| `objectives.md` bajo `persist_ref` | ✅ |
| Re-checkout rama existente (idempotencia) | ✅ fallback `checkout_feature_existing` |

## 3. Registry de acciones

| Acción | Contrato MD | Handler físico |
|--------|-------------|----------------|
| `emit-domain-mutation` | ✅ | ✅ |
| `emit-pr-presented-event` | ✅ (nuevo) | ✅ |
| `emit-pr-merged-event` | ✅ | ✅ |

## 4. Cierre EDA (IOTA testnet)

| event_id | event_type | `delivery_state.cumulo` |
|----------|------------|-------------------------|
| `5d8716d5-ed2e-4657-bc07-7bf5a7e84a29` | `PullRequest_Presented` | `success` |
| `34f30fb4-1e72-4de1-a809-faec07af8b3b` | `PullRequest_Merged` | `success` |

> Eventos `PullRequest_Presented` emitidos antes del fix `18d80ea` (`a4994fc2…`) quedaron con `delivery_state: {}` por suscripción vacía; no invalidan el cierre con `5d8716d5…`.

## 5. Compatibilidad Ola C (temporal)

| Mecanismo | Comportamiento verificado |
|-----------|-------------------------|
| `--input-file` envelope legacy | Warning + ejecución OK |
| `--action` en execute-process | Warning + subprocess execute-action |

Deuda de retirada: TODO Ola C en `docs/todos/`.

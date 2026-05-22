---
feature_name: delivery-close-hook-eda-governance
created: "2026-05-22"
process: bug-fix
branch_name: fix/delivery-close-hook-eda-governance
persist_ref: docs/fixes/delivery-close-hook-eda-governance
related_incident: "PR #20 — ampliacion-configuracion-entornos (merge f0ef7bf sin bus EDA)"
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — hooks EDA, evento Presented y gobernanza operador IA.md
---

# Objetivos — delivery-close-hook-eda-governance

## Misión

Corregir la recursión `pre-push ↔ delivery-close-cycle`, restaurar trazabilidad EDA del PR #20, instaurar gobernanza operador IA (Ley de Jurisdicción Delegada + protocolo Kintsugi) y habilitar el evento nativo `System_Fracture_Detected` como mecanismo de escalado automático de deuda.

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Anti-recursión hook | Push feature termina en ≤1 ciclo; guarda `SDDIA_HOOK_DELIVERY_CLOSE` + push interno con `SDDIA_SKIP_HOOKS=1` acotado al subproceso |
| **O2** | Retroactivo PR #20 | `PullRequest_Presented` + `PullRequest_Merged` en `docs/events/processed/` con `emitter_agent: retroactive-fix` |
| **O3** | Gobernanza IA | `obediencia-procesos.md` § Ley de Jurisdicción Delegada; prohibición bypass raw documentada |
| **O4** | Kintsugi EDA + Autoconocimiento | `System_Fracture_Detected`; fan-out Cúmulo (Qué) + Mayeuta (Por Qué); backfill Fase C |
| **O5** | Idempotencia Ola B | Re-push con PR MERGED u evento Presented existente no re-dispara ciclo ni duplica sello |
| **O6** | `resolve_persist_ref` fix/* | Ramas `fix/*` resuelven `docs/fixes/{slug}` además de `docs/features/{slug}` |

## No objetivos

- Refactor global de `git-manager` ni migración del bus fuera de `docs/events/`.
- Automatización completa del laudo humano post-fractura (Cúmulo + Mayeuta documentan; humano valida).

## Ley aplicada

- Proceso `bug-fix` v1.2.0
- `SddIA/norms/pull-request-orchestration.md` §3 y §4
- Protocolo Operador Kintsugi Ontológico (PBI §6 ampliado)

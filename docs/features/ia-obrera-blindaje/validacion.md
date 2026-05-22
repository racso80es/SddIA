---
feature_name: ia-obrera-blindaje
branch: feat/ia-obrera-blindaje
global: true
checks:
  - id: CA-1
    name: Norma external-ai-constraints
    status: pass
  - id: CA-2
    name: .cursorrules §8
    status: pass
  - id: CA-3
    name: touchpoints-ia.md
    status: pass
  - id: CA-4
    name: 8 creators con prefijo obrera
    status: pass
  - id: CA-5
    name: verify-process-integrity
    status: pass
  - id: CA-6
    name: evolution log
    status: pass
  - id: CA-7
    name: EDA orphan scan
    status: pass
git_changes:
  - SddIA/norms/external-ai-constraints.md
  - .cursorrules
  - SddIA/norms/touchpoints-ia.md
  - SddIA/process/tool-creator.md
  - SddIA/process/action-creator.md
  - SddIA/process/skill-creator.md
  - SddIA/process/agent-creator.md
  - SddIA/process/process-creator.md
  - SddIA/process/norm-creator.md
  - SddIA/process/codex-creator.md
  - SddIA/process/event-creator.md
  - SddIA/evolution/ef684063-d16f-4ee1-b5d0-d9fde843f105.md
---

# Validación — Blindaje IA Obrera

## Criterios de aceptación

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| CA-1 | `SddIA/norms/external-ai-constraints.md` con DA-1..3 | ✅ | UUID `95b5ac3a-…` |
| CA-2 | `.cursorrules` §8 referencia norma | ✅ | diff §8 |
| CA-3 | `touchpoints-ia.md` actualizado | ✅ | Cursor + Jules/Windsurf |
| CA-4 | 8/8 creators con «Directriz de ejecución obrera» | ✅ | grep count = 8 |
| CA-5 | `verify-process-integrity.py` | ✅ | exit 0 |
| CA-6 | Entrada `SddIA/evolution/` | ✅ | `ef684063-…` |
| CA-7 | EDA sin huérfanas nuevas | ✅ | `orphan_count: 0` |

## Notas Argos

- Norma motor en `SddIA/norms/` no entra en `ENTITY_DIRS` del audit EDA; coherente con `entity-manager` (evolution sin `Domain_Entity_*`).
- Cambios en creators: solo cuerpo; firmas YAML de fases intactas.

## Veredicto

**APTO** para cierre vía `delivery-close-cycle`.

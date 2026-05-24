---
feature_name: vanguardia-soberania-local
created: "2026-05-24"
process: feature
branch: feat/vanguardia-soberania-local
global: APTO
pbi_archived: true
checks:
  L1-CA1: pass
  L1-CA2: pass
  L1-CA3: pass
  L1-CA4: pass
  L1-CA5: pass
  E2-CA1: pass
  E2-CA2: pass
  E2-CA3: pass
  E2-CA4: pass
  verify-process-integrity: pass
git_changes:
  - SddIA/scripts/qa/ecst_validation.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/scripts/qa/execute-action.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/process/accept-pr.md
  - SddIA/actions/emit-domain-mutation.md
  - docs/features/vanguardia-soberania-local/
  - docs/todos/done/[FIX] accept-pr — higiene silenciosa delete_branch tras merge.md
---

# Validación — Vanguardia Soberanía Local (Argos)

**Veredicto global: APTO**

## Track L.1 — `accept-pr` higiene auditable

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| L1-CA1 | Delete OK → `closed_branch` | ✅ | Helper + contrato dual-op documentado |
| L1-CA2 | Delete fallo → `hygiene_failure` | ✅ | Smoke `accept-pr` + helper; sin `except` silencioso |
| L1-CA3 | `execution_report` fase 4 | ✅ | `operations[]` en fase Sincronización y Limpieza |
| L1-CA4 | stdout JSON puro | ✅ | Una emisión JSON en smokes |
| L1-CA5 | Genoma `accept-pr.md` | ✅ | § Fase 4 + output `hygiene_failure` |

## Track E.2 — Aduana ECST emisor

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| E2-CA1 | Emisión válida → `pending/` | ✅ | event `8916baa3-…` |
| E2-CA2 | FORBIDDEN → abort sin write | ✅ | `hash_signature_old` en create → exit 1 |
| E2-CA3 | Router usa módulo compartido | ✅ | Import `ecst_validation` en `route_domain_event_core.py` |
| E2-CA4 | Spec Paso 1b | ✅ | `emit-domain-mutation.md` |

## Integridad

| Check | Estado |
|-------|--------|
| `verify-process-integrity.py` | ✅ OK |
| Backfill Fase C | ✅ `orphan_count_after: 0`, Merkle anclado (lab IOTA) |

## Cierre documental

| Ítem | Estado |
|------|--------|
| FIX delete_branch | ✅ → `docs/todos/done/` |
| Backlog PBI § L.1 + E.2 | ✅ Criterios vanguardia satisfechos en rama feature |

## Referencias

- `execution.md` — comandos y IDs de smoke
- `implementation.md` — touchpoints
- Smoke inputs: `_smoke-*.json`

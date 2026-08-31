---
document_id: PBI-FIX-FRACTURE-01c9040df256
title: "[FIX] delivery-close-cycle — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-08-31"
status: "abierto"
priority: alta
process: bug-fix
fracture_hash: 01c9040df256
fracture_process: delivery-close-cycle
incident_ref: "System_Fracture_Detected — 01c9040df256"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
---

# [FIX] delivery-close-cycle — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `delivery-close-cycle` |
| Emisor | `execute-process` |
| Acción intentada | `Apertura en forja` |

## Traza de error

```
no se pudo resolver pr_url desde gh; gh_stdout=; gh_stderr=pull request create failed: GraphQL: Head sha can't be blank, Base sha can't be blank, No commits between main and feat/lancedb-real-vector-memory, Head ref must be a branch (createPullRequest)
; view_stdout=; view_stderr=no pull requests found for branch "feat/lancedb-real-vector-memory"
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async)*

### Diagnóstico de causa raíz

- Recursión o re-entrada en la cadena hook Git ↔ proceso de cierre (`delivery-close-cycle`).

### Veredicto evolutivo

**Refactor de herramienta / cápsula / handler lab** (`refactor_tool`)

### Propuestas

- **Refactor de herramienta / cápsula / handler lab:** Implementar guarda `SDDIA_HOOK_DELIVERY_CLOSE` y push interno con `SDDIA_SKIP_HOOKS=1` acotado al subproceso `git-manager`.

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.
## Criterio de cierre

- [ ] Causa raíz resuelta
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este TODO movido a `docs/todos/done/`

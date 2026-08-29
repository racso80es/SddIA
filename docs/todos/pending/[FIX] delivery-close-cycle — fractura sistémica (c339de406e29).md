---
document_id: PBI-FIX-FRACTURE-c339de406e29
title: "[FIX] delivery-close-cycle — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-08-29"
status: "abierto"
priority: alta
process: bug-fix
fracture_hash: c339de406e29
fracture_process: delivery-close-cycle
incident_ref: "System_Fracture_Detected — c339de406e29"
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
| Acción intentada | `Aduana evolution` |

## Traza de error

```
diff material sin evolution correlacionada
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

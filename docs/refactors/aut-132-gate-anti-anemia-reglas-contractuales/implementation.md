---
feature_name: aut-132-gate-anti-anemia-reglas-contractuales
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-132-gate-anti-anemia-reglas-contractuales
persist_ref: docs/refactors/aut-132-gate-anti-anemia-reglas-contractuales
tracker_ref: AUT-132
---
# Implementación — AUT-132

## Estrategia

El endurecimiento funcional del peaje quedó integrado en `main` con el ratchet AUT-125 (merge `a6ada07d`). Este PBI **no reabre** forjas (AUT-131) ni deuda de genoma (AUT-133).

## Entregables en rama

| Artefacto | Acción |
| --- | --- |
| Código peaje | **Sin delta** respecto a `main` — regresiones cubiertas por tests existentes |
| `SddIA/evolution/{id_cambio}.md` | Alta trazabilidad cierre AUT-132 |
| `docs/audits/AUT-132.md` | Auditoría interacciones LLM (Mayeuta) |
| `docs/refactors/aut-132-gate-anti-anemia-reglas-contractuales/` | Cascada contractual |

## Punto de anclaje

- Evolución padre: `a8f3c2e1-9d4b-4f7a-b6e5-1c2d3e4f5a6b` (AUT-125)
- Auditoría origen: `3899742f-04f5-480b-a1f6-866d93d1d5a3` (REJECT)
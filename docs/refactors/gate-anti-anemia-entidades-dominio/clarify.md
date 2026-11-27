---
feature_name: gate-anti-anemia-entidades-dominio
created: "2026-11-26"
process: refactorization
branch_name: refactor/gate-anti-anemia-entidades-dominio
persist_ref: docs/refactors/gate-anti-anemia-entidades-dominio
tracker_ref: AUT-125
---
# Clarificación — AUT-125

- **Proceso Linear:** `refactorization` (confirmado vía `fetch_issue`).
- **Testigo:** `SddIA/actions/switch-llm-provider.md` (truncado/anémico); causa raíz parcial en `run_action_forge` (stub 80 chars).
- **Q1:** Peaje = secciones `##` con cuerpo no vacío + sin marcadores stub + pares propósito/operación. La relajación «≥2 secciones sustantivas» se retiró: anulaba las keywords por familia (hallazgo de auditoría `3899742f…`).
- **Estado ciclo:** auditoría `REJECT`; genoma revertido a `main`; gate global con 322 hallazgos en 142 entidades (deuda AUT-133). Ver `execution.md`.
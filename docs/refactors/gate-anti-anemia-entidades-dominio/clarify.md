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
- **Estado ciclo:** auditoría `REJECT` del primer cierre vigente como histórico. AC-1 vigente (laudo 2026-11-27, `refine-item` `c3a91e04-7b2d-4f58-9a16-8e0d5c4b7a21`): ratchet `--if-touched` a 0 en CI y PPR; línea base heredada **322** sin incremento. La bajada a 0 global es AUT-133. AUT-131 (forjas) y AUT-132 (gate) son los otros hijos. Ver `execution.md`.
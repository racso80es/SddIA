---
feature_name: aut-131-forjas-preservar-contratos-rollback
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-131-forjas-preservar-contratos-rollback
persist_ref: docs/refactors/aut-131-forjas-preservar-contratos-rollback
tracker_ref: AUT-131
historia_ref: AUT-125
execution_id: "94c96aae-be68-47b9-99c8-21e65f9f5851"
---
# Clarificación — AUT-131

Origen: auditoría PR #480 (`audit_event_reference` `3899742f-04f5-480b-a1f6-866d93d1d5a3`, REJECT). Cinco fricciones en update de entidades vía forjas (`F-FORGE-*` en el issue Linear).

**Fuera de alcance (issue refinado):** rediseño de contratos de acción, remediación global del genoma anémico (AUT-133), cambios en `entity_no_anemia` / gate (AUT-132), semántica de create.

**Observación UUID:** AC-3 exige no ignorar `UUID_NOT_V4` como falso negativo; el código tolera **solo** `UUID_NOT_V4` preexistente en el artefacto (warnings en handoff); uuid nuevo no-v4 sigue bloqueando. Migración masiva de uuid → AUT-133.

**Herencia main:** La lógica funcional entró en merge AUT-125 (`dea3917b`); este ciclo sella regresiones, trazabilidad AUT-131 y evolución sin reabrir gate anti-anemia.
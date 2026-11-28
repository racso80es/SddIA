---
feature_name: aut-144-entity-uuid-migrate-forges
created: "2026-11-28"
process: bug-fix
branch_name: fix/aut-144-entity-uuid-migrate-forges
persist_ref: /home/racso/Proyectos/SddIA/docs/fixes/aut-144-entity-uuid-migrate-forges
pbi_ref: AUT-144
execution_id: "d0b4e016-3133-4fdd-ba51-38cd393156cc"
---

# Objetivos — aut-144-entity-uuid-migrate-forges

## Misión

\[FIX\] Forjas entity-manager (process/event/agent/skill/suite): entity_uuid_migrate atómico con sync index.md + transfer_entity_coverage + with_file_rollback

```yaml document_id: PBI-c4982d86 uuid: 7fdc48ab-bba1-4b7e-b621-7f32d51c5b32 tracker_column: todo refinement: refined process: bug-fix type: fix created: 2026-11-28 project_slug: core delivery_mode: branch_pr delivery_mode_source: project historia_ref: AUT-133 forge_key: 1636c8eead8980336d247cf7c138c96069bc2987ee6957870294cbedd6021789 tracker_ref: AUT-144 refined: 2026-11-28 refined_by: mayeuta re

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

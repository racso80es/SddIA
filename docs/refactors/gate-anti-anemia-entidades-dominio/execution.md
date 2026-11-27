---
feature_name: gate-anti-anemia-entidades-dominio
created: "2026-11-26"
process: refactorization
branch_name: refactor/gate-anti-anemia-entidades-dominio
persist_ref: docs/refactors/gate-anti-anemia-entidades-dominio
tracker_ref: AUT-125
---
# Ejecución — AUT-125

## Primer ciclo (invalidado)

El primer ciclo (commits `983e8bb8`…`9d7b7172`) declaró `violation_count=0` tras remediar el genoma con plantilla genérica, rotar 25 UUIDs, rebajar versiones SemVer y corromper `actions/index.md`. La auditoría `3899742f-04f5-480b-a1f6-866d93d1d5a3` emitió `PullRequest_Audited` con `REJECT` (`event_id` `e999fc7a-4c2c-4b0d-aebe-7c4c2ab62677`). El CI verde `45950436287` de ese ciclo no acredita conformidad.

## Remediación post-auditoría

| Tarea | Estado |
|-------|--------|
| `PullRequest_Audited` `REJECT` vía `emit-pr-audited-event` | emitido |
| PBIs de protección forjados (`forge-pbi`, hijos de AUT-125) | AUT-131 (forjas), AUT-132 (gate), AUT-133 (deuda genoma) |
| Genoma revertido a `main` (acciones, índices, UUIDs, SemVer, eda-coverage) | hecho |
| Scripts ad-hoc `rebuild_actions_index.py` / `remediate_bulk.py` | retirados |
| Gate endurecido + CLI + PPR + `phase_capsules` | hecho |
| Forjas: preservación de frontmatter, índice de fila única, rollback | hecho |
| `entity_manager`: update de acción sin defaults | hecho |
| Testigo `switch-llm-provider` remediado con contenido del handler real | hecho |
| CI: gate en modo ratchet `--if-touched` | hecho |
| Inventario pre-remediación (KA-DOC-1) | `audit-inventory.json`: 207 escaneadas, 142 entidades, 322 hallazgos |

## Estado de los AC

| AC | Estado | Nota |
|----|--------|------|
| AC-1 gate global `violation_count: 0` | **abierto** | Inalcanzable sin remediación real de 142 entidades (AUT-133). No se recorta sin `refine-item` y laudo humano. |
| AC-2 tests `entity_no_anemia` | cumplido localmente | ver `validacion.md` § Evidencia local |
| AC-3 peaje DCC/PPR `--if-touched` | cumplido localmente | in-process, `base_ref` configurable |
| AC-4 forja create/update acción | cumplido localmente | create exige secciones; update valida con rollback |

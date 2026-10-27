---
feature_name: refine-item-mayeuta
created: "2026-10-27"
process: refactorization
tracker_ref: AUT-19
execution_id: "51caf6b7-c386-447b-a380-edf5e8c97d73"
---

# Implementación — T2/T3

## Genoma (entity-manager)

- Norma `backlog-item-refinement` (`028fb30f-66e7-4811-bfc6-960c808b9517`) en `SddIA/library/norms/`.
- Proceso `refine-item` en `SddIA/library/codexes/codex-software-engineering/process/refine-item.md`.
- Códice `codex-software-engineering` **1.1.0**: `composition` + `process_membership` + `refine-item`.
- Eventos `pbi-refined` / `hu-refined`: emisor `refine-item`.
- Baja `refine-pbi` y `refine-hu` en `SddIA/process/`.
- Mayeuta: §3.2 contrato `refine-item` en cuerpo (sello EDA pudo fallar por lock `events.redb`; artefacto actualizado).

## Motor y consumidores

- `refine_item/run.rs`: `create_issue` para `hu` sin `tracker_ref` en remoto.
- `.cursorrules`, `task-closure-documental.mdc`, hooks QA, `ecst_validation`, `tracker_operations_gate`.

## Pendiente T4

- `execution.md`, `validacion.md` (`global: APTO`), commit en PR #404, CI, DCC, AUT-19 Done.

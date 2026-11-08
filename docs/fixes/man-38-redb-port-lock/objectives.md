---

feature_name: man-38-redb-port-lock
created: "2026-11-08"
process: bug-fix
branch_name: fix/man-38-redb-port-lock
persist_ref: /home/racso/Proyectos/SddIA/docs/fixes/man-38-redb-port-lock
pbi_ref: MAN-38
execution_id: "be63c70f-633c-452e-aaf1-8ffa645fd54b"
---
# Objetivos — man-38-redb-port-lock

## Misión

Liberar lock Redb: ciclo de vida acotado del EventManagerPort

```yaml document_id: PBI-90397f23 uuid: 4241c798-2cc8-4f40-8d1d-7f67f2ae77ba tracker_column: todo refinement: refined process: bug-fix created: 2026-11-08 project_slug: core delivery_mode: branch_pr delivery_mode_source: project historia_ref: MAN-35 forge_key: 20c249a3cb499c0627f270ca28c21ae6c1c813031ef67554c0f48f1645b7c7a8 tracker_ref: MAN-38 refined: 2026-11-08 refined_by: mayeuta refinement_exe

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

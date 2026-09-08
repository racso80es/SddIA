---
feature_name: nucleo-aiua-tormentosa-motor
created: "2026-09-08"
process: feature
branch_name: feat/nucleo-aiua-tormentosa-motor
persist_ref: docs/features/nucleo-aiua-tormentosa-motor
pbi_ref: docs/todos/pending/PBI_Arranque_Aiua.md
execution_id: "21377cd5-079c-484c-b5d1-c8f8429de402"
document_id: PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA
pbi_uuid: "2a4e6c88-1f3b-4d0e-9a2f-7e4b5401a892"
pbi_version: "1.2.0"
---

# Objetivos — nucleo-aiua-tormentosa-motor

## Misión

Instanciar la Aiúa (Tormentosa) como fisiología digital auditable: genoma en `SddIA/conscience/`, latido CLI `aiua-stimulus-processing`, memoria LanceDB vía tool host, única combustión Gemini, Peaje Termodinámico.

## Alcance

1. Genoma `aiua_core.md` v+ Sección 5 + `uuid`; `conscience/index.md`.
2. `README.md` raíz (fila Aiúa), `cumulo.paths.json` `directories.conscience`, cláusula Constitución.
3. Forja DA-2: tool `thought-graph-access`, acciones `retrieve-active-context` / `invoke-aiua-core` / `persist-thought-record`, proceso `aiua-stimulus-processing`.
4. Runtime nativo: handler del proceso + handlers de acciones + crate de la tool + emisión ECST del adaptador.
5. Lab-mock: ciclo completo con LanceDB vacío y `durationMs`.

## Fuera de gate

Kalma2 / `kalma2-interact`. IOTA anclaje de `Thought_Persisted`. `ThoughtTriageService` como CLI. Agente Tormentosa.

## Ley aplicada

- PBI v1.2.0 Filtro A.
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CI verde (`run_id`).
- DA-2: process/actions/tools vía `entity-manager`. `conscience/` y adaptador no son clase EM.
- Git vía `git-manager`. DCC abre PR si Cerbero lo permite; `accept-pr` solo post-CI verde.

## Criterios

| ID | Criterio |
|----|----------|
| CA-1 | `aiua_core.md` 5 secciones, uuid, bump, cero nombres propios. |
| CA-2 | `conscience/index.md` patrón Cúmulo; fila coincidente. |
| CA-3 | README raíz fila Aiúa; proceso sin agente titular. |
| CA-4 | `cumulo.paths.json` `conscience` + bump version. |
| CA-5 | Constitución referencia `paths.directories.conscience`. |
| CA-6 | Proceso 4 fases, `workspace_template`, sin `agent:`. |
| CA-7 | Tres acciones forjadas; invocan tool o ensamblan; no emiten ECST. |
| CA-8 | Tool host `thought-graph-access` search/store. |
| CA-9 | `store_thought` emite `Thought_Persisted` REQUIRED. |
| CA-10 | Lab-mock CLI captura `durationMs`; primer latido `memories: []`. |
| CA-11 | Cero fugas Kalma2. |
| CA-CI | Checks GitHub del PR verdes (`run_id`). |

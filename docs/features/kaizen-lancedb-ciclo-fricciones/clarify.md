---
feature_name: kaizen-lancedb-ciclo-fricciones
created: "2026-08-31"
process: feature
purpose: Estabilización Mayeuta — PBI-KAIZEN-LANCEDB-CICLO-FRICCIONES v1.1.0
branch_name: feat/kaizen-lancedb-ciclo-fricciones
persist_ref: docs/features/kaizen-lancedb-ciclo-fricciones
pbi_ref: docs/todos/pending/[KAIZEN] Post-ciclo LanceDB — PAT-workflow, Kintsugi saltado, Mayeuta ciego y correlato evolution.md
document_id: PBI-KAIZEN-LANCEDB-CICLO-FRICCIONES
uuid: "12250eca-49c6-4008-ac50-5c5722a7fe91"
execution_id: "b97c39ce-f5d6-4e26-92c6-68de26eedcf0"
mayeuta_verdict: ok
---

# Clarificación — kaizen-lancedb-ciclo-fricciones

Transcript Mayeuta. Semilla: PBI `PBI-KAIZEN-LANCEDB-CICLO-FRICCIONES` v1.1.0. Filtro A contra genoma vigente.

## Decisiones

| # | Decisión | Motivo |
|---|----------|--------|
| D1 | `F-DCC-WORKFLOW-SCOPE` = paridad `F-DCC-DNS-UNRESOLVED`: `blocked`, sin fractura Kintsugi de recursión | El rechazo GitHub es credencial, no colapso de Core |
| D2 | Halt de fases posteriores a Publicación remota `failed`/`blocked` | Genoma ya lo exige; el runtime lo viola; causa el PBI `01c9040df256` |
| D3 | Cubos Mayeuta **positivos** además de regresión anti-hook | El clasificador vigente ya no clasifica el specimen como hook; falta diagnóstico útil |
| D4 | Helper `relacionado` en cápsula `sddia-evolution-register` (código), no solo checklist en `.md` de skill | DA-2: no editar skill md a mano; testable |
| D5 | CA3 = cláusula en `external-ai-constraints` + `obediencia-procesos` vía EM | Barrera motora de «no git raw» en Tekton IDE es normativa; envelope DCC es el ancla motora |
| D6 | CA8 en `features-documentation-pattern` vía EM | Distinto de telemetría CI remota |
| D7 | Archivar `01c9040df256` en el mismo PR con diagnóstico §1+§1b, no ejecutar su propuesta | Done documental un PR |

## Fuera

LanceDB físico. MiniLM. Polling CI. Reabrir `0c5268362b9a`. `SDDIA_SKIP_HOOKS` global.

## Init lab

| Campo | Valor |
|-------|--------|
| Init | `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery |
| Semilla | `.tmp/feature-kaizen-lancedb-ciclo-fricciones.json` |

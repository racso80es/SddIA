---

feature_name: eda-redb-06-semantic-layer
created: "2026-10-21"
base: agents-contract
scope: descriptor cognitivo
execution_id: "14475d25-826d-4732-b88e-922c924b7344"
---
# Spec — eda-redb-06-semantic-layer

## Contrato

`agents-contract` 1.2.0 documenta el campo opcional. No tiene `uuid`: la forja reescribe `contract_version` y omite el sello de entidad.

## Agente

`entity-manager` escribe `cognitive_role_descriptor` en el YAML y añade la columna **Cognitive role** a la fila existente de `agents/index.md`. No llama a `append_row`. El hash se inserta con el cierre de frontmatter por línea, para no partir comentarios que contienen `---`.

## Parámetros de restricción

`CONSTITUTION_CORE.md` §3.1 describe la instancia sobre el store. Los Filtros C, A y B, el Protocolo de Acero y la Cicatriz Rúnica permanecen. `README.md` nombra el store, la frontera DLT y reproduce la tabla de descriptores.

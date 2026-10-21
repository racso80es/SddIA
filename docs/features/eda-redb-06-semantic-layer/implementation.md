---

feature_name: eda-redb-06-semantic-layer
created: "2026-10-21"
branch_name: feat/eda-redb-06-semantic-layer
execution_id: "14475d25-826d-4732-b88e-922c924b7344"
---
# Implementación — eda-redb-06-semantic-layer

La forja vive en `factory.rs`. `entity-manager` copia `cognitive_role_descriptor` desde la semilla. El índice solo edita la cabecera y la fila cuyo archivo es `` `{name}.md` ``.

`CONSTITUTION_CORE.md` y `README.md` se editan en esta entrega porque el PBI los nombra como parámetros de restricción. No se rebautizan los filtros.

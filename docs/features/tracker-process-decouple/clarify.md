---
feature_name: tracker-process-decouple
process: refactorization
created: "2026-10-23"
branch_name: refactor/tracker-process-decouple
persist_ref: docs/features/tracker-process-decouple
execution_id: "90e151ce-3830-4fa2-985a-784a93c90330"
document_id: PBI-KAIZEN-TRACKER-PROCESS-DECOUPLE
---

# Aclaración

La fuente de una HU, un PBI o un Kaizen la devuelve la implementación de `tracker:lifecycle`. El proceso pide un verbo del contrato y recibe un `tracker_ref` opaco.

`paths.todos` de Cúmulo y `docs_layout` del manifiesto siguen siendo la ley de los buckets. Solo la implementación de fichero los abre. `persist_ref` hacia `docs/features` y `docs/fixes` no es backlog.

MAN-22 registra las tres fricciones del primer corte: el id de proceso con el producto, los sumideros que abrían el directorio y el test que prohibía conservar `tracker_ref`.

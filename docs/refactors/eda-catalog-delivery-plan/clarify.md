---
feature_name: eda-catalog-delivery-plan
created: "2026-10-26"
process: refactorization
purpose: "El catálogo informa a los suscriptores en el alta, no en un paso posterior que nunca arranca."
---

# Clarificación

La suscripción y la lista de entregas eran dos registros. El centinela solo despacha entregas pendientes, y esas filas salían de `delivery_state`, que los emisores dejaban vacío. El catálogo se consulta ahora al publicar.

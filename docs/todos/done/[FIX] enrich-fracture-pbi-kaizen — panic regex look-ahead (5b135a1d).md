---
document_id: 5b135a1d-480d-4e8c-abca-3cca8fda97e9
title: "[FIX] enrich-fracture-pbi-kaizen — panic regex look-ahead"
format: markdown
version: "1.0.0"
created: "2026-08-15"
status: "done"
priority: critica
process: bug-fix
suggested_branch: fix/kaizen-regex-lookahead-panic
persist_ref: docs/fixes/kaizen-regex-lookahead-panic
related:
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
---

# [FIX] enrich-fracture-pbi-kaizen — panic regex look-ahead

## Problema

`start-sddia.sh` levanta `event-watcher` → `route-domain-event`. Al re-enriquecer un PBI de fractura que ya tiene sección Kaizen, el handler nativo `enrich-fracture-pbi-kaizen` panica:

```
regex parse error: (?m)^## Conclusión Analítica...[\s\S]*?(?=\n## |\Z)
error: look-around, including look-ahead and look-behind, is not supported
```

El panic envenena el `Mutex` del evento. Los hilos siguientes y el hilo main caen en `PoisonError` (`route_domain_core.rs` L1160 / L1192).

## Objetivo

Eliminar el look-ahead incompatible con `regex` y evitar que un panic de suscriptor tumbe el lote de dominio.

## Criterios de aceptación

- [x] Re-upsert de sección Kaizen existente no panica.
- [x] Se conservan headings posteriores (`## Criterio`, etc.).
- [x] Un panic de suscriptor async se registra como `failed` y no produce `PoisonError` en el orquestador.
- [x] Tests unitarios cubren placeholder y síntesis ya materializada.
---

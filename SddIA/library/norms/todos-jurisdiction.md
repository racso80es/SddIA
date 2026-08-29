---
uuid: "f0b8ce4a-2f79-4516-bee0-acfe0d25bd58"
name: "todos-jurisdiction"
version: "1.1.0"
nature: "tactical-norm"
author: "dedalo"
scope: "agnostic"
category: "workflow"
dependencies:
  - "4c448c82-de41-460f-b24f-82a84fa5ed69"
hash_signature: "sha256:6ee2ff69d955336488d4f1620eeecc808d150b34b6ebf7bfada1a2cf3b98ba01"
---

## Directriz Core

Jurisdicción de buckets bajo docs/todos/. SSOT de rutas despachables: paths.todos.pending y paths.todos.done en cumulo.paths.json. TQM extract_pbi_path y archivado feature-pbi-archive solo reconocen esos prefijos.

| Bucket | Despachable | Archivable | Ciclo |
|--------|-------------|------------|-------|
| pending/ | Sí | Sí → done/ | Cola operativa |
| done/ | Ancla en estímulo | Destino | Cierre documental |
| kitchen/ | No | No | Incubación; promoción manual a pending/ |
| historias/ | No | No | Narrativa/códice |
| tmp/ (docs/todos/tmp) | No | No | Deprecado; no depositar |
| DeudaTecnica/ | No | No | Retirado; no depositar |

Portador deuda no-fractura: type deuda + tech_debt_ids (prefijo DT-) en pending/ con dispatch false. friction_ids usa prefijo F-. Done inalterado: features-documentation-pattern v1.2.1.

## Restricciones Duras (Aduana de Fricción)

- Prohibido un tercer estado de Done.
- Prohibido anclar TQM/archivado a buckets inertes (kitchen/, historias/, tmp/, DeudaTecnica/).
- Prohibido mutar esta norma sin entity-manager.
- Prohibido reimplementar fracture_pbi / materialize-fracture-pbi / enrich-fracture-pbi-kaizen.

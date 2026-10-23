---
feature_name: tracker-process-decouple
process: refactorization
created: "2026-10-23"
branch_name: refactor/tracker-process-decouple
persist_ref: docs/features/tracker-process-decouple
---

# Plan

1. Puerto de layout y de proveedor local.
2. YAML de los ocho procesos sin producto ni bucket.
3. Renombre físico a `tracker-description-sync`.
4. Kaizen, CI crónica, plantilla de fractura, historias de `tracker-markdown-apply` y archivo de fase leen el puerto.
5. El sello de `refine-hu` conserva `tracker_ref` cuando ya existe.
6. Registro en `SddIA/evolution/` antes del cierre.

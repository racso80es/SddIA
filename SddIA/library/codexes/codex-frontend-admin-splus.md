---
uuid: "30dc1742-aba2-4c30-8a3a-eea0e256194b"
name: "SddIA Codex Frontend Admin S+"
version: "1.0.0"
nature: "domain-codex"
author: "codex-creator"
target_environment: ["frontend", "nextjs", "admin"]
certification_grade: "Pendiente"
composition:
  - norm: "4c448c82-de41-460f-b24f-82a84fa5ed69"
    path: "../norms/features-documentation-pattern.md"
  - norm: "1c6af49c-3091-4648-aa54-bbf6bcb90f82"
    path: "../norms/patterns-in-planning-implementation-execution.md"
  - norm: "7c18fe07-9567-4f06-8d2b-a58e04608171"
    path: "../norms/pr-acceptance-protocol.md"
  - norm: "86a2f359-8137-43e4-b5ff-61d235ef3cde"
    path: "../norms/nextjs-hydration-client-state.md"
  - norm: "c93387f0-d4c8-41ae-8d1a-12f2239d2710"
    path: "../norms/openapi-contract-rest-admin-frontend.md"
---

## Estrategia de Dominio

Gestión de estado pesada y validación de contratos administrativos con Next.js App Router (laboratorio **SddIA_2**). El códice asegura documentación de tarea unificada, patrones trazables en el ciclo feature, calidad de PR, paridad SSR/cliente en UI admin y alineación estricta con el Swagger del backend administrativo vía `ADMIN_API_URL`.

## Instrucciones de Prioridad

1. **`openapi-contract-rest-admin-frontend`**: prevalece sobre cualquier URL o tipo de cliente admin; `ADMIN_API_URL` es el único origen del API administrativo.
2. **`nextjs-hydration-client-state`**: prevalece en render/hidratación sobre lecturas de `localStorage`/`window` en primer paint.
3. **`pr-acceptance-protocol`**: bloquea merge y `finalize-process` si fallan checks obligatorios.
4. **`features-documentation-pattern`**: SSOT de artefactos de tarea; sin JSON duplicado.
5. **`patterns-in-planning-implementation-execution`**: desempate de diseño por `pattern_id` referenciado en plan/IMPL/execution.

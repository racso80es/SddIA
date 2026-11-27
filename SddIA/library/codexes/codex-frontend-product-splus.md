---
uuid: "0b681575-8d20-413c-bc0e-a5ef1a378f7b"
name: "SddIA Codex Frontend Product S+"
version: "1.0.0"
nature: "domain-codex"
author: "codex-creator"
target_environment: ["frontend", "nextjs", "product"]
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
  - norm: "e6ae3df7-9d47-4dd1-8051-025f9fd171c7"
    path: "../norms/openapi-contract-rest-frontend.md"
---

## Estrategia de Dominio

Foco en rendimiento de cliente, SEO y contratos REST para el portal público de GesFer (laboratorio **SddIA_4**). El códice combina documentación atómica de tareas, patrones en el ciclo de feature, barrera de PR, hidratación segura en App Router y contrato OpenAPI del API de producto.

## Instrucciones de Prioridad

1. **`openapi-contract-rest-frontend`**: prevalece sobre rutas y tipos del cliente producto; origen vía `NEXT_PUBLIC_API_URL` (o equivalente documentado).
2. **`nextjs-hydration-client-state`**: prevalece en SSR/primer render sobre estado solo-cliente en componentes `"use client"`.
3. **`pr-acceptance-protocol`**: condición necesaria para publicar rama o cerrar tarea.
4. **`features-documentation-pattern`**: un `.md` por acción; prohibido JSON paralelo.
5. **`patterns-in-planning-implementation-execution`**: precedencia de `pattern_id` en implementación y ejecución.

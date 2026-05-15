---
uuid: "04f0bc4e-ef1f-4431-a445-398f1820db07"
name: "SddIA Codex Backend Admin S+"
version: "1.0.0"
nature: "domain-codex"
author: "codex-creator"
target_environment: ["backend", "dotnet", "admin"]
certification_grade: "Pendiente"
composition:
  - norm: "4c448c82-de41-460f-b24f-82a84fa5ed69"
    path: "../norms/features-documentation-pattern.md"
  - norm: "1c6af49c-3091-4648-aa54-bbf6bcb90f82"
    path: "../norms/patterns-in-planning-implementation-execution.md"
  - norm: "7c18fe07-9567-4f06-8d2b-a58e04608171"
    path: "../norms/pr-acceptance-protocol.md"
---

## Estrategia de Dominio

Optimización de la integridad DLL y validación estricta de tipos en el ecosistema Admin .NET (laboratorios **SddIA_1** y **SddIA_3**). El códice prioriza trazabilidad de tareas, aplicación de patrones en planificación/implementación/ejecución y barrera de calidad en PR antes de integrar cambios en el dominio administrativo backend.

## Instrucciones de Prioridad

1. **`pr-acceptance-protocol`**: prevalece en cierre de rama y PR; ningún merge sin lint/build/tests y nomenclatura válida.
2. **`patterns-in-planning-implementation-execution`**: en conflicto con atajos de implementación, el `pattern_id` del plan/IMPL manda sobre convenciones locales no documentadas.
3. **`features-documentation-pattern`**: un `.md` por fase de tarea; no se aceptan artefactos JSON paralelos como SSOT de documentación.

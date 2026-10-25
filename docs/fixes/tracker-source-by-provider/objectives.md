---
feature_name: tracker-source-by-provider
created: "2026-10-25"
process: bug-fix
branch_name: fix/tracker-source-by-provider
persist_ref: docs/fixes/tracker-source-by-provider
pbi_ref: MAN-21
execution_id: "fd88acb8-0104-4101-875d-aef751fa47e6"
---

# Objetivos — tracker-source-by-provider

## Misión

MAN-21. La fuente de HU, PBI, Kaizen, fractura y fallo crónico de CI la devuelve la implementación de tracker:lifecycle que corresponde a tracker.provider. El árbol de backlog lo abre solo la implementación de fichero. El resolvedor elige la implementación. Los handlers de ciclo piden el verbo.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

---
feature_name: forge-pbi-lock-after-create
created: "2026-10-30"
process: bug-fix
branch_name: fix/forge-pbi-lock-after-create
persist_ref: /home/racso/Proyectos/SddIA/docs/fixes/forge-pbi-lock-after-create
pbi_ref: AUT-29
execution_id: "b7f670f6-35d3-411b-9e70-9fb68fb4fc61"
---

# Objetivos — forge-pbi-lock-after-create

## Misión

forge-pbi crea la issue y, si events.redb responde Locked al emitir PBI_Forged, el proceso falla sin tracker_ref ni document_id. Un reintento del mismo raw_idea crea otra issue. La autoridad de dominio con project_slug=core queda cubierta por el perfil de instancia (AUT-33) y no se reabre. Fuera: título de la issue (AUT-28).

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

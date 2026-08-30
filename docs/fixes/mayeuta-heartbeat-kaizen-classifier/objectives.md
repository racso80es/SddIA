---
feature_name: mayeuta-heartbeat-kaizen-classifier
created: "2026-08-30"
process: bug-fix
branch_name: fix/mayeuta-heartbeat-kaizen-classifier
persist_ref: docs/fixes/mayeuta-heartbeat-kaizen-classifier
pbi_ref: docs/todos/pending/[FIX] Mayeuta — clasificador Kaizen ciego a latido de centinelas.md
execution_id: "507e8ff0-388a-4040-8c52-c23b87af1dfd"
---

# Objetivos — mayeuta-heartbeat-kaizen-classifier

## Misión

F-MAYEUTA-HB-BLIND: analyze_fracture_kaizen no clasifica la traza canónica de Argos (Centinela {id} omitió N ciclos de Daemon_Heartbeat). Fallback process_fix en 24 PBIs. F-MAYEUTA-HB-TOKEN-TRAP: prohibido token heartbeat en blob general (attempted_action=daemon-heartbeat-audit). Corte: spec+plan+commit. Sin código. Cubo match solo error_trace. PBI-FIX-MAYEUTA-HB-KAIZEN-CLASSIFIER.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

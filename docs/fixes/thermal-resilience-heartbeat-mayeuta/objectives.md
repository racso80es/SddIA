---
feature_name: thermal-resilience-heartbeat-mayeuta
created: "2026-09-07"
process: bug-fix
branch_name: fix/thermal-resilience-heartbeat-mayeuta
persist_ref: docs/fixes/thermal-resilience-heartbeat-mayeuta
pbi_ref: docs/todos/done/[FIX] Resiliencia Térmica en Heartbeat Audit y Poda Ontológica en Mayeuta.md
execution_id: "0e0f6614-f1bd-40c5-9f63-af8e0a482786"
---

# Objetivos — thermal-resilience-heartbeat-mayeuta

## Misión

PBI-FIX-THERMAL-RESILIENCE-HEARTBEAT-MAYEUTA v1.1.0: (A) daemon-heartbeat-audit emite System_Fracture_Detected por lock huérfano cuando el PID murió con el host; falta puerta btime de /proc/stat antes de emit_orphan_lock_fracture. Invariante L3: PID muerto post-boot sigue fracturando. (B) analyze_fracture_kaizen clasifica la traza lock huérfano como EDA genómica por token huérfan. Cubo is_orphan_l

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

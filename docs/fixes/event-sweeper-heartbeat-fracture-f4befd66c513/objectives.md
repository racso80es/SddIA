---
feature_name: event-sweeper-heartbeat-fracture-f4befd66c513
created: "2026-07-19"
process: bug-fix
branch_name: fix/event-sweeper-heartbeat-fracture-f4befd66c513
persist_ref: docs/fixes/event-sweeper-heartbeat-fracture-f4befd66c513
---

# Objetivos — event-sweeper-heartbeat-fracture-f4befd66c513

## Misión

PBI-FIX-FRACTURE-f4befd66c513 | [FIX] event-sweeper — fractura sistémica
Incidente: System_Fracture_Detected — f4befd66c513
Proceso: event-sweeper | Emisor: argos | Acción: daemon-heartbeat-audit
Traza: Centinela event-sweeper omitió 39 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-07-19T17:13:18Z
Mandato: Corregir causa raíz. Prohibido bypass raw (gh, git, curl) hasta cierre documentado.
Veredicto Mayeuta: process_fix — Auditar proceso event-sweeper, acción daemon-heartbeat-audit y emisor argos.
PBI: docs/todos/pending/[FIX] event-sweeper — fractura sistémica (f4befd66c513).md

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

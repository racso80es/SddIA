---
feature_name: centinelas-fracture-ola-20260723
created: "2026-08-11"
process: bug-fix
branch_name: fix/centinelas-fracture-ola-20260723
persist_ref: docs/fixes/centinelas-fracture-ola-20260723
---

# Objetivos — centinelas-fracture-ola-20260723

## Motivo del fix

No hay defecto activo en centinelas al 2026-08-11. El fix existe para **cerrar deuda documental**: cinco satélites `System_Fracture_Detected` (2026-07-23…25) cuya causa operativa ya fue mitigada (olas 0716/0722 + PR #155) pero cuyos `document_id` distintos quedaron en `pending/` sin archivo. Auditoría empírica → laudo (B); genoma intacto.

## Misión

Ola unificada de 5 PBIs System_Fracture_Detected (Daemon_Heartbeat omitido, umbral=3, emisor argos / acción daemon-heartbeat-audit).

PBIs:
1) PBI-FIX-FRACTURE-21f55bcdecfb — event-sweeper — 469 ciclos; last_heartbeat=2026-07-23T06:10:33Z
2) PBI-FIX-FRACTURE-0d65b4775574 — event-watcher — 469 ciclos; last_heartbeat=2026-07-23T06:10:33Z
3) PBI-FIX-FRACTURE-a69be9535f82 — github-bridge-watcher — 234 ciclos; last_heartbeat=2026-07-23T06:10:31Z
4) PBI-FIX-FRACTURE-131fa2c33271 — telegram-watcher — 18 ciclos; last_heartbeat=2026-07-24T05:44:49Z
5) PBI-FIX-FRACTURE-d67f6c0b0195 — telegram-watcher — 17 ciclos; last_heartbeat=2026-07-25T08:07:09Z

Contexto: misma familia que olas docs/fixes/centinelas-fracture-ola-20260716 y centinelas-kalma2-fracture-ola-20260722; fix parcial posterior daemon-heartbeat-ingest-ignition (PR #155) no archivó estos document_id.

Estado empírico al arranque (2026-08-11): los 4 centinelas vivos; heartbeat-audit.json missed_cycles=0 en obligatorios y opcionales. Auditoría debe discriminar: (A) causa raíz residual / regresión latente vs (B) deuda documental de fracturas históricas ya mitigadas — y cerrar los 5 PBI en un solo bug-fix.

Mandato: prohibido bypass raw (gh/git/curl) hasta cierre documentado. Criterio: causa raíz o laudo de no-regresión + validacion.md APTO + PBI en docs/todos/done/ + ignición start-sddia verificada (2/2 obligatorios + heartbeats frescos).

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

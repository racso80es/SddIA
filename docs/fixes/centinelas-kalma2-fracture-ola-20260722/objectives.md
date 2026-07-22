---
feature_name: centinelas-kalma2-fracture-ola-20260722
created: "2026-07-22"
process: bug-fix
branch_name: fix/centinelas-kalma2-fracture-ola-20260722
persist_ref: docs/fixes/centinelas-kalma2-fracture-ola-20260722
---

# Objetivos — centinelas-kalma2-fracture-ola-20260722

## Misión

Ola unificada de 5 PBIs System_Fracture_Detected:
1) event-sweeper / event-watcher / github-bridge-watcher / telegram-watcher — omisión Daemon_Heartbeat (argos/daemon-heartbeat-audit).
2) kalma2-bridge — mayeuta-llm/prótesis exit 1 (sse_chat_stream).
Mandato: un solo bug-fix; causa raíz; validación empírica obligatoria del comportamiento correcto de start-sddia.sh (ignición 2/2 obligatorios + opcionales + Kalma2 HTTP; heartbeats vivos; cleanup sin locks huérfanos).
PBIs: dd1aea4a9a29, 84eb0394cd44, a669741ed066, 522e3a40e3de, cbe0c30b3695.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.

## Estado

Ejecutado en runtime IDE. Ver `validacion.md` APTO.

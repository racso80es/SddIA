---
feature_name: centinelas-kalma2-fracture-ola-20260722
created: "2026-07-22"
process: bug-fix
base: main
branch: fix/centinelas-kalma2-fracture-ola-20260722
uuid: 52dab164-f4a3-4504-bc4e-be5d95c030b2
scope: start-sddia-vault-cleanup-heartbeat-gate
consolidated_pbis:
  - PBI-FIX-FRACTURE-dd1aea4a9a29
  - PBI-FIX-FRACTURE-84eb0394cd44
  - PBI-FIX-FRACTURE-a669741ed066
  - PBI-FIX-FRACTURE-522e3a40e3de
  - PBI-FIX-FRACTURE-cbe0c30b3695
---

# Spec — Ola fracturas centinelas + kalma2 (2026-07-22)

## Decisión

Un solo `bug-fix` consolida 5 PBIs. Keepalive de centinelas ya existe; la fractura operativa nace de **apagado sucio** + **bóveda no inyectada a Kalma2**.

## Causa raíz (empírica)

| Hecho | Evidencia |
|-------|-----------|
| Heartbeats sanos con ecosistema vivo | Tras `./start-sddia.sh`, `heartbeat-audit.json` → `missed_cycles=0` y `last_heartbeat` fresco |
| Locks huérfanos post-Ctrl+C/timeout | `pkill` sin `rm` de `.SddIA/daemons/status/*.lock` (PID DEAD residual) |
| `kalma2-bridge` sin bóveda | Centinelas cargan vault vía `_exec_daemon.sh`; `start-sddia` lanzaba el bridge **sin** `_sddia_load_vault` → `mayeuta-llm` sin `SDDIA_LLM_*_COMMAND` → `mayeuta-llm/prótesis exit 1` |
| Traza PBI kalma2 | Exactamente `mayeuta-llm/prótesis exit 1` (reproducida sin vault) |

## Cambios

1. `start-sddia.sh` v1.2: `_sddia_load_vault` antes de lanzar componentes.
2. Cleanup: tras `pkill`, eliminar locks de los cuatro centinelas.
3. Gate post-HTTP: confirmar heartbeats auditados de obligatorios (`missed_cycles < 3`).
4. `start-sddia.md` v1.2.0 alineado.
5. Archivar 5 PBI satélite bajo esta ola.

## CA

| ID | Criterio |
|----|----------|
| CA1 | `./start-sddia.sh` alcanza “Ecosistema S+ Grade operativo” con 2/2 obligatorios + Kalma2 HTTP |
| CA2 | Heartbeats obligatorios auditados frescos antes del banner operativo |
| CA3 | Tras SIGTERM/timeout: cero `.lock` en `status/` y cero PIDs de centinelas/bridge |
| CA4 | Bridge hereda `SDDIA_LLM_CLI_COMMAND` o `CHAT_COMMAND` desde bóveda; chat SSE no colapsa por “comando ausente” |
| CA5 | 5 PBIs en `docs/todos/done/` + `validacion.md` APTO `pbi_archived: true` |

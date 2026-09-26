---
feature_name: sddia-installer-v2-clean-deploy
created: "2026-09-26"
process: feature
purpose: Estabilización Filtro A — auditoría installer-deploy-aplicaciones 20260926
version_clarify: "1.0.0"
execution_id: "8ff78e98-6e4f-4859-9ca9-60ba718703ea"
pbi_ref: docs/todos/pending/[ARQUITECTURA] Installer determinista v2 — despliegue limpio y anti-fricción (deploy + teardown).md
document_id: PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO
pbi_uuid: "bb30e934-7f1f-44cb-a51e-21c28ccf426b"
pbi_version: "1.0.0"
---

# Clarificación — sddia-installer-v2-clean-deploy

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery + `SDDIA_LAB_ALLOW_DIRTY=1`. `execution_id` `8ff78e98-6e4f-4859-9ca9-60ba718703ea`. Rama `feat/sddia-installer-v2-clean-deploy`. Mayeuta…Argos: simulated; relevo IDE.

## Decisiones (Dédalo)

| ID | Laudo |
|----|-------|
| L-PRECOND | `SddIA/scripts/daemons/_precondition.sh` + `ExecCondition=` en plantilla `sddia-daemon@.service.template` (R-SYS-2). |
| L-REGISTRY | `{FORGE}/.SddIA/instances.json`; clave Cúmulo `instance.host_registry` (R-REG-1). |
| L-VERIFY | Proceso nativo nuevo `instance-health-verify` (handler Rust); no extender `system-vitality-probe` (alcance forja vs instancia AP). |
| L-FACADE | Fachada `./sddia-installer.sh`: post-motor `deploy` invoca verify + emite eventos; motor sin HTTP/curl/ss. |
| L-VAULT | Layout `root.dev.env` + `instance.SddIA.dev.env`; lista negra R-VAULT-2 aplicada al root implícito. |
| L-NORM | `sddia-installer-contract` 1.1.0 vía `entity-manager` update. |
| L-EVENTS | `Instance_Deployed` / `Instance_Torn_Down` vía `entity-manager` create; suscripción Cúmulo/IOTA en `event-domain-subscriptions.json`. |
| L-CI | `validacion.md` APTO solo con `run_id` verde; `accept-pr` post-CI. |

## Fuera

Redeploy live Aplicaciones (§3.9 PBI). Archivar Paciente 0. Rollback auto tras NO-APTO.

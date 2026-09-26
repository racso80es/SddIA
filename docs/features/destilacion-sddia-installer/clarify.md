---
feature_name: destilacion-sddia-installer
created: "2026-09-26"
process: feature
purpose: Estabilización Filtro A del PBI destilación v1.2.0
version_clarify: "1.0.0"
execution_id: "4b9f0aaa-67f9-4213-8031-6dd9fc98dbcb"
pbi_ref: docs/todos/pending/PBI-ARQUITECTURA-DESTILACION-DEPLOY-DETERMINISTA.md
document_id: PBI-ARQUITECTURA-DESTILACION-DEPLOY-DETERMINISTA
pbi_uuid: "d239cb31-a937-448c-941e-3808a3a28874"
pbi_version: "1.2.0"
---

# Clarificación — destilacion-sddia-installer

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `4b9f0aaa-67f9-4213-8031-6dd9fc98dbcb`. Rama `feat/destilacion-sddia-installer`. Motor bilateral ya mergeado desde `feat/sddia-deterministic-teardown`.

## Decisiones

| ID | Laudo |
|----|-------|
| L-NORM-SSOT | Contrato forjado vía `entity-manager` → `norm-creator` en `SddIA/library/norms/sddia-installer-contract.md` (SSOT norm-creator). PBI §5 CA-NORM cita `SddIA/norms/` como familia knowledge Core; la destilación operativa vive en library (precedente `jurisdiccion-deuda-tecnica-todos`). |
| L-BILATERAL | Capítulos simétricos Deploy (I-DEP-*) y Teardown (I-TEAR-*); tabla códigos salida 1/2/3; matriz cuatro jurisdicciones. |
| L-CODE | Sin cambios al motor salvo gaps; smoke existente `test-sddia-installer.sh` sigue gate. |
| L-DEUDA | Opcional `installer_contract_ref` en frontmatter deploy/teardown Paciente 0. No archivar deudas. |
| L-CI | `validacion.md` sin `global: APTO` hasta `run_id` verde. `accept-pr` solo post-CI. |

## Fuera

`paciente0-deploy` / `paciente0-undeploy`. Segundo binario. Poll HTTP en motor. Archivar prompts deuda.

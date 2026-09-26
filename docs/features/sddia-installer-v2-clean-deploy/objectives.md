---
feature_name: sddia-installer-v2-clean-deploy
created: "2026-09-26"
process: feature
branch_name: feat/sddia-installer-v2-clean-deploy
persist_ref: docs/features/sddia-installer-v2-clean-deploy
pbi_ref: docs/todos/pending/[ARQUITECTURA] Installer determinista v2 — despliegue limpio y anti-fricción (deploy + teardown).md
execution_id: "8ff78e98-6e4f-4859-9ca9-60ba718703ea"
document_id: PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO
pbi_uuid: "bb30e934-7f1f-44cb-a51e-21c28ccf426b"
pbi_version: "1.0.0"
---

# Objetivos — sddia-installer-v2-clean-deploy

## Misión

Eliminar las nueve fricciones `DT-INST-*` del installer: instancia aislada, WUI en puerto propio, unidades solo con prerequisitos, bóveda compuesta, perfil de dominio declarado, verify post-deploy con acta, teardown simétrico incl. plantillas cuando procede.

## Alcance

Motor ciego + fachada verify/eventos; registro host; contrato 1.1.0; smoke/CI. Fuera: redeploy live Aplicaciones (laudo VB).

## Ley aplicada

- PBI Filtro A + CA-* §4.
- `features-documentation-pattern` v1.2.x: un PR; CI verde antes de APTO y `accept-pr`.
- DA-2: genoma vía `entity-manager`; Core subscriptions/Cúmulo en ciclo feature documentado.

## Criterios

Tabla CA-VAULT-COMPOSE … CA-REGRESION del PBI §4 (16 ítems). `CA-REDEPLOY-REAL` post-merge fuera de gate PR.

---
feature_name: destilacion-sddia-installer
created: "2026-09-26"
process: feature
branch_name: feat/destilacion-sddia-installer
persist_ref: docs/features/destilacion-sddia-installer
pbi_ref: docs/todos/pending/PBI-ARQUITECTURA-DESTILACION-DEPLOY-DETERMINISTA.md
execution_id: "4b9f0aaa-67f9-4213-8031-6dd9fc98dbcb"
document_id: PBI-ARQUITECTURA-DESTILACION-DEPLOY-DETERMINISTA
pbi_uuid: "d239cb31-a937-448c-941e-3808a3a28874"
pbi_version: "1.2.0"
---

# Objetivos — destilacion-sddia-installer

## Misión

Cristalizar en norma atómica DA-2 los invariantes medibles del orquestador físico `sddia-installer.sh` (deploy + teardown), sin reimplementar el motor ni mezclar gates HTTP de ola Paciente 0.

## Alcance

1. Norma `sddia-installer-contract` (I-DEP-*, I-TEAR-*, simetría ROOT/ESC, perfiles).
2. Punteros opcionales en deuda Paciente 0 (`installer_contract_ref`).
3. Evolution correlacionando uuid norma.

## Fuera de gate

Deploy/teardown live en rutas productivas. Procesos `paciente0-*`. Mutar `instance-creator.rs`. Atajos host en git.

## Criterios

| ID | Criterio |
|----|----------|
| CA-NORM | Norma forjada DA-2 con uuid/SemVer/hash_signature |
| CA-BILATERAL | Deploy ↔ teardown documentados |
| CA-NO-ENTROPÍA | Sin paths alucinados ni matriz tridimensional |
| CA-PERFIL | full-node vs consumer explícito |
| CA-CEGUERA | Sin HTTP/LLM en CLI |
| CA-TEAR-DESTIL | I-TEAR-CEGUERA sin curl |
| CA-DEUDA | Deudas Paciente 0 en pending/ |
| CA-CI | run_id verde antes de APTO y accept-pr |

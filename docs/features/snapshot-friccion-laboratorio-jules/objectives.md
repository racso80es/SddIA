---
feature_name: snapshot-friccion-laboratorio-jules
process: feature
created: "2026-06-11"
branch_name: feat/snapshot-friccion-laboratorio-jules
persist_ref: docs/features/snapshot-friccion-laboratorio-jules
related_todo: docs/todos/pending/Snapshot_Friccion_Laboratorio_Jules.md
---

# Objetivos — Cierre Snapshot Fricción Laboratorio Jules

## Meta

Completar la resolución táctica pendiente del PBI **Snapshot de Fricción** (incidente Jules / Raw Kernel): cerrar los gaps que impidieron la ejecución ordenada del laboratorio y neutralizar la paradoja Raw Kernel ↔ ciclo documental, sin reimplementar entregas ya certificadas en `main`.

## Contexto heredado (no reimplementar)

| Entrega | Estado | Evidencia |
|---------|--------|-----------|
| PyYAML + bootstrap lab | ✅ | `requirements.txt`, `sddia-run.sh` |
| PoC WASI | ✅ | `docs/features/wasi-poc-ignition/` (PR #74) |
| Migración Rust/WASI | ✅ | `docs/features/migracion-rust-wasi/` (PR #85) |
| Aduana física Husky + `--blocking` | ✅ | PR #73, `Local_QA_Requested` |
| Blindaje IA obrera (norma + creators) | ✅ | `docs/features/ia-obrera-blindaje/` |

## Objetivos medibles

| ID | Objetivo | Criterio de aceptación |
|----|----------|------------------------|
| **O1** | **Git failsoft offline** | Operaciones `fetch`/`pull`/`push` en `git-manager` degradan sin colapsar el orquestador cuando faltan credenciales de red o el remoto no responde; respuesta JSON con `success: false`, código de salida controlado y señal explícita (`offline_mode` / `network_unavailable`) |
| **O2** | **Acoplamiento Raw Kernel → feature** | Norma actualizada (`external-ai-constraints.md` y/o `interaction-triggers.json`) obliga a verificar o instanciar topología `feature` antes de mutar genoma cuando el prefijo RAW KERNEL está activo |
| **O3** | **Skill Transpilador de Intenciones** | Nueva skill en `SddIA/skills/` vía `skill-creator` / `entity-manager`: transcribe instrucción humana a payload estructurado, mapea ficheros destino, inyecta contexto SddIA y exige `persist_ref` válido antes de delegar a Tekton |
| **O4** | **Cierre documental PBI** | `Snapshot_Friccion_Laboratorio_Jules.md` en `docs/todos/done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, rama coherente |
| **O5** | **Evolución registrada** | Entrada en `SddIA/evolution/` vinculando UUIDs de entidades tocadas |

## No objetivos

- Reabrir migración WASI, PoC ignition o CI wasmtime.
- Reimplementar hooks Husky / `pre-commit` / Ola B.
- Eliminar `requirements.txt` mientras el orquestador lab dependa de PyYAML.
- Sustituir `execute-process.py` por Rust en esta entrega.

## Ley aplicada

- Manifiesto: `docs/todos/pending/Snapshot_Friccion_Laboratorio_Jules.md`
- Proceso: `feature` v1.3.0
- Patrón documental: `features-documentation-pattern` v1.2.1
- Normas: `external-ai-constraints.md`, `capsule-json-io.md`, `skill-io-git-manager-frozen.md`
- Precedencia blindaje: `docs/features/ia-obrera-blindaje/`

## Estado

| Fase feature | Estado |
|--------------|--------|
| Inicialización | ✅ rama `feat/snapshot-friccion-laboratorio-jules` |
| Clarificación | ✅ `clarify.md` |
| Especificación | ✅ `spec.md` (borrador inicial) |
| Planificación | ✅ `plan.md` |
| Implementación | ✅ O1–O3 |
| Validación | ✅ `validacion.md` APTO |

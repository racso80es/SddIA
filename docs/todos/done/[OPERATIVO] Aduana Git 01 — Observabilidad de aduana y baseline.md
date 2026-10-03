---
document_id: PBI-MERGE-THERMO-01-OBSERVABILITY
uuid: "8652b541-cf43-4d49-9cad-cb36b3089456"
title: "[OPERATIVO] Aduana Git 01 — Observabilidad de aduana y baseline"
format: markdown
version: "1.0.0"
status: done
persist_ref: docs/features/merge-thermo-01-observability
closed: "2026-10-03"
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 1
hu_order_total: 6
historia_ref: "docs/todos/historias/[OPERATIVO] Optimización Termodinámica de Aduana Git: Merge de Alta Eficiencia.md"
historia_document_id: HU-MERGE-THERMODYNAMICS
cola_ejecucion: docs/todos/pending/
unblocks:
  - PBI-MERGE-THERMO-02-NET-PRUNE
  - PBI-MERGE-THERMO-06-E2E-MEASURE
baseline_decisiones:
  - "HU §2.4 y §2.5-C. Sin baseline, AC-7 no es verificable."
  - "La purga de workspaces vacíos es subcomando idempotente; prohibido rm -rf."
  - "El baseline se captura sobre la cadena actual, antes de los PBI 02–04."
---

# Observabilidad de aduana y baseline

HU `HU-MERGE-THERMODYNAMICS`, orden **01/06**. Entrega la medición; no cambia el veredicto de ningún hook.

## 0. Filtro A

| Afirmación a evitar | Corrección |
|---|---|
| Los 817 directorios de `pull-request-review` «perdieron» el informe | 812 están vacíos porque `bootstrap_workspace` crea el directorio y el `execution_report` solo viaja en stdout. `thermodynamic::run` recibe `duration_ms` total, no por fase. |
| Persistir el informe dentro del peaje termodinámico | El peaje sella telemetría. El informe es artefacto de workspace. Puntos distintos. |

## 1. Intención

Atribuir el coste de la aduana a fase y a hook, y dejar 10 muestras de la cadena actual como baseline de AC-7.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | `executor.rs` cronometra cada fase (`Instant`) e inyecta `elapsed_ms` en la entrada de `phase_reports`. |
| R-2 | Al cerrar `run`, persiste `execution_report.json` en el workspace ya materializado: `process_name`, `execution_id`, `correlation_id`, `status_code`, `duration_ms`, `phases[]`. Escritura best-effort: un error de E/S no altera `status_code`. |
| R-3 | Proceso en `thermodynamic::is_exempt` no persiste informe. |
| R-4 | `sddia-qa workspace-prune --empty --json` borra solo directorios vacíos bajo `.SddIA/workspaces/*/`, es idempotente y emite `pruned_count`. |
| R-5 | `pre_push_gate.sh` y `post_merge_gate.sh` appenden una línea JSONL en `{proofs}/hook-timings/` (`hook`, `branch`, `total_ms`, `invoked_processes[]`). `{proofs}` vía el mismo criterio que `eda_instance.proofs`, sin ruta cableada nueva en `cumulo.paths.json`. `delta_class` y `attestation_hit` quedan reservados (null) hasta los PBI 03 y 04. |
| R-6 | `validacion.md` incluye ≥10 muestras de `pre-push` sobre la cadena actual (rama ya presentada y, si el laboratorio lo permite, rama nueva). |

## 3. Plan

1. Cronómetro por fase y persistencia del informe. Tests de motor.
2. Subcomando `workspace-prune`. Una ejecución documentada en `validacion.md`.
3. JSONL de hooks. Diez muestras.
4. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-11 | Ejecución no exenta deja `execution_report.json` con `elapsed_ms` por fase. `workspace-prune --empty` deja 0 directorios vacíos; segunda pasada `pruned_count: 0`. | Test de integración + ejecución documentada |
| AC-OBS-1 | Cada `pre-push` y `post-merge` añade una línea JSONL con `total_ms`. | Test shell con repo temporal |
| AC-7a | Tabla de ≥10 muestras de baseline en `validacion.md`, previa a los PBI 02–04. | Revisión del diff |

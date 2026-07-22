---
feature_name: kalma2-pasarela-asincrona-eda
created: "2026-07-22"
process: feature
phases: [T0, T1, T2, T3, T4]
document_id: PBI-044-KALMA2-PASARELA-ASINCRONA-EDA
branch_name: feat/kalma2-pasarela-asincrona-eda
persist_ref: docs/features/kalma2-pasarela-asincrona-eda
---

# Plan — kalma2-pasarela-asincrona-eda

Blueprint de forja Tekton (H1+H2 = Done mínimo PBI-044). H3/R6 fuera.

## Fases

### T0 — Documentación Dedalo
- [x] Consumir `objectives.md` / `clarify.md` (D0–D8, Q1–Q5)
- [x] `spec.md` laudos L1–L8 + contratos 202/UUID
- [x] este `plan.md`

### T1 — H1 Desacople HTTP (R1–R2)
- [x] Extraer helper `accept_execute(repo, prompt, process?) -> Result<AcceptedAck, SyncError>`
  - UUID v4 → `correlation_id`
  - `Command::spawn` + reaper (`wait` en hilo); `Stdio::null`
  - **sin** `run_orchestrator_inputs` / join en camino execute
- [x] `handle_execute` → 202 + envelope §3.1
- [x] `handle_interact` (`mode=execute`) → mismo camino
- [x] Chat / otros modos: intactos (síncronos)
- [x] Audit estático: bridge sin write EDA

### T2 — Plumb correlación genoma (R3/Q3 / L4)
- [x] `build_kalma2_process_event` / `emit_process_event`: honrar `process_inputs.correlation_id` si UUID válido
- [x] Test unitario: cid fijo → `event_id` / `correlation_id` idénticos
- [x] Compat: sin cid → `Uuid::new_v4()` como hoy

### T3 — H2 Contrato UI + regresión
- [x] `interfaces/kalma2/app.js`: rama `accepted` / HTTP 202 → `pollStatus`
- [x] Fallback legado `emitted`+`event_id` opcional
- [x] Smoke timing p99 &lt; 50 ms (lab) — p99 RTT 4.5 ms
- [x] Smoke correlación acuse ↔ dominio / status — cid 6178f1d1-…
- [x] Regresión suscripciones vs main = 0 (AC-R3)

### T4 — Cierre documental
- [x] `implementation.md` / `execution.md`
- [x] `validacion.md` APTO (AC-R1..AC-R4; AC-R5/R6 fuera/defer)
- [x] PBI → `docs/todos/done/` + `pbi_archived: true` en la rama
- [ ] Handoff Argos → `delivery-close-cycle`

## Orden de forja

```text
T1 (bridge spawn/202) → T2 (handler UUID) → T3 (UI + smokes) → T4 (docs/cierre)
```

T1 y T2 pueden ir en el mismo commit si los tests de correlación lo exigen; no mergear T1 sin T2 si el poll depende de cid preasignado.

## Delegación / RBAC (ejecutor Tekton)

| Fase | Capacidad | Notas |
|------|-----------|-------|
| T1–T3 | `filesystem-ops` + código bajo `SddIA/interfaces/kalma2-bridge`, `SddIA/engine/execute-process`, `interfaces/kalma2` | Sin nuevas cápsulas |
| Git | `skill:git-manager` vía ecosistema | Prohibido bypass raw destructivo |
| Genoma indexado | — | Sin `entity-manager` salvo se toque `.md` indexado (no previsto) |
| KM / `docs/todos/` semillas | Solo Cumulo / `Kaizen_Alert_Required` | Tekton no siembra TODOs |

## Criterios de salida por fase

| Fase | Done local |
|------|------------|
| T1 | POST execute responde 202+accepted+cid sin esperar hijo |
| T2 | Emisión usa cid del input; test verde |
| T3 | UI poll OK; smokes AC-R1/R2/R4; regresión AC-R3 |
| T4 | Cascada documental + PBI archivado en rama |

## Riesgos operativos

| Riesgo | Mitigación |
|--------|------------|
| Medición timing ruidosa en lab cargado | N≥10; excluir cold-start binario; documentar método en `execution.md` |
| Reaper no arranca | Test: spawn + process reap; no `mem::forget` del Child sin wait |
| Drift UI solo `emitted` | Checklist T3 antes de Argos |

## Explicitamente no planificado

H3 Telegram · cápsula ingest ciega · mutación `event-domain-subscriptions.json` · waiting-for-shell IDE · DI residual · F3 PPR#136.

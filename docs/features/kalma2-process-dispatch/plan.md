---
feature_name: kalma2-process-dispatch
created: "2026-07-20"
process: feature
phases: [T0, T1, T2, T3, T4]
---

# Plan — kalma2-process-dispatch

## Fases

### T0 — Documentación Dedalo
- [x] `clarify.md` laudos Q1–Q3
- [x] `spec.md` L1–L5
- [x] este `plan.md`

### T1 — Handler nativo TQM (B′)
- [x] `handlers/task_queue_manager.rs`: triaje paquete Kalma2 → `invoke_process_full` hijo
- [x] Registro en `handlers/mod.rs` + `engine/mod.rs` (alias `automatic_task` vía canonical)
- [x] Política L2: con `correlation_id`, setear en el subproceso hijo `SDDIA_LAB_SKIP_PBI_ARCHIVE` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE` si no hay `SDDIA_TQM_FULL_CYCLE=1`
- [x] Modo legado: `tasks_path` default → residual simulado
- [x] Tests unitarios slug/pbi/build inputs

### T2 — Matiz A′ `pbi_ref`
- [x] Reescribir `extract_pbi_ref` en `handlers/kalma2.rs` (anclas + `.md`)
- [x] Test: path con espacios y em-dash

### T3 — Contrato documental TQM (opcional si entity-manager)
- [ ] Diferido: handler early-return suficiente; documentar inputs YAML vía entity-manager en follow-up si se exige aduana genoma

### T4 — Verificación
- [x] `cargo build -p execute-process` + tests
- [x] Smoke TQM a7725b42-like + `SDDIA_LAB_SKIP_GIT=1`
- [x] Smoke `kalma2-interact` path espaciado → `pbi_ref`
- [x] `implementation.md` / `execution.md` / `validacion.md`

## Orden de forja Tekton

1. T1 handler  
2. T2 extract  
3. T4 smokes  
4. T3 solo si hace falta aduana documental genoma  

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Despacho hijo muta git de la rama activa | Smokes con `SDDIA_LAB_SKIP_GIT=1`; producción = comportamiento intencional del ciclo |
| Delivery hijo sin agentes → DLT | L2 skips con `correlation_id` |
| Recursión `process=task-queue-manager` | Modo legado, no re-despachar hijo TQM |

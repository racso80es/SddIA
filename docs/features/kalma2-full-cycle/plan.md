---
feature_name: kalma2-full-cycle
created: "2026-07-20"
process: feature
---

# Plan — kalma2-full-cycle

## Fases

### Fase A — Semántica cycle_phase (forja inmediata)

- [x] `derive_cycle_phase(phase_reports)` en `thermodynamic.rs` + campo PEC
- [x] `project_status` en `kalma2-bridge` consume `cycle_phase`
- [x] UI + CSS terminales `initialized` / `awaiting_agents`
- [x] Tests nativos
- [x] `cargo build -p execute-process -p kalma2-bridge` + test filters

### Fase B — Runtime agentes (contrato ahora; forja incremental)

- [x] Documentar env / evento handoff en `spec.md` (hecho en spec §3)
- [x] Hook nativo `agent_runtime` + tests mock CLI
- [ ] Wrapper instancia Cursor Agent / SDK (comando en bóveda)
- [ ] Futuro opcional: evento `Process_Agent_Handoff_Requested` (B2)

### Fase C — pbi_body (diferible)

- [ ] Lectura FS `pbi_ref` en despacho TQM / workspace-init
- [ ] Tests path con espacios (reusar A′)

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Compat status antiguos | L3: sin `cycle_phase` → completed |
| Poll infinito en `initialized` | Tratar como terminal en UI |
| Scope creep B/C | APTO parcial de A + deudas explícitas B/C |

## Orden de commits sugerido

1. docs cascada (clarify/objectives/spec/plan)
2. thermodynamic + tests
3. bridge + UI + tests
4. implementation/execution/validacion

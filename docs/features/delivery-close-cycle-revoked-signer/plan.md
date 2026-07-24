---
feature_name: delivery-close-cycle-revoked-signer
created: "2026-07-24"
process: feature
branch_name: feat/delivery-close-cycle-revoked-signer
persist_ref: docs/features/delivery-close-cycle-revoked-signer
document_id: PBI-PPR-136-DCC-REVOKED-SIGNER
execution_id: 00b9e53d-d231-45f5-9685-4d2b86b7ab63
phase: blueprint
agents: dedalo
phases: "E2-code → E1-instance → docs → verify"
---

# Plan — delivery-close-cycle-revoked-signer

## Fases

| # | Fase | Acciones | Done |
|---|------|----------|------|
| 1 | E2 código | Parche `emit_pr_presented` + bump `emit-pr-presented-event.md` (manual tras L-EM-ACTION-UPDATE) | [x] |
| 2 | E2 prueba | Smoke action → `signer_identity_rbac=Vertice_Biologico_Relay` | [x] |
| 3 | E1 instancia | Remove clave revoked; status Radamanto `healthy` | [x] |
| 4 | Docs | `implementation.md`, `execution.md`, evolution, laudo | [x] |
| 5 | Argos | `validacion.md` APTO + checks; preparar cierre documental | [x] |

## Orden de mutación

1. **Motor (no genoma indexado):** `actions.rs` — default signer.
2. **Genoma action:** `entity-manager` update `emit-pr-presented-event` (inputs + cápsula JSON + version patch).
3. **Instancia:** `.SddIA/cerbero/revoked_entities.json` + stats Radamanto (no van al PR; evidencia en `execution.md`).
4. **Evolution:** registro UUID ciclo / touchpoints.

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Re-revocación por nuevo cluster de fails | Aceptable; rate actual > umbral; no inventar exención |
| Drift genoma action vs Rust | Entity-manager + rebuild `execute-process` en misma sesión |
| Instancia no en git | AC-E1 se prueba local; aduana remota depende de instancia CI/host |

## Handoff Tekton

Ejecutar plan §1–4; no tocar `feature`/`bug-fix` en revoked.

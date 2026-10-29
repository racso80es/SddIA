---
feature_name: events-contract-redb-ssot
created: "2026-10-29"
process: bug-fix
phase: Verificación
branch: fix/events-contract-redb-ssot
global: NO_APTO
tracker_ref: AUT-45
pbi_archived: false
pr_url: https://github.com/racso80es/SddIA/pull/411
execution_id: "a629489e-2c6c-41fb-89db-26484a96fbd9"
approval_status: integrity_failure
checks:
  AC-CONTRACT: NO_EVIDENCIA_CAPSULA
  AC-STORE-IDEM: NO_EVIDENCIA_CAPSULA
  capsules-qa: BLOCKED
  git-evidence: BLOCKED
git_changes:
  - SddIA/events/events-contract.md
  - SddIA/sddia-event-store/src/gate.rs
  - SddIA/sddia-event-store/src/lib.rs
  - SddIA/engine/execute-process/src/engine/eda_bus_topology.rs
  - SddIA/engine/execute-process/tests/event_store_writers.rs
  - SddIA/evolution/f8a3c2d1-9e4b-4a7f-8c6d-5e4f3a2b1c0d.md
  - docs/fixes/events-contract-redb-ssot/
---

# Validación — AUT-45 (Argos)

## approval_status: integrity_failure

Inyección kalma2 con `process: ?` / `phase: ?` / `persist_ref` vacío. Contrato inferido desde cascada `docs/fixes/events-contract-redb-ssot` + `_agent_handoff.md` (`bug-fix` / Verificación / `a629489e-…` / AUT-45 / `fix/events-contract-redb-ssot`).

Sesión actual (2026-10-29T18:55Z): peaje Shell Rejected ×3 (incl. `request_smart_mode_approval`); GetDynamicTools sin namespace sddia/kalma; sin evidencia PPR materializada con stdout de cápsula.

## Evidencia de cápsulas

| Cápsula | Resultado |
|---------|-----------|
| `./sddia-run.sh --tool git-manager` | Shell Rejected (×3) — sin stdout |
| `cargo test -p execute-process … terminal_witness_exists_redb` | No invocado — peaje Shell ausente |
| MCP sddia / peaje execute-process | GetDynamicTools: sin namespace sddia/kalma |

`execution.md` afirma test OK; **L-ARGOS-SYNTHESIS**: no sustituye stdout de cápsula. `global: NO_APTO` mantenido.

## acceptance_criteria (spec.md) — estado

1. Contrato §2/§4 redb + `subscriber_delivery` — **sin re-verificación cápsula**.
2. `delivery_terminal_in_store` / `terminal_witness_exists` — **sin stdout test**.

## correction_blueprint_md

```yaml
name: reentrada-verificacion-aut-45
intent: Reinyectar Argos con peaje operativo y campos explícitos; no inventar APTO.
delegates_to:
  - tool:git-manager
  - action:execute-process
phases:
  - name: Evidencia-git
    intent: status/diff vía skill:git-manager (JSON stdin sddia-run)
    delegates_to:
      - tool:git-manager
  - name: Regresion-store
    intent: cargo test terminal_witness_exists_redb_without_fs_witness --test-threads=1
    delegates_to:
      - action:execute-process
  - name: Sello-validacion
    intent: Argos reescribe validacion.md solo tras stdout físico
    delegates_to:
      - agent:argos
```

Desbloqueo: habilitar Shell/MCP; reinyectar con `process: bug-fix`, `phase: Verificación`, `persist_ref: docs/fixes/events-contract-redb-ssot`, `execution_id: a629489e-2c6c-41fb-89db-26484a96fbd9`.

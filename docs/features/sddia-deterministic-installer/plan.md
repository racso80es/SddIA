---
feature_name: sddia-deterministic-installer
created: "2026-09-25"
process: feature
phases:
  - L0-design-commit
  - L1-bundle-full-node
  - L2-installer-engine
  - L3-wrapper-smoke
  - L4-docs-dcc-ci-accept
branch_name: feat/sddia-deterministic-installer
persist_ref: docs/features/sddia-deterministic-installer
pbi_ref: docs/todos/pending/PBI-ARQUITECTURA-DEPLOY-DETERMINISTA.md
document_id: PBI-ARQUITECTURA-DEPLOY-DETERMINISTA
uuid: "c154bea0-c4c1-457e-b070-f7dfd3bc5f1b"
execution_id: "7b22f932-c162-4104-b38d-b1c9c6068414"
---

# Plan — sddia-deterministic-installer

Corte Diseño: **clarify + objectives + spec + plan + commit**. Ejecución L1–L4 después.

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `7b22f932-c162-4104-b38d-b1c9c6068414`.

## L0 — Diseño (esta parada)

Artefactos bajo `persist_ref` + UUID PBI materializado. Commit de planificación **antes** de mutar scripts.

## L1 — Perfil `full-node`

`SddIA/scripts/build-release-bundle.sh`:

1. Help: `consumer|engineering|full-node`.
2. Tras escáner códice, si `PROFILE` ∈ `full-node|full`: descubrir Cargo.toml y sembrar `CAPSULE_SET`.
3. Filtro C intacto (solo consumer).
4. No romper smoke `test-build-release-bundle-filtro-c.sh`.

## L2 — Motor installer

`SddIA/scripts/sddia-installer.sh` según spec §1–5. Funciones: `resolve_root`, `is_live`, `validate_host`, `do_deploy`, `do_teardown`, `plan_json`. `--dry-run` primero (testeable).

## L3 — Wrapper + smoke

`./sddia-installer.sh` wrapper. `SddIA/scripts/qa/test-sddia-installer.sh` (casos spec §6). Ejecutar smoke local antes de DCC.

## L4 — Docs + PR + CI + accept

`implementation.md` / `execution.md` / `validacion.md` (`CA-CI: PENDIENTE-CI`). Archivar PBI en `done/` al sellar. DCC abre PR. `global: APTO` solo con `run_id` verde. Entonces `accept-pr`.

## Fuera

`instance-creator.rs`. Norma distribution-protocol. Deploy live ruta PBI.

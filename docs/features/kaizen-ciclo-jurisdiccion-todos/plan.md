---
feature_name: kaizen-ciclo-jurisdiccion-todos
created: "2026-08-29"
process: feature
branch_name: feat/kaizen-ciclo-jurisdiccion-todos
persist_ref: docs/features/kaizen-ciclo-jurisdiccion-todos
pbi_ref: docs/todos/pending/[KAIZEN] Ciclo jurisdicción todos — norm-creator parcial, huérfanos EDA y colapso DCC sin fractura.md
document_id: PBI-KAIZEN-CICLO-JURISDICCION-TODOS
uuid: 74c4e6e9-baef-4a08-aa44-4adb0ffe1dfe
execution_id: "1550128b-c2ef-4c4d-8cbb-181a15a66940"
phases:
  - l0-design
  - l1-norm-forge
  - l2-reforge-todos-jurisdiction
  - l3-hash-pending-forge
  - l3b-doc-backfill
  - l4-dcc-fracture
  - l5-gate-evolution-coverage
  - l6-gitignore-tmp
  - l7-obediencia-colapso-mudo
  - l8-evolution
  - l9-closure
---

# Plan — kaizen-ciclo-jurisdiccion-todos

Orden: L1 → L2 → L3 → L3b → L4 → L5 → L6 → L7 → L8 → L9. Este commit sella Diseño (`clarify.md` + `objectives.md` + `spec.md` + `plan.md`). Ejecución Tekton **después** de esta parada.

## Fase L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id`: `1550128b-c2ef-4c4d-8cbb-181a15a66940`.

## Fase L1 — Forge + test CA1

Parche `factory.rs` `run_norm_forge` y seed en `entity_manager.rs` (`tactical_norm_hard_constraints`, default `norms_contract_version` 1.1.0). Test unitario en crate `execute-process`.

```text
cd SddIA && cargo test -p execute-process run_norm_forge
```

## Fase L2 — Re-forja norma (CA2)

```text
./sddia-run.sh --process entity-manager --inputs-file .tmp/entity-manager-<uuid>.json
```

Semilla según `spec.md` Hito 2. Prefijo RAW. Prohibido `Write` a `SddIA/library/norms/`.

## Fase L3 — Hash real (CA3)

Dos updates EM (`tool` + `action`). Luego `sddia-qa audit-eda-coverage --scan --json`. No reescribir alcance kalma2.

## Fase L3b — Docs excepción (CA3b)

EM `update` process `delivery-close-cycle` + update norma `features-documentation-pattern` (versión bump). Texto = predicado Rust literal.

## Fase L4 — Fractura DCC (CA4)

Helper de emisión + idempotencia. Test con tempdir. No emitir en `success`/`warn`.

## Fase L5 — Gate evolution (CA5)

Exención `SddIA/core/eda-coverage.json` en `sddia-evolution-register`. Test: solo cobertura → OK; cobertura + material huérfano → UNREGISTERED del huérfano.

## Fase L6 — `.gitignore` (CA6)

`**/.tmp/`. Evidencia `git check-ignore -v`.

## Fase L7 — `obediencia-procesos` (CA7)

Parche motor bajo DA-4. Versión 1.2. Cláusula colapso mudo.

## Fase L8 — Evolution

Alta registro canónico. `relacionado`: uuid PBI `74c4e6e9-baef-4a08-aa44-4adb0ffe1dfe` + paths tocados (salvo `eda-coverage.json` si L5 ya lo exime). `gate-evolution --range` antes de push si el diff toca `directories.evolution`.

## Fase L9 — Cierre (fuera de esta parada)

`implementation.md` + `execution.md` → Argos `validacion.md` → PBI a `done/` → `delivery-close-cycle`.

## Touchpoints

| Path | Cambio | Vía |
|------|--------|-----|
| `factory.rs` / `entity_manager.rs` | Forge + seed | IDE (motor, no genoma md) |
| `SddIA/library/norms/todos-jurisdiction.md` | v1.1.0 | entity-manager |
| `SddIA/tools/github-raw-fetcher.md` | hash real | entity-manager |
| `SddIA/actions/download-remote-asset.md` | hash real | entity-manager |
| `delivery-close-cycle.md` | CA3b | entity-manager |
| `features-documentation-pattern.md` | CA3b | entity-manager |
| `delivery_close.rs` / `phase_capsules.rs` | CA4 | IDE |
| `sddia-evolution-register` | CA5 | IDE (cápsula; no editar `.md` de skill) |
| `.gitignore` | CA6 | IDE |
| `SddIA/norms/obediencia-procesos.md` | CA7 | DA-4 + evolution |
| `SddIA/evolution/{uuid}.md` | Hito | cápsula register |

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Re-forjar con forge roto | L1 bloquea L2 |
| Update tool/action altera contrato | Semilla mínima; solo hash/version |
| Fractura fan-out | Idempotencia `friction_id`+`process_name` |
| Exención evolution demasiado ancha | Solo el path SSOT de cobertura |
| EM update de process/norma de dominio | Verificar topología feature (DA-4) antes de RAW |

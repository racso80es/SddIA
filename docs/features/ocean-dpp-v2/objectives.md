---
feature_name: ocean-dpp-v2
created: "2026-08-07"
updated: "2026-08-10"
process: feature
branch_name: feat/ocean-dpp-v2-6416012709005756507
persist_ref: docs/features/ocean-dpp-v2
document_id: PBI-OCEAN-DPP-V2
pr_url: https://github.com/racso80es/SddIA/pull/160
---

# Objetivos — ocean-dpp-v2

## Misión

Integrar Ocean DPP V2 (Merkle Batching + Pasaporte Digital de Entidad) en el bus EDA / IOTA Rebased, con separación estricta entre **chunk físico** (I/O) y **batch semántico** (criptografía / ancla única).

## Alcance

- Cápsula `iota-immutable-publisher`: payload string | array → Merkle root + proofs.
- Orquestación `route-domain-event` / `route_domain_batch`: ancla semántica en lote.
- Persistencia `.SddIA/proofs/<uuid>.json` vía SSOT Cúmulo.
- Norma `entidad-digital-passport.md` (DEP).
- Validador `SddIA/scripts/qa/validate-merkle-proof.py`.

## Fuera de alcance (esta iteración)

- ZKP Groth16 / circuitos Circom (PBI §1.2).
- Tokenización completa DEP en cada activo (solo norma + schema).

## Ley aplicada

- Git vía flujo PR único; cierre documental en rama.
- Genoma indexado: sin forja manual de entidades; correcciones de motor/cápsula bajo feature activa + laudo humano.
- SSOT rutas: `SddIA/core/cumulo.paths.json`.

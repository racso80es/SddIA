---
feature_name: ocean-dpp-v2
created: "2026-08-10"
updated: "2026-08-10"
process: feature
phase: Verificación post-corrección
agent: argos
branch: feat/ocean-dpp-v2-6416012709005756507
persist_ref: docs/features/ocean-dpp-v2
document_id: PBI-OCEAN-DPP-V2
pr_url: https://github.com/racso80es/SddIA/pull/160
global: CONDICIONAL
pbi_archived: false
verdict: CONDICIONAL_APTO
---

# Validación — ocean-dpp-v2 (post-corrección)

## Veredicto

**CONDICIONAL_APTO** — bloqueantes H1–H5 del laudo inicial corregidos en código y documentados. APTO global pendiente de: CI verde en commit de corrección + confirmación humana de merge.

## Criterios

| AC | Resultado | Evidencia |
|----|-----------|-----------|
| H1 digest real | APTO | Pre-lote sella digest; `batch_mode` no emite `batched-digest` |
| H2 idempotencia | APTO | `is_valid_iota_anchor` + `merkle_anchored` |
| H3 sweeper | APTO | `try_sweep_event` restaurado; sin route desde sweep |
| H4 docs feature | APTO | Topología `docs/features/ocean-dpp-v2/` + evolution |
| H5 SSOT proofs | APTO | `eda_instance.proofs` + `resolve_eda_proofs_dir` |
| Merkle math | APTO | Unit publisher + `validate-merkle-proof.py` smoke 5 hojas |
| DoD 50→1 tx testnet | PENDIENTE | Requiere CI/lab con relay; lab-simulate demuestra 1 digest/lote |
| ZKP | N/A | Fuera de alcance |
| PBI archive | PENDIENTE | `pbi_archived: false` hasta Done en rama |

## Build local (2026-08-10)

```text
cargo build -p execute-process -p sddia-daemon-runtime -p iota-immutable-publisher -p event-watcher
cargo test -p iota-immutable-publisher
SMOKE_OK validate-merkle-proof.py (5 hojas)
```

---
feature_name: ocean-dpp-v2
created: "2026-08-10"
process: feature
phase: Auditoría pre-merge
agent: argos
branch: feat/ocean-dpp-v2-6416012709005756507
persist_ref: docs/features/ocean-dpp-v2
pr_url: https://github.com/racso80es/SddIA/pull/160
verdict: NO_APTO
---

# Auditoría — Ocean DPP V2 (PR #160)

## Contexto

- Rama: `feat/ocean-dpp-v2-6416012709005756507`
- Merge `main` → feature: `f302bc8` (2026-08-10)
- CI post-merge: PASS (índice / smokes EDA)
- Auditor: Tekton/Argos bajo mandato del Vértice Biológico

## Veredicto inicial

**NO_APTO** — CI verde no implica DoD. Fracturas de digest, idempotencia, sweeper y documentación.

## Hallazgos

| ID | Severidad | Hallazgo |
|----|-----------|----------|
| H1 | **P0** | Tras anclar Merkle, `route_domain_event(..., batch_mode_iota=true)` sobrescribe `transaction_digest` con literal `"batched-digest"`. Evidencia DLT destruida. |
| H2 | **P0** | Gate anti-doble-gasto mira `cumulo` / `cumulo.iota-immutable-publisher`, pero la fase pre-ancla solo escribe `transaction_digest` → re-ancla si hay crash intermedio. |
| H3 | **P0** | `sweep_once` dejó de purgar pending y reinvoca `route-domain-event` en chunks de 50. Confunde chunk físico con batch semántico (Dogma del Despertador). `SweepReport` vacío; `emit_kaizen_alert` dead. |
| H4 | **P1** | `docs/features/ocean-dpp-v2/` borrada en fix Jules; sin `validacion.md` / evolution / PBI archivado. |
| H5 | **P1** | `.SddIA/proofs/` hardcodeado; ausente en `cumulo.paths.json`. |
| H6 | **P2** | `BATCH_MODE` thread_local leído pero nunca seteado (código muerto). |
| H7 | **P2** | Tests publisher sin caso array/Merkle; watcher aún unitario → DoD «50→1 tx» no ejercitado en runtime. |
| H8 | Info | ZKP (PBI §1.2) fuera de alcance declarado. |

## Evidencia positiva (retenida)

- `rs_merkle` + array payload: root único + proofs por hoja.
- `validate-merkle-proof.py`: 3/3 APTO en smoke lab-simulate (2026-08-10).
- Norma DEP presente.
- Contratos I/O admiten arrays.

## Criterios de cierre (post-corrección)

1. Digest real de la tx Merkle persiste en `delivery_state` de cada evento del lote.
2. Idempotencia: evento con digest/merkle válido no re-ancla.
3. Sweeper restaura `try_sweep_event` + reporte; batch semántico solo en `route_domain_batch`.
4. Watcher puede delegar lotes (`event_file_paths`, ≤50) sin mezclar purge.
5. `eda_instance.proofs` en Cúmulo; proofs leen SSOT.
6. Topología `docs/features/ocean-dpp-v2/` + evolution + validacion.

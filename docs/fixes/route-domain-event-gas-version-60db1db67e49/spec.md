---
feature_name: route-domain-event-gas-version-60db1db67e49
created: "2026-09-09"
process: bug-fix
base: main
scope: dlt-gas-version-mutex-mayeuta
branch_name: fix/route-domain-event-gas-version-60db1db67e49
persist_ref: docs/fixes/route-domain-event-gas-version-60db1db67e49
pbi_ref: docs/todos/pending/[FIX] route-domain-event — fractura sistémica (60db1db67e49).md
document_id: PBI-FIX-FRACTURE-60db1db67e49
execution_id: "468e7f10-99aa-4c91-8803-e06e229742a3"
---

# Especificación — fractura `60db1db67e49` (gas version ≠ colapso)

## Problema

`route-domain-event` `merkle-batch-preseal` abortó con Object Version mismatch sobre la gas coin `0x93e4c1ee…` (v115 digest `5fX2xzFe…` vs current 116). Racer on-chain: `8Z9h67wr…` (115→116). `ANu3L1yn…` es anclaje posterior (116→117), no el racer.

Defectos:

| ID | Defecto |
|----|---------|
| F1 | `server.mjs` atiende `POST /v1/publish` en paralelo sin cola; misma wallet / misma gas coin. |
| F2 | `dlt_transient_network_trace` no cubre `is not available for consumption` ∧ `current version:` → `emit_dlt_batch_fracture` escala a Kintsugi aunque `dlt_reanchor` absorba. |
| F3 | Catch-all Mayeuta `failed` → `prompt_adjustment` (ELF fósil en el sello). Fuente post-41717: cubo DLT genérico diagnostica transporte. |

Laudo `L-ENRICH-KINTSUGI-DETERMINISTA` intacto. `mayeuta-llm` = `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION` (fuera).

## Cambio requerido

### Relay — cola serial (`publish-queue.mjs`)

`createSerialQueue()`: un `publishImmutableData` a la vez; `waitForTransaction` de la TX precedente antes de construir la siguiente. `server.mjs` envuelve el publish en esa cola.

### Core — predicado estrecho

`dlt_transient_gas_version_trace`: `is not available for consumption` **y** `current version:` (case-insensitive). `emit_dlt_batch_fracture` suprime si red (41717, intacto) **o** gas-version. Wrapper `issues with transaction inputs` **sin** esa firma **sí** emite. `config-missing` sí emite.

### Mayeuta nativo

- Retirar `failed` del catch-all. `{acción} failed:` sin cubo → fallback laudo humano / `process_fix`.
- Subtipar cubo DLT: gas/inputs vs transporte (`err.cause` red) vs opaco (sin afirmar transporte).
- Cero `llm:interact` en `enrich_fracture_pbi_kaizen.rs` (MAYEUTA-CA7).
- Bump `enrich-fracture-pbi-kaizen.md` vía `entity-manager` (DA-2).

## Criterios de aceptación

PBI §8: DLT-GAS-CA1..CA4, MAYEUTA-CA5..CA7.

## Fuera de alcance

- Reabrir taxonomía b3a715, cause-propagation a90fad, predicado **red** 41717.
- `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION`.
- Retry/sleep/backoff (DA-5). Bypass raw. Simular IOTA como Done.
- Mutar genoma `actions/` a mano.
